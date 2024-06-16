cmake_minimum_required(VERSION 3.28)
project(c)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)
link_libraries(ffi_lib)
link_directories(../../target/release ../../target/debug)

add_executable(program
        ffi_lib.hpp
        main.cpp)
