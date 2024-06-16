//
// Created by tosic-killer on 2024-06-16.
//
#include <cstdio>
#include "ffi_lib.hpp"

int main() {
    printf("Rust Benchmark took %lf\n", benchmark_rust());
    for (int i = 0; i < 10000; ++i)
        benchmark_rust_async();
    printf("Rust async Benchmark took %lf\n", benchmark_rust_async());
    {
        vec4* vecA;
        vec4* vecACopy;
        vec4* vecB;
        ffierror err;

        // Create vec4 instances
        err = vec4_new(&vecA, 1.0f, 2.0f, 3.0f, 4.0f);
        if (err != FFIERROR_OK) {
            fprintf(stderr, "Failed to create vec4 instance vecA\n");
            return 1;
        }

        err = vec4_new(&vecACopy, 1.0f, 2.0f, 3.0f, 4.0f);
        if (err != FFIERROR_OK) {
            fprintf(stderr, "Failed to create vec4 instance vecACopy\n");
            vec4_destroy(&vecA); // Clean up the first instance before exiting
            return 1;
        }

        err = vec4_new(&vecB, 4.0f, 3.0f, 2.0f, 1.0f);
        if (err != FFIERROR_OK) {
            fprintf(stderr, "Failed to create vec4 instance vecB\n");
            vec4_destroy(&vecA); // Clean up the first instance before exiting
            vec4_destroy(&vecACopy);
            return 1;
        }

        // Use vec4_dot function
        float dot_product = vec4_dot(vecA, vecB);
        printf("Dot product of vecA and vecB: %f\n", dot_product);

        // Clean up vec4 instances
        vec4_destroy(&vecA);
        vec4_destroy(&vecACopy);
        vec4_destroy(&vecB);
    }
    #ifdef __cplusplus
    {
        Vec4 vecA = Vec4::New(1.0f, 2.0f, 3.0f, 4.0f);
        Vec4 vecB = Vec4::New(4.0f, 3.0f, 2.0f, 1.0f);
        vecB.SetX(2.0f);
        float dot_product = vecA.Dot(vecB.Context());
        printf("Dot product of vecA and vecB: %f\n", dot_product);
    }
    #endif

    return 0;
}