#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <math.h>

typedef struct CVec3 {
  int32_t x;
  int32_t y;
  int32_t z;
} CVec3;

struct CVec3 *c_vec3_new(int32_t x, int32_t y, int32_t z);

int32_t c_add(struct CVec3 *slf, int32_t x, int32_t y, int32_t z);

int32_t c_add_reverse_args(int32_t x, int32_t y, int32_t z, struct CVec3 *slf);

int32_t c_dot(struct CVec3 *slf, const struct CVec3 *other);

struct CVec3 c_cross(struct CVec3 *slf, const struct CVec3 *other);

struct CVec3 c_normalize(struct CVec3 *slf);