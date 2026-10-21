#include <stdint.h>
#include <arm_mve.h>

__attribute__((always_inline))
static inline float32x4_t load_f32(const uint8_t *p) {
    return vld1q_f32((const float *)(const void *)p);
}

__attribute__((always_inline))
static inline void store_f32(uint8_t *out, float32x4_t value) {
    vst1q_f32((float *)(void *)out, value);
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_abs(const uint8_t *a, uint8_t *out) {
    store_f32(out, __arm_vabsq_f32(load_f32(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f32(out, vaddq_f32(load_f32(a), load_f32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_cmp(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    const mve_pred16_t predicate = __arm_vcmpeqq_f32(load_f32(a), load_f32(b));
    vst1q_u32((uint32_t *)(void *)out,
              __arm_vpselq_u32(vdupq_n_u32(0xffffffffu), vdupq_n_u32(0), predicate));
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_cvt(const uint8_t *a, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, __arm_vcvtq_s32_f32(load_f32(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_cvt_f32(const uint8_t *a, uint8_t *out) {
    store_f32(out, __arm_vcvtq_f32_s32(vld1q_s32((const int32_t *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_div(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    const float *va = (const float *)(const void *)a;
    const float *vb = (const float *)(const void *)b;
    float *result = (float *)(void *)out;
    for (unsigned lane = 0; lane < 4; lane++) {
        result[lane] = va[lane] / vb[lane];
    }
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_fma(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f32(out, __arm_vfmaq_f32(load_f32(acc), load_f32(a), load_f32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_fms(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f32(out, __arm_vfmsq_f32(load_f32(acc), load_f32(a), load_f32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_max(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f32(out, __arm_vmaxnmq_f32(load_f32(a), load_f32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_min(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f32(out, __arm_vminnmq_f32(load_f32(a), load_f32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f32(out, vmulq_f32(load_f32(a), load_f32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_neg(const uint8_t *a, uint8_t *out) {
    store_f32(out, __arm_vnegq_f32(load_f32(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_sqrt(const uint8_t *a, uint8_t *out) {
    const float *va = (const float *)(const void *)a;
    float *result = (float *)(void *)out;
    for (unsigned lane = 0; lane < 4; lane++) {
        result[lane] = __builtin_sqrtf(va[lane]);
    }
}

__attribute__((always_inline))
void rust_isa_arm_mve_fp_sub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f32(out, vsubq_f32(load_f32(a), load_f32(b)));
}
