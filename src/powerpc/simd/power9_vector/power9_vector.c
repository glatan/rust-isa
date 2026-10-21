#include <stdint.h>
#include <altivec.h>

typedef vector signed int v4i32;
typedef vector signed long long v2i64;

__attribute__((always_inline))
void rust_isa_power9_vector_neg_i32(const uint8_t *a, uint8_t *out) {
    *(v4i32 *)(void *)out = vec_neg(*(const v4i32 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_neg_i64(const uint8_t *a, uint8_t *out) {
    *(v2i64 *)(void *)out = vec_neg(*(const v2i64 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_abs_i32(const uint8_t *a, uint8_t *out) {
    *(v4i32 *)(void *)out = vec_abs(*(const v4i32 *)(const void *)a);
}
