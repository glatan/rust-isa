#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_arm_i8mm_smmla(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmmlaq_s32(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1q_s8((const int8_t *)(const void *)a),
        vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_i8mm_vummla(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmmlaq_u32(
        vld1q_u32((const uint32_t *)(const void *)acc),
        vld1q_u8(a),
        vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_arm_i8mm_usmmla(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vusmmlaq_s32(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1q_u8(a),
        vld1q_s8((const int8_t *)(const void *)b)));
}
