#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_arm_fp16_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vaddq_f16(
        vld1q_f16((const float16_t *)(const void *)a),
        vld1q_f16((const float16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_fp16_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f16((float16_t *)(void *)out, vmulq_f16(
        vld1q_f16((const float16_t *)(const void *)a),
        vld1q_f16((const float16_t *)(const void *)b)));
}
