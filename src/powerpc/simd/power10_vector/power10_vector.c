#include <stdint.h>
#include <altivec.h>

typedef vector unsigned int v4u32;

__attribute__((always_inline))
void rust_isa_power10_vector_popcnt_i32(const uint8_t *a, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_popcnt(*(const v4u32 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power10_vector_cntlz_i32(const uint8_t *a, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_cntlz(*(const v4u32 *)(const void *)a);
}

typedef vector unsigned long long v2u64;

__attribute__((always_inline))
void rust_isa_power10_vector_vclzd(const uint8_t *a, uint8_t *out) {
    *(v2u64 *)(void *)out = vec_cntlz(*(const v2u64 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power10_vector_vpopcntd(const uint8_t *a, uint8_t *out) {
    *(v2u64 *)(void *)out = vec_popcnt(*(const v2u64 *)(const void *)a);
}
