/* Generated with cbindgen:0.26.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
namespace ffi {
#endif // __cplusplus

#if defined(DEFINE_INTEROPTOPUS)
typedef enum FFIError {
#if defined(DEFINE_INTEROPTOPUS)
  Ok = 0,
#endif
#if defined(DEFINE_INTEROPTOPUS)
  Null = 100,
#endif
#if defined(DEFINE_INTEROPTOPUS)
  Panic = 200,
#endif
#if defined(DEFINE_INTEROPTOPUS)
  Fail = 300,
#endif
} FFIError;
#endif

#if defined(DEFINE_INTEROPTOPUS)
/**
 * A simple type in our FFI layer.
 */
typedef struct Vec2 {
  float x;
  float y;
} Vec2;
#endif

#if defined(DEFINE_INTEROPTOPUS)
typedef struct SimpleService {

} SimpleService;
#endif

#if defined(DEFINE_INTEROPTOPUS)
typedef struct Vec4 {
  float x;
  float y;
  float z;
  float w;
} Vec4;
#endif

/**
 * The basic struct we will call methods in our FFI layer.
 */
typedef struct Vec3 {
  int32_t x;
  int32_t y;
  int32_t z;
} Vec3;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#if defined(DEFINE_INTEROPTOPUS)
/**
 * Function using the type.
 */
struct Vec2 my_function(struct Vec2 input);
#endif

#if defined(DEFINE_INTEROPTOPUS)
void hello(const char *name);
#endif

#if defined(DEFINE_INTEROPTOPUS)
enum FFIError simple_service_new_with(struct SimpleService **context, uint32_t _some_value);
#endif

#if defined(DEFINE_INTEROPTOPUS)
/**
 * Destroys the given instance.
 *
 * # Safety
 *
 * The passed parameter MUST have been created with the corresponding init function;
 * passing any other value results in undefined behavior.
 */
enum FFIError simple_service_destroy(struct SimpleService **context);
#endif

#if defined(DEFINE_INTEROPTOPUS)
enum FFIError vec4_new(struct Vec4 **context, float x, float y, float z, float w);
#endif

#if defined(DEFINE_INTEROPTOPUS)
float vec4_dot(const struct Vec4 *context, const struct Vec4 *other);
#endif

#if defined(DEFINE_INTEROPTOPUS)
float vec4_get_x(const struct Vec4 *context);
#endif

#if defined(DEFINE_INTEROPTOPUS)
float vec4_get_y(const struct Vec4 *context);
#endif

#if defined(DEFINE_INTEROPTOPUS)
float vec4_get_z(const struct Vec4 *context);
#endif

#if defined(DEFINE_INTEROPTOPUS)
float vec4_get_w(const struct Vec4 *context);
#endif

#if defined(DEFINE_INTEROPTOPUS)
void vec4_set_x(struct Vec4 *context, float value);
#endif

#if defined(DEFINE_INTEROPTOPUS)
void vec4_set_y(struct Vec4 *context, float value);
#endif

#if defined(DEFINE_INTEROPTOPUS)
void vec4_set_z(struct Vec4 *context, float value);
#endif

#if defined(DEFINE_INTEROPTOPUS)
void vec4_set_w(struct Vec4 *context, float value);
#endif

#if defined(DEFINE_INTEROPTOPUS)
/**
 * Destroys the given instance.
 *
 * # Safety
 *
 * The passed parameter MUST have been created with the corresponding init function;
 * passing any other value results in undefined behavior.
 */
enum FFIError vec4_destroy(struct Vec4 **context);
#endif

#if defined(DEFINE_INTEROPTOPUS)
double benchmark_rust_async(void);
#endif

#if ((defined(DEFINE_FFI) && defined(DEFINE_INTEROPTOPUS)) && defined(DEFINE_INTEROPTOPUS))
void init_logger(void);
#endif

#if ((defined(DEFINE_FFI) && defined(DEFINE_INTEROPTOPUS)) && defined(DEFINE_INTEROPTOPUS))
double benchmark_rust(void);
#endif

#if (defined(DEFINE_FFI) && defined(DEFINE_INTEROPTOPUS))
enum FFIError vec3_new(struct Vec3 **context, int32_t x, int32_t y, int32_t z);
#endif

#if (defined(DEFINE_FFI) && defined(DEFINE_INTEROPTOPUS))
int32_t vec3_add(struct Vec3 *context, int32_t x, int32_t y, int32_t z);
#endif

#if (defined(DEFINE_FFI) && defined(DEFINE_INTEROPTOPUS))
int32_t vec3_add_reverse_args(struct Vec3 *context, int32_t x, int32_t y, int32_t z);
#endif

#if (defined(DEFINE_FFI) && defined(DEFINE_INTEROPTOPUS))
int32_t vec3_dot(const struct Vec3 *context, const struct Vec3 *other);
#endif

#if (defined(DEFINE_FFI) && defined(DEFINE_INTEROPTOPUS))
struct Vec3 vec3_cross(const struct Vec3 *context, const struct Vec3 *other);
#endif

#if (defined(DEFINE_FFI) && defined(DEFINE_INTEROPTOPUS))
struct Vec3 vec3_normalize(const struct Vec3 *context);
#endif

#if (defined(DEFINE_FFI) && defined(DEFINE_INTEROPTOPUS))
/**
 * Destroys the given instance.
 *
 * # Safety
 *
 * The passed parameter MUST have been created with the corresponding init function;
 * passing any other value results in undefined behavior.
 */
enum FFIError vec3_destroy(struct Vec3 **context);
#endif

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#ifdef __cplusplus
} // namespace ffi
#endif // __cplusplus
