#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbssd(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpbssd_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbssds(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpbssds_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbsud(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpbsud_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbsuds(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpbsuds_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}
