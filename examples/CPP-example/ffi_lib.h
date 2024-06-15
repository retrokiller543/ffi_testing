#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Vec3 {
  int32_t x;
  int32_t y;
  int32_t z;

  Vec3(int32_t const& x,
       int32_t const& y,
       int32_t const& z)
    : x(x),
      y(y),
      z(z)
  {}

  bool operator==(const Vec3& other) const {
    return x == other.x &&
           y == other.y &&
           z == other.z;
  }
  bool operator!=(const Vec3& other) const {
    return x != other.x ||
           y != other.y ||
           z != other.z;
  }
};

extern "C" {

Vec3 *vec3_new(int32_t x, int32_t y, int32_t z);

int32_t add(Vec3 *slf, int32_t x, int32_t y, int32_t z);

int32_t add_reverse_args(int32_t x, int32_t y, int32_t z, Vec3 *slf);

int32_t dot(Vec3 *slf, const Vec3 *other);

Vec3 cross(Vec3 *slf, const Vec3 *other);

Vec3 normalize(Vec3 *slf);

} // extern "C"
