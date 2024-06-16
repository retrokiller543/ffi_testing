#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[cfg(feature = "c-ffi")]
pub mod hello_c {}
#[cfg(feature = "interoptopus")]
pub mod interop {
    use std::ffi::c_char;
    use std::ffi::CStr;
    use interoptopus::ffi_service;
    use interoptopus::ffi_service_ctor;
    use interoptopus::ffi_service_method;
    use interoptopus::pattern;
    use interoptopus::{ffi_function, ffi_type, function, Inventory, InventoryBuilder};
    use crate::benchmark_rust;
    use crate::vec3_new;
    use crate::vec3::*;
    use crate::results::{Error, FFIError};
    /// A simple type in our FFI layer.
    #[repr(C)]
    pub struct Vec2 {
        pub x: f32,
        pub y: f32,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Vec2 {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line(
                " A simple type in our FFI layer.",
            );
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Vec2".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "y".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    /// Function using the type.
    #[no_mangle]
    pub extern "C" fn my_function(input: Vec2) -> Vec2 {
        input
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct my_function {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for my_function {
        type Signature = extern "C" fn(Vec2) -> Vec2;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "input".to_string(),
                        <Vec2 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            doc_lines.push(" Function using the type.".to_string());
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <Vec2 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "my_function".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn hello(name: *const c_char) {
        if name.is_null() {
            {
                ::std::io::_print(format_args!("Hello, stranger!\n"));
            };
            return;
        }
        let c_str = unsafe { CStr::from_ptr(name) };
        let str_slice = c_str.to_str().unwrap_or("Invalid UTF-8");
        {
            ::std::io::_print(format_args!("Hello, {0}!\n", str_slice));
        };
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct hello {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for hello {
        type Signature = extern "C" fn(*const c_char);
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "name".to_string(),
                        <*const c_char as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                ::interoptopus::lang::c::CType::Primitive(
                    interoptopus::lang::c::PrimitiveType::Void,
                ),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new("hello".to_string(), signature, meta)
        }
    }
    #[repr(C)]
    pub struct SimpleService {}
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for SimpleService {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "{0}{1}",
                        "SimpleService".to_string(),
                        generics.join(""),
                    ),
                );
                res
            };
            let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
            ::interoptopus::lang::c::CType::Opaque(rval)
        }
    }
    impl SimpleService {
        pub fn new_with(some_value: u32) -> Result<Self, Error> {
            Ok(Self {})
        }
    }
    #[no_mangle]
    #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
    #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
    pub extern "C" fn simple_service_new_with(
        context: &mut *mut SimpleService,
        some_value: u32,
    ) -> FFIError {
        *context = ::std::ptr::null_mut();
        let result_result = std::panic::catch_unwind(
            ::std::panic::AssertUnwindSafe(|| { <SimpleService>::new_with(some_value) }),
        );
        match result_result {
            Ok(Ok(obj)) => {
                let boxed = ::std::boxed::Box::new(obj);
                let raw = ::std::boxed::Box::into_raw(boxed);
                *context = raw;
                <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS
            }
            Ok(Err(e)) => {
                ::interoptopus::util::log_error(|| {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Error in ({0}): {1:?}",
                            "simple_service_new_with",
                            e,
                        ),
                    );
                    res
                });
                e.into()
            }
            Err(e) => {
                ::interoptopus::util::log_error(|| {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Panic in ({0}): {1}",
                            "simple_service_new_with",
                            ::interoptopus::patterns::result::get_panic_message(
                                e.as_ref(),
                            ),
                        ),
                    );
                    res
                });
                <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct simple_service_new_with {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for simple_service_new_with {
        type Signature = extern "C" fn(&mut *mut SimpleService, u32) -> FFIError;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "context".to_string(),
                        <&mut *mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "some_value".to_string(),
                        <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "simple_service_new_with".to_string(),
                signature,
                meta,
            )
        }
    }
    /// Destroys the given instance.
    ///
    /// # Safety
    ///
    /// The passed parameter MUST have been created with the corresponding init function;
    /// passing any other value results in undefined behavior.
    #[allow(unused_mut, unsafe_op_in_unsafe_fn, unused_unsafe)]
    #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
    #[no_mangle]
    pub unsafe extern "C" fn simple_service_destroy(
        context: &mut *mut SimpleService,
    ) -> FFIError {
        if context.is_null() {
            return <FFIError as ::interoptopus::patterns::result::FFIError>::NULL;
        }
        let result_result = ::std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                unsafe { drop(::std::boxed::Box::from_raw(*context)) };
            }),
        );
        *context = ::std::ptr::null_mut();
        match result_result {
            Ok(_) => <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS,
            Err(e) => {
                ::interoptopus::util::log_error(|| {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Panic in ({0}): {1}",
                            "simple_service_destroy",
                            ::interoptopus::patterns::result::get_panic_message(
                                e.as_ref(),
                            ),
                        ),
                    );
                    res
                });
                <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct simple_service_destroy {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for simple_service_destroy {
        type Signature = extern "C" fn(&mut *mut SimpleService) -> FFIError;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "context".to_string(),
                        <&mut *mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            doc_lines.push(" Destroys the given instance.".to_string());
            doc_lines.push("".to_string());
            doc_lines.push(" # Safety".to_string());
            doc_lines.push("".to_string());
            doc_lines
                .push(
                    " The passed parameter MUST have been created with the corresponding init function;"
                        .to_string(),
                );
            doc_lines
                .push(
                    " passing any other value results in undefined behavior.".to_string(),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "simple_service_destroy".to_string(),
                signature,
                meta,
            )
        }
    }
    impl ::interoptopus::patterns::LibraryPatternInfo for SimpleService {
        fn pattern_info() -> ::interoptopus::patterns::LibraryPattern {
            use ::interoptopus::lang::rust::CTypeInfo;
            use ::interoptopus::lang::rust::FunctionInfo;
            let mut methods = Vec::new();
            let mut ctors = Vec::new();
            {
                use simple_service_new_with as x;
                ctors.push(x::function_info());
            }
            let dtor = {
                use simple_service_destroy as x;
                x::function_info()
            };
            let service = ::interoptopus::patterns::service::Service::new(
                ctors,
                dtor,
                methods,
            );
            service.assert_valid();
            ::interoptopus::patterns::LibraryPattern::Service(service)
        }
    }
    #[repr(C)]
    pub struct Vec4 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Vec4 {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Vec4".to_string(), generics.join("")),
                );
                res
            };
            let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
            ::interoptopus::lang::c::CType::Opaque(rval)
        }
    }
    impl Vec4 {
        pub fn new(x: f32, y: f32, z: f32, w: f32) -> Result<Self, Error> {
            Ok(Self { x, y, z, w })
        }
        pub fn dot(&self, other: &Vec4) -> f32 {
            self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
        }
    }
    #[no_mangle]
    #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
    #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
    pub extern "C" fn vec4_new(
        context: &mut *mut Vec4,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> FFIError {
        *context = ::std::ptr::null_mut();
        let result_result = std::panic::catch_unwind(
            ::std::panic::AssertUnwindSafe(|| { <Vec4>::new(x, y, z, w) }),
        );
        match result_result {
            Ok(Ok(obj)) => {
                let boxed = ::std::boxed::Box::new(obj);
                let raw = ::std::boxed::Box::into_raw(boxed);
                *context = raw;
                <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS
            }
            Ok(Err(e)) => {
                ::interoptopus::util::log_error(|| {
                    let res = ::alloc::fmt::format(
                        format_args!("Error in ({0}): {1:?}", "vec4_new", e),
                    );
                    res
                });
                e.into()
            }
            Err(e) => {
                ::interoptopus::util::log_error(|| {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Panic in ({0}): {1}",
                            "vec4_new",
                            ::interoptopus::patterns::result::get_panic_message(
                                e.as_ref(),
                            ),
                        ),
                    );
                    res
                });
                <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct vec4_new {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for vec4_new {
        type Signature = extern "C" fn(&mut *mut Vec4, f32, f32, f32, f32) -> FFIError;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "context".to_string(),
                        <&mut *mut Vec4 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "y".to_string(),
                        <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "z".to_string(),
                        <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "w".to_string(),
                        <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "vec4_new".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
    #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
    pub extern "C" fn vec4_dot(context: &Vec4, other: &Vec4) -> f32 {
        let result_result = ::std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let context = context;
                let other = other;
                <Vec4>::dot(context, other)
            }),
        );
        match result_result {
            Ok(x) => x,
            Err(e) => {
                ::interoptopus::util::log_error(|| {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Panic in ({0}): {1}",
                            "vec4_dot",
                            ::interoptopus::patterns::result::get_panic_message(
                                e.as_ref(),
                            ),
                        ),
                    );
                    res
                });
                <f32>::default()
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct vec4_dot {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for vec4_dot {
        type Signature = extern "C" fn(&Vec4, &Vec4) -> f32;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "context".to_string(),
                        <&Vec4 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "other".to_string(),
                        <&Vec4 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "vec4_dot".to_string(),
                signature,
                meta,
            )
        }
    }
    /// Destroys the given instance.
    ///
    /// # Safety
    ///
    /// The passed parameter MUST have been created with the corresponding init function;
    /// passing any other value results in undefined behavior.
    #[allow(unused_mut, unsafe_op_in_unsafe_fn, unused_unsafe)]
    #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
    #[no_mangle]
    pub unsafe extern "C" fn vec4_destroy(context: &mut *mut Vec4) -> FFIError {
        if context.is_null() {
            return <FFIError as ::interoptopus::patterns::result::FFIError>::NULL;
        }
        let result_result = ::std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                unsafe { drop(::std::boxed::Box::from_raw(*context)) };
            }),
        );
        *context = ::std::ptr::null_mut();
        match result_result {
            Ok(_) => <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS,
            Err(e) => {
                ::interoptopus::util::log_error(|| {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Panic in ({0}): {1}",
                            "vec4_destroy",
                            ::interoptopus::patterns::result::get_panic_message(
                                e.as_ref(),
                            ),
                        ),
                    );
                    res
                });
                <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct vec4_destroy {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for vec4_destroy {
        type Signature = extern "C" fn(&mut *mut Vec4) -> FFIError;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "context".to_string(),
                        <&mut *mut Vec4 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            doc_lines.push(" Destroys the given instance.".to_string());
            doc_lines.push("".to_string());
            doc_lines.push(" # Safety".to_string());
            doc_lines.push("".to_string());
            doc_lines
                .push(
                    " The passed parameter MUST have been created with the corresponding init function;"
                        .to_string(),
                );
            doc_lines
                .push(
                    " passing any other value results in undefined behavior.".to_string(),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "vec4_destroy".to_string(),
                signature,
                meta,
            )
        }
    }
    impl ::interoptopus::patterns::LibraryPatternInfo for Vec4 {
        fn pattern_info() -> ::interoptopus::patterns::LibraryPattern {
            use ::interoptopus::lang::rust::CTypeInfo;
            use ::interoptopus::lang::rust::FunctionInfo;
            let mut methods = Vec::new();
            let mut ctors = Vec::new();
            {
                use vec4_dot as x;
                methods.push(x::function_info());
            }
            {
                use vec4_new as x;
                ctors.push(x::function_info());
            }
            let dtor = {
                use vec4_destroy as x;
                x::function_info()
            };
            let service = ::interoptopus::patterns::service::Service::new(
                ctors,
                dtor,
                methods,
            );
            service.assert_valid();
            ::interoptopus::patterns::LibraryPattern::Service(service)
        }
    }
    pub fn my_inventory() -> Inventory {
        {
            InventoryBuilder::new()
                .register({
                    use ::interoptopus::lang::rust::FunctionInfo;
                    let info = <my_function>::function_info();
                    ::interoptopus::Symbol::Function(info)
                })
                .register({
                    use ::interoptopus::lang::rust::FunctionInfo;
                    let info = <vec3_new>::function_info();
                    ::interoptopus::Symbol::Function(info)
                })
                .register({
                    use ::interoptopus::lang::rust::FunctionInfo;
                    let info = <add>::function_info();
                    ::interoptopus::Symbol::Function(info)
                })
                .register({
                    use ::interoptopus::lang::rust::FunctionInfo;
                    let info = <add_reverse_args>::function_info();
                    ::interoptopus::Symbol::Function(info)
                })
                .register({
                    use ::interoptopus::lang::rust::FunctionInfo;
                    let info = <dot>::function_info();
                    ::interoptopus::Symbol::Function(info)
                })
                .register({
                    use ::interoptopus::lang::rust::FunctionInfo;
                    let info = <cross>::function_info();
                    ::interoptopus::Symbol::Function(info)
                })
                .register({
                    use ::interoptopus::lang::rust::FunctionInfo;
                    let info = <normalize>::function_info();
                    ::interoptopus::Symbol::Function(info)
                })
                .register({
                    use ::interoptopus::lang::rust::FunctionInfo;
                    let info = <hello>::function_info();
                    ::interoptopus::Symbol::Function(info)
                })
                .register({
                    use ::interoptopus::lang::rust::FunctionInfo;
                    let info = <benchmark_rust>::function_info();
                    ::interoptopus::Symbol::Function(info)
                })
                .register({
                    let info: ::interoptopus::patterns::LibraryPattern = <SimpleService as ::interoptopus::patterns::LibraryPatternInfo>::pattern_info();
                    ::interoptopus::Symbol::Pattern(info)
                })
                .register({
                    let info: ::interoptopus::patterns::LibraryPattern = <Vec4 as ::interoptopus::patterns::LibraryPatternInfo>::pattern_info();
                    ::interoptopus::Symbol::Pattern(info)
                })
                .inventory()
        }
    }
}
pub mod results {
    use interoptopus::ffi_type;
    use std::fmt::{Display, Formatter};
    #[repr(C)]
    pub enum FFIError {
        Ok = 0,
        Null = 100,
        Panic = 200,
        Fail = 300,
    }
    unsafe impl ::interoptopus::lang::rust::VariantInfo for FFIError {
        fn variant_info(&self) -> ::interoptopus::lang::c::Variant {
            match self {
                Self::Ok => {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    ::interoptopus::lang::c::Variant::new(
                        "Ok".to_string(),
                        0i32 as usize,
                        documentation,
                    )
                }
                Self::Null => {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    ::interoptopus::lang::c::Variant::new(
                        "Null".to_string(),
                        100i32 as usize,
                        documentation,
                    )
                }
                Self::Panic => {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    ::interoptopus::lang::c::Variant::new(
                        "Panic".to_string(),
                        200i32 as usize,
                        documentation,
                    )
                }
                Self::Fail => {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    ::interoptopus::lang::c::Variant::new(
                        "Fail".to_string(),
                        300i32 as usize,
                        documentation,
                    )
                }
            }
        }
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for FFIError {
        fn type_info() -> ::interoptopus::lang::c::CType {
            use ::interoptopus::lang::rust::VariantInfo;
            let mut variants = ::std::vec::Vec::new();
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            {
                variants.push(Self::Ok.variant_info());
            }
            {
                variants.push(Self::Null.variant_info());
            }
            {
                variants.push(Self::Panic.variant_info());
            }
            {
                variants.push(Self::Fail.variant_info());
            }
            let rval = ::interoptopus::lang::c::EnumType::new(
                "FFIError".to_string(),
                variants,
                meta,
            );
            use ::interoptopus::patterns::result::FFIError as _;
            let success_variant = Self::SUCCESS.variant_info();
            let the_success_enum = ::interoptopus::patterns::result::FFIErrorEnum::new(
                rval,
                success_variant,
            );
            let the_pattern = ::interoptopus::patterns::TypePattern::FFIErrorEnum(
                the_success_enum,
            );
            ::interoptopus::lang::c::CType::Pattern(the_pattern)
        }
    }
    pub enum Error {
        Bad,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Bad")
        }
    }
    impl From<Error> for FFIError {
        fn from(x: Error) -> Self {
            match x {
                Error::Bad => Self::Fail,
            }
        }
    }
    impl Default for FFIError {
        fn default() -> Self {
            Self::Ok
        }
    }
    impl interoptopus::patterns::result::FFIError for FFIError {
        const SUCCESS: Self = Self::Ok;
        const NULL: Self = Self::Null;
        const PANIC: Self = Self::Panic;
    }
    impl Display for Error {
        fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }
    impl std::error::Error for Error {}
}
#[cfg(feature = "c-ffi")]
use ffi::ffi;
use interoptopus::ffi_function;
/// A basic vector 3 in our FFI layer using integers.
#[repr(C)]
pub struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Vec3",
            "x",
            &self.x,
            "y",
            &self.y,
            "z",
            &&self.z,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Vec3 {
    #[inline]
    fn clone(&self) -> Vec3 {
        Vec3 {
            x: ::core::clone::Clone::clone(&self.x),
            y: ::core::clone::Clone::clone(&self.y),
            z: ::core::clone::Clone::clone(&self.z),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Vec3 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Vec3 {
    #[inline]
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Vec3 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i32>;
    }
}
unsafe impl ::interoptopus::lang::rust::CTypeInfo for Vec3 {
    fn type_info() -> ::interoptopus::lang::c::CType {
        let documentation = ::interoptopus::lang::c::Documentation::from_line(
            " A basic vector 3 in our FFI layer using integers.",
        );
        let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
            "".to_string(),
            documentation,
            None,
        );
        let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
        let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
        let name = {
            let res = ::alloc::fmt::format(
                format_args!("{0}{1}", "Vec3".to_string(), generics.join("")),
            );
            res
        };
        {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let the_type = <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
            let field = ::interoptopus::lang::c::Field::with_documentation(
                "x".to_string(),
                the_type,
                interoptopus::lang::c::Visibility::Private,
                documentation,
            );
            fields.push(field);
        }
        {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let the_type = <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
            let field = ::interoptopus::lang::c::Field::with_documentation(
                "y".to_string(),
                the_type,
                interoptopus::lang::c::Visibility::Private,
                documentation,
            );
            fields.push(field);
        }
        {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let the_type = <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
            let field = ::interoptopus::lang::c::Field::with_documentation(
                "z".to_string(),
                the_type,
                interoptopus::lang::c::Visibility::Private,
                documentation,
            );
            fields.push(field);
        }
        let rval = ::interoptopus::lang::c::CompositeType::with_meta(name, fields, meta);
        ::interoptopus::lang::c::CType::Composite(rval)
    }
}
impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3 { x, y, z }
    }
}
#[no_mangle]
pub extern "C" fn vec3_new(x: i32, y: i32, z: i32) -> *mut Vec3 {
    Box::into_raw(Box::new(Vec3::new(x, y, z)))
}
#[allow(non_camel_case_types)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) struct vec3_new {}
unsafe impl ::interoptopus::lang::rust::FunctionInfo for vec3_new {
    type Signature = extern "C" fn(i32, i32, i32) -> *mut Vec3;
    fn function_info() -> ::interoptopus::lang::c::Function {
        let mut doc_lines = ::std::vec::Vec::new();
        let mut params = ::std::vec::Vec::new();
        params
            .push(
                ::interoptopus::lang::c::Parameter::new(
                    "x".to_string(),
                    <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                ),
            );
        params
            .push(
                ::interoptopus::lang::c::Parameter::new(
                    "y".to_string(),
                    <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                ),
            );
        params
            .push(
                ::interoptopus::lang::c::Parameter::new(
                    "z".to_string(),
                    <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                ),
            );
        let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
            params,
            <*mut Vec3 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
        );
        let documentation = ::interoptopus::lang::c::Documentation::from_lines(
            doc_lines,
        );
        let meta = ::interoptopus::lang::c::Meta::with_documentation(
            documentation,
            None,
        );
        ::interoptopus::lang::c::Function::new("vec3_new".to_string(), signature, meta)
    }
}
#[no_mangle]
pub extern "C" fn benchmark_rust() {
    const ITERATIONS: usize = 100_000;
    let v1 = Vec3::new(1, 2, 3);
    let v2 = Vec3::new(4, 5, 6);
    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        v1.dot(&v2);
        v1.cross(&v2);
        v1.normalize();
    }
    let end = std::time::Instant::now();
    {
        ::std::io::_print(
            format_args!("Rust: {0} seconds\n", (end - start).as_secs_f64()),
        );
    };
}
#[allow(non_camel_case_types)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) struct benchmark_rust {}
unsafe impl ::interoptopus::lang::rust::FunctionInfo for benchmark_rust {
    type Signature = extern "C" fn();
    fn function_info() -> ::interoptopus::lang::c::Function {
        let mut doc_lines = ::std::vec::Vec::new();
        let mut params = ::std::vec::Vec::new();
        let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
            params,
            ::interoptopus::lang::c::CType::Primitive(
                interoptopus::lang::c::PrimitiveType::Void,
            ),
        );
        let documentation = ::interoptopus::lang::c::Documentation::from_lines(
            doc_lines,
        );
        let meta = ::interoptopus::lang::c::Meta::with_documentation(
            documentation,
            None,
        );
        ::interoptopus::lang::c::Function::new(
            "benchmark_rust".to_string(),
            signature,
            meta,
        )
    }
}
impl Vec3 {
    pub fn add(&mut self, x: i32, y: i32, z: i32) -> i32 {
        self.x += x;
        self.y += y;
        self.z += z;
        self.x + self.y + self.z
    }
    pub fn add_reverse_args(&mut self, x: i32, y: i32, z: i32) -> i32 {
        self.add(x, y, z)
    }
    pub fn dot(&self, other: &Vec3) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn normalize(&self) -> Vec3 {
        let len = ((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt();
        Vec3 {
            x: (self.x as f64 / len) as i32,
            y: (self.y as f64 / len) as i32,
            z: (self.z as f64 / len) as i32,
        }
    }
}
impl From<*mut Vec3> for &'static mut Vec3 {
    fn from(value: *mut Vec3) -> Self {
        let ptr: *mut Vec3 = value as *mut Vec3;
        unsafe { &mut *ptr }
    }
}
impl From<*mut Vec3> for &'static Vec3 {
    fn from(value: *mut Vec3) -> Self {
        let ptr: *mut Vec3 = value as *mut Vec3;
        unsafe { &*ptr }
    }
}
pub mod vec3 {
    use super::*;
    #[no_mangle]
    pub extern "C" fn add(slf: *mut Vec3, x: i32, y: i32, z: i32) -> i32 {
        Into::<&mut Vec3>::into(slf).add(x, y, z)
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct add {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for add {
        type Signature = extern "C" fn(*mut Vec3, i32, i32, i32) -> i32;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "slf".to_string(),
                        <*mut Vec3 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "y".to_string(),
                        <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "z".to_string(),
                        <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new("add".to_string(), signature, meta)
        }
    }
    #[no_mangle]
    pub extern "C" fn add_reverse_args(x: i32, y: i32, z: i32, slf: *mut Vec3) -> i32 {
        Into::<&mut Vec3>::into(slf).add_reverse_args(x, y, z)
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct add_reverse_args {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for add_reverse_args {
        type Signature = extern "C" fn(i32, i32, i32, *mut Vec3) -> i32;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "y".to_string(),
                        <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "z".to_string(),
                        <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "slf".to_string(),
                        <*mut Vec3 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "add_reverse_args".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn dot(slf: *mut Vec3, other: &Vec3) -> i32 {
        Into::<&Vec3>::into(slf).dot(other)
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct dot {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for dot {
        type Signature = extern "C" fn(*mut Vec3, &Vec3) -> i32;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "slf".to_string(),
                        <*mut Vec3 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "other".to_string(),
                        <&Vec3 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new("dot".to_string(), signature, meta)
        }
    }
    #[no_mangle]
    pub extern "C" fn cross(slf: *mut Vec3, other: &Vec3) -> Vec3 {
        Into::<&Vec3>::into(slf).cross(other)
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct cross {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for cross {
        type Signature = extern "C" fn(*mut Vec3, &Vec3) -> Vec3;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "slf".to_string(),
                        <*mut Vec3 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "other".to_string(),
                        <&Vec3 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <Vec3 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new("cross".to_string(), signature, meta)
        }
    }
    #[no_mangle]
    pub extern "C" fn normalize(slf: *mut Vec3) -> Vec3 {
        Into::<&Vec3>::into(slf).normalize()
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct normalize {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for normalize {
        type Signature = extern "C" fn(*mut Vec3) -> Vec3;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "slf".to_string(),
                        <*mut Vec3 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <Vec3 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "normalize".to_string(),
                signature,
                meta,
            )
        }
    }
}
