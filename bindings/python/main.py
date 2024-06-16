# ffi_example_run.py

from ffi_lib import *
import ctypes

def main():
    # Path to the native library
    library_path = "/Users/emil/projects/ffi_lib/target/release/libffi_lib.dylib"  # Update this with your library path

    # Initialize the library
    init_lib(library_path)

    init_logger()

    # Create instances of Vec2 and Vec3
    vec2_instance = Vec2(1.0, 2.0)
    vec3_instance = vec3_new(1, 2, 3)

    # Call functions with Vec2
    result_vec2 = my_function(vec2_instance)
    print(f"my_function(Vec2): x={result_vec2.x}, y={result_vec2.y}")

    # Call functions with Vec3
    result_add = add(vec3_instance, 4, 5, 6)
    print(f"add(Vec3): {result_add}")

    result_add_reverse = add_reverse_args(4, 5, 6, vec3_instance)
    print(f"add_reverse_args: {result_add_reverse}")

    vec3_instance2 = vec3_new(7, 8, 9)
    result_dot = dot(vec3_instance, vec3_instance2)
    print(f"dot(Vec3): {result_dot}")

    result_cross = cross(vec3_instance, vec3_instance2)
    print(f"cross(Vec3): x={result_cross.x}, y={result_cross.y}, z={result_cross.z}")

    result_normalize = normalize(vec3_instance)
    print(f"normalize(Vec3): x={result_normalize.x}, y={result_normalize.y}, z={result_normalize.z}")

    # Benchmark Rust function
    benchmark_rust()
    print("benchmark_rust: completed")

    benchmark_rust_async()
    print("benchmark_rust_async: completed")

    # Create and use SimpleService
    simple_service = SimpleService.new_with(10)
    print(f"SimpleService created with value: 10")

    # Create and use Vec4
    vec4_instance = Vec4.new(1.0, 2.0, 3.0, 4.0)
    vec4_instance2 = Vec4.new(5.0, 6.0, 7.0, 8.0)
    result_vec4_dot = vec4_instance.dot(vec4_instance2._ctx)
    print(f"vec4_dot: {result_vec4_dot}")

if __name__ == "__main__":
    main()
