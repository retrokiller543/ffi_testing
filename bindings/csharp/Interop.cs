// Automatically generated by Interoptopus.

#pragma warning disable 0105
using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using ffi_lib.testing;
#pragma warning restore 0105

namespace ffi_lib.testing
{
    public static partial class InteropClass
    {
        public const string NativeLib = "ffi_lib";

        static InteropClass()
        {
        }


        /// Function using the type.
        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "my_function")]
        public static extern Vec2 my_function(Vec2 input);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "vec3_new")]
        public static extern IntPtr vec3_new(int x, int y, int z);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "add")]
        public static extern int add(out Vec3 slf, int x, int y, int z);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "add_reverse_args")]
        public static extern int add_reverse_args(int x, int y, int z, out Vec3 slf);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "dot")]
        public static extern int dot(out Vec3 slf, ref Vec3 other);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "cross")]
        public static extern Vec3 cross(out Vec3 slf, ref Vec3 other);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "normalize")]
        public static extern Vec3 normalize(out Vec3 slf);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "hello")]
        public static extern void hello(ref sbyte name);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "benchmark_rust")]
        public static extern void benchmark_rust();

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "benchmark_rust_async")]
        public static extern void benchmark_rust_async();

        /// Destroys the given instance.
        ///
        /// # Safety
        ///
        /// The passed parameter MUST have been created with the corresponding init function;
        /// passing any other value results in undefined behavior.
        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "simple_service_destroy")]
        public static extern FFIError simple_service_destroy(ref IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "simple_service_new_with")]
        public static extern FFIError simple_service_new_with(ref IntPtr context, uint some_value);

        /// Destroys the given instance.
        ///
        /// # Safety
        ///
        /// The passed parameter MUST have been created with the corresponding init function;
        /// passing any other value results in undefined behavior.
        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "vec4_destroy")]
        public static extern FFIError vec4_destroy(ref IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "vec4_new")]
        public static extern FFIError vec4_new(ref IntPtr context, float x, float y, float z, float w);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "vec4_dot")]
        public static extern float vec4_dot(IntPtr context, IntPtr other);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "init_logger")]
        public static extern void init_logger();

    }

    /// A simple type in our FFI layer.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Vec2
    {
        public float x;
        public float y;
    }

    /// A basic vector 3 in our FFI layer using integers.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Vec3
    {
        int x;
        int y;
        int z;
    }

    public enum FFIError
    {
        Ok = 0,
        Null = 100,
        Panic = 200,
        Fail = 300,
    }


    public partial class SimpleService : IDisposable
    {
        private IntPtr _context;

        private SimpleService() {}

        public static SimpleService NewWith(uint some_value)
        {
            var self = new SimpleService();
            var rval = InteropClass.simple_service_new_with(ref self._context, some_value);
            if (rval != FFIError.Ok)
            {
                throw new InteropException<FFIError>(rval);
            }
            return self;
        }

        public void Dispose()
        {
            var rval = InteropClass.simple_service_destroy(ref _context);
            if (rval != FFIError.Ok)
            {
                throw new InteropException<FFIError>(rval);
            }
        }

        public IntPtr Context => _context;
    }


    public partial class Vec4 : IDisposable
    {
        private IntPtr _context;

        private Vec4() {}

        public static Vec4 New(float x, float y, float z, float w)
        {
            var self = new Vec4();
            var rval = InteropClass.vec4_new(ref self._context, x, y, z, w);
            if (rval != FFIError.Ok)
            {
                throw new InteropException<FFIError>(rval);
            }
            return self;
        }

        public void Dispose()
        {
            var rval = InteropClass.vec4_destroy(ref _context);
            if (rval != FFIError.Ok)
            {
                throw new InteropException<FFIError>(rval);
            }
        }

        public float Dot(IntPtr other)
        {
            return InteropClass.vec4_dot(_context, other);
        }

        public IntPtr Context => _context;
    }



    public class InteropException<T> : Exception
    {
        public T Error { get; private set; }

        public InteropException(T error): base($"Something went wrong: {error}")
        {
            Error = error;
        }
    }

}