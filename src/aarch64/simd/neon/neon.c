#include <stdint.h>
#include <arm_neon.h>

int32_t rust_isa_neon_extract_i32_0(const uint8_t *a);
int32_t rust_isa_neon_extract_i32_1(const uint8_t *a);
int32_t rust_isa_neon_extract_i32_2(const uint8_t *a);
int32_t rust_isa_neon_extract_i32_3(const uint8_t *a);
void rust_isa_neon_replace_i32_0(const uint8_t *a, int32_t value, uint8_t *out);
void rust_isa_neon_replace_i32_1(const uint8_t *a, int32_t value, uint8_t *out);
void rust_isa_neon_replace_i32_2(const uint8_t *a, int32_t value, uint8_t *out);
void rust_isa_neon_replace_i32_3(const uint8_t *a, int32_t value, uint8_t *out);


__attribute__((always_inline))
void rust_isa_neon_v128_load(const uint8_t *p, uint8_t out[16]) {
    vst1q_u8(out, vld1q_u8(p));
}

__attribute__((always_inline))
void rust_isa_neon_v128_store(uint8_t *p, const uint8_t value[16]) {
    vst1q_u8(p, vld1q_u8(value));
}

__attribute__((always_inline))
void rust_isa_neon_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vaddq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmulq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vaddq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vmulq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_tbl(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    vst1q_u8(out, vqtbl1q_u8(vld1q_u8(a), vld1q_u8(idx)));
}

