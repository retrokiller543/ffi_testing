use robusta_jni::bridge;
use robusta_jni::convert::{Signature, IntoJavaValue};
use robusta_jni::jni::errors::Result as JniResult;
use robusta_jni::jni::JNIEnv;

use crate::Vec3;

#[bridge]
mod jniv2 {
    use robusta_jni::{convert::{Field, FromJavaValue, TryFromJavaValue, TryIntoJavaValue}, jni::objects::AutoLocal};

    use super::*;

    #[derive(Signature, IntoJavaValue, TryIntoJavaValue, TryFromJavaValue, FromJavaValue)]
    #[package(fuck)]
    pub struct Vec3JniV2<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
        #[field]
        x: Field<'env, 'borrow, i32>,
        #[field]
        y: Field<'env, 'borrow, i32>,
        #[field]
        z: Field<'env, 'borrow, i32>,
    }

    impl<'env: 'borrow, 'borrow> Vec3JniV2<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>, x: i32, y: i32, z: i32) -> JniResult<Self> {}

        pub extern "jni" fn add(mut self, _env: &JNIEnv, x: i32, y: i32, z: i32) -> JniResult<i32> {
            self.x.set(self.x.get()? + x)?;
            self.y.set(self.y.get()? + y)?;
            self.z.set(self.z.get()? + z)?;
            
            let result: i32 = self.x.get()? + self.y.get()? + self.z.get()?;
            return Ok(result);
        }

        pub extern "jni" fn addReverseArgs(self, env: &JNIEnv, x: i32, y: i32, z: i32) -> JniResult<i32> {
            self.add(env, x, y, z)
        }

        pub extern "jni" fn dot(self, _env: &JNIEnv, other: Vec3JniV2<'env, 'borrow>) -> JniResult<i32> {
            Ok(self.x.get()? * other.x.get()? + self.y.get()? * other.y.get()? + self.z.get()? * other.z.get()?)
        }

        pub extern "jni" fn cross(mut self, _env: &JNIEnv, other: Vec3JniV2<'env, 'borrow>) -> JniResult<Vec3JniV2<'env, 'borrow>> {
            println!("[rust] cross: {}, {}, {}", self.x.get()?, self.y.get()?, self.z.get()?);
            let x: i32 = self.y.get()? * other.z.get()? - self.z.get()? * other.y.get()?;
            let y: i32 = self.z.get()? * other.x.get()? - self.x.get()? * other.z.get()?;
            let z: i32 = self.x.get()? * other.y.get()? - self.y.get()? * other.x.get()?;

            println!("[rust] cross-result: {}, {}, {}", x, y, z);

            self.x.set(x)?;
            self.y.set(y)?;
            self.z.set(z)?;

            println!("[rust] cross-set-result: {}, {}, {}", self.x.get()?, self.y.get()?, self.z.get()?);
            
            return Ok(self);
        }

        pub extern "jni" fn normalize(mut self, _env: &JNIEnv) -> JniResult<Vec3JniV2<'env, 'borrow>> {
            println!("[rust] normalize: {}, {}, {}", self.x.get()?, self.y.get()?, self.z.get()?);
            let len = ((self.x.get()? * self.x.get()? + self.y.get()? * self.y.get()? + self.z.get()? * self.z.get()?) as f64).sqrt();

            let x = (self.x.get()? as f64 / len) as i32;
            let y = (self.y.get()? as f64 / len) as i32;
            let z = (self.z.get()? as f64 / len) as i32;
            self.x.set(x)?;
            self.y.set(y)?;
            self.z.set(z)?;
            Ok(self)
        }

    }
}

#[bridge]
mod jni {
    use super::*;

    #[derive(Signature)]
    #[package(fuck)]
    struct Vec3Jni;

    impl Vec3Jni {
        pub extern "jni" fn vec3New(x: i32, y: i32, z: i32) -> i64 {
            Box::into_raw(Box::new(Vec3::new(x, y, z))) as i64
        }

        pub extern "jni" fn vec3Add(vec_ptr: i64, x: i32, y: i32, z: i32) -> i32 {
            let vec = unsafe { &mut *(vec_ptr as *mut Vec3) };
            vec.add(x, y, z)
        }

        pub extern "jni" fn vec3AddReverseArgs(vec_ptr: i64, x: i32, y: i32, z: i32) -> i32 {
            let vec = unsafe { &mut *(vec_ptr as *mut Vec3) };
            vec.add_reverse_args(x, y, z)
        }

        pub extern "jni" fn vec3Dot(vec_ptr: i64, other_ptr: i64) -> i32 {
            let vec = unsafe { &*(vec_ptr as *mut Vec3) };
            let other = unsafe { &*(other_ptr as *mut Vec3) };
            vec.dot(other)
        }

        pub extern "jni" fn vec3Cross(vec_ptr: i64, other_ptr: i64) -> i64 {
            let vec = unsafe { &*(vec_ptr as *mut Vec3) };
            let other = unsafe { &*(other_ptr as *mut Vec3) };
            Box::into_raw(Box::new(vec.cross(other))) as i64
        }

        pub extern "jni" fn vec3Normalize(vec_ptr: i64) -> i64 {
            let vec = unsafe { &*(vec_ptr as *mut Vec3) };
            Box::into_raw(Box::new(vec.normalize())) as i64
        }
    }
}
