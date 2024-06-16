/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (https://www.swig.org).
 * Version 4.2.1
 *
 * Do not make changes to this file unless you know what you are doing - modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */


#define SWIG_VERSION 0x040201
#define SWIGJAVA

/* -----------------------------------------------------------------------------
 *  This section contains generic SWIG labels for method/variable
 *  declarations/attributes, and other compiler dependent labels.
 * ----------------------------------------------------------------------------- */

/* template workaround for compilers that cannot correctly implement the C++ standard */
#ifndef SWIGTEMPLATEDISAMBIGUATOR
# if defined(__SUNPRO_CC) && (__SUNPRO_CC <= 0x560)
#  define SWIGTEMPLATEDISAMBIGUATOR template
# elif defined(__HP_aCC)
/* Needed even with `aCC -AA' when `aCC -V' reports HP ANSI C++ B3910B A.03.55 */
/* If we find a maximum version that requires this, the test would be __HP_aCC <= 35500 for A.03.55 */
#  define SWIGTEMPLATEDISAMBIGUATOR template
# else
#  define SWIGTEMPLATEDISAMBIGUATOR
# endif
#endif

/* inline attribute */
#ifndef SWIGINLINE
# if defined(__cplusplus) || (defined(__GNUC__) && !defined(__STRICT_ANSI__))
#   define SWIGINLINE inline
# else
#   define SWIGINLINE
# endif
#endif

/* attribute recognised by some compilers to avoid 'unused' warnings */
#ifndef SWIGUNUSED
# if defined(__GNUC__)
#   if !(defined(__cplusplus)) || (__GNUC__ > 3 || (__GNUC__ == 3 && __GNUC_MINOR__ >= 4))
#     define SWIGUNUSED __attribute__ ((__unused__))
#   else
#     define SWIGUNUSED
#   endif
# elif defined(__ICC)
#   define SWIGUNUSED __attribute__ ((__unused__))
# else
#   define SWIGUNUSED
# endif
#endif

#ifndef SWIG_MSC_UNSUPPRESS_4505
# if defined(_MSC_VER)
#   pragma warning(disable : 4505) /* unreferenced local function has been removed */
# endif
#endif

#ifndef SWIGUNUSEDPARM
# ifdef __cplusplus
#   define SWIGUNUSEDPARM(p)
# else
#   define SWIGUNUSEDPARM(p) p SWIGUNUSED
# endif
#endif

/* internal SWIG method */
#ifndef SWIGINTERN
# define SWIGINTERN static SWIGUNUSED
#endif

/* internal inline SWIG method */
#ifndef SWIGINTERNINLINE
# define SWIGINTERNINLINE SWIGINTERN SWIGINLINE
#endif

/* exporting methods */
#if defined(__GNUC__)
#  if (__GNUC__ >= 4) || (__GNUC__ == 3 && __GNUC_MINOR__ >= 4)
#    ifndef GCC_HASCLASSVISIBILITY
#      define GCC_HASCLASSVISIBILITY
#    endif
#  endif
#endif

#ifndef SWIGEXPORT
# if defined(_WIN32) || defined(__WIN32__) || defined(__CYGWIN__)
#   if defined(STATIC_LINKED)
#     define SWIGEXPORT
#   else
#     define SWIGEXPORT __declspec(dllexport)
#   endif
# else
#   if defined(__GNUC__) && defined(GCC_HASCLASSVISIBILITY)
#     define SWIGEXPORT __attribute__ ((visibility("default")))
#   else
#     define SWIGEXPORT
#   endif
# endif
#endif

/* calling conventions for Windows */
#ifndef SWIGSTDCALL
# if defined(_WIN32) || defined(__WIN32__) || defined(__CYGWIN__)
#   define SWIGSTDCALL __stdcall
# else
#   define SWIGSTDCALL
# endif
#endif

/* Deal with Microsoft's attempt at deprecating C standard runtime functions */
#if !defined(SWIG_NO_CRT_SECURE_NO_DEPRECATE) && defined(_MSC_VER) && !defined(_CRT_SECURE_NO_DEPRECATE)
# define _CRT_SECURE_NO_DEPRECATE
#endif

/* Deal with Microsoft's attempt at deprecating methods in the standard C++ library */
#if !defined(SWIG_NO_SCL_SECURE_NO_DEPRECATE) && defined(_MSC_VER) && !defined(_SCL_SECURE_NO_DEPRECATE)
# define _SCL_SECURE_NO_DEPRECATE
#endif

/* Deal with Apple's deprecated 'AssertMacros.h' from Carbon-framework */
#if defined(__APPLE__) && !defined(__ASSERT_MACROS_DEFINE_VERSIONS_WITHOUT_UNDERSCORES)
# define __ASSERT_MACROS_DEFINE_VERSIONS_WITHOUT_UNDERSCORES 0
#endif

/* Intel's compiler complains if a variable which was never initialised is
 * cast to void, which is a common idiom which we use to indicate that we
 * are aware a variable isn't used.  So we just silence that warning.
 * See: https://github.com/swig/swig/issues/192 for more discussion.
 */
#ifdef __INTEL_COMPILER
# pragma warning disable 592
#endif

#if defined(__cplusplus) && __cplusplus >=201103L
# define SWIG_NULLPTR nullptr
#else
# define SWIG_NULLPTR NULL
#endif 

/* -----------------------------------------------------------------------------
 * swigcompat.swg
 *
 * Macros to provide support compatibility with older C and C++ standards.
 * ----------------------------------------------------------------------------- */

/* C99 and C++11 should provide snprintf, but define SWIG_NO_SNPRINTF
 * if you're missing it.
 */
#if ((defined __STDC_VERSION__ && __STDC_VERSION__ >= 199901L) || \
     (defined __cplusplus && __cplusplus >= 201103L) || \
     defined SWIG_HAVE_SNPRINTF) && \
    !defined SWIG_NO_SNPRINTF
