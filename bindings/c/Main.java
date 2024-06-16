package com.example;

import static com.example.ffi_lib.*;

public class Main {
    static {
        System.loadLibrary("ffi_lib");
    }

    public static void main(String[] args) {
        init_logger();
        benchmark_rust_async();

    }
}