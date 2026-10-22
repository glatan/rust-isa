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
void rust_isa_arm_neon_zip1_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vzipq_u8(vld1q_u8(a), vld1q_u8(b)).val[0]);
}

__attribute__((always_inline))
void rust_isa_arm_neon_uzp1_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vuzpq_u8(vld1q_u8(a), vld1q_u8(b)).val[0]);
}

__attribute__((always_inline))
void rust_isa_arm_neon_trn1_u8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vtrnq_u8(vld1q_u8(a), vld1q_u8(b)).val[0]);
}

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
