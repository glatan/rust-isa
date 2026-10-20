#include <stdint.h>
#include <msa.h>

__attribute__((always_inline))
void rust_isa_msa_v128_load(const uint8_t *p, uint8_t out[16]) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)p;
}

__attribute__((always_inline))
void rust_isa_msa_v128_store(uint8_t *p, const uint8_t value[16]) {
    *(v4i32 *)(void *)p = *(const v4i32 *)(const void *)value;
}

__attribute__((always_inline))
void rust_isa_msa_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a + *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_msa_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a * *(const v4i32 *)(const void *)b;
}
