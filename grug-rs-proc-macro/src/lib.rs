use std::{collections::HashMap, mem::swap};

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Abi, Block, FnArg, Ident, ItemFn, Pat, Stmt, Type, TypePtr, parse_macro_input,
    token::{Const, Star, Unsafe},
};

/// Attribute to make error handlers easily
///
/// # Example
/// ```
/// #[error_handler]
/// fn error_handler(reason: String, ty: GrugRuntimeError, on_fn_name: String, on_fn_path: String) {
///     eprintln!(
///         "Grug runtime error: {}\n  at {} ({})",
///         reason, on_fn_name, on_fn_path
///     );
/// }
/// ```
#[proc_macro_attribute]
pub fn error_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    assert!(
        input.sig.inputs.len() == 4,
        "Error handler takes 4 arguments"
    );

    let mut names = vec![];

    // Replace the types of each of them with the C types
    for (i, arg) in input.sig.inputs.iter_mut().enumerate() {
        match arg {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(pat_type) => {
                // Grab the name of the variable
                let var_name = {
                    if let Pat::Ident(ident) = *pat_type.pat.clone() {
                        ident.ident.to_string()
                    } else {
                        unreachable!()
                    }
                };

                names.push(var_name);

                let ty = grab_type_for_error_handler(i);
                swap(
                    &mut pat_type.ty,
                    &mut Box::new(parse_macro_input!(ty as Type)),
                );
            }
        }
    }

    let conversion = format!(
        "{{// Convert inputs safely
    let {0} = if !{0}.is_null() {{
        unsafe {{ std::ffi::CStr::from_ptr({0}).to_string_lossy() }}
    }} else {{
        \"<unknown>\".into()
    }}.to_string();

    let {1} = unsafe {{ std::mem::transmute::<_, grug_rs::GrugRuntimeError>({1}) }};

    let {2} = if !{2}.is_null() {{
        unsafe {{ std::ffi::CStr::from_ptr({2}).to_string_lossy() }}
    }} else {{
        \"<unknown>\".into()
    }}.to_string();

    let {3} = if !{3}.is_null() {{
        unsafe {{ std::ffi::CStr::from_ptr({3}).to_string_lossy() }}
    }} else {{
        \"<unknown>\".into()
    }}.to_string();
}}",
        names[0], names[1], names[2], names[3]
    )
    .parse()
    .unwrap();

    input.block.stmts = [
        parse_macro_input!(conversion as Block).stmts,
        input.block.stmts,
    ]
    .concat();

    let abi = "extern \"C\"".parse().unwrap();
    input.sig.abi = Some(parse_macro_input!(abi as Abi));

    input.sig.unsafety = Some(Unsafe::default());

    TokenStream::from(quote! {
        #input
    })
}

fn grab_type_for_error_handler(idx: usize) -> TokenStream {
    match idx {
        0 => "*const std::ffi::c_char",
        1 => "grug_rs::grug_sys::grug_runtime_error_type",
        2 => "*const std::ffi::c_char",
        3 => "*const std::ffi::c_char",
        _ => unreachable!(),
    }
    .parse()
    .unwrap()
}

/// Attribute to make game function easily
///
/// Only appliable to functions
///
/// # Example
/// ```
/// #[game_function]
/// fn println(message: String) {
///     println!("{message}");
/// }
/// ```
#[proc_macro_attribute]
pub fn game_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let args = &mut input.sig.inputs;

    let mut types = HashMap::new();

    for arg in args {
        if let FnArg::Typed(pattern) = arg {
            match *pattern.ty.clone() {
                Type::Path(type_path) => {
                    // Grab the name of the variable
                    let var_name = {
                        if let Pat::Ident(ident) = *pattern.pat.clone() {
                            ident.ident.to_string()
                        } else {
                            unreachable!()
                        }
                    };

                    let type_name = type_path.path.to_token_stream().to_string();

                    // Ensure it's valid type
                    if type_name == "String" {
                        types.insert(var_name, "String".to_string());

                        let c_string_type = "*const std::ffi::c_char".parse().unwrap();

                        swap(
                            &mut *pattern.ty,
                            &mut Type::Ptr(parse_macro_input!(c_string_type as TypePtr)),
                        );
                    }
                }
                Type::Reference(reference) => {
                    // Grab the name of the variable
                    let var_name = {
                        if let Pat::Ident(ident) = *pattern.pat.clone() {
                            ident.ident.to_string()
                        } else {
                            unreachable!()
                        }
                    };

                    if reference.mutability.is_some() {
                        types.insert(var_name, "PointerMut".to_string());
                    } else {
                        types.insert(var_name, "Pointer".to_string());
                    }

                    swap(
                        &mut *pattern.ty,
                        &mut Type::Ptr(TypePtr {
                            star_token: Star::default(),
                            const_token: reference
                                .mutability
                                .map_or(Some(Const::default()), |_| None),
                            mutability: reference.mutability,
                            elem: reference.elem,
                        }),
                    )
                }
                _ => panic!(
                    "You can't use the `{}` type",
                    pattern.ty.to_token_stream().to_string()
                ),
            }
        }
    }

    for (name, type_) in types {
        if type_ == "String" {
            // Only need to modify string types
            let to_string = format!(
                "    let {0} = if !{0}.is_null() {{
                unsafe {{ std::ffi::CStr::from_ptr({0}).to_string_lossy() }}
            }} else {{
                panic!(\"`{0}` is null.\")
            }};
        ",
                name
            )
            .parse()
            .unwrap();

            input
                .block
                .stmts
                .insert(0, parse_macro_input!(to_string as Stmt));
        } else if type_ == "Pointer" {
            let to_reference = format!(
                "let {0} = if !{0}.is_null() {{
                    unsafe {{ &*{0} }}
                }} else {{
                    panic!(\"`{0}` is null.\")
                }};",
                name
            )
            .parse()
            .unwrap();

            input
                .block
                .stmts
                .insert(0, parse_macro_input!(to_reference as Stmt));
        } else if type_ == "PointerMut" {
            let to_reference = format!(
                "let {0} = if !{0}.is_null() {{
                    unsafe {{ &mut *{0} }}
                }} else {{
                    panic!(\"`{0}` is null.\")
                }};",
                name
            )
            .parse()
            .unwrap();

            input
                .block
                .stmts
                .insert(0, parse_macro_input!(to_reference as Stmt));
        }
    }

    // Need to add `unsafe extern "C"` to the function
    input.sig.unsafety = Some(Unsafe::default());

    let abi = "extern \"C\"".parse().unwrap();

    input.sig.abi = Some(parse_macro_input!(abi as Abi));

    let ident = input.sig.ident.clone();
    let ident = quote! {
        #ident
    };
    // Rename the function to have `game_fn_` in front of it
    let ident = format!("game_fn_{}", ident.to_string()).parse().unwrap();

    input.sig.ident = parse_macro_input!(ident as Ident);

    TokenStream::from(quote! {
        #[unsafe(no_mangle)]
        #input
    })
}
