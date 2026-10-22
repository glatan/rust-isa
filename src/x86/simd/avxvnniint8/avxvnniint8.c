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

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbuud(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpbuud_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbuuds(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpbuuds_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)acc),
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbssd_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpbssd_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbssds_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpbssds_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbsud_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpbsud_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbsuds_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpbsuds_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbuud_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpbuud_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avxvnniint8_vpdpbuuds_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpbuuds_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)acc),
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}
