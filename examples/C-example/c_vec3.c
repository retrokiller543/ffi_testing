#include "c_vec3.h"

struct CVec3 *c_vec3_new(int32_t x, int32_t y, int32_t z) {
    struct CVec3 *slf = (struct CVec3 *)malloc(sizeof(struct CVec3));
    slf->x = x;
    slf->y = y;
    slf->z = z;
    return slf;
}

int32_t c_add(struct CVec3 *slf, int32_t x, int32_t y, int32_t z) {
    slf->x += x;
    slf->y += y;
    slf->z += z;
    return slf->x + slf->y + slf->z;
}

int32_t c_add_reverse_args(int32_t x, int32_t y, int32_t z, struct CVec3 *slf) {
    slf->x = x;
    slf->y = y;
    slf->z = z;
    return slf->x + slf->y + slf->z;
}

int32_t c_dot(struct CVec3 *slf, const struct CVec3 *other) {
    return slf->x * other->x +
           slf->y * other->y +
           slf->z * other->z;
}

struct CVec3 c_cross(struct CVec3 *slf, const struct CVec3 *other) {
    slf->x = slf->y * other->z - slf->z * other->y;
    slf->y = slf->z * other->x - slf->x * other->z;
    slf->z = slf->x * other->y - slf->y * other->x;
    return *slf;
}

struct CVec3 c_normalize(struct CVec3 *slf) {
    int32_t length = slf->x * slf->x + slf->y * slf->y + slf->z * slf->z;
    if (length == 0) {
        slf->x = 0;
        slf->y = 0;
        slf->z = 0;
    } else {
        int32_t inv = 1 / (int32_t)sqrt(length);
        slf->x *= inv;
        slf->y *= inv;
        slf->z *= inv;
    }
    return *slf;
}