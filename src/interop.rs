use std::ffi::c_char;
use std::ffi::CStr;

use interoptopus::ffi_service;
use interoptopus::ffi_service_ctor;
use interoptopus::ffi_service_method;
use interoptopus::pattern;
use interoptopus::{ffi_function, ffi_type, function, Inventory, InventoryBuilder};
use log::info;

use crate::benchmark_rust;
use crate::init_logger;
use crate::threads_example::benchmark_rust_async;
use crate::vec3_new;
use crate::vec3::*;
use crate::results::{Error, FFIError};

/// A simple type in our FFI layer.
#[ffi_type]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Function using the type.
#[ffi_function]
#[no_mangle]
pub extern "C" fn my_function(input: Vec2) -> Vec2 {
    info!("my_function({:?})", input);
    input
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn hello(name: *const c_char) {
    info!("hello({:?})", name);
    if name.is_null() {
        println!("Hello, stranger!");
        return;
    }
    
    // Convert the raw pointer to a CStr
    let c_str = unsafe { CStr::from_ptr(name) };

    // Convert the CStr to a Rust string slice
    let str_slice = c_str.to_str().unwrap_or("Invalid UTF-8");

    // Print the message
    println!("Hello, {}!", str_slice);
}

#[ffi_type(opaque)]
#[repr(C)]
pub struct SimpleService { }

#[ffi_service(error = "FFIError", prefix = "simple_service_")]
impl SimpleService {

    #[ffi_service_ctor]
    pub fn new_with(_some_value: u32) -> Result<Self, Error> {
        info!("SimpleService::new_with({})", _some_value);
        Ok(Self { })
    }
}

#[ffi_type(opaque)]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[ffi_service(error = "FFIError", prefix = "vec4_")]
impl Vec4 {
    #[ffi_service_ctor]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Result<Self, Error> {
        info!("Vec4::new(x: {}, y: {}, z: {}, w: {})", x, y, z, w);
        Ok(Self { x, y, z, w })
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn dot(&self, other: &Vec4) -> f32 {
        info!("Vec4::dot({:?}, {:?})", self, other);
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn get_x(&self) -> f32 {
        self.x
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn get_y(&self) -> f32 {
        self.y
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn get_z(&self) -> f32 {
        self.z
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn get_w(&self) -> f32 {
        self.w
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn set_x(&mut self, value: f32) {
        self.x = value;
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn set_y(&mut self, value: f32) {
        self.y = value;
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn set_z(&mut self, value: f32) {
        self.z = value;
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn set_w(&mut self, value: f32) {
        self.w = value;
    }
}

// This will create a function `my_inventory` which can produce
// an abstract FFI representation (called `Library`) for this crate.
pub fn my_inventory() -> Inventory {
    {
        InventoryBuilder::new()
        .register(function!(my_function))
        .register(function!(vec3_new))
        .register(function!(add))
        .register(function!(add_reverse_args))
        .register(function!(dot))
        .register(function!(cross))
        .register(function!(normalize))
        .register(function!(hello))
        .register(function!(benchmark_rust))
        .register(function!(benchmark_rust_async))
        .register(pattern!(SimpleService))
        .register(pattern!(Vec4))
        .register(function!(init_logger))
        .inventory()
    }
}