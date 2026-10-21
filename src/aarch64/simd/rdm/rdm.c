#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_rdm_vqrdmlahq_s32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqrdmlahq_s32(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1q_s32((const int32_t *)(const void *)a),
        vld1q_s32((const int32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_rdm_vqrdmlshq_s32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vqrdmlshq_s32(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1q_s32((const int32_t *)(const void *)a),
        vld1q_s32((const int32_t *)(const void *)b)));
}
