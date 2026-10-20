#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_neon_v128_load(const uint8_t *p, uint8_t out[16]) {
    vst1q_u8(out, vld1q_u8(p));
}

__attribute__((always_inline))
void rust_isa_neon_v128_store(uint8_t *p, const uint8_t value[16]) {
    vst1q_u8(p, vld1q_u8(value));
}

__attribute__((always_inline))
void rust_isa_neon_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vaddq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u32((uint32_t *)(void *)out, vmulq_u32(vld1q_u32((const uint32_t *)(const void *)a), vld1q_u32((const uint32_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vaddq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_fmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_f32((float *)(void *)out, vmulq_f32(vld1q_f32((const float *)(const void *)a), vld1q_f32((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_neon_tbl(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    vst1q_u8(out, vqtbl1q_u8(vld1q_u8(a), vld1q_u8(idx)));
}
