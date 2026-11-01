#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_fhm_fmlal_f16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_f32((float *)(void *)out, vfmlal_low_f16(
        vld1_f32((const float *)(const void *)acc),
        vld1_f16((const float16_t *)(const void *)a),
        vld1_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fhm_fmlal2_f16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_f32((float *)(void *)out, vfmlal_high_f16(
        vld1_f32((const float *)(const void *)acc),
        vld1_f16((const float16_t *)(const void *)a),
        vld1_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fhm_fmlsl_f16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_f32((float *)(void *)out, vfmlsl_low_f16(
        vld1_f32((const float *)(const void *)acc),
        vld1_f16((const float16_t *)(const void *)a),
        vld1_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fhm_fmlsl2_f16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1_f32((float *)(void *)out, vfmlsl_high_f16(
        vld1_f32((const float *)(const void *)acc),
        vld1_f16((const float16_t *)(const void *)a),
        vld1_f16((const float16_t *)(const void *)b)));
}
