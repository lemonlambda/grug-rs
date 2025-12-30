use grug_rs::{Arguments, Grug, GrugValue};

use anyhow::Result;
use grug_rs_proc_macro::game_function;

fn main() -> Result<()> {
    // Initializes grug
    let grug = Grug::new(
        "./examples/custom_type/mod_api.json",
        "./examples/custom_type/mods",
        "./examples/custom_type/mods_dll",
        1000,
    )?;

    #[allow(clippy::disallowed_names)]
    let mut foo = Foo { value: 10 };
    let mut args = Arguments::new(vec![GrugValue::custom(&mut foo)]);
    grug.activate_on_function("CustomType", "on_update", &mut args)?;
    Ok(())
}

#[repr(C)]
#[derive(Debug, Clone)]
struct Foo {
    value: i32,
}

#[game_function]
fn println_foo(message: &mut Foo) {
    println!("{message:?}");
}
