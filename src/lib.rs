#[cfg(feature = "jni")]
pub mod hello_jni;

#[cfg(feature = "c-ffi")]
pub mod hello_c;

#[cfg(feature = "interoptopus")]
pub mod interop;
pub mod results;
pub mod threads_example;

use std::time::Instant;

use aes::Aes128;
use cipher::{BlockEncrypt, KeyInit};
#[cfg(feature = "c-ffi")]
use ffi::ffi;

use interoptopus::ffi_function;
use log::info;
#[cfg(feature = "pyo3")]
use pyo3_helper_macros::py3_bind_pub;


/// A basic vector 3 in our FFI layer using integers.
#[interoptopus::ffi_type]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all))]
pub struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn init_logger() {
    env_logger::init();
}

#[cfg_attr(feature = "pyo3", py3_bind_pub)]
impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3 { x, y, z }
    }
}

#[no_mangle]
#[ffi_function]
pub extern "C" fn vec3_new(x: i32, y: i32, z: i32) -> *mut Vec3 {
    info!("Vec3::new({:?}, {:?}, {:?})", x, y, z);
    Box::into_raw(Box::new(Vec3::new(x, y, z)))
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn benchmark_rust() {
    const ITERATIONS: usize = 100_000;

    // AES encryption key and data
    let key = [0u8; 16];
    let data = [0u8; 16];

    // Start the clock
    let start = Instant::now();

    // Perform the AES encryption iteratively in a single thread
    let cipher = Aes128::new(&key.into());
    let block = data.clone();
    for _ in 0..ITERATIONS {
        cipher.encrypt_block(&mut block.into());
    }

    // Stop the clock
    let end = Instant::now();

    println!("Rust (single-threaded with AES encryption): {} seconds", (end - start).as_secs_f64());
}

#[ffi(from_ptr, self_ty = "*mut Vec3")]
#[cfg_attr(feature = "pyo3", py3_bind_pub)]
impl Vec3 {
    #[ffi(arg(self), arg(rest))]
    #[ffi_function]
    pub fn add(&mut self, x: i32, y: i32, z: i32) -> i32 {
        self.x += x;
        self.y += y;
        self.z += z;
        self.x + self.y + self.z
    }

    #[ffi(arg(rest), arg(self))]
    #[ffi_function]
    pub fn add_reverse_args(&mut self, x: i32, y: i32, z: i32) -> i32 {
        self.add(x, y, z)
    }

    #[ffi(arg(self), arg(rest))]
    #[ffi_function]
    pub fn dot(&self, other: &Vec3) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[ffi(arg(self), arg(rest))]
    #[ffi_function]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[ffi(arg(self), arg(rest))]
    pub fn normalize(&self) -> Vec3 {
        let len = ((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt();
        Vec3 {
            x: (self.x as f64 / len) as i32,
            y: (self.y as f64 / len) as i32,
            z: (self.z as f64 / len) as i32,
        }
    }
}