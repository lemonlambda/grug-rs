use grug_rs::{Arguments, Grug, GrugValue};

use anyhow::Result;
use grug_rs_proc_macro::{error_handler, game_function};

fn main() -> Result<()> {
    // Initializes grug
    let grug = Grug::new(
        Some(custom_error_handler),
        "./examples/hello_world/mod_api.json",
        "./examples/hello_world/mods",
        "./examples/hello_world/mods_dll",
        1000,
    )?;

    let mut args = Arguments::new(vec![GrugValue::String("hello, world".to_string())]);
    grug.activate_on_function("World", "on_update", &mut Arguments::empty())?;
    Ok(())
}

#[error_handler]
fn custom_error_handler(
    reason: String,
    ty: GrugRuntimeError,
    on_fn_name: String,
    on_fn_path: String,
) {
    eprintln!(
        "Grug runtime error: {}\n  at {} ({})",
        reason, on_fn_name, on_fn_path
    );
}

#[game_function]
fn println(message: String) {
    println!("{message}");
}
