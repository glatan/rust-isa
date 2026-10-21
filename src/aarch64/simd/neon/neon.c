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

__attribute__((always_inline))
void rust_isa_neon_bsl(const uint8_t *mask, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vbslq_u8(vld1q_u8(mask), vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_neon_bit(const uint8_t *a, const uint8_t *b, const uint8_t *mask, uint8_t *out) {
    vst1q_u8(out, vbslq_u8(vld1q_u8(mask), vld1q_u8(b), vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_neon_bif(const uint8_t *a, const uint8_t *b, const uint8_t *mask, uint8_t *out) {
    vst1q_u8(out, vbslq_u8(vld1q_u8(mask), vld1q_u8(a), vld1q_u8(b)));
}

#define RUST_ISA_NEON_UNARY(NAME, VLD, TYPE, INTRIN) \
__attribute__((always_inline)) \
void rust_isa_neon_##NAME(const uint8_t *a, uint8_t *out) { \
    vst1q_u8(out, (uint8x16_t)INTRIN(vld1q_##VLD((const TYPE *)(const void *)a))); \
}

RUST_ISA_NEON_UNARY(cls_i8, s8, int8_t, vclsq_s8)
RUST_ISA_NEON_UNARY(cls_i16, s16, int16_t, vclsq_s16)
RUST_ISA_NEON_UNARY(cls_i32, s32, int32_t, vclsq_s32)
RUST_ISA_NEON_UNARY(clz_i8, s8, int8_t, vclzq_s8)
RUST_ISA_NEON_UNARY(clz_i16, s16, int16_t, vclzq_s16)
RUST_ISA_NEON_UNARY(clz_i32, s32, int32_t, vclzq_s32)
RUST_ISA_NEON_UNARY(cnt_i8, u8, uint8_t, vcntq_u8)
RUST_ISA_NEON_UNARY(rbit, u8, uint8_t, vrbitq_u8)
RUST_ISA_NEON_UNARY(rev16, u8, uint8_t, vrev16q_u8)
RUST_ISA_NEON_UNARY(rev32, u8, uint8_t, vrev32q_u8)
RUST_ISA_NEON_UNARY(rev64, u8, uint8_t, vrev64q_u8)

__attribute__((always_inline))
void rust_isa_neon_cmpeq_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, (uint8x16_t)vceqq_s64(
        vld1q_s64((const int64_t *)(const void *)a),
        vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_cmpgt_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, (uint8x16_t)vcgtq_s64(
        vld1q_s64((const int64_t *)(const void *)a),
        vld1q_s64((const int64_t *)(const void *)b)));
}

void rust_isa_neon_cls_i8(const uint8_t *a, uint8_t *out);
void rust_isa_neon_cls_i16(const uint8_t *a, uint8_t *out);
void rust_isa_neon_cls_i32(const uint8_t *a, uint8_t *out);
void rust_isa_neon_clz_i8(const uint8_t *a, uint8_t *out);
void rust_isa_neon_clz_i16(const uint8_t *a, uint8_t *out);
void rust_isa_neon_clz_i32(const uint8_t *a, uint8_t *out);
void rust_isa_neon_cnt_i8(const uint8_t *a, uint8_t *out);
void rust_isa_neon_rbit(const uint8_t *a, uint8_t *out);
void rust_isa_neon_rev16(const uint8_t *a, uint8_t *out);
void rust_isa_neon_rev32(const uint8_t *a, uint8_t *out);
void rust_isa_neon_rev64(const uint8_t *a, uint8_t *out);
__attribute__((always_inline))
void rust_isa_neon_cmge_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s8(vcgeq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmge_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s16(vcgeq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmge_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s32(vcgeq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmle_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s8(vcleq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmle_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s16(vcleq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmle_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s32(vcleq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmlt_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s8(vcltq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmlt_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s16(vcltq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmlt_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s32(vcltq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmtst_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s8(vtstq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmtst_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s16(vtstq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_cmtst_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s32(vtstq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_fcmeq_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u32(vceqq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_fcmeq_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u64(vceqq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_fcmge_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u32(vcgeq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_fcmge_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u64(vcgeq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_fcmgt_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u32(vcgtq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_fcmgt_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u64(vcgtq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_fcmle_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u32(vcleq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_fcmle_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u64(vcleq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_fcmlt_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u32(vcltq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_fcmlt_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u64(vcltq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_facge_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u32(vcageq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_facge_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u64(vcageq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_facgt_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u32(vcagtq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_facgt_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_u64(vcagtq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_neon_faddp_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vpaddq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_faddp_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vpaddq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fmaxnm_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vmaxnmq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fmaxnm_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vmaxnmq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fminnm_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vminnmq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fminnm_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vminnmq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fmla_f32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vmlaq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b), vld1q_f32((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_neon_fmla_f64(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vmlaq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b), vld1q_f64((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_neon_fmls_f32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vmlsq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b), vld1q_f32((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_neon_fmls_f64(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vmlsq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b), vld1q_f64((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_neon_fmulx_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vmulxq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fmulx_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vmulxq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fabd_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vabdq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fabd_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vabdq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b)));
}

__attribute__((always_inline))
float rust_isa_neon_faddv_f32(const uint8_t *a) {
    return vaddvq_f32(vld1q_f32((const float *)(const void *)a));
}

__attribute__((always_inline))
double rust_isa_neon_faddv_f64(const uint8_t *a) {
    return vaddvq_f64(vld1q_f64((const double *)(const void *)a));
}

__attribute__((always_inline))
float rust_isa_neon_fmaxnmv_f32(const uint8_t *a) {
    return vmaxnmvq_f32(vld1q_f32((const float *)(const void *)a));
}

__attribute__((always_inline))
double rust_isa_neon_fmaxnmv_f64(const uint8_t *a) {
    return vmaxnmvq_f64(vld1q_f64((const double *)(const void *)a));
}

__attribute__((always_inline))
float rust_isa_neon_fmaxv_f32(const uint8_t *a) {
    return vmaxvq_f32(vld1q_f32((const float *)(const void *)a));
}

__attribute__((always_inline))
double rust_isa_neon_fmaxv_f64(const uint8_t *a) {
    return vmaxvq_f64(vld1q_f64((const double *)(const void *)a));
}

__attribute__((always_inline))
float rust_isa_neon_fminnmv_f32(const uint8_t *a) {
    return vminnmvq_f32(vld1q_f32((const float *)(const void *)a));
}

__attribute__((always_inline))
double rust_isa_neon_fminnmv_f64(const uint8_t *a) {
    return vminnmvq_f64(vld1q_f64((const double *)(const void *)a));
}

__attribute__((always_inline))
float rust_isa_neon_fminv_f32(const uint8_t *a) {
    return vminvq_f32(vld1q_f32((const float *)(const void *)a));
}

__attribute__((always_inline))
double rust_isa_neon_fminv_f64(const uint8_t *a) {
    return vminvq_f64(vld1q_f64((const double *)(const void *)a));
}

__attribute__((always_inline))
void rust_isa_neon_addp_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vpaddq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_addp_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vpaddq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_addp_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vpaddq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
int8_t rust_isa_neon_addv_s8(const uint8_t *a) {
    return (int8_t)vaddvq_s8(vld1q_s8((const int8_t *)(const void *)a));
}

__attribute__((always_inline))
int16_t rust_isa_neon_addv_s16(const uint8_t *a) {
    return (int16_t)vaddvq_s16(vld1q_s16((const int16_t *)(const void *)a));
}

__attribute__((always_inline))
int32_t rust_isa_neon_addv_s32(const uint8_t *a) {
    return (int32_t)vaddvq_s32(vld1q_s32((const int32_t *)(const void *)a));
}

__attribute__((always_inline))
int8_t rust_isa_neon_smaxv_s8(const uint8_t *a) {
    return (int8_t)vmaxvq_s8(vld1q_s8((const int8_t *)(const void *)a));
}

__attribute__((always_inline))
int16_t rust_isa_neon_smaxv_s16(const uint8_t *a) {
    return (int16_t)vmaxvq_s16(vld1q_s16((const int16_t *)(const void *)a));
}

__attribute__((always_inline))
int32_t rust_isa_neon_smaxv_s32(const uint8_t *a) {
    return (int32_t)vmaxvq_s32(vld1q_s32((const int32_t *)(const void *)a));
}

__attribute__((always_inline))
int8_t rust_isa_neon_sminv_s8(const uint8_t *a) {
    return (int8_t)vminvq_s8(vld1q_s8((const int8_t *)(const void *)a));
}

__attribute__((always_inline))
int16_t rust_isa_neon_sminv_s16(const uint8_t *a) {
    return (int16_t)vminvq_s16(vld1q_s16((const int16_t *)(const void *)a));
}

__attribute__((always_inline))
int32_t rust_isa_neon_sminv_s32(const uint8_t *a) {
    return (int32_t)vminvq_s32(vld1q_s32((const int32_t *)(const void *)a));
}

__attribute__((always_inline))
uint8_t rust_isa_neon_umaxv_u8(const uint8_t *a) {
    return (uint8_t)vmaxvq_u8(vld1q_u8((const uint8_t *)(const void *)a));
}

__attribute__((always_inline))
uint16_t rust_isa_neon_umaxv_u16(const uint8_t *a) {
    return (uint16_t)vmaxvq_u16(vld1q_u16((const uint16_t *)(const void *)a));
}

__attribute__((always_inline))
uint32_t rust_isa_neon_umaxv_u32(const uint8_t *a) {
    return (uint32_t)vmaxvq_u32(vld1q_u32((const uint32_t *)(const void *)a));
}

__attribute__((always_inline))
uint8_t rust_isa_neon_uminv_u8(const uint8_t *a) {
    return (uint8_t)vminvq_u8(vld1q_u8((const uint8_t *)(const void *)a));
}

__attribute__((always_inline))
uint16_t rust_isa_neon_uminv_u16(const uint8_t *a) {
    return (uint16_t)vminvq_u16(vld1q_u16((const uint16_t *)(const void *)a));
}

__attribute__((always_inline))
uint32_t rust_isa_neon_uminv_u32(const uint8_t *a) {
    return (uint32_t)vminvq_u32(vld1q_u32((const uint32_t *)(const void *)a));
}

__attribute__((always_inline))
void rust_isa_neon_smax_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vmaxq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smax_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmaxq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smax_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmaxq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smin_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vminq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smin_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vminq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smin_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vminq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umax_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vmaxq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umax_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vmaxq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umax_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmaxq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umin_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vminq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umin_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vminq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umin_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vminq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smaxp_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vpmaxq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smaxp_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vpmaxq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smaxp_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vpmaxq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sminp_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vpminq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sminp_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vpminq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sminp_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vpminq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umaxp_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vpmaxq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umaxp_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vpmaxq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umaxp_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vpmaxq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uminp_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vpminq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uminp_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vpminq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uminp_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vpminq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_mla_s8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vmlaq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b), vld1q_s8((const int8_t *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_neon_mla_s16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmlaq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b), vld1q_s16((const int16_t *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_neon_mla_s32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmlaq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b), vld1q_s32((const int32_t *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_neon_mls_s8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vmlsq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b), vld1q_s8((const int8_t *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_neon_mls_s16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmlsq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b), vld1q_s16((const int16_t *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_neon_mls_s32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmlsq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b), vld1q_s32((const int32_t *)(const void *)c)));
}


/* Additional NEON instructions: widening, narrowing, saturating, and shifts. */

__attribute__((always_inline))
void rust_isa_neon_abs_i64(const uint8_t *a, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vabsq_s64(vld1q_s64((const int64_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_sabd_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vabdq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sabd_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vabdq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sabd_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vabdq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uabd_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vabdq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uabd_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vabdq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uabd_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vabdq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sabdl_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vabdl_s8(vld1_s8((const int8_t *)(const void *)a), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sabdl_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vabdl_s16(vld1_s16((const int16_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sabdl_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vabdl_s32(vld1_s32((const int32_t *)(const void *)a), vld1_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uabdl_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vabdl_u8(vld1_u8((const uint8_t *)(const void *)a), vld1_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uabdl_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vabdl_u16(vld1_u16((const uint16_t *)(const void *)a), vld1_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uabdl_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vabdl_u32(vld1_u32((const uint32_t *)(const void *)a), vld1_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_shadd_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vhaddq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_shsub_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vhsubq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_srhadd_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vrhaddq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_shadd_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vhaddq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_shsub_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vhsubq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_srhadd_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vrhaddq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_shadd_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vhaddq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_shsub_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vhsubq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_srhadd_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vrhaddq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uhadd_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vhaddq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uhsub_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vhsubq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_urhadd_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vrhaddq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uhadd_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vhaddq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uhsub_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vhsubq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_urhadd_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vrhaddq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uhadd_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vhaddq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uhsub_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vhsubq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_urhadd_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vrhaddq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smull_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmull_s8(vld1_s8((const int8_t *)(const void *)a), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smull2_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmull_high_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlal_s8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmlal_s8(vld1q_s16((const int16_t *)(const void *)c), vld1_s8((const int8_t *)(const void *)a), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlal2_s8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmlal_high_s8(vld1q_s16((const int16_t *)(const void *)c), vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlsl_s8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmlsl_s8(vld1q_s16((const int16_t *)(const void *)c), vld1_s8((const int8_t *)(const void *)a), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlsl2_s8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmlsl_high_s8(vld1q_s16((const int16_t *)(const void *)c), vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smull_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmull_s16(vld1_s16((const int16_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smull2_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmull_high_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlal_s16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmlal_s16(vld1q_s32((const int32_t *)(const void *)c), vld1_s16((const int16_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlal2_s16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmlal_high_s16(vld1q_s32((const int32_t *)(const void *)c), vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlsl_s16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmlsl_s16(vld1q_s32((const int32_t *)(const void *)c), vld1_s16((const int16_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlsl2_s16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmlsl_high_s16(vld1q_s32((const int32_t *)(const void *)c), vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smull_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vmull_s32(vld1_s32((const int32_t *)(const void *)a), vld1_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smull2_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vmull_high_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlal_s32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vmlal_s32(vld1q_s64((const int64_t *)(const void *)c), vld1_s32((const int32_t *)(const void *)a), vld1_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlal2_s32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vmlal_high_s32(vld1q_s64((const int64_t *)(const void *)c), vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlsl_s32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vmlsl_s32(vld1q_s64((const int64_t *)(const void *)c), vld1_s32((const int32_t *)(const void *)a), vld1_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_smlsl2_s32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vmlsl_high_s32(vld1q_s64((const int64_t *)(const void *)c), vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umull_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vmull_u8(vld1_u8((const uint8_t *)(const void *)a), vld1_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umull2_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vmull_high_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlal_u8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vmlal_u8(vld1q_u16((const uint16_t *)(const void *)c), vld1_u8((const uint8_t *)(const void *)a), vld1_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlal2_u8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vmlal_high_u8(vld1q_u16((const uint16_t *)(const void *)c), vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlsl_u8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vmlsl_u8(vld1q_u16((const uint16_t *)(const void *)c), vld1_u8((const uint8_t *)(const void *)a), vld1_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlsl2_u8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vmlsl_high_u8(vld1q_u16((const uint16_t *)(const void *)c), vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umull_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmull_u16(vld1_u16((const uint16_t *)(const void *)a), vld1_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umull2_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmull_high_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlal_u16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmlal_u16(vld1q_u32((const uint32_t *)(const void *)c), vld1_u16((const uint16_t *)(const void *)a), vld1_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlal2_u16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmlal_high_u16(vld1q_u32((const uint32_t *)(const void *)c), vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlsl_u16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmlsl_u16(vld1q_u32((const uint32_t *)(const void *)c), vld1_u16((const uint16_t *)(const void *)a), vld1_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlsl2_u16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmlsl_high_u16(vld1q_u32((const uint32_t *)(const void *)c), vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umull_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vmull_u32(vld1_u32((const uint32_t *)(const void *)a), vld1_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umull2_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vmull_high_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlal_u32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vmlal_u32(vld1q_u64((const uint64_t *)(const void *)c), vld1_u32((const uint32_t *)(const void *)a), vld1_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlal2_u32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vmlal_high_u32(vld1q_u64((const uint64_t *)(const void *)c), vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlsl_u32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vmlsl_u32(vld1q_u64((const uint64_t *)(const void *)c), vld1_u32((const uint32_t *)(const void *)a), vld1_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_umlsl2_u32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vmlsl_high_u32(vld1q_u64((const uint64_t *)(const void *)c), vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmull_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqdmull_s16(vld1_s16((const int16_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmull2_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqdmull_high_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmlal_s16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqdmlal_s16(vld1q_s32((const int32_t *)(const void *)c), vld1_s16((const int16_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmlal2_s16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqdmlal_high_s16(vld1q_s32((const int32_t *)(const void *)c), vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmlsl_s16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqdmlsl_s16(vld1q_s32((const int32_t *)(const void *)c), vld1_s16((const int16_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmlsl2_s16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqdmlsl_high_s16(vld1q_s32((const int32_t *)(const void *)c), vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmulh_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vqdmulhq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqrdmulh_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vqrdmulhq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmull_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vqdmull_s32(vld1_s32((const int32_t *)(const void *)a), vld1_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmull2_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vqdmull_high_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmlal_s32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vqdmlal_s32(vld1q_s64((const int64_t *)(const void *)c), vld1_s32((const int32_t *)(const void *)a), vld1_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmlal2_s32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vqdmlal_high_s32(vld1q_s64((const int64_t *)(const void *)c), vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmlsl_s32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vqdmlsl_s32(vld1q_s64((const int64_t *)(const void *)c), vld1_s32((const int32_t *)(const void *)a), vld1_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmlsl2_s32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vqdmlsl_high_s32(vld1q_s64((const int64_t *)(const void *)c), vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqdmulh_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqdmulhq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqrdmulh_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqrdmulhq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqadd_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vqaddq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqsub_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vqsubq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqadd_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vqaddq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqsub_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vqsubq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqadd_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqaddq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqsub_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqsubq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqadd_s64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vqaddq_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqsub_s64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vqsubq_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqadd_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vqaddq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqsub_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vqsubq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqadd_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vqaddq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqsub_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vqsubq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqadd_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vqaddq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqsub_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vqsubq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqadd_u64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vqaddq_u64(vld1q_u64((const uint64_t *)(const void *)a), vld1q_u64((const uint64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqsub_u64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vqsubq_u64(vld1q_u64((const uint64_t *)(const void *)a), vld1q_u64((const uint64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqabs_s8(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vqabsq_s8(vld1q_s8((const int8_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_sqneg_s8(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vqnegq_s8(vld1q_s8((const int8_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_sqabs_s16(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vqabsq_s16(vld1q_s16((const int16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_sqneg_s16(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vqnegq_s16(vld1q_s16((const int16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_sqabs_s32(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqabsq_s32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_sqneg_s32(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqnegq_s32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn_s8(const uint8_t *a, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vmovn_s16(vld1q_s16((const int16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn2_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vmovn_high_s16(vld1_s8((const int8_t *)(const void *)b), vld1q_s16((const int16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn_s16(const uint8_t *a, uint8_t *out) {
    vst1_s16((int16_t *)(void *)out, vmovn_s32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn2_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmovn_high_s32(vld1_s16((const int16_t *)(const void *)b), vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn_s32(const uint8_t *a, uint8_t *out) {
    vst1_s32((int32_t *)(void *)out, vmovn_s64(vld1q_s64((const int64_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn2_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmovn_high_s64(vld1_s32((const int32_t *)(const void *)b), vld1q_s64((const int64_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn_u8(const uint8_t *a, uint8_t *out) {
    vst1_u8((uint8_t *)(void *)out, vmovn_u16(vld1q_u16((const uint16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn2_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vmovn_high_u16(vld1_u8((const uint8_t *)(const void *)b), vld1q_u16((const uint16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn_u16(const uint8_t *a, uint8_t *out) {
    vst1_u16((uint16_t *)(void *)out, vmovn_u32(vld1q_u32((const uint32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn2_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vmovn_high_u32(vld1_u16((const uint16_t *)(const void *)b), vld1q_u32((const uint32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn_u32(const uint8_t *a, uint8_t *out) {
    vst1_u32((uint32_t *)(void *)out, vmovn_u64(vld1q_u64((const uint64_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_xtn2_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmovn_high_u64(vld1_u32((const uint32_t *)(const void *)b), vld1q_u64((const uint64_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_addhn_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vaddhn_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_addhn2_i16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vaddhn_high_s16(vld1_s8((const int8_t *)(const void *)c), vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_addhn_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s16((int16_t *)(void *)out, vaddhn_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_addhn2_i32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vaddhn_high_s32(vld1_s16((const int16_t *)(const void *)c), vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_addhn_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s32((int32_t *)(void *)out, vaddhn_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_addhn2_i64(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vaddhn_high_s64(vld1_s32((const int32_t *)(const void *)c), vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_raddhn_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vraddhn_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_raddhn2_i16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vraddhn_high_s16(vld1_s8((const int8_t *)(const void *)c), vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_raddhn_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s16((int16_t *)(void *)out, vraddhn_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_raddhn2_i32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vraddhn_high_s32(vld1_s16((const int16_t *)(const void *)c), vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_raddhn_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s32((int32_t *)(void *)out, vraddhn_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_raddhn2_i64(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vraddhn_high_s64(vld1_s32((const int32_t *)(const void *)c), vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_subhn_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vsubhn_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_subhn2_i16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsubhn_high_s16(vld1_s8((const int8_t *)(const void *)c), vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_subhn_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s16((int16_t *)(void *)out, vsubhn_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_subhn2_i32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsubhn_high_s32(vld1_s16((const int16_t *)(const void *)c), vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_subhn_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s32((int32_t *)(void *)out, vsubhn_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_subhn2_i64(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vsubhn_high_s64(vld1_s32((const int32_t *)(const void *)c), vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_rsubhn_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vrsubhn_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_rsubhn2_i16(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vrsubhn_high_s16(vld1_s8((const int8_t *)(const void *)c), vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_rsubhn_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s16((int16_t *)(void *)out, vrsubhn_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_rsubhn2_i32(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vrsubhn_high_s32(vld1_s16((const int16_t *)(const void *)c), vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_rsubhn_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s32((int32_t *)(void *)out, vrsubhn_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_rsubhn2_i64(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vrsubhn_high_s64(vld1_s32((const int32_t *)(const void *)c), vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_pmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_p8((poly8_t *)(void *)out, vmulq_p8(vld1q_p8((const poly8_t *)(const void *)a), vld1q_p8((const poly8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_pmull_p8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_p16((poly16_t *)(void *)out, vmull_p8(vld1_p8((const poly8_t *)(const void *)a), vld1_p8((const poly8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_pmull2_p8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_p16((poly16_t *)(void *)out, vmull_high_p8(vld1q_p8((const poly8_t *)(const void *)a), vld1q_p8((const poly8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_frecpe_f32(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vrecpeq_f32(vld1q_f32((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_frsqrte_f32(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vrsqrteq_f32(vld1q_f32((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_frecps_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vrecpsq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_frsqrts_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vrsqrtsq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_frecpe_f64(const uint8_t *a, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vrecpeq_f64(vld1q_f64((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_frsqrte_f64(const uint8_t *a, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vrsqrteq_f64(vld1q_f64((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_frecps_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vrecpsq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_frsqrts_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vrsqrtsq_f64(vld1q_f64((const double *)(const void *)a), vld1q_f64((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_urecpe_u32(const uint8_t *a, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vrecpeq_u32(vld1q_u32((const uint32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_ursqrte_u32(const uint8_t *a, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vrsqrteq_u32(vld1q_u32((const uint32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_shl_i64(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)count)));
}

__attribute__((always_inline))
float rust_isa_neon_frecpx_f32(float a) {
    return vrecpxs_f32(a);
}

__attribute__((always_inline))
double rust_isa_neon_frecpx_f64(double a) {
    return vrecpxd_f64(a);
}

__attribute__((always_inline))
void rust_isa_neon_scvtf_f32_s32(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vcvtq_f32_s32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_ucvtf_f32_u32(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vcvtq_f32_u32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_scvtf_f64_s64(const uint8_t *a, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vcvtq_f64_s64(vld1q_s64((const int64_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_ucvtf_f64_u64(const uint8_t *a, uint8_t *out) {
    vst1q_f64((double *)(void *)out, vcvtq_f64_u64(vld1q_s64((const int64_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_fcvtzs_s32_f32(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vcvtq_s32_f32(vld1q_f32((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_fcvtzu_u32_f32(const uint8_t *a, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vcvtq_u32_f32(vld1q_f32((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_fcvtzs_s64_f64(const uint8_t *a, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vcvtq_s64_f64(vld1q_f64((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_fcvtzu_u64_f64(const uint8_t *a, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vcvtq_u64_f64(vld1q_f64((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_neon_shl_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vshlq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sshl_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vshlq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_srshl_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vrshlq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqshl_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vqshlq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshl_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vqrshlq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_shl_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sshl_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_srshl_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vrshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqshl_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vqshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshl_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vqrshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_shl_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sshl_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_srshl_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vrshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqshl_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshl_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqrshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_shl_s64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sshl_s64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_srshl_s64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vrshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqshl_s64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vqshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshl_s64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vqrshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_ushl_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vshlq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_urshl_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vrshlq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqshl_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vqshlq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqrshl_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vqrshlq_u8(vld1q_u8((const uint8_t *)(const void *)a), vld1q_u8((const uint8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_ushl_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_urshl_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vrshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqshl_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vqshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqrshl_u16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vqrshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vld1q_u16((const uint16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_ushl_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_urshl_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vrshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqshl_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vqshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqrshl_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vqrshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_ushl_u64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vld1q_u64((const uint64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_urshl_u64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vrshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vld1q_u64((const uint64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqshl_u64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vqshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vld1q_u64((const uint64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_uqrshl_u64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vqrshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vld1q_u64((const uint64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_sshr_s8(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vshlq_s8(vld1q_s8((const int8_t *)(const void *)a), vdupq_n_s8((int8_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_srshr_s8(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vrshlq_s8(vld1q_s8((const int8_t *)(const void *)a), vdupq_n_s8((int8_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_ssra_s8(const uint8_t *a, int count, const uint8_t *c, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vaddq_s8(vld1q_s8((const int8_t *)(const void *)c), vshlq_s8(vld1q_s8((const int8_t *)(const void *)a), vdupq_n_s8((int8_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_srsra_s8(const uint8_t *a, int count, const uint8_t *c, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vaddq_s8(vld1q_s8((const int8_t *)(const void *)c), vrshlq_s8(vld1q_s8((const int8_t *)(const void *)a), vdupq_n_s8((int8_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sshr_s16(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_srshr_s16(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vrshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_ssra_s16(const uint8_t *a, int count, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vaddq_s16(vld1q_s16((const int16_t *)(const void *)c), vshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_srsra_s16(const uint8_t *a, int count, const uint8_t *c, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vaddq_s16(vld1q_s16((const int16_t *)(const void *)c), vrshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sshr_s32(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_srshr_s32(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vrshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_ssra_s32(const uint8_t *a, int count, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vaddq_s32(vld1q_s32((const int32_t *)(const void *)c), vshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_srsra_s32(const uint8_t *a, int count, const uint8_t *c, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vaddq_s32(vld1q_s32((const int32_t *)(const void *)c), vrshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sshr_s64(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_srshr_s64(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vrshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count)));
}

__attribute__((always_inline))
void rust_isa_neon_ssra_s64(const uint8_t *a, int count, const uint8_t *c, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vaddq_s64(vld1q_s64((const int64_t *)(const void *)c), vshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_srsra_s64(const uint8_t *a, int count, const uint8_t *c, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vaddq_s64(vld1q_s64((const int64_t *)(const void *)c), vrshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count))));
}

/* Additional NEON instructions: immediate lanes and structure loads/stores. */

__attribute__((always_inline))
void rust_isa_neon_shrn_i16(const uint8_t *a, int count, uint8_t *out) {
    vst1_u8((uint8_t *)(void *)out, vmovn_u16(vshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vdupq_n_s16((int16_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_shrn2_i16(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vmovn_high_u16(vld1_u8((const uint8_t *)(const void *)b), vshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vdupq_n_s16((int16_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_shrn_i32(const uint8_t *a, int count, uint8_t *out) {
    vst1_u16((uint16_t *)(void *)out, vmovn_u32(vshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vdupq_n_s32((int32_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_shrn2_i32(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vmovn_high_u32(vld1_u16((const uint16_t *)(const void *)b), vshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vdupq_n_s32((int32_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_shrn_i64(const uint8_t *a, int count, uint8_t *out) {
    vst1_u32((uint32_t *)(void *)out, vmovn_u64(vshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vdupq_n_s64((int64_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_shrn2_i64(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmovn_high_u64(vld1_u32((const uint32_t *)(const void *)b), vshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vdupq_n_s64((int64_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshrn_i16(const uint8_t *a, int count, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vqmovn_s16(vshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshrn_i32(const uint8_t *a, int count, uint8_t *out) {
    vst1_s16((int16_t *)(void *)out, vqmovn_s32(vshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshrn_i64(const uint8_t *a, int count, uint8_t *out) {
    vst1_s32((int32_t *)(void *)out, vqmovn_s64(vshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshrun_i16(const uint8_t *a, int count, uint8_t *out) {
    vst1_u8((uint8_t *)(void *)out, vqmovun_s16(vshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshrun_i32(const uint8_t *a, int count, uint8_t *out) {
    vst1_u16((uint16_t *)(void *)out, vqmovun_s32(vshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshrun_i64(const uint8_t *a, int count, uint8_t *out) {
    vst1_u32((uint32_t *)(void *)out, vqmovun_s64(vshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_uqshrn_i16(const uint8_t *a, int count, uint8_t *out) {
    vst1_u8((uint8_t *)(void *)out, vqmovn_u16(vshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vdupq_n_u16((int16_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_uqshrn_i32(const uint8_t *a, int count, uint8_t *out) {
    vst1_u16((uint16_t *)(void *)out, vqmovn_u32(vshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vdupq_n_u32((int32_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_uqshrn_i64(const uint8_t *a, int count, uint8_t *out) {
    vst1_u32((uint32_t *)(void *)out, vqmovn_u64(vshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vdupq_n_u64((int64_t)-count))));
}

/* saturating narrow, high half */
__attribute__((always_inline))
void rust_isa_neon_sqshrn2_i16(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vcombine_s8(vld1_s8((const int8_t *)(const void *)b), vqmovn_s16(vshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshrn2_i32(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vcombine_s16(vld1_s16((const int16_t *)(const void *)b), vqmovn_s32(vshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshrn2_i64(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vcombine_s32(vld1_s32((const int32_t *)(const void *)b), vqmovn_s64(vshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshrun2_i16(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vcombine_u8(vld1_u8((const uint8_t *)(const void *)b), vqmovun_s16(vshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshrun2_i32(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vcombine_u16(vld1_u16((const uint16_t *)(const void *)b), vqmovun_s32(vshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshrun2_i64(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vcombine_u32(vld1_u32((const uint32_t *)(const void *)b), vqmovun_s64(vshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_uqshrn2_i16(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vcombine_u8(vld1_u8((const uint8_t *)(const void *)b), vqmovn_u16(vshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vdupq_n_u16((int16_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_uqshrn2_i32(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vcombine_u16(vld1_u16((const uint16_t *)(const void *)b), vqmovn_u32(vshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vdupq_n_u32((int32_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_uqshrn2_i64(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vcombine_u32(vld1_u32((const uint32_t *)(const void *)b), vqmovn_u64(vshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vdupq_n_u64((int64_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrn_i16(const uint8_t *a, int count, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vqmovn_s16(vrshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrn2_i16(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vcombine_s8(vld1_s8((const int8_t *)(const void *)b), vqmovn_s16(vrshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrn_i32(const uint8_t *a, int count, uint8_t *out) {
    vst1_s16((int16_t *)(void *)out, vqmovn_s32(vrshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrn2_i32(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vcombine_s16(vld1_s16((const int16_t *)(const void *)b), vqmovn_s32(vrshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrn_i64(const uint8_t *a, int count, uint8_t *out) {
    vst1_s32((int32_t *)(void *)out, vqmovn_s64(vrshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrn2_i64(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vcombine_s32(vld1_s32((const int32_t *)(const void *)b), vqmovn_s64(vrshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrun_i16(const uint8_t *a, int count, uint8_t *out) {
    vst1_u8((uint8_t *)(void *)out, vqmovun_s16(vrshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrun2_i16(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vcombine_u8(vld1_u8((const uint8_t *)(const void *)b), vqmovun_s16(vrshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vdupq_n_s16((int16_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrun_i32(const uint8_t *a, int count, uint8_t *out) {
    vst1_u16((uint16_t *)(void *)out, vqmovun_s32(vrshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrun2_i32(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vcombine_u16(vld1_u16((const uint16_t *)(const void *)b), vqmovun_s32(vrshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vdupq_n_s32((int32_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrun_i64(const uint8_t *a, int count, uint8_t *out) {
    vst1_u32((uint32_t *)(void *)out, vqmovun_s64(vrshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_sqrshrun2_i64(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vcombine_u32(vld1_u32((const uint32_t *)(const void *)b), vqmovun_s64(vrshlq_s64(vld1q_s64((const int64_t *)(const void *)a), vdupq_n_s64((int64_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_uqrshrn_i16(const uint8_t *a, int count, uint8_t *out) {
    vst1_u8((uint8_t *)(void *)out, vqmovn_u16(vrshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vdupq_n_u16((int16_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_uqrshrn2_i16(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vcombine_u8(vld1_u8((const uint8_t *)(const void *)b), vqmovn_u16(vrshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vdupq_n_u16((int16_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_uqrshrn_i32(const uint8_t *a, int count, uint8_t *out) {
    vst1_u16((uint16_t *)(void *)out, vqmovn_u32(vrshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vdupq_n_u32((int32_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_uqrshrn2_i32(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vcombine_u16(vld1_u16((const uint16_t *)(const void *)b), vqmovn_u32(vrshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vdupq_n_u32((int32_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_uqrshrn_i64(const uint8_t *a, int count, uint8_t *out) {
    vst1_u32((uint32_t *)(void *)out, vqmovn_u64(vrshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vdupq_n_u64((int64_t)-count))));
}

__attribute__((always_inline))
void rust_isa_neon_uqrshrn2_i64(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vcombine_u32(vld1_u32((const uint32_t *)(const void *)b), vqmovn_u64(vrshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vdupq_n_u64((int64_t)-count)))));
}

__attribute__((always_inline))
void rust_isa_neon_shll_i8(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vshlq_s16(vmovl_s8(vld1_s8((const int8_t *)(const void *)a)), vdupq_n_s16((int16_t)count)));
}

__attribute__((always_inline))
void rust_isa_neon_shll_i16(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vshlq_s32(vmovl_s16(vld1_s16((const int16_t *)(const void *)a)), vdupq_n_s32((int32_t)count)));
}

__attribute__((always_inline))
void rust_isa_neon_shll_i32(const uint8_t *a, int count, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vshlq_s64(vmovl_s32(vld1_s32((const int32_t *)(const void *)a)), vdupq_n_s64((int64_t)count)));
}

__attribute__((always_inline))
void rust_isa_neon_sli_s8(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vreinterpretq_s8_u8(vorrq_u8(vshlq_u8(vld1q_u8((const uint8_t *)(const void *)a), vdupq_n_s8((int8_t)count)), vandq_u8(vld1q_u8((const uint8_t *)(const void *)b), vdupq_n_u8((uint8_t)(((uint8_t)1 << count) - 1))))));
}

__attribute__((always_inline))
void rust_isa_neon_sri_s8(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vreinterpretq_s8_u8(vorrq_u8(vshlq_u8(vld1q_u8((const uint8_t *)(const void *)a), vdupq_n_s8((int8_t)-count)), vandq_u8(vld1q_u8((const uint8_t *)(const void *)b), vdupq_n_u8((uint8_t)(~((((uint8_t)1 << (8 - count)) - 1))))))));
}

__attribute__((always_inline))
void rust_isa_neon_sli_s16(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vreinterpretq_s16_u16(vorrq_u16(vshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vdupq_n_s16((int16_t)count)), vandq_u16(vld1q_u16((const uint16_t *)(const void *)b), vdupq_n_u16((uint16_t)(((uint16_t)1 << count) - 1))))));
}

__attribute__((always_inline))
void rust_isa_neon_sri_s16(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vreinterpretq_s16_u16(vorrq_u16(vshlq_u16(vld1q_u16((const uint16_t *)(const void *)a), vdupq_n_s16((int16_t)-count)), vandq_u16(vld1q_u16((const uint16_t *)(const void *)b), vdupq_n_u16((uint16_t)(~((((uint16_t)1 << (16 - count)) - 1))))))));
}

__attribute__((always_inline))
void rust_isa_neon_sli_s32(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vreinterpretq_s32_u32(vorrq_u32(vshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vdupq_n_s32((int32_t)count)), vandq_u32(vld1q_u32((const uint32_t *)(const void *)b), vdupq_n_u32((uint32_t)(((uint32_t)1 << count) - 1))))));
}

__attribute__((always_inline))
void rust_isa_neon_sri_s32(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vreinterpretq_s32_u32(vorrq_u32(vshlq_u32(vld1q_u32((const uint32_t *)(const void *)a), vdupq_n_s32((int32_t)-count)), vandq_u32(vld1q_u32((const uint32_t *)(const void *)b), vdupq_n_u32((uint32_t)(~((((uint32_t)1 << (32 - count)) - 1))))))));
}

__attribute__((always_inline))
void rust_isa_neon_sli_s64(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vreinterpretq_s64_u64(vorrq_u64(vshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vdupq_n_s64((int64_t)count)), vandq_u64(vld1q_u64((const uint64_t *)(const void *)b), vdupq_n_u64((uint64_t)(((uint64_t)1 << count) - 1))))));
}

__attribute__((always_inline))
void rust_isa_neon_sri_s64(const uint8_t *a, const uint8_t *b, int count, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vreinterpretq_s64_u64(vorrq_u64(vshlq_u64(vld1q_u64((const uint64_t *)(const void *)a), vdupq_n_s64((int64_t)-count)), vandq_u64(vld1q_u64((const uint64_t *)(const void *)b), vdupq_n_u64((uint64_t)(~((((uint64_t)1 << (64 - count)) - 1))))))));
}

__attribute__((always_inline))
void rust_isa_neon_sqshlu_s8(const uint8_t *a, int count, uint8_t *out) {
    int8x16_t x = vld1q_s8((const int8_t *)(const void *)a);
    switch (count) {
        case 0: vst1q_s8((int8_t *)(void *)out, vqshluq_n_s8(x, 0)); break;
        case 1: vst1q_s8((int8_t *)(void *)out, vqshluq_n_s8(x, 1)); break;
        case 2: vst1q_s8((int8_t *)(void *)out, vqshluq_n_s8(x, 2)); break;
        case 3: vst1q_s8((int8_t *)(void *)out, vqshluq_n_s8(x, 3)); break;
        case 4: vst1q_s8((int8_t *)(void *)out, vqshluq_n_s8(x, 4)); break;
        case 5: vst1q_s8((int8_t *)(void *)out, vqshluq_n_s8(x, 5)); break;
        case 6: vst1q_s8((int8_t *)(void *)out, vqshluq_n_s8(x, 6)); break;
        case 7: vst1q_s8((int8_t *)(void *)out, vqshluq_n_s8(x, 7)); break;
        default: vst1q_s8((int8_t *)(void *)out, vdupq_n_s8(0)); break;
    }
}

__attribute__((always_inline))
void rust_isa_neon_sqshlu_s16(const uint8_t *a, int count, uint8_t *out) {
    int16x8_t x = vld1q_s16((const int16_t *)(const void *)a);
    switch (count) {
        case 0: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 0)); break;
        case 1: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 1)); break;
        case 2: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 2)); break;
        case 3: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 3)); break;
        case 4: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 4)); break;
        case 5: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 5)); break;
        case 6: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 6)); break;
        case 7: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 7)); break;
        case 8: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 8)); break;
        case 9: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 9)); break;
        case 10: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 10)); break;
        case 11: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 11)); break;
        case 12: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 12)); break;
        case 13: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 13)); break;
        case 14: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 14)); break;
        case 15: vst1q_s16((int16_t *)(void *)out, vqshluq_n_s16(x, 15)); break;
        default: vst1q_s16((int16_t *)(void *)out, vdupq_n_s16(0)); break;
    }
}

__attribute__((always_inline))
void rust_isa_neon_sqshlu_s32(const uint8_t *a, int count, uint8_t *out) {
    int32x4_t x = vld1q_s32((const int32_t *)(const void *)a);
    switch (count) {
        case 0: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 0)); break;
        case 1: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 1)); break;
        case 2: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 2)); break;
        case 3: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 3)); break;
        case 4: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 4)); break;
        case 5: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 5)); break;
        case 6: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 6)); break;
        case 7: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 7)); break;
        case 8: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 8)); break;
        case 9: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 9)); break;
        case 10: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 10)); break;
        case 11: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 11)); break;
        case 12: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 12)); break;
        case 13: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 13)); break;
        case 14: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 14)); break;
        case 15: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 15)); break;
        case 16: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 16)); break;
        case 17: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 17)); break;
        case 18: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 18)); break;
        case 19: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 19)); break;
        case 20: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 20)); break;
        case 21: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 21)); break;
        case 22: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 22)); break;
        case 23: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 23)); break;
        case 24: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 24)); break;
        case 25: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 25)); break;
        case 26: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 26)); break;
        case 27: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 27)); break;
        case 28: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 28)); break;
        case 29: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 29)); break;
        case 30: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 30)); break;
        case 31: vst1q_s32((int32_t *)(void *)out, vqshluq_n_s32(x, 31)); break;
        default: vst1q_s32((int32_t *)(void *)out, vdupq_n_s32(0)); break;
    }
}

__attribute__((always_inline))
void rust_isa_neon_sqshlu_s64(const uint8_t *a, int count, uint8_t *out) {
    int64x2_t x = vld1q_s64((const int64_t *)(const void *)a);
    switch (count) {
        case 0: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 0)); break;
        case 1: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 1)); break;
        case 2: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 2)); break;
        case 3: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 3)); break;
        case 4: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 4)); break;
        case 5: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 5)); break;
        case 6: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 6)); break;
        case 7: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 7)); break;
        case 8: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 8)); break;
        case 9: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 9)); break;
        case 10: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 10)); break;
        case 11: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 11)); break;
        case 12: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 12)); break;
        case 13: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 13)); break;
        case 14: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 14)); break;
        case 15: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 15)); break;
        case 16: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 16)); break;
        case 17: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 17)); break;
        case 18: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 18)); break;
        case 19: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 19)); break;
        case 20: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 20)); break;
        case 21: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 21)); break;
        case 22: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 22)); break;
        case 23: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 23)); break;
        case 24: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 24)); break;
        case 25: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 25)); break;
        case 26: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 26)); break;
        case 27: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 27)); break;
        case 28: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 28)); break;
        case 29: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 29)); break;
        case 30: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 30)); break;
        case 31: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 31)); break;
        case 32: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 32)); break;
        case 33: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 33)); break;
        case 34: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 34)); break;
        case 35: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 35)); break;
        case 36: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 36)); break;
        case 37: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 37)); break;
        case 38: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 38)); break;
        case 39: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 39)); break;
        case 40: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 40)); break;
        case 41: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 41)); break;
        case 42: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 42)); break;
        case 43: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 43)); break;
        case 44: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 44)); break;
        case 45: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 45)); break;
        case 46: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 46)); break;
        case 47: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 47)); break;
        case 48: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 48)); break;
        case 49: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 49)); break;
        case 50: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 50)); break;
        case 51: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 51)); break;
        case 52: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 52)); break;
        case 53: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 53)); break;
        case 54: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 54)); break;
        case 55: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 55)); break;
        case 56: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 56)); break;
        case 57: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 57)); break;
        case 58: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 58)); break;
        case 59: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 59)); break;
        case 60: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 60)); break;
        case 61: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 61)); break;
        case 62: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 62)); break;
        case 63: vst1q_s64((int64_t *)(void *)out, vqshluq_n_s64(x, 63)); break;
        default: vst1q_s64((int64_t *)(void *)out, vdupq_n_s64(0)); break;
    }
}

__attribute__((always_inline))
uint64_t rust_isa_neon_cnt_u64(uint64_t a) { return (uint64_t)__builtin_popcountll(a); }
__attribute__((always_inline))
uint64_t rust_isa_neon_clz_u64(uint64_t a) { return a == 0 ? 64 : (uint64_t)__builtin_clzll(a); }
__attribute__((always_inline))
uint64_t rust_isa_neon_cls_u64(uint64_t a) {
    uint64_t sign = (uint64_t)((int64_t)a >> 63);
    return (uint64_t)__builtin_clzll(a ^ sign) - 1;
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_0(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 0));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_0(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 0));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_1(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 1));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_1(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 1));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_2(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 2));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_2(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 2));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_3(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 3));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_3(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 3));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_4(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 4));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_4(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 4));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_5(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 5));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_5(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 5));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_6(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 6));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_6(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 6));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_7(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 7));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_7(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 7));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_8(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 8));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_8(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 8));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_9(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 9));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_9(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 9));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_10(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 10));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_10(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 10));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_11(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 11));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_11(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 11));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_12(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 12));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_12(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 12));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_13(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 13));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_13(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 13));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_14(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 14));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_14(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 14));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s8_15(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_laneq_s8(vld1q_s8((const int8_t *)(const void *)a), 15));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s8_15(const uint8_t *a, int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsetq_lane_s8(value, vld1q_s8((const int8_t *)(const void *)a), 15));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s16_0(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vdupq_laneq_s16(vld1q_s16((const int16_t *)(const void *)a), 0));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s16_0(const uint8_t *a, int16_t value, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsetq_lane_s16(value, vld1q_s16((const int16_t *)(const void *)a), 0));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s16_1(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vdupq_laneq_s16(vld1q_s16((const int16_t *)(const void *)a), 1));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s16_1(const uint8_t *a, int16_t value, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsetq_lane_s16(value, vld1q_s16((const int16_t *)(const void *)a), 1));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s16_2(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vdupq_laneq_s16(vld1q_s16((const int16_t *)(const void *)a), 2));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s16_2(const uint8_t *a, int16_t value, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsetq_lane_s16(value, vld1q_s16((const int16_t *)(const void *)a), 2));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s16_3(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vdupq_laneq_s16(vld1q_s16((const int16_t *)(const void *)a), 3));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s16_3(const uint8_t *a, int16_t value, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsetq_lane_s16(value, vld1q_s16((const int16_t *)(const void *)a), 3));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s16_4(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vdupq_laneq_s16(vld1q_s16((const int16_t *)(const void *)a), 4));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s16_4(const uint8_t *a, int16_t value, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsetq_lane_s16(value, vld1q_s16((const int16_t *)(const void *)a), 4));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s16_5(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vdupq_laneq_s16(vld1q_s16((const int16_t *)(const void *)a), 5));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s16_5(const uint8_t *a, int16_t value, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsetq_lane_s16(value, vld1q_s16((const int16_t *)(const void *)a), 5));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s16_6(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vdupq_laneq_s16(vld1q_s16((const int16_t *)(const void *)a), 6));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s16_6(const uint8_t *a, int16_t value, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsetq_lane_s16(value, vld1q_s16((const int16_t *)(const void *)a), 6));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s16_7(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vdupq_laneq_s16(vld1q_s16((const int16_t *)(const void *)a), 7));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s16_7(const uint8_t *a, int16_t value, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsetq_lane_s16(value, vld1q_s16((const int16_t *)(const void *)a), 7));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s32_0(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vdupq_laneq_s32(vld1q_s32((const int32_t *)(const void *)a), 0));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s32_0(const uint8_t *a, int32_t value, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vsetq_lane_s32(value, vld1q_s32((const int32_t *)(const void *)a), 0));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s32_1(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vdupq_laneq_s32(vld1q_s32((const int32_t *)(const void *)a), 1));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s32_1(const uint8_t *a, int32_t value, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vsetq_lane_s32(value, vld1q_s32((const int32_t *)(const void *)a), 1));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s32_2(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vdupq_laneq_s32(vld1q_s32((const int32_t *)(const void *)a), 2));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s32_2(const uint8_t *a, int32_t value, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vsetq_lane_s32(value, vld1q_s32((const int32_t *)(const void *)a), 2));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s32_3(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vdupq_laneq_s32(vld1q_s32((const int32_t *)(const void *)a), 3));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s32_3(const uint8_t *a, int32_t value, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vsetq_lane_s32(value, vld1q_s32((const int32_t *)(const void *)a), 3));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s64_0(const uint8_t *a, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vdupq_laneq_s64(vld1q_s64((const int64_t *)(const void *)a), 0));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s64_0(const uint8_t *a, int64_t value, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vsetq_lane_s64(value, vld1q_s64((const int64_t *)(const void *)a), 0));
}

__attribute__((always_inline))
void rust_isa_neon_dup_s64_1(const uint8_t *a, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vdupq_laneq_s64(vld1q_s64((const int64_t *)(const void *)a), 1));
}

__attribute__((always_inline))
void rust_isa_neon_ins_s64_1(const uint8_t *a, int64_t value, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vsetq_lane_s64(value, vld1q_s64((const int64_t *)(const void *)a), 1));
}

__attribute__((always_inline))
void rust_isa_neon_ext(const uint8_t *a, const uint8_t *b, int index, uint8_t *out) {
    switch (index) {
        case 0: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 0)); break;
        case 1: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 1)); break;
        case 2: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 2)); break;
        case 3: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 3)); break;
        case 4: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 4)); break;
        case 5: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 5)); break;
        case 6: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 6)); break;
        case 7: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 7)); break;
        case 8: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 8)); break;
        case 9: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 9)); break;
        case 10: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 10)); break;
        case 11: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 11)); break;
        case 12: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 12)); break;
        case 13: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 13)); break;
        case 14: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 14)); break;
        case 15: vst1q_u8(out, vextq_u8(vld1q_u8(a), vld1q_u8(b), 15)); break;
        default: vst1q_u8(out, vdupq_n_u8(0)); break;
    }
}

__attribute__((always_inline))
void rust_isa_neon_ld2(const uint8_t *p, uint8_t *o0, uint8_t *o1) {
    uint8x16x2_t t = vld2q_u8(p);
    vst1q_u8(o0, t.val[0]);
    vst1q_u8(o1, t.val[1]);
}

__attribute__((always_inline))
void rust_isa_neon_st2(uint8_t *p, const uint8_t *v0, const uint8_t *v1) {
    vst2q_u8(p, ((uint8x16x2_t){vld1q_u8(v0), vld1q_u8(v1)}));
}

__attribute__((always_inline))
void rust_isa_neon_ld3(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2) {
    uint8x16x3_t t = vld3q_u8(p);
    vst1q_u8(o0, t.val[0]);
    vst1q_u8(o1, t.val[1]);
    vst1q_u8(o2, t.val[2]);
}

__attribute__((always_inline))
void rust_isa_neon_st3(uint8_t *p, const uint8_t *v0, const uint8_t *v1, const uint8_t *v2) {
    vst3q_u8(p, ((uint8x16x3_t){vld1q_u8(v0), vld1q_u8(v1), vld1q_u8(v2)}));
}

__attribute__((always_inline))
void rust_isa_neon_ld4(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2, uint8_t *o3) {
    uint8x16x4_t t = vld4q_u8(p);
    vst1q_u8(o0, t.val[0]);
    vst1q_u8(o1, t.val[1]);
    vst1q_u8(o2, t.val[2]);
    vst1q_u8(o3, t.val[3]);
}

__attribute__((always_inline))
void rust_isa_neon_st4(uint8_t *p, const uint8_t *v0, const uint8_t *v1, const uint8_t *v2, const uint8_t *v3) {
    vst4q_u8(p, ((uint8x16x4_t){vld1q_u8(v0), vld1q_u8(v1), vld1q_u8(v2), vld1q_u8(v3)}));
}

__attribute__((always_inline))
void rust_isa_neon_ld1r_i8(const uint8_t *p, uint8_t *out) {
    vst1q_u8((uint8_t *)(void *)out, vld1q_dup_u8((const uint8_t *)(const void *)p));
}

__attribute__((always_inline))
void rust_isa_neon_ld2r_i8(const uint8_t *p, uint8_t *o0, uint8_t *o1) {
    uint8x16x2_t t = vld2q_dup_u8((const uint8_t *)(const void *)p);
    vst1q_u8((uint8_t *)(void *)o0, t.val[0]);
    vst1q_u8((uint8_t *)(void *)o1, t.val[1]);
}

__attribute__((always_inline))
void rust_isa_neon_ld3r_i8(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2) {
    uint8x16x3_t t = vld3q_dup_u8((const uint8_t *)(const void *)p);
    vst1q_u8((uint8_t *)(void *)o0, t.val[0]);
    vst1q_u8((uint8_t *)(void *)o1, t.val[1]);
    vst1q_u8((uint8_t *)(void *)o2, t.val[2]);
}

__attribute__((always_inline))
void rust_isa_neon_ld4r_i8(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2, uint8_t *o3) {
    uint8x16x4_t t = vld4q_dup_u8((const uint8_t *)(const void *)p);
    vst1q_u8((uint8_t *)(void *)o0, t.val[0]);
    vst1q_u8((uint8_t *)(void *)o1, t.val[1]);
    vst1q_u8((uint8_t *)(void *)o2, t.val[2]);
    vst1q_u8((uint8_t *)(void *)o3, t.val[3]);
}

__attribute__((always_inline))
void rust_isa_neon_ld1r_i16(const uint8_t *p, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out, vld1q_dup_u16((const uint16_t *)(const void *)p));
}

__attribute__((always_inline))
void rust_isa_neon_ld2r_i16(const uint8_t *p, uint8_t *o0, uint8_t *o1) {
    uint16x8x2_t t = vld2q_dup_u16((const uint16_t *)(const void *)p);
    vst1q_u16((uint16_t *)(void *)o0, t.val[0]);
    vst1q_u16((uint16_t *)(void *)o1, t.val[1]);
}

__attribute__((always_inline))
void rust_isa_neon_ld3r_i16(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2) {
    uint16x8x3_t t = vld3q_dup_u16((const uint16_t *)(const void *)p);
    vst1q_u16((uint16_t *)(void *)o0, t.val[0]);
    vst1q_u16((uint16_t *)(void *)o1, t.val[1]);
    vst1q_u16((uint16_t *)(void *)o2, t.val[2]);
}

__attribute__((always_inline))
void rust_isa_neon_ld4r_i16(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2, uint8_t *o3) {
    uint16x8x4_t t = vld4q_dup_u16((const uint16_t *)(const void *)p);
    vst1q_u16((uint16_t *)(void *)o0, t.val[0]);
    vst1q_u16((uint16_t *)(void *)o1, t.val[1]);
    vst1q_u16((uint16_t *)(void *)o2, t.val[2]);
    vst1q_u16((uint16_t *)(void *)o3, t.val[3]);
}

__attribute__((always_inline))
void rust_isa_neon_ld1r_i32(const uint8_t *p, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vld1q_dup_u32((const uint32_t *)(const void *)p));
}

__attribute__((always_inline))
void rust_isa_neon_ld2r_i32(const uint8_t *p, uint8_t *o0, uint8_t *o1) {
    uint32x4x2_t t = vld2q_dup_u32((const uint32_t *)(const void *)p);
    vst1q_u32((uint32_t *)(void *)o0, t.val[0]);
    vst1q_u32((uint32_t *)(void *)o1, t.val[1]);
}

__attribute__((always_inline))
void rust_isa_neon_ld3r_i32(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2) {
    uint32x4x3_t t = vld3q_dup_u32((const uint32_t *)(const void *)p);
    vst1q_u32((uint32_t *)(void *)o0, t.val[0]);
    vst1q_u32((uint32_t *)(void *)o1, t.val[1]);
    vst1q_u32((uint32_t *)(void *)o2, t.val[2]);
}

__attribute__((always_inline))
void rust_isa_neon_ld4r_i32(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2, uint8_t *o3) {
    uint32x4x4_t t = vld4q_dup_u32((const uint32_t *)(const void *)p);
    vst1q_u32((uint32_t *)(void *)o0, t.val[0]);
    vst1q_u32((uint32_t *)(void *)o1, t.val[1]);
    vst1q_u32((uint32_t *)(void *)o2, t.val[2]);
    vst1q_u32((uint32_t *)(void *)o3, t.val[3]);
}

__attribute__((always_inline))
void rust_isa_neon_ld1r_i64(const uint8_t *p, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vld1q_dup_u64((const uint64_t *)(const void *)p));
}

__attribute__((always_inline))
void rust_isa_neon_ld2r_i64(const uint8_t *p, uint8_t *o0, uint8_t *o1) {
    uint64x2x2_t t = vld2q_dup_u64((const uint64_t *)(const void *)p);
    vst1q_u64((uint64_t *)(void *)o0, t.val[0]);
    vst1q_u64((uint64_t *)(void *)o1, t.val[1]);
}

__attribute__((always_inline))
void rust_isa_neon_ld3r_i64(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2) {
    uint64x2x3_t t = vld3q_dup_u64((const uint64_t *)(const void *)p);
    vst1q_u64((uint64_t *)(void *)o0, t.val[0]);
    vst1q_u64((uint64_t *)(void *)o1, t.val[1]);
    vst1q_u64((uint64_t *)(void *)o2, t.val[2]);
}

__attribute__((always_inline))
void rust_isa_neon_ld4r_i64(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2, uint8_t *o3) {
    uint64x2x4_t t = vld4q_dup_u64((const uint64_t *)(const void *)p);
    vst1q_u64((uint64_t *)(void *)o0, t.val[0]);
    vst1q_u64((uint64_t *)(void *)o1, t.val[1]);
    vst1q_u64((uint64_t *)(void *)o2, t.val[2]);
    vst1q_u64((uint64_t *)(void *)o3, t.val[3]);
}

__attribute__((always_inline))
void rust_isa_neon_ldp(const uint8_t *p, uint8_t *o0, uint8_t *o1) {
    vst1q_u8(o0, vld1q_u8(p));
    vst1q_u8(o1, vld1q_u8(p + 16));
}

__attribute__((always_inline))
void rust_isa_neon_ldnp(const uint8_t *p, uint8_t *o0, uint8_t *o1) {
    vst1q_u8(o0, vld1q_u8(p));
    vst1q_u8(o1, vld1q_u8(p + 16));
}

__attribute__((always_inline))
void rust_isa_neon_stp(uint8_t *p, const uint8_t *v0, const uint8_t *v1) {
    vst1q_u8(p, vld1q_u8(v0));
    vst1q_u8(p + 16, vld1q_u8(v1));
}

__attribute__((always_inline))
void rust_isa_neon_stnp(uint8_t *p, const uint8_t *v0, const uint8_t *v1) {
    vst1q_u8(p, vld1q_u8(v0));
    vst1q_u8(p + 16, vld1q_u8(v1));
}

__attribute__((always_inline))
void rust_isa_neon_zip2_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vzip2q_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_neon_uzp2_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vuzp2q_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_neon_trn2_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vtrn2q_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
__attribute__((target("crypto")))
void rust_isa_neon_pmull_p64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vreinterpretq_u64_p128(vmull_p64(
        vgetq_lane_p64(vreinterpretq_p64_u64(vld1q_u64((const uint64_t *)(const void *)a)), 0),
        vgetq_lane_p64(vreinterpretq_p64_u64(vld1q_u64((const uint64_t *)(const void *)b)), 0))));
}
