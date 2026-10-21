#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_sm4_vsm4eq_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vsm4eq_u32(
        vld1q_u32((const uint32_t *)(const void *)a),
        vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sm4_vsm4ekeyq_u32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vsm4ekeyq_u32(
        vld1q_u32((const uint32_t *)(const void *)a),
        vld1q_u32((const uint32_t *)(const void *)b)));
}
