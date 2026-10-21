#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_arm_sha2_sha256h(const uint8_t *hash_abcd, const uint8_t *hash_efgh, const uint8_t *wk, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vsha256hq_u32(
        vld1q_u32((const uint32_t *)(const void *)hash_abcd),
        vld1q_u32((const uint32_t *)(const void *)hash_efgh),
        vld1q_u32((const uint32_t *)(const void *)wk)));
}

__attribute__((always_inline))
void rust_isa_arm_sha2_sha256h2(const uint8_t *hash_abcd, const uint8_t *hash_efgh, const uint8_t *wk, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vsha256h2q_u32(
        vld1q_u32((const uint32_t *)(const void *)hash_abcd),
        vld1q_u32((const uint32_t *)(const void *)hash_efgh),
        vld1q_u32((const uint32_t *)(const void *)wk)));
}

__attribute__((always_inline))
void rust_isa_arm_sha2_sha256su0(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vsha256su0q_u32(
        vld1q_u32((const uint32_t *)(const void *)a),
        vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_sha2_sha256su1(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vsha256su1q_u32(
        vld1q_u32((const uint32_t *)(const void *)a),
        vld1q_u32((const uint32_t *)(const void *)b),
        vld1q_u32((const uint32_t *)(const void *)c)));
}
