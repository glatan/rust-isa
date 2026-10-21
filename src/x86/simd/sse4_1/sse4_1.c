#include <stdint.h>
#include <smmintrin.h>

__attribute__((always_inline))
void rust_isa_sse41_pmulld(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mullo_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse41_pminud(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_min_epu32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse41_pmaxud(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_max_epu32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse41_pminuw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_min_epu16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse41_pmaxuw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_max_epu16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
int32_t rust_isa_sse41_ptest(const uint8_t *a, const uint8_t *b) {
    return _mm_testz_si128(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b));
}

__attribute__((always_inline))
void rust_isa_sse41_blendvps(const uint8_t *a, const uint8_t *b, const uint8_t *mask, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_blendv_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)mask)));
}

__attribute__((always_inline))
void rust_isa_sse41_blendvpd(const uint8_t *a, const uint8_t *b, const uint8_t *mask, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_blendv_pd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)mask)));
}

__attribute__((always_inline))
void rust_isa_sse41_pblendvb(const uint8_t *a, const uint8_t *b, const uint8_t *mask, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_blendv_epi8(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)mask)));
}

__attribute__((always_inline))
void rust_isa_sse41_packusdw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_packus_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse41_phminposuw(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_minpos_epu16(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse41_pmovsxbd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cvtepi8_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse41_pmovsxbw(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cvtepi8_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse41_pmovzxbw(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cvtepu8_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}
