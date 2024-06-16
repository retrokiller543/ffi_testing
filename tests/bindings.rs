use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    Generator::new(
        Config {
            class: "InteropClass".to_string(),
            dll_name: "ffi_lib".to_string(),
            namespace_mappings: NamespaceMappings::new("ffi_lib.testing"),
            ..Config::default()
        },
        ffi_lib::interop::my_inventory(),
    )
    .write_file("bindings/csharp/Interop.cs")?;

    Ok(())
}

#[test]
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
    .write_file("bindings/c/ffi_lib.h")?;

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_cpython_cffi() -> Result<(), Error> {
    use interoptopus_backend_cpython::{Config, Generator};

    let library = ffi_lib::interop::my_inventory();
    Generator::new(Config::default(), library).write_file("bindings/python/ffi_lib.py")?;

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_c_custom() -> Result<(), Error> {
    use interoptopus_backend_c::{Config, Generator};

    Generator::new(
        Config {
            ifndef: "ffi_lib".to_string(),
            ..Config::default()
        },
        ffi_lib::interop::my_inventory(),
    )
    .write_file("bindings/c_v2/ffi_lib.h")?;

    Ok(())
}