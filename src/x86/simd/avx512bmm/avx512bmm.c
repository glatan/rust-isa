#include <immintrin.h>
#include <stdint.h>

__attribute__((always_inline))
void rust_isa_avx512bmm_vbitrevb_v128(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm128_bitrev_epi8(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bmm_vbitrevb_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_bitrev_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bmm_vbitrevb_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_bitrev_epi8(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bmm_vbmacor16x16x16_v256(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_bmacor16x16x16(
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b),
        _mm256_loadu_si256((const __m256i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512bmm_vbmacor16x16x16_v512(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_bmacor16x16x16(
        _mm512_loadu_si512((const void *)a),
        _mm512_loadu_si512((const void *)b),
        _mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512bmm_vbmacxor16x16x16_v256(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_bmacxor16x16x16(
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b),
        _mm256_loadu_si256((const __m256i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512bmm_vbmacxor16x16x16_v512(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_bmacxor16x16x16(
        _mm512_loadu_si512((const void *)a),
        _mm512_loadu_si512((const void *)b),
        _mm512_loadu_si512((const void *)c)));
}