# define SWIG_snprintf(O,S,F,A) snprintf(O,S,F,A)
# define SWIG_snprintf2(O,S,F,A,B) snprintf(O,S,F,A,B)
#else
/* Fallback versions ignore the buffer size, but most of our uses either have a
 * fixed maximum possible size or dynamically allocate a buffer that's large
 * enough.
 */
# define SWIG_snprintf(O,S,F,A) sprintf(O,F,A)
# define SWIG_snprintf2(O,S,F,A,B) sprintf(O,F,A,B)
#endif


#include <jni.h>
#include <stdlib.h>
#include <string.h>


/* Support for throwing Java exceptions */
typedef enum {
  SWIG_JavaOutOfMemoryError = 1,
  SWIG_JavaIOException,
  SWIG_JavaRuntimeException,
  SWIG_JavaIndexOutOfBoundsException,
  SWIG_JavaArithmeticException,
  SWIG_JavaIllegalArgumentException,
  SWIG_JavaNullPointerException,
  SWIG_JavaDirectorPureVirtual,
  SWIG_JavaUnknownError,
  SWIG_JavaIllegalStateException,
} SWIG_JavaExceptionCodes;

typedef struct {
  SWIG_JavaExceptionCodes code;
  const char *java_exception;
} SWIG_JavaExceptions_t;


