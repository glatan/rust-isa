#include <stdint.h>

#include <lsxintrin.h>

typedef int v4i32 __attribute__((vector_size(16)));

__attribute__((always_inline))
void rust_isa_lsx_v128_load(const uint8_t *p, uint8_t out[16]) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)p;
}

__attribute__((always_inline))
void rust_isa_lsx_v128_store(uint8_t *p, const uint8_t value[16]) {
    *(v4i32 *)(void *)p = *(const v4i32 *)(const void *)value;
}

__attribute__((always_inline))
void rust_isa_lsx_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a + *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a * *(const v4i32 *)(const void *)b;
}

typedef signed char v16i8 __attribute__((vector_size(16)));
typedef short v8i16 __attribute__((vector_size(16)));
typedef long long v2i64 __attribute__((vector_size(16)));
typedef float v4f32 __attribute__((vector_size(16)));

__attribute__((always_inline))
void rust_isa_lsx_add_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16i8 *)(void *)out = *(const v16i8 *)(const void *)a + *(const v16i8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_add_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a + *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_add_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a + *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_sub_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16i8 *)(void *)out = *(const v16i8 *)(const void *)a - *(const v16i8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_sub_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a - *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_sub_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a - *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_sub_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a - *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_mul_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a * *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_mul_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a * *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_and(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a & *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_or(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a | *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_xor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a ^ *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_cmpeq_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a == *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_cmpgt_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a > *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_fadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a + *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_fsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a - *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_fmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a * *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_fdiv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a / *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lsx_splat_i32(int32_t value, uint8_t *out) {
    *(v4i32 *)(void *)out = (v4i32){value, value, value, value};
}

__attribute__((always_inline))
void rust_isa_lsx_vavgr_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vavgr_b(
        *(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavgr_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vavgr_h(
        *(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavgr_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vavgr_w(
        *(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavgr_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vavgr_d(
        *(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavgr_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vavgr_bu(
        *(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavgr_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vavgr_hu(
        *(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavgr_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vavgr_wu(
        *(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavgr_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vavgr_du(
        *(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vexth_h_b(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vexth_h_b(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vexth_w_h(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vexth_w_h(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vexth_d_w(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vexth_d_w(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vexth_q_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vexth_q_d(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vexth_hu_bu(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vexth_hu_bu(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vexth_wu_hu(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vexth_wu_hu(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vexth_du_wu(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vexth_du_wu(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vexth_qu_du(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vexth_qu_du(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vextl_q_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vextl_q_d(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vextl_qu_du(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vextl_qu_du(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfadd_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128 *)(void *)out = __lsx_vfadd_s(
        *(const __m128 *)(const void *)a, *(const __m128 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfadd_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vfadd_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfsub_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128 *)(void *)out = __lsx_vfsub_s(
        *(const __m128 *)(const void *)a, *(const __m128 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfsub_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vfsub_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfmul_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128 *)(void *)out = __lsx_vfmul_s(
        *(const __m128 *)(const void *)a, *(const __m128 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfmul_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vfmul_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfdiv_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128 *)(void *)out = __lsx_vfdiv_s(
        *(const __m128 *)(const void *)a, *(const __m128 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfdiv_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vfdiv_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfmax_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128 *)(void *)out = __lsx_vfmax_s(
        *(const __m128 *)(const void *)a, *(const __m128 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfmax_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vfmax_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfmin_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128 *)(void *)out = __lsx_vfmin_s(
        *(const __m128 *)(const void *)a, *(const __m128 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfmin_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vfmin_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfcvt_s_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128 *)(void *)out = __lsx_vfcvt_s_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfcvtl_d_s(const uint8_t *a, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vfcvtl_d_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfcvth_d_s(const uint8_t *a, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vfcvth_d_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftint_w_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftint_w_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftint_wu_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftint_wu_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftint_l_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftint_l_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftint_lu_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftint_lu_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftint_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftint_w_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrz_w_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrz_w_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrm_w_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrm_w_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrp_w_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrp_w_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrne_w_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrne_w_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrz_wu_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrz_wu_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrz_l_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrz_l_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrm_l_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrm_l_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrp_l_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrp_l_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrne_l_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrne_l_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrz_lu_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrz_lu_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrz_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrz_w_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrm_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrm_w_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrp_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrp_w_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrne_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vftintrne_w_d(
        *(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vffint_s_w(const uint8_t *a, uint8_t *out) {
    *(__m128 *)(void *)out = __lsx_vffint_s_w(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vffint_s_wu(const uint8_t *a, uint8_t *out) {
    *(__m128 *)(void *)out = __lsx_vffint_s_wu(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vffint_d_l(const uint8_t *a, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vffint_d_l(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vffint_d_lu(const uint8_t *a, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vffint_d_lu(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vffint_s_l(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128 *)(void *)out = __lsx_vffint_s_l(
        *(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vffintl_d_w(const uint8_t *a, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vffintl_d_w(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vffinth_d_w(const uint8_t *a, uint8_t *out) {
    *(__m128d *)(void *)out = __lsx_vffinth_d_w(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vreplgr2vr_b(int32_t value, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vreplgr2vr_b(value);
}

__attribute__((always_inline))
void rust_isa_lsx_vreplgr2vr_h(int32_t value, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vreplgr2vr_h(value);
}

__attribute__((always_inline))
void rust_isa_lsx_vreplgr2vr_w(int32_t value, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vreplgr2vr_w(value);
}

__attribute__((always_inline))
void rust_isa_lsx_vreplgr2vr_d(int64_t value, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vreplgr2vr_d(value);
}

void rust_isa_lsx_vfmadd_s(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vfmadd_s(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

void rust_isa_lsx_vfmadd_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vfmadd_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

void rust_isa_lsx_vfmsub_s(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vfmsub_s(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

void rust_isa_lsx_vfmsub_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vfmsub_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

void rust_isa_lsx_vfnmadd_s(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vfnmadd_s(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

void rust_isa_lsx_vfnmadd_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vfnmadd_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

void rust_isa_lsx_vfnmsub_s(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vfnmsub_s(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

void rust_isa_lsx_vfnmsub_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vfnmsub_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitsel_v(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitsel_v(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vshuf_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vshuf_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vldx(const void *p, long offset, uint8_t *out) {
    *(__m128i *)(void *)out = __lsx_vldx(p, offset);
}

__attribute__((always_inline))
void rust_isa_lsx_vstx(const uint8_t *value, void *p, long offset) {
    __lsx_vstx(*(const __m128i *)(const void *)value, p, offset);
}

__attribute__((always_inline))
void rust_isa_lsx_vseq_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vseq_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vseq_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vseq_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vseq_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vseq_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vseq_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vseq_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsle_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsle_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsle_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsle_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsle_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsle_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsle_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsle_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsle_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsle_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsle_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsle_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsle_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsle_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsle_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsle_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vslt_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vslt_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vslt_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vslt_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vslt_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vslt_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vslt_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vslt_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vslt_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vslt_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vslt_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vslt_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vslt_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vslt_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vslt_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vslt_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_h_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_w_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_d_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_q_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwev_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwev_h_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwev_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwev_w_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwev_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwev_d_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwev_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwev_q_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_h_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_w_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_d_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_q_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwod_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwod_h_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwod_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwod_w_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwod_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwod_d_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwod_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwod_q_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_h_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_w_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_d_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_q_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwev_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwev_h_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwev_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwev_w_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwev_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwev_d_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwev_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwev_q_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_h_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_w_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_d_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_q_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwod_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwod_h_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwod_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwod_w_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwod_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwod_d_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsubwod_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsubwod_q_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_h_bu_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_h_bu_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_w_hu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_w_hu_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_d_wu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_d_wu_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwev_q_du_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwev_q_du_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_h_bu_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_h_bu_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_w_hu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_w_hu_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_d_wu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_d_wu_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vaddwod_q_du_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vaddwod_q_du_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsadd_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsadd_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsadd_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsadd_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsadd_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsadd_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsadd_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsadd_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssub_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssub_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssub_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssub_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssub_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssub_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssub_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssub_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsadd_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsadd_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsadd_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsadd_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsadd_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsadd_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsadd_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsadd_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssub_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssub_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssub_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssub_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssub_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssub_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssub_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssub_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhaddw_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhaddw_h_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhaddw_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhaddw_w_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhaddw_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhaddw_d_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhaddw_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhaddw_q_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhsubw_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhsubw_h_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhsubw_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhsubw_w_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhsubw_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhsubw_d_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhsubw_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhsubw_q_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhaddw_hu_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhaddw_hu_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhaddw_wu_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhaddw_wu_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhaddw_du_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhaddw_du_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhaddw_qu_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhaddw_qu_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhsubw_hu_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhsubw_hu_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhsubw_wu_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhsubw_wu_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhsubw_du_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhsubw_du_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vhsubw_qu_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vhsubw_qu_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vadda_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vadda_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vadda_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vadda_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vadda_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vadda_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vadda_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vadda_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vabsd_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vabsd_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vabsd_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vabsd_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vabsd_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vabsd_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vabsd_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vabsd_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vabsd_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vabsd_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vabsd_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vabsd_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vabsd_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vabsd_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vabsd_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vabsd_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavg_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vavg_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavg_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vavg_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavg_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vavg_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavg_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vavg_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavg_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vavg_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavg_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vavg_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavg_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vavg_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vavg_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vavg_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmax_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmax_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmax_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmax_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmax_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmax_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmax_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmax_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmin_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmin_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmin_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmin_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmin_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmin_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmin_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmin_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmax_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmax_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmax_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmax_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmax_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmax_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmax_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmax_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmin_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmin_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmin_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmin_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmin_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmin_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmin_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmin_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmul_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmul_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmul_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmul_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmuh_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmuh_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmuh_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmuh_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmuh_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmuh_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmuh_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmuh_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmuh_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmuh_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmuh_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmuh_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmuh_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmuh_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmuh_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmuh_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_h_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_w_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_d_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_q_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_h_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_w_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_d_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_q_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_h_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_w_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_d_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_q_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_h_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_w_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_d_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_q_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_h_bu_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_h_bu_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_w_hu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_w_hu_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_d_wu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_d_wu_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwev_q_du_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwev_q_du_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_h_bu_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_h_bu_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_w_hu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_w_hu_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_d_wu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_d_wu_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmulwod_q_du_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmulwod_q_du_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmadd_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmadd_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmadd_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmadd_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmadd_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmadd_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmadd_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmadd_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmsub_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmsub_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmsub_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmsub_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmsub_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmsub_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmsub_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmsub_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_h_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_h_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_w_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_w_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_d_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_d_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_q_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_q_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_h_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_h_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_w_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_w_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_d_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_d_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_q_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_q_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_h_bu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_h_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_w_hu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_w_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_d_wu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_d_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_q_du(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_q_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_h_bu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_h_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_w_hu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_w_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_d_wu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_d_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_q_du(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_q_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_h_bu_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_h_bu_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_w_hu_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_w_hu_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_d_wu_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_d_wu_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwev_q_du_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwev_q_du_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_h_bu_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_h_bu_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_w_hu_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_w_hu_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_d_wu_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_d_wu_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vmaddwod_q_du_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmaddwod_q_du_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vdiv_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vdiv_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vdiv_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vdiv_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vdiv_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vdiv_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vdiv_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vdiv_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmod_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmod_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmod_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmod_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmod_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmod_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmod_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmod_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vdiv_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vdiv_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vdiv_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vdiv_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vdiv_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vdiv_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vdiv_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vdiv_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmod_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmod_bu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmod_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmod_hu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmod_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmod_wu(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vmod_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmod_du(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsll_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsll_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsll_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsll_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsll_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsll_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsll_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsll_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrl_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrl_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrl_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrl_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrl_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrl_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrl_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrl_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsra_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsra_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsra_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsra_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsra_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsra_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsra_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsra_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vrotr_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vrotr_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vrotr_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vrotr_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vrotr_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vrotr_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vrotr_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vrotr_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrlr_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrlr_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrlr_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrlr_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrlr_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrlr_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrlr_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrlr_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrar_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrar_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrar_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrar_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrar_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrar_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrar_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrar_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrln_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrln_b_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrln_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrln_h_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrln_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrln_w_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsran_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsran_b_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsran_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsran_h_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsran_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsran_w_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrlrn_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrlrn_b_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrlrn_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrlrn_h_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrlrn_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrlrn_w_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrarn_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrarn_b_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrarn_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrarn_h_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsrarn_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsrarn_w_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrln_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrln_b_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrln_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrln_h_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssran_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssran_b_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssran_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssran_h_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssran_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssran_w_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrlrn_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrlrn_b_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrlrn_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrlrn_h_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrarn_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrarn_b_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrarn_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrarn_h_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrln_bu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrln_bu_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrln_hu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrln_hu_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrln_wu_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrln_wu_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssran_bu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssran_bu_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssran_hu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssran_hu_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssran_wu_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssran_wu_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrlrn_bu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrlrn_bu_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrlrn_hu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrlrn_hu_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrlrn_wu_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrlrn_wu_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrarn_bu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrarn_bu_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrarn_hu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrarn_hu_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vssrarn_wu_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vssrarn_wu_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitclr_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitclr_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitclr_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitclr_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitclr_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitclr_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitclr_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitclr_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitset_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitset_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitset_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitset_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitset_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitset_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitset_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitset_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitrev_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitrev_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitrev_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitrev_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitrev_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitrev_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vbitrev_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vbitrev_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpackev_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpackev_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpackev_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpackev_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpackev_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpackev_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpackev_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpackev_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpackod_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpackod_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpackod_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpackod_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpackod_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpackod_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpackod_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpackod_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vilvl_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vilvl_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vilvl_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vilvl_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vilvl_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vilvl_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vilvl_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vilvl_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vilvh_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vilvh_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vilvh_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vilvh_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vilvh_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vilvh_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vilvh_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vilvh_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpickev_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpickev_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpickev_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpickev_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpickev_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpickev_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpickev_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpickev_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpickod_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpickod_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpickod_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpickod_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpickod_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpickod_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vpickod_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpickod_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vand_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vand_v(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vor_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vor_v(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vxor_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vxor_v(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vnor_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vnor_v(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vandn_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vandn_v(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vorn_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vorn_v(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstp_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstp_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstp_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstp_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vadd_q(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vadd_q(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsub_q(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsub_q(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsigncov_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsigncov_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsigncov_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsigncov_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsigncov_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsigncov_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vsigncov_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vsigncov_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfcvt_h_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfcvt_h_s(*(const __m128 *)(const void *)a, *(const __m128 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vshuf_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vshuf_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vshuf_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vshuf_w(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vshuf_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vshuf_d(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, *(const __m128i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lsx_vclo_b(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vclo_b(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vclo_h(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vclo_h(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vclo_w(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vclo_w(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vclo_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vclo_d(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vclz_b(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vclz_b(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vclz_h(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vclz_h(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vclz_w(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vclz_w(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vclz_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vclz_d(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vpcnt_b(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpcnt_b(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vpcnt_h(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpcnt_h(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vpcnt_w(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpcnt_w(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vpcnt_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vpcnt_d(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vmskltz_b(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmskltz_b(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vmskltz_h(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmskltz_h(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vmskltz_w(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmskltz_w(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vmskltz_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmskltz_d(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vmskgez_b(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmskgez_b(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vmsknz_b(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vmsknz_b(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfclass_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfclass_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfclass_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfclass_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintl_l_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vftintl_l_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftinth_l_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vftinth_l_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrml_l_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vftintrml_l_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrmh_l_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vftintrmh_l_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrpl_l_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vftintrpl_l_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrph_l_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vftintrph_l_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrzl_l_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vftintrzl_l_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrzh_l_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vftintrzh_l_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrnel_l_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vftintrnel_l_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vftintrneh_l_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vftintrneh_l_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfmaxa_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfmaxa_s(*(const __m128 *)(const void *)a, *(const __m128 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfmaxa_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfmaxa_d(*(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfmina_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfmina_s(*(const __m128 *)(const void *)a, *(const __m128 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vfmina_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfmina_d(*(const __m128d *)(const void *)a, *(const __m128d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lsx_vflogb_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vflogb_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vflogb_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vflogb_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfsqrt_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfsqrt_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfsqrt_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfsqrt_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrecip_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrecip_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrecip_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrecip_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrsqrt_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrsqrt_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrsqrt_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrsqrt_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrint_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrint_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrint_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrint_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrintrm_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrintrm_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrintrm_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrintrm_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrintrp_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrintrp_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrintrp_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrintrp_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrintrz_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrintrz_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrintrz_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrintrz_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrintrne_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrintrne_s(*(const __m128 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrintrne_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrintrne_d(*(const __m128d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfcvtl_s_h(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfcvtl_s_h(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfcvth_s_h(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfcvth_s_h(*(const __m128i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_0(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 0);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_1(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 1);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_2(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 2);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_3(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 3);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_4(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 4);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_5(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 5);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_6(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 6);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_7(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 7);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 8);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_9(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 9);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_10(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 10);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_11(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 11);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_12(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 12);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_13(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 13);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_14(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 14);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_15(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 15);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 16);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_17(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 17);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_18(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 18);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_19(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 19);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_20(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 20);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_21(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 21);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_22(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 22);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_23(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 23);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_24(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 24);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_25(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 25);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_26(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 26);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_27(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 27);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_28(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 28);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_29(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 29);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_30(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 30);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_b_31(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 31);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_0(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 0);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_1(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 1);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_2(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 2);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_3(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 3);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_4(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 4);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_5(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 5);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_6(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 6);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_7(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 7);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 8);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_9(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 9);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_10(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 10);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_11(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 11);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_12(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 12);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_13(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 13);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_14(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 14);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_15(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 15);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 16);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_17(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 17);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_18(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 18);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_19(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 19);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_20(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 20);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_21(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 21);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_22(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 22);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_23(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 23);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_24(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 24);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_25(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 25);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_26(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 26);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_27(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 27);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_28(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 28);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_29(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 29);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_30(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 30);
}

__attribute__((always_inline))
void rust_isa_lsx_vfrstpi_h_31(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 31);
}
