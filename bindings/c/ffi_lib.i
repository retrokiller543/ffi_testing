// ffi_lib.i
%module ffi_lib
%{
#include "ffi_lib.h"
%}

%include "cpointer.i"

%include "ffi_lib.hpp"