static void SWIGUNUSED SWIG_JavaThrowException(JNIEnv *jenv, SWIG_JavaExceptionCodes code, const char *msg) {
  jclass excep;
  static const SWIG_JavaExceptions_t java_exceptions[] = {
    { SWIG_JavaOutOfMemoryError, "java/lang/OutOfMemoryError" },
    { SWIG_JavaIOException, "java/io/IOException" },
    { SWIG_JavaRuntimeException, "java/lang/RuntimeException" },
    { SWIG_JavaIndexOutOfBoundsException, "java/lang/IndexOutOfBoundsException" },
    { SWIG_JavaArithmeticException, "java/lang/ArithmeticException" },
    { SWIG_JavaIllegalArgumentException, "java/lang/IllegalArgumentException" },
    { SWIG_JavaNullPointerException, "java/lang/NullPointerException" },
    { SWIG_JavaDirectorPureVirtual, "java/lang/RuntimeException" },
    { SWIG_JavaUnknownError,  "java/lang/UnknownError" },
    { SWIG_JavaIllegalStateException, "java/lang/IllegalStateException" },
    { (SWIG_JavaExceptionCodes)0,  "java/lang/UnknownError" }
  };
  const SWIG_JavaExceptions_t *except_ptr = java_exceptions;

  while (except_ptr->code != code && except_ptr->code)
    except_ptr++;

  (*jenv)->ExceptionClear(jenv);
  excep = (*jenv)->FindClass(jenv, except_ptr->java_exception);
  if (excep)
    (*jenv)->ThrowNew(jenv, excep, msg);
}


/* Contract support */

#define SWIG_contract_assert(nullreturn, expr, msg) do { if (!(expr)) {SWIG_JavaThrowException(jenv, SWIG_JavaIllegalArgumentException, msg); return nullreturn; } } while (0)


#define SWIG_FILE_WITH_INIT
#include "ffi_lib.h"


