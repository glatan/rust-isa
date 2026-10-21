#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_fhm_vfmaq_f16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vfmaq_f16(
        vld1q_f16((const float16_t *)(const void *)acc),
        vld1q_f16((const float16_t *)(const void *)a),
        vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_fhm_vfmsq_f16(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vfmsq_f16(
        vld1q_f16((const float16_t *)(const void *)acc),
        vld1q_f16((const float16_t *)(const void *)a),
        vld1q_f16((const float16_t *)(const void *)b)));
}
