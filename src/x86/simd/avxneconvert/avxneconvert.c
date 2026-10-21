#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avxneconvert_vcvtneps2bf16(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm256_cvtneps_avx_pbh(
        _mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avxneconvert_vbcstnebf162ps(const uint16_t *value, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_bcstnebf16_ps(value));
}

__attribute__((always_inline))
void rust_isa_avxneconvert_vbcstnesh2ps(const uint16_t *value, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_bcstnesh_ps(value));
}
