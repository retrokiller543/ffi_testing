#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "ffi_lib.h"
#include "c_vec3.h"

#if defined(DEFINE_FFI)
void benchmark_c() {
    const int iterations = 1000000;
    CVec3 *v1 = c_vec3_new(1, 2, 3);
    CVec3 *v2 = c_vec3_new(4, 5, 6);

    clock_t start = clock();

    for (int i = 0; i < iterations; i++) {
        c_dot(v1, v2);
        c_cross(v1, v2);
        c_normalize(v1);
    }

    clock_t end = clock();
    double time_spent = (double)(end - start) / CLOCKS_PER_SEC;

    printf("C benchmark time: %f seconds\n", time_spent);

    free(v1);
    free(v2);
}

void benchmark_rust() {
    const int iterations = 1000000;
    Vec3 *v1 = vec3_new(1, 2, 3);
    Vec3 *v2 = vec3_new(4, 5, 6);

    clock_t start = clock();

    for (int i = 0; i < iterations; i++) {
        dot(v1, v2);
        cross(v1, v2);
        normalize(v1);
    }

    clock_t end = clock();
    double time_spent = (double)(end - start) / CLOCKS_PER_SEC;

    printf("Rust benchmark time: %f seconds\n", time_spent);

    free(v1);
    free(v2);
}

int main() {
    benchmark_c();
    benchmark_rust();
    return 0;
}
#endif