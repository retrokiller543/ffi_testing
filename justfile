#!/usr/bin/env just --justfile


BINDINGS_DIR := "bindings/c"

generate-headers:
    cbindgen --config cbindgen.toml --crate ffi_lib --output ./examples/C-example/ffi_lib.h
    cbindgen --config cppbindgen.toml --crate ffi_lib --output ./examples/CPP-example/ffi_lib.h
    @cargo test -r

compile-c-debug:
    @mkdir -p examples/C-example/out
    @gcc -DDEFINE_FFI -O3 -c examples/C-example/c_vec3.c -o examples/C-example/out/c_vec3.o
    @gcc -DDEFINE_FFI -DDEFINE_INTEROPTOPUS -O3 -c examples/C-example/ffi_lib.c -o examples/C-example/out/ffi_lib.o
    @gcc -L./target/debug -o examples/C-example/out/ffi_lib examples/C-example/out/ffi_lib.o examples/C-example/out/c_vec3.o -lffi_lib

compile-cpp-debug:
    @mkdir -p examples/CPP-example/out
    @g++ -DDEFINE_FFI -Iexamples/CPP-example -c examples/CPP-example/main.cpp -o examples/CPP-example/out/main.o
    @g++ -L./target/debug -o examples/CPP-example/out/ffi_lib examples/CPP-example/out/main.o -lffi_lib

compile-c:
    @mkdir -p examples/C-example/out
    @gcc -DDEFINE_FFI -O3 -c examples/C-example/c_vec3.c -o examples/C-example/out/c_vec3.o
    @gcc -DDEFINE_FFI -DDEFINE_INTEROPTOPUS -O3 -c examples/C-example/ffi_lib.c -o examples/C-example/out/ffi_lib.o
    @gcc -O9 -L./target/release -o examples/C-example/out/ffi_lib examples/C-example/out/ffi_lib.o examples/C-example/out/c_vec3.o -lffi_lib

compile-cpp:
    @mkdir -p examples/CPP-example/out
    @g++ -DDEFINE_FFI -Iexamples/CPP-example -c examples/CPP-example/main.cpp -o examples/CPP-example/out/main.o
    @g++ -L./target/release -o examples/CPP-example/out/ffi_lib examples/CPP-example/out/main.o -lffi_lib

build-jni-ffi:
    cargo build --features jni -r

build-c-ffi: generate-headers
    cargo build --features "c-ffi interoptopus" -r

build-java-lib:
    export BINDINGS_DIR=bindings/c
    rm -rf {{BINDINGS_DIR}}/src
    mkdir -p {{BINDINGS_DIR}}/src/com/example
    cargo test --features "c-ffi interoptopus" --release
    cargo build --features "c-ffi interoptopus" --release
    swig -java -package com.example -outdir {{BINDINGS_DIR}}/src/com/example -o {{BINDINGS_DIR}}/ffi_lib_wrap.c {{BINDINGS_DIR}}/ffi_lib.i
    gcc -shared -O3 -o {{BINDINGS_DIR}}/libffi_lib.so {{BINDINGS_DIR}}/ffi_lib_wrap.c -I$JAVA_HOME/include -I$JAVA_HOME/include/darwin -L./target/release -lffi_lib

install-c:
    @sudo mkdir -p /usr/local/include
    @sudo mkdir -p /usr/local/lib
    @sudo cp {{BINDINGS_DIR}}/ffi_lib.h /usr/local/include/
    @if [ "$(uname)" = "Darwin" ]; then \
        sudo cp target/release/libffi_lib.dylib /usr/local/lib/; \
    elif [ "$(expr substr $(uname -s) 1 5)" = "Linux" ]; then \
        sudo cp target/release/libffi_lib.so /usr/local/lib/; \
    elif [ "$(expr substr $(uname -s) 1 5)" = "MINGW" ]; then \
        sudo cp target/release/ffi_lib.dll /usr/local/lib/; \
    fi
    @sudo ldconfig