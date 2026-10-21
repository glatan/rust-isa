#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_fp16_vaddq_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vaddq_f16(
        vld1q_f16((const float16_t *)(const void *)a),
        vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_vmulq_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vmulq_f16(
        vld1q_f16((const float16_t *)(const void *)a),
        vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_vsubq_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vsubq_f16(
        vld1q_f16((const float16_t *)(const void *)a),
        vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_vdivq_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vdivq_f16(
        vld1q_f16((const float16_t *)(const void *)a),
        vld1q_f16((const float16_t *)(const void *)b)));
}

/* Additional FP16 instructions. */

__attribute__((always_inline))
void rust_isa_fp16_fabd_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vabdq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_facge_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vcageq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_facgt_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vcagtq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_fcmeq_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vceqq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_fcmge_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vcgeq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_fcmgt_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vcgtq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_fcmle_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vcleq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_fcmlt_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vcltq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_fmax_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vmaxq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_fmaxnm_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vmaxnmq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_fmin_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vminq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_fminnm_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vminnmq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_frecps_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vrecpsq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_frsqrts_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vrsqrtsq_f16(vld1q_f16((const float16_t *)(const void *)a), vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_fabs_f16(const uint8_t *a, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vabsq_f16(vld1q_f16((const float16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_fp16_fneg_f16(const uint8_t *a, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vnegq_f16(vld1q_f16((const float16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_fp16_fsqrt_f16(const uint8_t *a, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vsqrtq_f16(vld1q_f16((const float16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_fp16_frecpe_f16(const uint8_t *a, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vrecpeq_f16(vld1q_f16((const float16_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_fp16_frsqrte_f16(const uint8_t *a, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vrsqrteq_f16(vld1q_f16((const float16_t *)(const void *)a)));
}


__attribute__((always_inline))
void rust_isa_fp16_fmlal_f16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_f32((float *)(void *)out, vfmlal_low_f16(
        vld1_f32((const float *)(const void *)acc),
        vld1_f16((const float16_t *)(const void *)a),
        vld1_f16((const float16_t *)(const void *)b)));
}
__attribute__((always_inline))
void rust_isa_fp16_fmlal2_f16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_f32((float *)(void *)out, vfmlal_high_f16(
        vld1_f32((const float *)(const void *)acc),
        vld1_f16((const float16_t *)(const void *)a),
        vld1_f16((const float16_t *)(const void *)b)));
}
__attribute__((always_inline))
void rust_isa_fp16_fmlsl_f16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_f32((float *)(void *)out, vfmlsl_low_f16(
        vld1_f32((const float *)(const void *)acc),
        vld1_f16((const float16_t *)(const void *)a),
        vld1_f16((const float16_t *)(const void *)b)));
}
__attribute__((always_inline))
void rust_isa_fp16_fmlsl2_f16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_f32((float *)(void *)out, vfmlsl_high_f16(
        vld1_f32((const float *)(const void *)acc),
        vld1_f16((const float16_t *)(const void *)a),
        vld1_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fp16_fcvtl_f32_f16(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vcvt_f32_f16(vld1_f16((const float16_t *)(const void *)a)));
}
__attribute__((always_inline))
void rust_isa_fp16_fcvtl2_f32_f16(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vcvt_high_f32_f16(vld1q_f16((const float16_t *)(const void *)a)));
}
__attribute__((always_inline))
void rust_isa_fp16_fcvtn_f16_f32(const uint8_t *a, uint8_t *out) {
    vst1_f16((float16_t *)(void *)out, vcvt_f16_f32(vld1q_f32((const float *)(const void *)a)));
}
__attribute__((always_inline))
void rust_isa_fp16_fcvtn2_f16_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vcvt_high_f16_f32(
        vld1_f16((const float16_t *)(const void *)b),
        vld1q_f32((const float *)(const void *)a)));
}
__attribute__((always_inline))
void rust_isa_fp16_fcvtxn_f16_f64(const uint8_t *a, uint8_t *out) {
    float32x2_t f = vcvt_f32_f64(vld1q_f64((const double *)(const void *)a));
    vst1_f16((float16_t *)(void *)out, vcvt_f16_f32(vcombine_f32(f, f)));
}

__attribute__((always_inline))
void rust_isa_fp16_fcvtzs_f16_s32(const uint8_t *a, uint8_t *out) {
    float32x4_t f = vcvt_f32_f16(vld1_f16((const float16_t *)(const void *)a));
    vst1q_s32((int32_t *)(void *)out, vcvtq_s32_f32(f));
}
__attribute__((always_inline))
void rust_isa_fp16_fcvtzs_f16_s64(const uint8_t *a, uint8_t *out) {
    float32x4_t f = vcvt_f32_f16(vld1_f16((const float16_t *)(const void *)a));
    vst1q_s64((int64_t *)(void *)out, vcvtq_s64_f64(vcvt_f64_f32(vget_low_f32(f))));
}
__attribute__((always_inline))
void rust_isa_fp16_fcvtzu_f16_u32(const uint8_t *a, uint8_t *out) {
    float32x4_t f = vcvt_f32_f16(vld1_f16((const float16_t *)(const void *)a));
    vst1q_u32((uint32_t *)(void *)out, vcvtq_u32_f32(f));
}
__attribute__((always_inline))
void rust_isa_fp16_fcvtzu_f16_u64(const uint8_t *a, uint8_t *out) {
    float32x4_t f = vcvt_f32_f16(vld1_f16((const float16_t *)(const void *)a));
    vst1q_u64((uint64_t *)(void *)out, vcvtq_u64_f64(vcvt_f64_f32(vget_low_f32(f))));
}

__attribute__((always_inline))
void rust_isa_fp16_scvtf_f16_s32(const uint8_t *a, uint8_t *out) {
    vst1_f16((float16_t *)(void *)out, vcvt_f16_f32(vcvtq_f32_s32(vld1q_s32((const int32_t *)(const void *)a))));
}
__attribute__((always_inline))
void rust_isa_fp16_scvtf_f16_s64(const uint8_t *a, uint8_t *out) {
    float32x2_t f = vcvt_f32_f64(vcvtq_f64_s64(vld1q_s64((const int64_t *)(const void *)a)));
    vst1_f16((float16_t *)(void *)out, vcvt_f16_f32(vcombine_f32(f, f)));
}
__attribute__((always_inline))
void rust_isa_fp16_ucvtf_f16_u32(const uint8_t *a, uint8_t *out) {
    vst1_f16((float16_t *)(void *)out, vcvt_f16_f32(vcvtq_f32_u32(vld1q_u32((const uint32_t *)(const void *)a))));
}
__attribute__((always_inline))
void rust_isa_fp16_ucvtf_f16_u64(const uint8_t *a, uint8_t *out) {
    float32x2_t f = vcvt_f32_f64(vcvtq_f64_u64(vld1q_u64((const uint64_t *)(const void *)a)));
    vst1_f16((float16_t *)(void *)out, vcvt_f16_f32(vcombine_f32(f, f)));
}

__attribute__((always_inline))
uint16_t rust_isa_fp16_fcvt_f16_f32(float a) {
    union { _Float16 h; uint16_t u; } v;
    v.h = (_Float16)a;
    return v.u;
}

__attribute__((always_inline))
float rust_isa_fp16_fcvt_f32_f16(uint16_t a) {
    union { _Float16 h; uint16_t u; } v;
    v.u = a;
    return (float)v.h;
}

__attribute__((always_inline))
uint16_t rust_isa_fp16_fcvt_f16_f64(double a) {
    union { _Float16 h; uint16_t u; } v;
    v.h = (_Float16)a;
    return v.u;
}

__attribute__((always_inline))
double rust_isa_fp16_fcvt_f64_f16(uint16_t a) {
    union { _Float16 h; uint16_t u; } v;
    v.u = a;
    return (double)v.h;
}

__attribute__((always_inline))
uint16_t rust_isa_fp16_frecpx_f16(uint16_t a) {
    uint16_t out;
    __asm__("frecpx %h0, %h1" : "=w"(out) : "w"(a));
    return out;
}
