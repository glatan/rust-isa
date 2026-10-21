#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_sha3_veor3q_u8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u8(out, veor3q_u8(vld1q_u8(a), vld1q_u8(b), vld1q_u8(c)));
}

__attribute__((always_inline))
void rust_isa_sha3_vbcaxq_u8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u8(out, vbcaxq_u8(vld1q_u8(a), vld1q_u8(b), vld1q_u8(c)));
}

__attribute__((always_inline))
void rust_isa_sha3_vrax1q_u64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vrax1q_u64(
        vld1q_u64((const uint64_t *)(const void *)a),
        vld1q_u64((const uint64_t *)(const void *)b)));
}
