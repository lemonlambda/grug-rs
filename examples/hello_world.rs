use grug_rs::Grug;

use anyhow::Result;
use grug_rs_proc_macro::game_function;

fn main() -> Result<()> {
    // Initializes grug
    let grug = Grug::new(
        "./examples/mod_api.json",
        "./examples/mods",
        "./examples/mods_dll",
        1000,
    )?;

    loop {
        grug.activate_on_function("World", "on_update")?;
    }
}

#[game_function]
fn println(message: String) {
    println!("{message}");
}
