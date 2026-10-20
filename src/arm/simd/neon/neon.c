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
