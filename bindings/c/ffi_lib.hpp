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
    int x;
    int y;
    int z;
    } vec3;


/// Function using the type.
vec2 my_function(vec2 input);

vec3* vec3_new(int x, int y, int z);

int add(vec3* slf, int x, int y, int z);

int add_reverse_args(int x, int y, int z, vec3* slf);

int dot(vec3* slf, const vec3* other);

vec3 cross(vec3* slf, const vec3* other);

vec3 normalize(vec3* slf);

void hello(const char* name);

double benchmark_rust();

double benchmark_rust_async();

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror simple_service_destroy(simpleservice** context);

ffierror simple_service_new_with(simpleservice** context, unsigned int some_value);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror vec4_destroy(vec4** context);

ffierror vec4_new(vec4** context, float x, float y, float z, float w);

float vec4_dot(const vec4* context, const vec4* other);

float vec4_get_x(const vec4* context);

float vec4_get_y(const vec4* context);

float vec4_get_z(const vec4* context);

float vec4_get_w(const vec4* context);

void vec4_set_x(vec4* context, float value);

void vec4_set_y(vec4* context, float value);

void vec4_set_z(vec4* context, float value);

void vec4_set_w(vec4* context, float value);

void init_logger();

#ifdef __cplusplus
class SimpleService
{
    private:
        simpleservice* _context;
    public:

        SimpleService() : _context(nullptr) {}
        ~SimpleService() { Dispose(); }

        static SimpleService NewWith(unsigned int some_value)
        {
            SimpleService self;
            ffierror rval = simple_service_new_with(&self._context, some_value);
            if (rval != FFIERROR_OK)
            {
                throw rval;
            }
            return self;
        }

        void Dispose()
        {
            ffierror rval = simple_service_destroy(&_context);
            if (rval != FFIERROR_OK)
            {
                throw rval;
            }
        }

        simpleservice* Context() const { return _context; }
    private:
        explicit SimpleService(simpleservice* context) : _context(context) {}
    public:
        static SimpleService FromContext(simpleservice* context) { return SimpleService(context); }
};
#endif /* __cplusplus */

#ifdef __cplusplus
class Vec4
{
    private:
        vec4* _context;
    public:

        Vec4() : _context(nullptr) {}
        ~Vec4() { Dispose(); }

        static Vec4 New(float x, float y, float z, float w)
        {
            Vec4 self;
            ffierror rval = vec4_new(&self._context, x, y, z, w);
            if (rval != FFIERROR_OK)
            {
                throw rval;
            }
            return self;
        }

        void Dispose()
        {
            ffierror rval = vec4_destroy(&_context);
            if (rval != FFIERROR_OK)
            {
                throw rval;
            }
        }

        float Dot(const vec4* other)
        {
            return vec4_dot(_context, other);
        }

        float GetX()
        {
            return vec4_get_x(_context);
        }

        float GetY()
        {
            return vec4_get_y(_context);
        }

        float GetZ()
        {
            return vec4_get_z(_context);
        }

        float GetW()
        {
            return vec4_get_w(_context);
        }

        void SetX(float value)
        {
            vec4_set_x(_context, value);
        }

        void SetY(float value)
        {
            vec4_set_y(_context, value);
        }

        void SetZ(float value)
        {
            vec4_set_z(_context, value);
        }

        void SetW(float value)
        {
            vec4_set_w(_context, value);
        }

        vec4* Context() const { return _context; }
    private:
        explicit Vec4(vec4* context) : _context(context) {}
    public:
        static Vec4 FromContext(vec4* context) { return Vec4(context); }
};
#endif /* __cplusplus */


#ifdef __cplusplus
}
#endif

#endif /* ffi_lib */