__attribute__((always_inline))
void rust_isa_neon_add_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vaddq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_add_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vaddq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_add_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vaddq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_add_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vaddq_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sub_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsubq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sub_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsubq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sub_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vsubq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sub_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vsubq_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_mul_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vmulq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_mul_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmulq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_mul_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmulq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_and(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vandq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_neon_orr(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vorrq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_neon_eor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, veorq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_neon_bic(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vbicq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_neon_orn(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vornq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_neon_not(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, vmvnq_u8(vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_neon_cmpeq_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s8(vceqq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmpeq_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s16(vceqq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmpeq_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s32(vceqq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmpgt_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s8(vcgtq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmpgt_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s16(vcgtq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmpgt_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s32(vcgtq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_min_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vminq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_max_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vmaxq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_min_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vminq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_max_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmaxq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_min_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vminq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_max_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmaxq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_abs_i8(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vabsq_s8(vld1q_s8((const int8_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_abs_i16(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vabsq_s16(vld1q_s16((const int16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_abs_i32(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vabsq_s32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_neg_i8(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vnegq_s8(vld1q_s8((const int8_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_neg_i16(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vnegq_s16(vld1q_s16((const int16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_neg_i32(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vnegq_s32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_shl_i8(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vshlq_s8(vld1q_s8((const int8_t *)(const void *)a), vdupq_n_s8((int8_t)count)));
}

__attribute__((always_inline))
void rust_isa_neon_shl_i16(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)count)));
}

__attribute__((always_inline))
void rust_isa_neon_shl_i32(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)count)));
}

__attribute__((always_inline))
void rust_isa_neon_shr_i8(const uint8_t *a, int count, uint8_t *out) {
    vst1q_u8(out, vshlq_u8(vld1q_u8(a), vdupq_n_s8((int8_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_shr_i16(const uint8_t *a, int count, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vdupq_n_s16((int16_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_shr_i32(const uint8_t *a, int count, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vdupq_n_s32((int32_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_sar_i8(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vshlq_s8(vld1q_s8((const int8_t *)(const void *)a), vdupq_n_s8((int8_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_sar_i16(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_sar_i32(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_fsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vsubq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fdiv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vdivq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fmin(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vminq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fmax(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vmaxq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fabs(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vabsq_f32(vld1q_f32((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_fneg(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vnegq_f32(vld1q_f32((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_fsqrt(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vsqrtq_f32(vld1q_f32((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_fadd_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vaddq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fmul_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vmulq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fsub_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vsubq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fdiv_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vdivq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_splat_i8(int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_n_s8(value));
}

__attribute__((always_inline))
void rust_isa_neon_splat_i16(int16_t value, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vdupq_n_s16(value));
}

__attribute__((always_inline))
void rust_isa_neon_splat_i32(int32_t value, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vdupq_n_s32(value));
}

__attribute__((always_inline))
void rust_isa_neon_splat_i64(int64_t value, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vdupq_n_s64(value));
}

__attribute__((always_inline))
void rust_isa_neon_splat_f32(float value, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vdupq_n_f32(value));
}

__attribute__((always_inline))
void rust_isa_neon_splat_f64(double value, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vdupq_n_f64(value));
}

__attribute__((always_inline))
void rust_isa_neon_zip1_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vzip1q_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_neon_uzp1_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vuzp1q_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_neon_trn1_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vtrn1q_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_neon_tbx(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    vst1q_u8(out, vqtbx1q_u8(vdupq_n_u8(0), vld1q_u8(a), vld1q_u8(idx)));
}

#define RUST_ISA_NEON_EXTRACT_I32(N) \
__attribute__((always_inline)) \
int32_t rust_isa_neon_extract_i32_##N(const uint8_t *a) { \
    return vgetq_lane_s32(vld1q_s32((const int32_t *)(const void *)a), N); \
}

RUST_ISA_NEON_EXTRACT_I32(0)
RUST_ISA_NEON_EXTRACT_I32(1)
RUST_ISA_NEON_EXTRACT_I32(2)
RUST_ISA_NEON_EXTRACT_I32(3)

#define RUST_ISA_NEON_REPLACE_I32(N) \
__attribute__((always_inline)) \
void rust_isa_neon_replace_i32_##N(const uint8_t *a, int32_t value, uint8_t *out) { \
    int32x4_t v = vld1q_s32((const int32_t *)(const void *)a); \
    v = vsetq_lane_s32(value, v, N); \
    vst1q_s32((int32_t *)(void *)out, v); \
}

RUST_ISA_NEON_REPLACE_I32(0)
RUST_ISA_NEON_REPLACE_I32(1)
RUST_ISA_NEON_REPLACE_I32(2)
RUST_ISA_NEON_REPLACE_I32(3)

__attribute__((always_inline))
void rust_isa_neon_saddl_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vaddl_s8(vld1_s8((const int8_t *)(const void *)a), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_saddl_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vaddl_s16(vld1_s16((const int16_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_saddl_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vaddl_s32(vld1_s32((const int32_t *)(const void *)a), vld1_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uaddl_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vaddl_u8(vld1_u8((const uint8_t *)(const void *)a), vld1_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uaddl_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vaddl_u16(vld1_u16((const uint16_t *)(const void *)a), vld1_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uaddl_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vaddl_u32(vld1_u32((const uint32_t *)(const void *)a), vld1_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_ssubl_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsubl_s8(vld1_s8((const int8_t *)(const void *)a), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_ssubl_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vsubl_s16(vld1_s16((const int16_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_ssubl_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vsubl_s32(vld1_s32((const int32_t *)(const void *)a), vld1_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_usubl_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vsubl_u8(vld1_u8((const uint8_t *)(const void *)a), vld1_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_usubl_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vsubl_u16(vld1_u16((const uint16_t *)(const void *)a), vld1_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_usubl_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vsubl_u32(vld1_u32((const uint32_t *)(const void *)a), vld1_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_saddw_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vaddw_s8(vld1q_s16((const int16_t *)(const void *)a), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_saddw_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vaddw_s16(vld1q_s32((const int32_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_saddw_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vaddw_s32(vld1q_s64((const int64_t *)(const void *)a), vld1_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uaddw_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vaddw_u8(vld1q_u16((const uint16_t *)(const void *)a), vld1_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uaddw_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vaddw_u16(vld1q_u32((const uint32_t *)(const void *)a), vld1_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uaddw_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vaddw_u32(vld1q_u64((const uint64_t *)(const void *)a), vld1_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_ssubw_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsubw_s8(vld1q_s16((const int16_t *)(const void *)a), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_ssubw_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vsubw_s16(vld1q_s32((const int32_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_ssubw_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vsubw_s32(vld1q_s64((const int64_t *)(const void *)a), vld1_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_usubw_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vsubw_u8(vld1q_u16((const uint16_t *)(const void *)a), vld1_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_usubw_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vsubw_u16(vld1q_u32((const uint32_t *)(const void *)a), vld1_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_usubw_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vsubw_u32(vld1q_u64((const uint64_t *)(const void *)a), vld1_u32((const uint32_t *)(const void *)b)));
}

