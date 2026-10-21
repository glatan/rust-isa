#include <stdint.h>
#include <altivec.h>

typedef vector double v2f64;
typedef vector float v4f32;

__attribute__((always_inline))
void rust_isa_vsx_add_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a + *(const v2f64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_vsx_sub_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a - *(const v2f64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_vsx_mul_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a * *(const v2f64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_vsx_div_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a / *(const v2f64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_vsx_add_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a + *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_vsx_abs_f64(const uint8_t *a, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_abs(*(const v2f64 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_vsx_sqrt_f64(const uint8_t *a, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_sqrt(*(const v2f64 *)(const void *)a);
}
