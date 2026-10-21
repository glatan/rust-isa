#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
static inline float16x8_t load_f16(const uint8_t *p) {
    return vld1q_f16((const float16_t *)(const void *)p);
}

__attribute__((always_inline))
static inline void store_f16(uint8_t *out, float16x8_t value) {
    vst1q_f16((float16_t *)(void *)out, value);
}

__attribute__((always_inline))
void rust_isa_arm_fp16_abs(const uint8_t *a, uint8_t *out) {
    store_f16(out, vabsq_f16(load_f16(a)));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f16(out, vaddq_f16(load_f16(a), load_f16(b)));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_cmp(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u16((uint16_t *)(void *)out,
              vreinterpretq_u16_f16(vceqq_f16(load_f16(a), load_f16(b))));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_cvt_f32(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vcvt_f32_f16(vget_low_f16(load_f16(a))));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_cvt_high_f32(const uint8_t *a, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vcvt_f32_f16(vget_high_f16(load_f16(a))));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_cvt_f16(const uint8_t *a, uint8_t *out) {
    const float32x4_t value = vld1q_f32((const float *)(const void *)a);
    store_f16(out, vcombine_f16(vcvt_f16_f32(value), vdup_n_f16((float16_t)0)));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_cvt_high_f16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f16(out, vcombine_f16(vget_low_f16(load_f16(a)),
                                vcvt_f16_f32(vld1q_f32((const float *)(const void *)b))));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_div(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    const _Float16 *va = (const _Float16 *)(const void *)a;
    const _Float16 *vb = (const _Float16 *)(const void *)b;
    _Float16 *result = (_Float16 *)(void *)out;
    for (unsigned lane = 0; lane < 8; lane++) {
        result[lane] = va[lane] / vb[lane];
    }
}

__attribute__((always_inline))
void rust_isa_arm_fp16_fma(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f16(out, vfmaq_f16(load_f16(acc), load_f16(a), load_f16(b)));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_fms(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f16(out, vfmsq_f16(load_f16(acc), load_f16(a), load_f16(b)));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_max(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f16(out, vmaxq_f16(load_f16(a), load_f16(b)));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_min(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f16(out, vminq_f16(load_f16(a), load_f16(b)));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f16(out, vmulq_f16(load_f16(a), load_f16(b)));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_neg(const uint8_t *a, uint8_t *out) {
    store_f16(out, vnegq_f16(load_f16(a)));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_sqrt(const uint8_t *a, uint8_t *out) {
    const _Float16 *va = (const _Float16 *)(const void *)a;
    _Float16 *result = (_Float16 *)(void *)out;
    for (unsigned lane = 0; lane < 8; lane++) {
        result[lane] = __builtin_sqrtf((float)va[lane]);
    }
}

__attribute__((always_inline))
void rust_isa_arm_fp16_sub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_f16(out, vsubq_f16(load_f16(a), load_f16(b)));
}
