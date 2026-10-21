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

typedef vector unsigned char v16u8;
typedef vector unsigned short v8u16;
typedef vector unsigned int v4u32;
typedef vector unsigned long long v2u64;
typedef vector signed char v16i8;
typedef vector signed short v8i16;

__attribute__((always_inline))
void rust_isa_power9_vector_vnegw(const uint8_t *a, uint8_t *out) {
    *(v4i32 *)(void *)out = vec_neg(*(const v4i32 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vnegd(const uint8_t *a, uint8_t *out) {
    *(v2i64 *)(void *)out = vec_neg(*(const v2i64 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vabsduw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_absd(
        *(const v4u32 *)(const void *)a,
        *(const v4u32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vabsd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    v4i32 va = *(const v4i32 *)(const void *)a;
    v4i32 vb = *(const v4i32 *)(const void *)b;
    *(v4i32 *)(void *)out = vec_abs(va - vb);
}

__attribute__((always_inline))
uint32_t rust_isa_power9_vector_vctzlsbb(const uint8_t *a) {
    return vec_cnttz_lsbb(*(const v16u8 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vextsb2w(const uint8_t *a, uint8_t *out) {
    *(v4i32 *)(void *)out = vec_signexti(*(const v16i8 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vextsh2w(const uint8_t *a, uint8_t *out) {
    *(v4i32 *)(void *)out = vec_signexti(*(const v8i16 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vextsb2d(const uint8_t *a, uint8_t *out) {
    *(v2i64 *)(void *)out = vec_signextll(*(const v16i8 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vextsh2d(const uint8_t *a, uint8_t *out) {
    *(v2i64 *)(void *)out = vec_signextll(*(const v8i16 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vextsw2d(const uint8_t *a, uint8_t *out) {
    *(v2i64 *)(void *)out = vec_signextll(*(const v4i32 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vctzb(const uint8_t *a, uint8_t *out) {
    *(v16u8 *)(void *)out = (v16u8)vec_cnttz(*(const v16u8 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vctzh(const uint8_t *a, uint8_t *out) {
    *(v8u16 *)(void *)out = (v8u16)vec_cnttz(*(const v8u16 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vctzw(const uint8_t *a, uint8_t *out) {
    *(v4u32 *)(void *)out = (v4u32)vec_cnttz(*(const v4u32 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power9_vector_vctzd(const uint8_t *a, uint8_t *out) {
    *(v2u64 *)(void *)out = vec_cnttz(*(const v2u64 *)(const void *)a);
}
