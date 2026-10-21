#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_fcma_vcmlaq_f32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vcmlaq_f32(
        vld1q_f32((const float *)(const void *)acc),
        vld1q_f32((const float *)(const void *)a),
        vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fcma_vcmlaq_rot90_f32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vcmlaq_rot90_f32(
        vld1q_f32((const float *)(const void *)acc),
        vld1q_f32((const float *)(const void *)a),
        vld1q_f32((const float *)(const void *)b)));
}
