use std::{collections::HashMap, mem::swap};

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Abi, FnArg, Ident, ItemFn, Pat, Stmt, Type, TypePtr, parse_macro_input, token::Unsafe};

/// Attribute to make game function easily
///
/// Only appliable to functions
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
                    } else if type_name != "i32" && type_name != "f32" && type_name != "bool" {
                        panic!(
                            "You can't use the `{}` type",
                            pattern.ty.to_token_stream().to_string()
                        )
                    }
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
