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
