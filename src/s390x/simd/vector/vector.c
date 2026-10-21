#include <stdint.h>
#include <vecintrin.h>

typedef int v4i32 __attribute__((vector_size(16)));

__attribute__((always_inline))
void rust_isa_s390x_vector_v128_load(const uint8_t *p, uint8_t out[16]) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)p;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_v128_store(uint8_t *p, const uint8_t value[16]) {
    *(v4i32 *)(void *)p = *(const v4i32 *)(const void *)value;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a + *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a * *(const v4i32 *)(const void *)b;
}

typedef signed char v16i8 __attribute__((vector_size(16)));
typedef unsigned char v16u8 __attribute__((vector_size(16)));
typedef short v8i16 __attribute__((vector_size(16)));
typedef long long v2i64 __attribute__((vector_size(16)));
typedef float v4f32 __attribute__((vector_size(16)));

__attribute__((always_inline))
void rust_isa_s390x_vector_add_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16i8 *)(void *)out = *(const v16i8 *)(const void *)a + *(const v16i8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_add_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a + *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_add_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a + *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_sub_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16i8 *)(void *)out = *(const v16i8 *)(const void *)a - *(const v16i8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_sub_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a - *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_sub_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a - *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_sub_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a - *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_mul_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a * *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_mul_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a * *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_and(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a & *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_or(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a | *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_xor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a ^ *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_cmpeq_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = (v4i32)(*(const v4i32 *)(const void *)a == *(const v4i32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_s390x_vector_cmpgt_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = (v4i32)(*(const v4i32 *)(const void *)a > *(const v4i32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_s390x_vector_fadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a + *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_fsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a - *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_fmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a * *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_fdiv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a / *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_s390x_vector_permute(const uint8_t *a, const uint8_t *b, const uint8_t *index, uint8_t *out) {
    *(v16i8 *)(void *)out = (v16i8)vec_perm(
        *(const v16u8 *)(const void *)a,
        *(const v16u8 *)(const void *)b,
        *(const v16u8 *)(const void *)index);
}

__attribute__((always_inline))
void rust_isa_s390x_vector_splat_i32(int32_t value, uint8_t *out) {
    *(v4i32 *)(void *)out = (v4i32){value, value, value, value};
}

typedef signed char sv16i8 __attribute__((vector_size(16)));
typedef unsigned char sv16u8 __attribute__((vector_size(16)));
typedef signed short sv8i16 __attribute__((vector_size(16)));
typedef unsigned short sv8u16 __attribute__((vector_size(16)));
typedef signed int sv4i32 __attribute__((vector_size(16)));
typedef unsigned int sv4u32 __attribute__((vector_size(16)));
typedef signed long long sv2i64 __attribute__((vector_size(16)));
typedef unsigned long long sv2u64 __attribute__((vector_size(16)));

__attribute__((always_inline))
void rust_isa_s390x_vector_vab(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv16u8 *)(void *)out = *(const sv16u8 *)(const void *)a + *(const sv16u8 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vah(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv8u16 *)(void *)out = *(const sv8u16 *)(const void *)a + *(const sv8u16 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vaf(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv4u32 *)(void *)out = *(const sv4u32 *)(const void *)a + *(const sv4u32 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vag(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv2u64 *)(void *)out = *(const sv2u64 *)(const void *)a + *(const sv2u64 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vsb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv16u8 *)(void *)out = *(const sv16u8 *)(const void *)a - *(const sv16u8 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vsh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv8u16 *)(void *)out = *(const sv8u16 *)(const void *)a - *(const sv8u16 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vsf(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv4u32 *)(void *)out = *(const sv4u32 *)(const void *)a - *(const sv4u32 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vsg(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv2u64 *)(void *)out = *(const sv2u64 *)(const void *)a - *(const sv2u64 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vmin(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv4i32 *)(void *)out = vec_min(
        *(const sv4i32 *)(const void *)a,
        *(const sv4i32 *)(const void *)b);
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vmax(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv4i32 *)(void *)out = vec_max(
        *(const sv4i32 *)(const void *)a,
        *(const sv4i32 *)(const void *)b);
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vavg(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    sv16u8 va = *(const sv16u8 *)(const void *)a;
    sv16u8 vb = *(const sv16u8 *)(const void *)b;
    *(sv16u8 *)(void *)out = (va + vb + (sv16u8){1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1}) >> 1;
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vavgl(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    sv8u16 va = *(const sv8u16 *)(const void *)a;
    sv8u16 vb = *(const sv8u16 *)(const void *)b;
    *(sv8u16 *)(void *)out = (va + vb + (sv8u16){1,1,1,1,1,1,1,1}) >> 1;
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vceq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv4i32 *)(void *)out = (sv4i32)(*(const sv4i32 *)(const void *)a == *(const sv4i32 *)(const void *)b);
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vch(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv4i32 *)(void *)out = (sv4i32)(*(const sv4i32 *)(const void *)a > *(const sv4i32 *)(const void *)b);
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vcl(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv4i32 *)(void *)out = (sv4i32)(*(const sv4i32 *)(const void *)a < *(const sv4i32 *)(const void *)b);
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vsel(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    sv16u8 va = *(const sv16u8 *)(const void *)a;
    sv16u8 vb = *(const sv16u8 *)(const void *)b;
    sv16u8 vc = *(const sv16u8 *)(const void *)c;
    *(sv16u8 *)(void *)out = (va & vc) | (vb & ~vc);
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vsl(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv2u64 *)(void *)out = *(const sv2u64 *)(const void *)a << *(const sv2u64 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_s390x_vector_vsr(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(sv2u64 *)(void *)out = *(const sv2u64 *)(const void *)a >> *(const sv2u64 *)(const void *)b;
}
