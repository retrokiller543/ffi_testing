generate-headers:
    cbindgen --config cbindgen.toml --crate ffi_lib --output ./examples/C-example/ffi_lib.h
    cbindgen --config cppbindgen.toml --crate ffi_lib --output ./examples/CPP-example/ffi_lib.h

compile-c-debug:
    @mkdir -p examples/C-example/out
    @gcc -DDEFINE_FFI -c examples/C-example/ffi_lib.c -o examples/C-example/out/ffi_lib.o
    @gcc -L./target/debug -o examples/C-example/out/ffi_lib examples/C-example/out/ffi_lib.o -lffi_lib

compile-cpp-debug:
    @mkdir -p examples/CPP-example/out
    @g++ -DDEFINE_FFI -Iexamples/CPP-example -c examples/CPP-example/main.cpp -o examples/CPP-example/out/main.o
    @g++ -L./target/debug -o examples/CPP-example/out/ffi_lib examples/CPP-example/out/main.o -lffi_lib

compile-c:
    @mkdir -p examples/C-example/out
    @gcc -DDEFINE_FFI -O3 -c examples/C-example/c_vec3.c -o examples/C-example/out/c_vec3.o
    @gcc -DDEFINE_FFI -O3 -c examples/C-example/ffi_lib.c -o examples/C-example/out/ffi_lib.o
    @gcc -O3 -L./target/release -o examples/C-example/out/ffi_lib examples/C-example/out/ffi_lib.o examples/C-example/out/c_vec3.o -lffi_lib

compile-cpp:
    @mkdir -p examples/CPP-example/out
    @g++ -DDEFINE_FFI -Iexamples/CPP-example -c examples/CPP-example/main.cpp -o examples/CPP-example/out/main.o
    @g++ -L./target/release -o examples/CPP-example/out/ffi_lib examples/CPP-example/out/main.o -lffi_lib

build-jni-ffi: generate-headers
    cargo build --features jni -r

build-c-ffi: generate-headers
    cargo build --features c-ffi -r