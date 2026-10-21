#include <stdint.h>
#include <altivec.h>

typedef vector unsigned long long v2u64;
typedef vector unsigned int v4u32;

__attribute__((always_inline))
void rust_isa_power8_vector_add_u64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2u64 *)(void *)out = *(const v2u64 *)(const void *)a + *(const v2u64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_power8_vector_sub_u64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2u64 *)(void *)out = *(const v2u64 *)(const void *)a - *(const v2u64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_power8_vector_mul_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = *(const v4u32 *)(const void *)a * *(const v4u32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_power8_vector_mul_even_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    typedef vector unsigned short v8u16;
    *(v4u32 *)(void *)out = vec_mule(
        *(const v8u16 *)(const void *)a,
        *(const v8u16 *)(const void *)b);
}
