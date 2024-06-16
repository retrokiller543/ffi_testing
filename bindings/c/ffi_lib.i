// mylib.i
%module ffi_lib
%{
#define SWIG_FILE_WITH_INIT
#include "ffi_lib.h"
%}
%include "ffi_lib.h"
