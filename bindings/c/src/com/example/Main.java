package com.example;

import static com.example.ffi_lib.*;

public class Main {
    static {
        System.out.println("Library path: " + System.getProperty("java.library.path"));
        System.out.println("Attempting to load ffi_lib...");
        try {
            System.loadLibrary("ffi_lib_jni");
            System.out.println("ffi_lib loaded successfully.");
        } catch (UnsatisfiedLinkError e) {
            System.err.println("Native code library failed to load.\n" + e);
        }
    }

    public static void main(String[] args) {
        hello("World");
        double benchmark_rust = benchmark_rust();
        System.out.println("benchmark_rust: " + benchmark_rust);
        double benchmark_rust_async = benchmark_rust_async();
        System.out.println("benchmark_rust_async: " + benchmark_rust_async);
    }
}