#ifdef __cplusplus
extern "C" {
#endif

SWIGEXPORT jint JNICALL Java_com_example_ffi_1libJNI_FFIERROR_1OK_1get(JNIEnv *jenv, jclass jcls) {
  jint jresult = 0 ;
  enum ffierror result;
  
  (void)jenv;
  (void)jcls;
  result = (enum ffierror)FFIERROR_OK;
  jresult = (jint)result; 
  return jresult;
}


SWIGEXPORT jint JNICALL Java_com_example_ffi_1libJNI_FFIERROR_1NULL_1get(JNIEnv *jenv, jclass jcls) {
  jint jresult = 0 ;
  enum ffierror result;
  
  (void)jenv;
  (void)jcls;
  result = (enum ffierror)FFIERROR_NULL;
  jresult = (jint)result; 
  return jresult;
}


SWIGEXPORT jint JNICALL Java_com_example_ffi_1libJNI_FFIERROR_1PANIC_1get(JNIEnv *jenv, jclass jcls) {
  jint jresult = 0 ;
  enum ffierror result;
  
  (void)jenv;
  (void)jcls;
  result = (enum ffierror)FFIERROR_PANIC;
  jresult = (jint)result; 
  return jresult;
}


SWIGEXPORT jint JNICALL Java_com_example_ffi_1libJNI_FFIERROR_1FAIL_1get(JNIEnv *jenv, jclass jcls) {
  jint jresult = 0 ;
  enum ffierror result;
  
  (void)jenv;
  (void)jcls;
  result = (enum ffierror)FFIERROR_FAIL;
  jresult = (jint)result; 
  return jresult;
}


SWIGEXPORT void JNICALL Java_com_example_ffi_1libJNI_vec2_1x_1set(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_, jfloat jarg2) {
  struct vec2 *arg1 = (struct vec2 *) 0 ;
  float arg2 ;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(struct vec2 **)&jarg1; 
  arg2 = (float)jarg2; 
  if (arg1) (arg1)->x = arg2;
}


SWIGEXPORT jfloat JNICALL Java_com_example_ffi_1libJNI_vec2_1x_1get(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_) {
  jfloat jresult = 0 ;
  struct vec2 *arg1 = (struct vec2 *) 0 ;
  float result;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(struct vec2 **)&jarg1; 
  result = (float) ((arg1)->x);
  jresult = (jfloat)result; 
  return jresult;
}


SWIGEXPORT void JNICALL Java_com_example_ffi_1libJNI_vec2_1y_1set(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_, jfloat jarg2) {
  struct vec2 *arg1 = (struct vec2 *) 0 ;
  float arg2 ;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(struct vec2 **)&jarg1; 
  arg2 = (float)jarg2; 
  if (arg1) (arg1)->y = arg2;
}


SWIGEXPORT jfloat JNICALL Java_com_example_ffi_1libJNI_vec2_1y_1get(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_) {
  jfloat jresult = 0 ;
  struct vec2 *arg1 = (struct vec2 *) 0 ;
  float result;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(struct vec2 **)&jarg1; 
  result = (float) ((arg1)->y);
  jresult = (jfloat)result; 
  return jresult;
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_new_1vec2(JNIEnv *jenv, jclass jcls) {
  jlong jresult = 0 ;
  struct vec2 *result = 0 ;
  
  (void)jenv;
  (void)jcls;
  result = (struct vec2 *)calloc(1, sizeof(struct vec2));
  *(struct vec2 **)&jresult = result; 
  return jresult;
}


SWIGEXPORT void JNICALL Java_com_example_ffi_1libJNI_delete_1vec2(JNIEnv *jenv, jclass jcls, jlong jarg1) {
  struct vec2 *arg1 = (struct vec2 *) 0 ;
  
  (void)jenv;
  (void)jcls;
  arg1 = *(struct vec2 **)&jarg1; 
  free((char *) arg1);
}


SWIGEXPORT void JNICALL Java_com_example_ffi_1libJNI_vec3_1x_1set(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_, jlong jarg2) {
  struct vec3 *arg1 = (struct vec3 *) 0 ;
  int32_t arg2 ;
  int32_t *argp2 ;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(struct vec3 **)&jarg1; 
  argp2 = *(int32_t **)&jarg2; 
  if (!argp2) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return ;
  }
  arg2 = *argp2; 
  if (arg1) (arg1)->x = arg2;
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_vec3_1x_1get(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_) {
  jlong jresult = 0 ;
  struct vec3 *arg1 = (struct vec3 *) 0 ;
  int32_t result;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(struct vec3 **)&jarg1; 
  result =  ((arg1)->x);
  {
    int32_t * resultptr = (int32_t *) malloc(sizeof(int32_t));
    memmove(resultptr, &result, sizeof(int32_t));
    *(int32_t **)&jresult = resultptr;
  }
  return jresult;
}


SWIGEXPORT void JNICALL Java_com_example_ffi_1libJNI_vec3_1y_1set(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_, jlong jarg2) {
  struct vec3 *arg1 = (struct vec3 *) 0 ;
  int32_t arg2 ;
  int32_t *argp2 ;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(struct vec3 **)&jarg1; 
  argp2 = *(int32_t **)&jarg2; 
  if (!argp2) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return ;
  }
  arg2 = *argp2; 
  if (arg1) (arg1)->y = arg2;
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_vec3_1y_1get(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_) {
  jlong jresult = 0 ;
  struct vec3 *arg1 = (struct vec3 *) 0 ;
  int32_t result;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(struct vec3 **)&jarg1; 
  result =  ((arg1)->y);
  {
    int32_t * resultptr = (int32_t *) malloc(sizeof(int32_t));
    memmove(resultptr, &result, sizeof(int32_t));
    *(int32_t **)&jresult = resultptr;
  }
  return jresult;
}


SWIGEXPORT void JNICALL Java_com_example_ffi_1libJNI_vec3_1z_1set(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_, jlong jarg2) {
  struct vec3 *arg1 = (struct vec3 *) 0 ;
  int32_t arg2 ;
  int32_t *argp2 ;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(struct vec3 **)&jarg1; 
  argp2 = *(int32_t **)&jarg2; 
  if (!argp2) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return ;
  }
  arg2 = *argp2; 
  if (arg1) (arg1)->z = arg2;
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_vec3_1z_1get(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_) {
  jlong jresult = 0 ;
  struct vec3 *arg1 = (struct vec3 *) 0 ;
  int32_t result;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(struct vec3 **)&jarg1; 
  result =  ((arg1)->z);
  {
    int32_t * resultptr = (int32_t *) malloc(sizeof(int32_t));
    memmove(resultptr, &result, sizeof(int32_t));
    *(int32_t **)&jresult = resultptr;
  }
  return jresult;
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_new_1vec3(JNIEnv *jenv, jclass jcls) {
  jlong jresult = 0 ;
  struct vec3 *result = 0 ;
  
  (void)jenv;
  (void)jcls;
  result = (struct vec3 *)calloc(1, sizeof(struct vec3));
  *(struct vec3 **)&jresult = result; 
  return jresult;
}


SWIGEXPORT void JNICALL Java_com_example_ffi_1libJNI_delete_1vec3(JNIEnv *jenv, jclass jcls, jlong jarg1) {
  struct vec3 *arg1 = (struct vec3 *) 0 ;
  
  (void)jenv;
  (void)jcls;
  arg1 = *(struct vec3 **)&jarg1; 
  free((char *) arg1);
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_my_1function(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_) {
  jlong jresult = 0 ;
  vec2 arg1 ;
  vec2 *argp1 ;
  vec2 result;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  argp1 = *(vec2 **)&jarg1; 
  if (!argp1) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null vec2");
    return 0;
  }
  arg1 = *argp1; 
  result = my_function(arg1);
  {
    vec2 * resultptr = (vec2 *) malloc(sizeof(vec2));
    memmove(resultptr, &result, sizeof(vec2));
    *(vec2 **)&jresult = resultptr;
  }
  return jresult;
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_vec3_1new(JNIEnv *jenv, jclass jcls, jlong jarg1, jlong jarg2, jlong jarg3) {
  jlong jresult = 0 ;
  int32_t arg1 ;
  int32_t arg2 ;
  int32_t arg3 ;
  int32_t *argp1 ;
  int32_t *argp2 ;
  int32_t *argp3 ;
  vec3 *result = 0 ;
  
  (void)jenv;
  (void)jcls;
  argp1 = *(int32_t **)&jarg1; 
  if (!argp1) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return 0;
  }
  arg1 = *argp1; 
  argp2 = *(int32_t **)&jarg2; 
  if (!argp2) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return 0;
  }
  arg2 = *argp2; 
  argp3 = *(int32_t **)&jarg3; 
  if (!argp3) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return 0;
  }
  arg3 = *argp3; 
  result = (vec3 *)vec3_new(arg1,arg2,arg3);
  *(vec3 **)&jresult = result; 
  return jresult;
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_add(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_, jlong jarg2, jlong jarg3, jlong jarg4) {
  jlong jresult = 0 ;
  vec3 *arg1 = (vec3 *) 0 ;
  int32_t arg2 ;
  int32_t arg3 ;
  int32_t arg4 ;
  int32_t *argp2 ;
  int32_t *argp3 ;
  int32_t *argp4 ;
  int32_t result;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(vec3 **)&jarg1; 
  argp2 = *(int32_t **)&jarg2; 
  if (!argp2) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return 0;
  }
  arg2 = *argp2; 
  argp3 = *(int32_t **)&jarg3; 
  if (!argp3) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return 0;
  }
  arg3 = *argp3; 
  argp4 = *(int32_t **)&jarg4; 
  if (!argp4) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return 0;
  }
  arg4 = *argp4; 
  result = add(arg1,arg2,arg3,arg4);
  {
    int32_t * resultptr = (int32_t *) malloc(sizeof(int32_t));
    memmove(resultptr, &result, sizeof(int32_t));
    *(int32_t **)&jresult = resultptr;
  }
  return jresult;
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_add_1reverse_1args(JNIEnv *jenv, jclass jcls, jlong jarg1, jlong jarg2, jlong jarg3, jlong jarg4, jobject jarg4_) {
  jlong jresult = 0 ;
  int32_t arg1 ;
  int32_t arg2 ;
  int32_t arg3 ;
  vec3 *arg4 = (vec3 *) 0 ;
  int32_t *argp1 ;
  int32_t *argp2 ;
  int32_t *argp3 ;
  int32_t result;
  
  (void)jenv;
  (void)jcls;
  (void)jarg4_;
  argp1 = *(int32_t **)&jarg1; 
  if (!argp1) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return 0;
  }
  arg1 = *argp1; 
  argp2 = *(int32_t **)&jarg2; 
  if (!argp2) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return 0;
  }
  arg2 = *argp2; 
  argp3 = *(int32_t **)&jarg3; 
  if (!argp3) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null int32_t");
    return 0;
  }
  arg3 = *argp3; 
  arg4 = *(vec3 **)&jarg4; 
  result = add_reverse_args(arg1,arg2,arg3,arg4);
  {
    int32_t * resultptr = (int32_t *) malloc(sizeof(int32_t));
    memmove(resultptr, &result, sizeof(int32_t));
    *(int32_t **)&jresult = resultptr;
  }
  return jresult;
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_dot(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_, jlong jarg2, jobject jarg2_) {
  jlong jresult = 0 ;
  vec3 *arg1 = (vec3 *) 0 ;
  vec3 *arg2 = (vec3 *) 0 ;
  int32_t result;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  (void)jarg2_;
  arg1 = *(vec3 **)&jarg1; 
  arg2 = *(vec3 **)&jarg2; 
  result = dot(arg1,(struct vec3 const *)arg2);
  {
    int32_t * resultptr = (int32_t *) malloc(sizeof(int32_t));
    memmove(resultptr, &result, sizeof(int32_t));
    *(int32_t **)&jresult = resultptr;
  }
  return jresult;
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_cross(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_, jlong jarg2, jobject jarg2_) {
  jlong jresult = 0 ;
  vec3 *arg1 = (vec3 *) 0 ;
  vec3 *arg2 = (vec3 *) 0 ;
  vec3 result;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  (void)jarg2_;
  arg1 = *(vec3 **)&jarg1; 
  arg2 = *(vec3 **)&jarg2; 
  result = cross(arg1,(struct vec3 const *)arg2);
  {
    vec3 * resultptr = (vec3 *) malloc(sizeof(vec3));
    memmove(resultptr, &result, sizeof(vec3));
    *(vec3 **)&jresult = resultptr;
  }
  return jresult;
}


SWIGEXPORT jlong JNICALL Java_com_example_ffi_1libJNI_normalize(JNIEnv *jenv, jclass jcls, jlong jarg1, jobject jarg1_) {
  jlong jresult = 0 ;
  vec3 *arg1 = (vec3 *) 0 ;
  vec3 result;
  
  (void)jenv;
  (void)jcls;
  (void)jarg1_;
  arg1 = *(vec3 **)&jarg1; 
  result = normalize(arg1);
  {
    vec3 * resultptr = (vec3 *) malloc(sizeof(vec3));
    memmove(resultptr, &result, sizeof(vec3));
    *(vec3 **)&jresult = resultptr;
  }
  return jresult;
}


SWIGEXPORT void JNICALL Java_com_example_ffi_1libJNI_hello(JNIEnv *jenv, jclass jcls, jlong jarg1) {
  int8_t *arg1 = (int8_t *) 0 ;
  
  (void)jenv;
  (void)jcls;
  arg1 = *(int8_t **)&jarg1; 
  hello((int8_t const *)arg1);
}


SWIGEXPORT void JNICALL Java_com_example_ffi_1libJNI_benchmark_1rust(JNIEnv *jenv, jclass jcls) {
  (void)jenv;
  (void)jcls;
  benchmark_rust();
}


SWIGEXPORT void JNICALL Java_com_example_ffi_1libJNI_benchmark_1rust_1async(JNIEnv *jenv, jclass jcls) {
  (void)jenv;
  (void)jcls;
  benchmark_rust_async();
}


SWIGEXPORT jint JNICALL Java_com_example_ffi_1libJNI_simple_1service_1destroy(JNIEnv *jenv, jclass jcls, jlong jarg1) {
  jint jresult = 0 ;
  simpleservice **arg1 = (simpleservice **) 0 ;
  ffierror result;
  
  (void)jenv;
  (void)jcls;
  arg1 = *(simpleservice ***)&jarg1; 
  result = (ffierror)simple_service_destroy(arg1);
  jresult = (jint)result; 
  return jresult;
}


SWIGEXPORT jint JNICALL Java_com_example_ffi_1libJNI_simple_1service_1new_1with(JNIEnv *jenv, jclass jcls, jlong jarg1, jlong jarg2) {
  jint jresult = 0 ;
  simpleservice **arg1 = (simpleservice **) 0 ;
  uint32_t arg2 ;
  uint32_t *argp2 ;
  ffierror result;
  
  (void)jenv;
  (void)jcls;
  arg1 = *(simpleservice ***)&jarg1; 
  argp2 = *(uint32_t **)&jarg2; 
  if (!argp2) {
    SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Attempt to dereference null uint32_t");
    return 0;
  }
  arg2 = *argp2; 
  result = (ffierror)simple_service_new_with(arg1,arg2);
  jresult = (jint)result; 
  return jresult;
}


SWIGEXPORT jint JNICALL Java_com_example_ffi_1libJNI_vec4_1destroy(JNIEnv *jenv, jclass jcls, jlong jarg1) {
  jint jresult = 0 ;
  vec4 **arg1 = (vec4 **) 0 ;
  ffierror result;
  
  (void)jenv;
  (void)jcls;
  arg1 = *(vec4 ***)&jarg1; 
  result = (ffierror)vec4_destroy(arg1);
  jresult = (jint)result; 
  return jresult;
}


SWIGEXPORT jint JNICALL Java_com_example_ffi_1libJNI_vec4_1new(JNIEnv *jenv, jclass jcls, jlong jarg1, jfloat jarg2, jfloat jarg3, jfloat jarg4, jfloat jarg5) {
  jint jresult = 0 ;
  vec4 **arg1 = (vec4 **) 0 ;
  float arg2 ;
  float arg3 ;
  float arg4 ;
  float arg5 ;
  ffierror result;
  
  (void)jenv;
  (void)jcls;
  arg1 = *(vec4 ***)&jarg1; 
  arg2 = (float)jarg2; 
  arg3 = (float)jarg3; 
  arg4 = (float)jarg4; 
  arg5 = (float)jarg5; 
  result = (ffierror)vec4_new(arg1,arg2,arg3,arg4,arg5);
  jresult = (jint)result; 
  return jresult;
}


SWIGEXPORT jfloat JNICALL Java_com_example_ffi_1libJNI_vec4_1dot(JNIEnv *jenv, jclass jcls, jlong jarg1, jlong jarg2) {
  jfloat jresult = 0 ;
  vec4 *arg1 = (vec4 *) 0 ;
  vec4 *arg2 = (vec4 *) 0 ;
  float result;
  
  (void)jenv;
  (void)jcls;
  arg1 = *(vec4 **)&jarg1; 
  arg2 = *(vec4 **)&jarg2; 
  result = (float)vec4_dot((struct vec4 const *)arg1,(struct vec4 const *)arg2);
  jresult = (jfloat)result; 
  return jresult;
}


SWIGEXPORT void JNICALL Java_com_example_ffi_1libJNI_init_1logger(JNIEnv *jenv, jclass jcls) {
  (void)jenv;
  (void)jcls;
  init_logger();
}


#ifdef __cplusplus
}
#endif
