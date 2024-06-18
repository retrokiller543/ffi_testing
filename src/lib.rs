#![allow(clippy::redundant_locals)]

use std::time::Instant;

use aes::Aes128;
use cipher::{BlockEncrypt, KeyInit};

#[cfg(feature = "jni")]
pub mod hello_jni;

#[cfg(all(feature = "c-ffi", feature = "interoptopus"))]
pub mod c_ffi;
#[cfg(feature = "interoptopus")]
pub mod interop;
#[cfg(feature = "interoptopus")]
pub mod results;
#[cfg(feature = "interoptopus")]
pub mod threads_example;
#[cfg(feature = "wasm-bindgen")]
pub mod wasm;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
use crate::wasm::set_panic_hook;

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
pub fn benchmark_rust() -> f64 {
    const ITERATIONS: usize = 100_000;

    // AES encryption key and data
    let key = [0u8; 16];
    let data = [0u8; 16];

    // Start the clock
    let start = Instant::now();

    // Perform the AES encryption iteratively in a single thread
    let cipher = Aes128::new(&key.into());
    let block = data;
    for _ in 0..ITERATIONS {
        cipher.encrypt_block(&mut block.into());
    }

    // Stop the clock
    let end = Instant::now();

    //println!("Rust (single-threaded with AES encryption): {} seconds", );
    (end - start).as_secs_f64()
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
pub fn init_logger() {
    env_logger::init();
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(any(feature = "interoptopus", feature = "c-ffi"), repr(C))]
#[cfg_attr(
    any(feature = "interoptopus", feature = "c-ffi"),
    interoptopus::ffi_type
)]
/// The basic struct we will call methods in our FFI layer.
pub struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Vec3 {
        #[cfg(feature = "wasm-bindgen")]
        set_panic_hook();
        Vec3 { x, y, z }
    }
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
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
