use interoptopus::{Error, Interop};

#[cfg_attr(miri, ignore)]
fn bindings_c() -> Result<(), Error> {
    use interoptopus_custom_c_backend::{Config, Generator};

    Generator::new(
        Config {
            ifndef: "ffi_lib".to_string(),
            cpp_compatibility: true,
            namespace_id: "ffi_lib".to_string(),
            ..Config::default()
        },
        ffi_lib::interop::my_inventory(),
    )
    .write_file("bindings/c/ffi_lib.hpp")?;

    Ok(())
}

fn main() {
    bindings_c().unwrap();
}
