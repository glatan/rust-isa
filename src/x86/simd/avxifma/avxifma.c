#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avxifma_vpmadd52luq(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_madd52lo_epu64(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxifma_vpmadd52huq(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_madd52hi_epu64(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}
