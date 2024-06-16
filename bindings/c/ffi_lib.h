// Automatically generated by Interoptopus.

#ifndef ffi_lib
#define ffi_lib

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>




typedef struct simpleservice simpleservice;

typedef struct vec4 vec4;

typedef enum ffierror
    {
    FFIERROR_OK = 0,
    FFIERROR_NULL = 100,
    FFIERROR_PANIC = 200,
    FFIERROR_FAIL = 300,
    } ffierror;

/// A simple type in our FFI layer.
typedef struct vec2
    {
    float x;
    float y;
    } vec2;

/// A basic vector 3 in our FFI layer using integers.
typedef struct vec3
    {
    int32_t x;
    int32_t y;
    int32_t z;
    } vec3;


/// Function using the type.
vec2 my_function(vec2 input);

vec3* vec3_new(int32_t x, int32_t y, int32_t z);

int32_t add(vec3* slf, int32_t x, int32_t y, int32_t z);

int32_t add_reverse_args(int32_t x, int32_t y, int32_t z, vec3* slf);

int32_t dot(vec3* slf, const vec3* other);

vec3 cross(vec3* slf, const vec3* other);

vec3 normalize(vec3* slf);

void hello(const int8_t* name);

void benchmark_rust();

void benchmark_rust_async();

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror simple_service_destroy(simpleservice** context);

ffierror simple_service_new_with(simpleservice** context, uint32_t some_value);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror vec4_destroy(vec4** context);

ffierror vec4_new(vec4** context, float x, float y, float z, float w);

float vec4_dot(const vec4* context, const vec4* other);

void init_logger();


#ifdef __cplusplus
}
#endif

#endif /* ffi_lib */