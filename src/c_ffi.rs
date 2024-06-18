use crate::benchmark_rust as benchmark_rust_inner;
use crate::init_logger as init_logger_inner;
use crate::results::{Error, FFIError};
use crate::Vec3 as Vec3Inner;
#[cfg(feature = "interoptopus")]
use interoptopus::ffi_function;
use interoptopus::ffi_service;
use interoptopus::ffi_service_ctor;
use interoptopus::ffi_service_method;
#[cfg(feature = "pyo3")]
use pyo3_helper_macros::py3_bind_pub;

/// A basic vector 3 in our FFI layer using integers.
#[cfg(feature = "interoptopus")]
#[interoptopus::ffi_type(opaque)]
#[repr(transparent)]
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all))]
pub struct Vec3 {
    inner: Vec3Inner,
}

#[cfg(feature = "interoptopus")]
#[ffi_function]
#[no_mangle]
pub extern "C" fn init_logger() {
    init_logger_inner()
}

#[cfg(feature = "interoptopus")]
#[ffi_function]
#[no_mangle]
pub extern "C" fn benchmark_rust() -> f64 {
    benchmark_rust_inner()
}

#[cfg(feature = "interoptopus")]
#[ffi_service(error = "FFIError", prefix = "vec3_")]
#[cfg_attr(feature = "pyo3", py3_bind_pub)]
impl Vec3 {
    #[ffi_service_ctor]
    pub fn new(x: i32, y: i32, z: i32) -> Result<Self, Error> {
        Ok(Vec3 {
            inner: Vec3Inner::new(x, y, z),
        })
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn add(&mut self, x: i32, y: i32, z: i32) -> i32 {
        self.inner.add(x, y, z)
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn add_reverse_args(&mut self, x: i32, y: i32, z: i32) -> i32 {
        self.inner.add_reverse_args(x, y, z)
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn dot(&self, other: &Vec3) -> i32 {
        self.inner.dot(&other.inner)
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            inner: self.inner.cross(&other.inner),
        }
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn normalize(&self) -> Vec3 {
        Vec3 {
            inner: self.inner.normalize(),
        }
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn get_x(&self) -> i32 {
        self.inner.x
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn get_y(&self) -> i32 {
        self.inner.y
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn get_z(&self) -> i32 {
        self.inner.z
    }
}
