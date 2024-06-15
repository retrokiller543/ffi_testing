#include <iostream>
#include "ffi_lib.h"

#if defined(DEFINE_FFI)
int main() {
    Vec3 vec(1, 2, 3);
    std::cout << "Initial Vec3: x=" << vec.x << ", y=" << vec.y << ", z=" << vec.z << std::endl;

    add(&vec, 4, 5, 6);
    std::cout << "After add: x=" << vec.x << ", y=" << vec.y << ", z=" << vec.z << std::endl;

    add_reverse_args(1, 2, 3, &vec);
    std::cout << "After add_reverse_args: x=" << vec.x << ", y=" << vec.y << ", z=" << vec.z << std::endl;

    return 0;
}
#endif
