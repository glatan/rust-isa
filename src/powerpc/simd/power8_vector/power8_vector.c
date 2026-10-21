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

typedef vector signed int v4i32;
typedef vector unsigned int v4u32;
typedef vector signed long long v2i64;
typedef vector unsigned long long v2u64;

__attribute__((always_inline))
void rust_isa_power8_vector_vmuleuw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2u64 *)(void *)out = vec_mule(
        *(const v4u32 *)(const void *)a,
        *(const v4u32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vmulouw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2u64 *)(void *)out = vec_mulo(
        *(const v4u32 *)(const void *)a,
        *(const v4u32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vmaxud(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_max(
        *(const v4u32 *)(const void *)a,
        *(const v4u32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vminud(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_min(
        *(const v4u32 *)(const void *)a,
        *(const v4u32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vmaxsd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = vec_max(
        *(const v4i32 *)(const void *)a,
        *(const v4i32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vminsd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = vec_min(
        *(const v4i32 *)(const void *)a,
        *(const v4i32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vpkudum(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_pack(
        *(const v2u64 *)(const void *)a,
        *(const v2u64 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vpksdus(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_packsu(
        *(const v2i64 *)(const void *)a,
        *(const v2i64 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vpksdss(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = vec_packs(
        *(const v2i64 *)(const void *)a,
        *(const v2i64 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vupkhsw(const uint8_t *a, uint8_t *out) {
    *(v2i64 *)(void *)out = vec_unpackh(*(const v4i32 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vupklsw(const uint8_t *a, uint8_t *out) {
    *(v2i64 *)(void *)out = vec_unpackl(*(const v4i32 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vclzd(const uint8_t *a, uint8_t *out) {
    *(v2u64 *)(void *)out = vec_cntlz(*(const v2u64 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power8_vector_vpopcntd(const uint8_t *a, uint8_t *out) {
    *(v2u64 *)(void *)out = vec_popcnt(*(const v2u64 *)(const void *)a);
}
