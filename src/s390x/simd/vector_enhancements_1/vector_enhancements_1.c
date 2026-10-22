#include <stdint.h>
#include <vecintrin.h>

typedef signed int v4i32 __attribute__((vector_size(16)));
typedef unsigned char v16u8 __attribute__((vector_size(16)));
typedef unsigned int v4u32 __attribute__((vector_size(16)));
typedef float v4f32 __attribute__((vector_size(16)));

__attribute__((always_inline))
void rust_isa_s390x_enh_cntlz_i32(const uint8_t *a, uint8_t *out) {
    *(v4i32 *)(void *)out = vec_cntlz(*(const v4i32 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_popcnt_i8(const uint8_t *a, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_popcnt(*(const v16u8 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_popcnt_i32(const uint8_t *a, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_popcnt(*(const v4u32 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_fma(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)acc
        + *(const v4f32 *)(const void *)a * *(const v4f32 *)(const void *)b;
}

typedef unsigned short v8u16 __attribute__((vector_size(16)));
typedef unsigned long long v2u64 __attribute__((vector_size(16)));

__attribute__((always_inline))
void rust_isa_s390x_enh_vfa(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a + *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vfs(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a - *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vfm(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a * *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vfd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a / *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vfma(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = vec_madd(
        *(const v4f32 *)(const void *)a,
        *(const v4f32 *)(const void *)b,
        *(const v4f32 *)(const void *)acc);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vfms(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = vec_msub(
        *(const v4f32 *)(const void *)a,
        *(const v4f32 *)(const void *)b,
        *(const v4f32 *)(const void *)acc);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vfnma(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = vec_nmadd(
        *(const v4f32 *)(const void *)a,
        *(const v4f32 *)(const void *)b,
        *(const v4f32 *)(const void *)acc);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vfnms(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = vec_nmsub(
        *(const v4f32 *)(const void *)a,
        *(const v4f32 *)(const void *)b,
        *(const v4f32 *)(const void *)acc);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vfsq(const uint8_t *a, uint8_t *out) {
    *(v4f32 *)(void *)out = vec_sqrt(*(const v4f32 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vfce(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = (v4u32)vec_cmpeq(
        *(const v4f32 *)(const void *)a,
        *(const v4f32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vfch(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = (v4u32)vec_cmpgt(
        *(const v4f32 *)(const void *)a,
        *(const v4f32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vfche(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = (v4u32)vec_cmpge(
        *(const v4f32 *)(const void *)a,
        *(const v4f32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vclz(const uint8_t *a, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_cntlz(*(const v16u8 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vctz(const uint8_t *a, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_cnttz(*(const v16u8 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vpopct(const uint8_t *a, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_popcnt(*(const v16u8 *)(const void *)a);
}




