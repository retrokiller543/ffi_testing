#[cfg(feature = "jni")]
pub mod hello_jni;

#[cfg(feature = "c-ffi")]
pub mod hello_c;

#[cfg(feature = "c-ffi")]
use ffi::ffi;

#[cfg(feature = "pyo3")]
use pyo3_helper_macros::py3_bind_pub;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all))]
pub struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

#[cfg_attr(feature = "pyo3", py3_bind_pub)]
impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3 { x, y, z }
    }
}

#[no_mangle]
pub extern "C" fn vec3_new(x: i32, y: i32, z: i32) -> *mut Vec3 {
    Box::into_raw(Box::new(Vec3::new(x, y, z)))
}

#[ffi(from_ptr, self_ty = "*mut Vec3")]
#[cfg_attr(feature = "pyo3", py3_bind_pub)]
impl Vec3 {
    #[ffi(arg(self), arg(rest))]
    pub fn add(&mut self, x: i32, y: i32, z: i32) -> i32 {
        self.x += x;
        self.y += y;
        self.z += z;
        self.x + self.y + self.z
    }

    #[ffi(arg(rest), arg(self))]
    pub fn add_reverse_args(&mut self, x: i32, y: i32, z: i32) -> i32 {
        self.add(x, y, z)
    }

    #[ffi(arg(self), arg(rest))]
    pub fn dot(&self, other: &Vec3) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[ffi(arg(self), arg(rest))]
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