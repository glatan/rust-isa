#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_arm_neon_v128_load(const uint8_t *p, uint8_t out[16]) {
    vst1q_u8(out, vld1q_u8(p));
}

__attribute__((always_inline))
void rust_isa_arm_neon_v128_store(uint8_t *p, const uint8_t value[16]) {
    vst1q_u8(p, vld1q_u8(value));
}

__attribute__((always_inline))
void rust_isa_arm_neon_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vaddq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmulq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_add_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vaddq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_add_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vaddq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_add_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vaddq_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_sub_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vsubq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_sub_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsubq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_sub_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vsubq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_sub_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s64((int64_t *)(void *)out, vsubq_s64(vld1q_s64((const int64_t *)(const void *)a), vld1q_s64((const int64_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_mul_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vmulq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_mul_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmulq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_and(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vandq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_orr(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vorrq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_eor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, veorq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_cmpeq_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s8(vceqq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_arm_neon_cmpeq_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s16(vceqq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_arm_neon_cmpeq_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s32(vceqq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_arm_neon_cmpgt_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s8(vcgtq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_arm_neon_cmpgt_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s16(vcgtq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_arm_neon_cmpgt_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vreinterpretq_u8_s32(vcgtq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_arm_neon_fsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vsubq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_splat_i8(int8_t value, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vdupq_n_s8(value));
}

__attribute__((always_inline))
void rust_isa_arm_neon_splat_i16(int16_t value, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vdupq_n_s16(value));
}

__attribute__((always_inline))
void rust_isa_arm_neon_splat_i32(int32_t value, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vdupq_n_s32(value));
}

__attribute__((always_inline))
__attribute__((always_inline))
__attribute__((always_inline))
__attribute__((always_inline))
void rust_isa_arm_neon_fadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vaddq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_fmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vmulq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vaba_s8(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vabaq_s8(
        vld1q_s8((const int8_t *)(const void *)acc),
        vld1q_s8((const int8_t *)(const void *)a),
        vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vabd_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vabdq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vabs_s32(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vabsq_s32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vceq_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vceqq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vcge_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vcgeq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vcgt_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vcgtq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vcle_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vcleq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vclt_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vcltq_s8(vld1q_s8((const int8_t *)(const void *)a), vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vcls_s8(const uint8_t *a, uint8_t *out) {
    vst1q_s8((int8_t *)(void *)out, vclsq_s8(vld1q_s8((const int8_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vclz_s8(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, vclzq_u8(vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vcnt_s8(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, vcntq_u8(vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vcvt_s32_f32(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vcvtq_s32_f32(vld1q_f32((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vhadd_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vhaddq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vhsub_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vhsubq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vmax_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmaxq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vmin_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vminq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vmla_s32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmlaq_s32(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1q_s32((const int32_t *)(const void *)a),
        vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vmls_s32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmlsq_s32(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1q_s32((const int32_t *)(const void *)a),
        vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vneg_s32(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vnegq_s32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqabs_s32(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqabsq_s32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqadd_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqaddq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqdmulh_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vqdmulhq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqneg_s32(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqnegq_s32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqrdmulh_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vqrdmulhq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqshl_s32(const uint8_t *a, const uint8_t *shift, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)shift)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqsub_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqsubq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrshl_s32(const uint8_t *a, const uint8_t *shift, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vrshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)shift)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vshl_s32(const uint8_t *a, const uint8_t *shift, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vshlq_s32(vld1q_s32((const int32_t *)(const void *)a), vld1q_s32((const int32_t *)(const void *)shift)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vbic_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vbicq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vbsl_u8(const uint8_t *mask, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vbslq_u8(vld1q_u8(mask), vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vmvn_u8(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, vmvnq_u8(vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vorn_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vornq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vtst_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vtstq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vfma_f32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vfmaq_f32(
        vld1q_f32((const float *)(const void *)acc),
        vld1q_f32((const float *)(const void *)a),
        vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vfms_f32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vfmsq_f32(
        vld1q_f32((const float *)(const void *)acc),
        vld1q_f32((const float *)(const void *)a),
        vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vfnma_f32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vfmaq_f32(
        vnegq_f32(vld1q_f32((const float *)(const void *)acc)),
        vld1q_f32((const float *)(const void *)a),
        vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vfnms_f32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vfmsq_f32(
        vnegq_f32(vld1q_f32((const float *)(const void *)acc)),
        vld1q_f32((const float *)(const void *)a),
        vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrecpe_f32(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vrecpeq_f32(vld1q_f32((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrecps_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vrecpsq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrsqrte_f32(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vrsqrteq_f32(vld1q_f32((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrsqrts_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vrsqrtsq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vabal_s8(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vabal_s8(
        vld1q_s16((const int16_t *)(const void *)acc),
        vld1_s8((const int8_t *)(const void *)a),
        vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vabdl_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vabdl_s8(vld1_s8((const int8_t *)(const void *)a), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vaddhn_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vaddhn_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vaddl_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vaddl_s8(vld1_s8((const int8_t *)(const void *)a), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vaddw_s8(const uint8_t *acc, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vaddw_s8(vld1q_s16((const int16_t *)(const void *)acc), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vmlal_s16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmlal_s16(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1_s16((const int16_t *)(const void *)a),
        vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vmlsl_s16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmlsl_s16(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1_s16((const int16_t *)(const void *)a),
        vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vmovn_s16(const uint8_t *a, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vmovn_s16(vld1q_s16((const int16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vmull_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmull_s16(vld1_s16((const int16_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vpadal_s16(const uint8_t *acc, const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vpadalq_s16(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1q_s16((const int16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqdmlal_s16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqdmlal_s16(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1_s16((const int16_t *)(const void *)a),
        vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqdmlsl_s16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqdmlsl_s16(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1_s16((const int16_t *)(const void *)a),
        vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqdmull_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqdmull_s16(vld1_s16((const int16_t *)(const void *)a), vld1_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqmovn_s16(const uint8_t *a, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vqmovn_s16(vld1q_s16((const int16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqmovun_s16(const uint8_t *a, uint8_t *out) {
    vst1_u8(out, vqmovun_s16(vld1q_s16((const int16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqrshl_s16(const uint8_t *a, const uint8_t *shift, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vqrshlq_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)shift)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vraddhn_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vraddhn_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrsubhn_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vrsubhn_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vsubhn_s16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_s8((int8_t *)(void *)out, vsubhn_s16(vld1q_s16((const int16_t *)(const void *)a), vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vsubl_s8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsubl_s8(vld1_s8((const int8_t *)(const void *)a), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vsubw_s8(const uint8_t *acc, const uint8_t *b, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out, vsubw_s8(vld1q_s16((const int16_t *)(const void *)acc), vld1_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqshrn_s16(const uint8_t *a, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    int8x8_t r = vdup_n_s8(0);
    switch (n) {
        case 1: r = vqshrn_n_s16(va, 1); break;
        case 2: r = vqshrn_n_s16(va, 2); break;
        case 3: r = vqshrn_n_s16(va, 3); break;
        case 4: r = vqshrn_n_s16(va, 4); break;
        case 5: r = vqshrn_n_s16(va, 5); break;
        case 6: r = vqshrn_n_s16(va, 6); break;
        case 7: r = vqshrn_n_s16(va, 7); break;
        case 8: r = vqshrn_n_s16(va, 8); break;
    }
    vst1_s8((int8_t *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqshrun_s16(const uint8_t *a, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    uint8x8_t r = vdup_n_u8(0);
    switch (n) {
        case 1: r = vqshrun_n_s16(va, 1); break;
        case 2: r = vqshrun_n_s16(va, 2); break;
        case 3: r = vqshrun_n_s16(va, 3); break;
        case 4: r = vqshrun_n_s16(va, 4); break;
        case 5: r = vqshrun_n_s16(va, 5); break;
        case 6: r = vqshrun_n_s16(va, 6); break;
        case 7: r = vqshrun_n_s16(va, 7); break;
        case 8: r = vqshrun_n_s16(va, 8); break;
    }
    vst1_u8(out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqrshrn_s16(const uint8_t *a, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    int8x8_t r = vdup_n_s8(0);
    switch (n) {
        case 1: r = vqrshrn_n_s16(va, 1); break;
        case 2: r = vqrshrn_n_s16(va, 2); break;
        case 3: r = vqrshrn_n_s16(va, 3); break;
        case 4: r = vqrshrn_n_s16(va, 4); break;
        case 5: r = vqrshrn_n_s16(va, 5); break;
        case 6: r = vqrshrn_n_s16(va, 6); break;
        case 7: r = vqrshrn_n_s16(va, 7); break;
        case 8: r = vqrshrn_n_s16(va, 8); break;
    }
    vst1_s8((int8_t *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqrshrun_s16(const uint8_t *a, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    uint8x8_t r = vdup_n_u8(0);
    switch (n) {
        case 1: r = vqrshrun_n_s16(va, 1); break;
        case 2: r = vqrshrun_n_s16(va, 2); break;
        case 3: r = vqrshrun_n_s16(va, 3); break;
        case 4: r = vqrshrun_n_s16(va, 4); break;
        case 5: r = vqrshrun_n_s16(va, 5); break;
        case 6: r = vqrshrun_n_s16(va, 6); break;
        case 7: r = vqrshrun_n_s16(va, 7); break;
        case 8: r = vqrshrun_n_s16(va, 8); break;
    }
    vst1_u8(out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vshrn_s16(const uint8_t *a, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    int8x8_t r = vdup_n_s8(0);
    switch (n) {
        case 1: r = vshrn_n_s16(va, 1); break;
        case 2: r = vshrn_n_s16(va, 2); break;
        case 3: r = vshrn_n_s16(va, 3); break;
        case 4: r = vshrn_n_s16(va, 4); break;
        case 5: r = vshrn_n_s16(va, 5); break;
        case 6: r = vshrn_n_s16(va, 6); break;
        case 7: r = vshrn_n_s16(va, 7); break;
        case 8: r = vshrn_n_s16(va, 8); break;
    }
    vst1_s8((int8_t *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrshrn_s16(const uint8_t *a, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    int8x8_t r = vdup_n_s8(0);
    switch (n) {
        case 1: r = vrshrn_n_s16(va, 1); break;
        case 2: r = vrshrn_n_s16(va, 2); break;
        case 3: r = vrshrn_n_s16(va, 3); break;
        case 4: r = vrshrn_n_s16(va, 4); break;
        case 5: r = vrshrn_n_s16(va, 5); break;
        case 6: r = vrshrn_n_s16(va, 6); break;
        case 7: r = vrshrn_n_s16(va, 7); break;
        case 8: r = vrshrn_n_s16(va, 8); break;
    }
    vst1_s8((int8_t *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vqshlu_s16(const uint8_t *a, int n, uint8_t *out) {
    int16x8_t va = vld1q_u16((const uint16_t *)(const void *)a);
    uint16x8_t r = vdupq_n_u16(0);
    switch (n) {
        case 0: r = vqshluq_n_s16(va, 0); break;
        case 1: r = vqshluq_n_s16(va, 1); break;
        case 2: r = vqshluq_n_s16(va, 2); break;
        case 3: r = vqshluq_n_s16(va, 3); break;
        case 4: r = vqshluq_n_s16(va, 4); break;
        case 5: r = vqshluq_n_s16(va, 5); break;
        case 6: r = vqshluq_n_s16(va, 6); break;
        case 7: r = vqshluq_n_s16(va, 7); break;
        case 8: r = vqshluq_n_s16(va, 8); break;
        case 9: r = vqshluq_n_s16(va, 9); break;
        case 10: r = vqshluq_n_s16(va, 10); break;
        case 11: r = vqshluq_n_s16(va, 11); break;
        case 12: r = vqshluq_n_s16(va, 12); break;
        case 13: r = vqshluq_n_s16(va, 13); break;
        case 14: r = vqshluq_n_s16(va, 14); break;
        case 15: r = vqshluq_n_s16(va, 15); break;
    }
    vst1q_u16((void *)out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrshr_s16(const uint8_t *a, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    int16x8_t r = vdupq_n_s16(0);
    switch (n) {
        case 1: r = vrshrq_n_s16(va, 1); break;
        case 2: r = vrshrq_n_s16(va, 2); break;
        case 3: r = vrshrq_n_s16(va, 3); break;
        case 4: r = vrshrq_n_s16(va, 4); break;
        case 5: r = vrshrq_n_s16(va, 5); break;
        case 6: r = vrshrq_n_s16(va, 6); break;
        case 7: r = vrshrq_n_s16(va, 7); break;
        case 8: r = vrshrq_n_s16(va, 8); break;
        case 9: r = vrshrq_n_s16(va, 9); break;
        case 10: r = vrshrq_n_s16(va, 10); break;
        case 11: r = vrshrq_n_s16(va, 11); break;
        case 12: r = vrshrq_n_s16(va, 12); break;
        case 13: r = vrshrq_n_s16(va, 13); break;
        case 14: r = vrshrq_n_s16(va, 14); break;
        case 15: r = vrshrq_n_s16(va, 15); break;
        case 16: r = vrshrq_n_s16(va, 16); break;
    }
    vst1q_s16((void *)out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vsli_s16(const uint8_t *a, const uint8_t *b, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    int16x8_t vb = vld1q_s16((const int16_t *)(const void *)b);
    int16x8_t r = vdupq_n_s16(0);
    switch (n) {
        case 0: r = vsliq_n_s16(va, vb, 0); break;
        case 1: r = vsliq_n_s16(va, vb, 1); break;
        case 2: r = vsliq_n_s16(va, vb, 2); break;
        case 3: r = vsliq_n_s16(va, vb, 3); break;
        case 4: r = vsliq_n_s16(va, vb, 4); break;
        case 5: r = vsliq_n_s16(va, vb, 5); break;
        case 6: r = vsliq_n_s16(va, vb, 6); break;
        case 7: r = vsliq_n_s16(va, vb, 7); break;
        case 8: r = vsliq_n_s16(va, vb, 8); break;
        case 9: r = vsliq_n_s16(va, vb, 9); break;
        case 10: r = vsliq_n_s16(va, vb, 10); break;
        case 11: r = vsliq_n_s16(va, vb, 11); break;
        case 12: r = vsliq_n_s16(va, vb, 12); break;
        case 13: r = vsliq_n_s16(va, vb, 13); break;
        case 14: r = vsliq_n_s16(va, vb, 14); break;
        case 15: r = vsliq_n_s16(va, vb, 15); break;
    }
    vst1q_s16((int16_t *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vsra_s16(const uint8_t *a, const uint8_t *b, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    int16x8_t vb = vld1q_s16((const int16_t *)(const void *)b);
    int16x8_t r = vdupq_n_s16(0);
    switch (n) {
        case 1: r = vsraq_n_s16(va, vb, 1); break;
        case 2: r = vsraq_n_s16(va, vb, 2); break;
        case 3: r = vsraq_n_s16(va, vb, 3); break;
        case 4: r = vsraq_n_s16(va, vb, 4); break;
        case 5: r = vsraq_n_s16(va, vb, 5); break;
        case 6: r = vsraq_n_s16(va, vb, 6); break;
        case 7: r = vsraq_n_s16(va, vb, 7); break;
        case 8: r = vsraq_n_s16(va, vb, 8); break;
        case 9: r = vsraq_n_s16(va, vb, 9); break;
        case 10: r = vsraq_n_s16(va, vb, 10); break;
        case 11: r = vsraq_n_s16(va, vb, 11); break;
        case 12: r = vsraq_n_s16(va, vb, 12); break;
        case 13: r = vsraq_n_s16(va, vb, 13); break;
        case 14: r = vsraq_n_s16(va, vb, 14); break;
        case 15: r = vsraq_n_s16(va, vb, 15); break;
        case 16: r = vsraq_n_s16(va, vb, 16); break;
    }
    vst1q_s16((int16_t *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrsra_s16(const uint8_t *a, const uint8_t *b, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    int16x8_t vb = vld1q_s16((const int16_t *)(const void *)b);
    int16x8_t r = vdupq_n_s16(0);
    switch (n) {
        case 1: r = vrsraq_n_s16(va, vb, 1); break;
        case 2: r = vrsraq_n_s16(va, vb, 2); break;
        case 3: r = vrsraq_n_s16(va, vb, 3); break;
        case 4: r = vrsraq_n_s16(va, vb, 4); break;
        case 5: r = vrsraq_n_s16(va, vb, 5); break;
        case 6: r = vrsraq_n_s16(va, vb, 6); break;
        case 7: r = vrsraq_n_s16(va, vb, 7); break;
        case 8: r = vrsraq_n_s16(va, vb, 8); break;
        case 9: r = vrsraq_n_s16(va, vb, 9); break;
        case 10: r = vrsraq_n_s16(va, vb, 10); break;
        case 11: r = vrsraq_n_s16(va, vb, 11); break;
        case 12: r = vrsraq_n_s16(va, vb, 12); break;
        case 13: r = vrsraq_n_s16(va, vb, 13); break;
        case 14: r = vrsraq_n_s16(va, vb, 14); break;
        case 15: r = vrsraq_n_s16(va, vb, 15); break;
        case 16: r = vrsraq_n_s16(va, vb, 16); break;
    }
    vst1q_s16((int16_t *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vsri_s16(const uint8_t *a, const uint8_t *b, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    int16x8_t vb = vld1q_s16((const int16_t *)(const void *)b);
    int16x8_t r = vdupq_n_s16(0);
    switch (n) {
        case 1: r = vsriq_n_s16(va, vb, 1); break;
        case 2: r = vsriq_n_s16(va, vb, 2); break;
        case 3: r = vsriq_n_s16(va, vb, 3); break;
        case 4: r = vsriq_n_s16(va, vb, 4); break;
        case 5: r = vsriq_n_s16(va, vb, 5); break;
        case 6: r = vsriq_n_s16(va, vb, 6); break;
        case 7: r = vsriq_n_s16(va, vb, 7); break;
        case 8: r = vsriq_n_s16(va, vb, 8); break;
        case 9: r = vsriq_n_s16(va, vb, 9); break;
        case 10: r = vsriq_n_s16(va, vb, 10); break;
        case 11: r = vsriq_n_s16(va, vb, 11); break;
        case 12: r = vsriq_n_s16(va, vb, 12); break;
        case 13: r = vsriq_n_s16(va, vb, 13); break;
        case 14: r = vsriq_n_s16(va, vb, 14); break;
        case 15: r = vsriq_n_s16(va, vb, 15); break;
        case 16: r = vsriq_n_s16(va, vb, 16); break;
    }
    vst1q_s16((int16_t *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vshll_s16(const uint8_t *a, int n, uint8_t *out) {
    int16x4_t va = vld1_s16((const int16_t *)(const void *)a);
    int32x4_t r = vdupq_n_s32(0);
    switch (n) {
        case 0: r = vshll_n_s16(va, 0); break;
        case 1: r = vshll_n_s16(va, 1); break;
        case 2: r = vshll_n_s16(va, 2); break;
        case 3: r = vshll_n_s16(va, 3); break;
        case 4: r = vshll_n_s16(va, 4); break;
        case 5: r = vshll_n_s16(va, 5); break;
        case 6: r = vshll_n_s16(va, 6); break;
        case 7: r = vshll_n_s16(va, 7); break;
        case 8: r = vshll_n_s16(va, 8); break;
        case 9: r = vshll_n_s16(va, 9); break;
        case 10: r = vshll_n_s16(va, 10); break;
        case 11: r = vshll_n_s16(va, 11); break;
        case 12: r = vshll_n_s16(va, 12); break;
        case 13: r = vshll_n_s16(va, 13); break;
        case 14: r = vshll_n_s16(va, 14); break;
        case 15: r = vshll_n_s16(va, 15); break;
    }
    vst1q_s32((int32_t *)(void *)out, r);
}


__attribute__((always_inline))
void rust_isa_arm_neon_vld1_u8(const uint8_t *p, uint8_t *out) {
    vst1q_u8(out, vld1q_u8(p));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vld2_u8(const uint8_t *p, uint8_t *o0, uint8_t *o1) {
    uint8x16x2_t r = vld2q_u8(p);
    vst1q_u8(o0, r.val[0]);
    vst1q_u8(o1, r.val[1]);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vld3_u8(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2) {
    uint8x16x3_t r = vld3q_u8(p);
    vst1q_u8(o0, r.val[0]);
    vst1q_u8(o1, r.val[1]);
    vst1q_u8(o2, r.val[2]);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vld4_u8(const uint8_t *p, uint8_t *o0, uint8_t *o1, uint8_t *o2, uint8_t *o3) {
    uint8x16x4_t r = vld4q_u8(p);
    vst1q_u8(o0, r.val[0]);
    vst1q_u8(o1, r.val[1]);
    vst1q_u8(o2, r.val[2]);
    vst1q_u8(o3, r.val[3]);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vmov_u8(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, vld1q_u8(a));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vpadd_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    int32x4_t va = vld1q_s32((const int32_t *)(const void *)a);
    int32x4_t vb = vld1q_s32((const int32_t *)(const void *)b);
    vst1q_s32((int32_t *)(void *)out, vcombine_s32(
        vpadd_s32(vget_low_s32(va), vget_high_s32(va)),
        vpadd_s32(vget_low_s32(vb), vget_high_s32(vb))));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vpmax_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    int32x4_t va = vld1q_s32((const int32_t *)(const void *)a);
    int32x4_t vb = vld1q_s32((const int32_t *)(const void *)b);
    vst1q_s32((int32_t *)(void *)out, vcombine_s32(
        vpmax_s32(vget_low_s32(va), vget_high_s32(va)),
        vpmax_s32(vget_low_s32(vb), vget_high_s32(vb))));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vpmin_s32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    int32x4_t va = vld1q_s32((const int32_t *)(const void *)a);
    int32x4_t vb = vld1q_s32((const int32_t *)(const void *)b);
    vst1q_s32((int32_t *)(void *)out, vcombine_s32(
        vpmin_s32(vget_low_s32(va), vget_high_s32(va)),
        vpmin_s32(vget_low_s32(vb), vget_high_s32(vb))));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrev16_u8(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, vrev16q_u8(vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrev32_u8(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, vrev32q_u8(vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vrev64_u8(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, vrev64q_u8(vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vswp_u8(const uint8_t *a, const uint8_t *b, uint8_t *o0, uint8_t *o1) {
    vst1q_u8(o0, vld1q_u8(b));
    vst1q_u8(o1, vld1q_u8(a));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vtrn_u8(const uint8_t *a, const uint8_t *b, uint8_t *o0, uint8_t *o1) {
    uint8x16x2_t r = vtrnq_u8(vld1q_u8(a), vld1q_u8(b));
    vst1q_u8(o0, r.val[0]);
    vst1q_u8(o1, r.val[1]);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vuzp_u8(const uint8_t *a, const uint8_t *b, uint8_t *o0, uint8_t *o1) {
    uint8x16x2_t r = vuzpq_u8(vld1q_u8(a), vld1q_u8(b));
    vst1q_u8(o0, r.val[0]);
    vst1q_u8(o1, r.val[1]);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vzip_u8(const uint8_t *a, const uint8_t *b, uint8_t *o0, uint8_t *o1) {
    uint8x16x2_t r = vzipq_u8(vld1q_u8(a), vld1q_u8(b));
    vst1q_u8(o0, r.val[0]);
    vst1q_u8(o1, r.val[1]);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vtbl_u8(const uint8_t *table, const uint8_t *idx, uint8_t *out) {
    int8x8x2_t t;
    t.val[0] = vld1_s8((const int8_t *)(const void *)table);
    t.val[1] = vld1_s8((const int8_t *)(const void *)(table + 8));
    vst1_s8((int8_t *)(void *)out, vtbl2_s8(t, vld1_s8((const int8_t *)(const void *)idx)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vtbx_u8(const uint8_t *table, const uint8_t *idx, const uint8_t *src, uint8_t *out) {
    int8x8x2_t t;
    t.val[0] = vld1_s8((const int8_t *)(const void *)table);
    t.val[1] = vld1_s8((const int8_t *)(const void *)(table + 8));
    vst1_s8((int8_t *)(void *)out, vtbx2_s8(
        vld1_s8((const int8_t *)(const void *)src),
        t,
        vld1_s8((const int8_t *)(const void *)idx)));
}

__attribute__((always_inline))
void rust_isa_arm_neon_vext_u8(const uint8_t *a, const uint8_t *b, int n, uint8_t *out) {
    uint8x16_t va = vld1q_u8(a);
    uint8x16_t vb = vld1q_u8(b);
    uint8x16_t r = vdupq_n_u8(0);
    switch (n) {
        case 0: r = vextq_u8(va, vb, 0); break;
        case 1: r = vextq_u8(va, vb, 1); break;
        case 2: r = vextq_u8(va, vb, 2); break;
        case 3: r = vextq_u8(va, vb, 3); break;
        case 4: r = vextq_u8(va, vb, 4); break;
        case 5: r = vextq_u8(va, vb, 5); break;
        case 6: r = vextq_u8(va, vb, 6); break;
        case 7: r = vextq_u8(va, vb, 7); break;
        case 8: r = vextq_u8(va, vb, 8); break;
        case 9: r = vextq_u8(va, vb, 9); break;
        case 10: r = vextq_u8(va, vb, 10); break;
        case 11: r = vextq_u8(va, vb, 11); break;
        case 12: r = vextq_u8(va, vb, 12); break;
        case 13: r = vextq_u8(va, vb, 13); break;
        case 14: r = vextq_u8(va, vb, 14); break;
        case 15: r = vextq_u8(va, vb, 15); break;
    }
    vst1q_u8(out, r);
}

__attribute__((always_inline))
void rust_isa_arm_neon_vshr_s16(const uint8_t *a, int n, uint8_t *out) {
    int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    int16x8_t r = vdupq_n_s16(0);
    switch (n) {
        case 1: r = vshrq_n_s16(va, 1); break;
        case 2: r = vshrq_n_s16(va, 2); break;
        case 3: r = vshrq_n_s16(va, 3); break;
        case 4: r = vshrq_n_s16(va, 4); break;
        case 5: r = vshrq_n_s16(va, 5); break;
        case 6: r = vshrq_n_s16(va, 6); break;
        case 7: r = vshrq_n_s16(va, 7); break;
        case 8: r = vshrq_n_s16(va, 8); break;
        case 9: r = vshrq_n_s16(va, 9); break;
        case 10: r = vshrq_n_s16(va, 10); break;
        case 11: r = vshrq_n_s16(va, 11); break;
        case 12: r = vshrq_n_s16(va, 12); break;
        case 13: r = vshrq_n_s16(va, 13); break;
        case 14: r = vshrq_n_s16(va, 14); break;
        case 15: r = vshrq_n_s16(va, 15); break;
        case 16: r = vshrq_n_s16(va, 16); break;
    }
    vst1q_s16((int16_t *)(void *)out, r);
}
