#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwsud(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpwsud_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwsuds(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpwsuds_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwusd(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpwusd_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwusds(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpwusds_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwuud(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpwuud_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwuuds(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpwuuds_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwsud_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpwsud_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwsuds_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpwsuds_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwusd_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpwusd_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwusds_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpwusds_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwuud_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpwuud_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint16_vpdpwuuds_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpwuuds_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}
