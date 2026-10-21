#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512bf16_vcvtneps2bf16(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, (__m256i)_mm512_cvtneps_pbh(
        _mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bf16_vdpbf16ps(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_dpbf16_ps(
        _mm512_loadu_si512((const void *)acc),
        (__m512bh)_mm512_loadu_si512((const void *)a),
        (__m512bh)_mm512_loadu_si512((const void *)b)));
}
