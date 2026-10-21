#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_arm_aes_aese(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vaeseq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_arm_aes_aesd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u8(out, vaesdq_u8(vld1q_u8(a), vld1q_u8(b)));
}

__attribute__((always_inline))
void rust_isa_arm_aes_aesmc(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, vaesmcq_u8(vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_arm_aes_aesimc(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, vaesimcq_u8(vld1q_u8(a)));
}
