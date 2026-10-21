#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_sha512_vsha512msg1(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sha512msg1_epi64(
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sha512_vsha512msg2(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sha512msg2_epi64(
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sha512_vsha512rnds2(const uint8_t *a, const uint8_t *b, const uint8_t *k, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sha512rnds2_epi64(
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)k)));
}
