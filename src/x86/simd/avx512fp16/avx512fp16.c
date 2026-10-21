#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512fp16_vaddph(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_ph(
        _mm512_loadu_si512((const void *)a),
        _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vsubph(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_ph(
        _mm512_loadu_si512((const void *)a),
        _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vmulph(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mul_ph(
        _mm512_loadu_si512((const void *)a),
        _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vdivph(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_div_ph(
        _mm512_loadu_si512((const void *)a),
        _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vcvtph2psx(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_cvtxph_ps(
        (__m256h)_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vcvtps2phx(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, (__m256i)_mm512_cvtx_roundps_ph(
        _mm512_loadu_ps((const float *)(const void *)a),
        _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC));
}
