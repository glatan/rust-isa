#include <stdint.h>
#include <altivec.h>

typedef vector unsigned int v4u32;

__attribute__((always_inline))
void rust_isa_altivec_v128_load(const uint8_t *p, uint8_t out[16]) {
    *(v4u32 *)(void *)out = *(const v4u32 *)(const void *)p;
}

__attribute__((always_inline))
void rust_isa_altivec_v128_store(uint8_t *p, const uint8_t value[16]) {
    *(v4u32 *)(void *)p = *(const v4u32 *)(const void *)value;
}

__attribute__((always_inline))
void rust_isa_altivec_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_add(*(const v4u32 *)(const void *)a, *(const v4u32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_mul(*(const v4u32 *)(const void *)a, *(const v4u32 *)(const void *)b);
}
