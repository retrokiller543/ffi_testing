package com.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("hello world");

    }

    static {
        System.out.println("Library path: " + System.getProperty("java.library.path"));
        System.out.println("Attempting to load ffi_lib...");
        try {
            System.loadLibrary("ffi_lib");
            System.out.println("ffi_lib loaded successfully.");
        } catch (UnsatisfiedLinkError e) {
            System.err.println("Native code library failed to load.\n" + e);
        }
    }
}
