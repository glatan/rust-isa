#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_dotprod_sdot(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_s32((int32_t *)(void *)out, vdotq_s32(vld1q_s32((const int32_t *)(const void *)acc), vld1q_u8(a), vld1q_s8((const int8_t *)(const void *)b)));
}
