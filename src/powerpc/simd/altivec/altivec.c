#include <stdint.h>
#include <altivec.h>

typedef vector unsigned int v4u32;
typedef vector signed int v4i32;

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

typedef signed char v16i8 __attribute__((vector_size(16)));
typedef unsigned char v16u8 __attribute__((vector_size(16)));
typedef short v8i16 __attribute__((vector_size(16)));
typedef long long v2i64 __attribute__((vector_size(16)));
typedef float v4f32 __attribute__((vector_size(16)));

__attribute__((always_inline))
void rust_isa_altivec_add_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16i8 *)(void *)out = *(const v16i8 *)(const void *)a + *(const v16i8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_add_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a + *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_add_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a + *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_sub_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16i8 *)(void *)out = *(const v16i8 *)(const void *)a - *(const v16i8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_sub_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a - *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_sub_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = *(const v4u32 *)(const void *)a - *(const v4u32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_sub_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a - *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_mul_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a * *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_mul_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = *(const v4u32 *)(const void *)a * *(const v4u32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_and(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a & *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_or(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a | *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_xor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a ^ *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_cmpeq_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = (v4u32)vec_cmpeq(
        *(const v4u32 *)(const void *)a,
        *(const v4u32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_cmpgt_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = (v4u32)vec_cmpgt(
        *(const v4i32 *)(const void *)a,
        *(const v4i32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_fadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a + *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_fsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a - *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_fmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a * *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_fdiv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a / *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_permute(const uint8_t *a, const uint8_t *b, const uint8_t *index, uint8_t *out) {
    *(v16i8 *)(void *)out = (v16i8)vec_perm(
        *(const v16u8 *)(const void *)a,
        *(const v16u8 *)(const void *)b,
        *(const v16u8 *)(const void *)index);
}

__attribute__((always_inline))
void rust_isa_altivec_splat_i32(int32_t value, uint8_t *out) {
    *(v4u32 *)(void *)out = (v4u32){value, value, value, value};
}
