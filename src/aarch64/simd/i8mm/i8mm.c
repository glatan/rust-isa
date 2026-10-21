#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_i8mm_vmmlaq_s32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vmmlaq_s32(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1q_s8((const int8_t *)(const void *)a),
        vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_i8mm_vusmmlaq_s32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vusmmlaq_s32(
        vld1q_s32((const int32_t *)(const void *)acc),
        vld1q_u8(a),
        vld1q_s8((const int8_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_i8mm_vummlaq_s32(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    uint32x4_t result = vld1q_u32((const uint32_t *)(const void *)acc);
    __asm__("ummla %0.4s, %1.16b, %2.16b"
            : "+w"(result)
            : "w"(vld1q_u8(a)), "w"(vld1q_u8(b)));
    vst1q_u32((uint32_t *)(void *)out, result);
}
