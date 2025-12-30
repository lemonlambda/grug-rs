use grug_rs::{Arguments, Grug, GrugValue};

use anyhow::Result;
use grug_rs_proc_macro::game_function;

fn main() -> Result<()> {
    // Initializes grug
    let grug = Grug::new(
        "./examples/hello_world/mod_api.json",
        "./examples/hello_world/mods",
        "./examples/hello_world/mods_dll",
        1000,
    )?;

    let mut args = Arguments::new(vec![GrugValue::String("hello, world".to_string())]);
    loop {
        grug.activate_on_function("World", "on_update", &mut Arguments::empty())?;
    }
    Ok(())
}

#[game_function]
fn println(message: String) {
    println!("{message}");
}
