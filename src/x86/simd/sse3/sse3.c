#include <stdint.h>
#include <pmmintrin.h>

__attribute__((always_inline))
void rust_isa_sse3_addsubps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_addsub_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse3_addsubpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_addsub_pd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse3_haddps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_hadd_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse3_haddpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_hadd_pd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse3_hsubps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_hsub_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse3_hsubpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_hsub_pd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse3_lddqu(const uint8_t *p, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_lddqu_si128((const __m128i *)(const void *)p));
}

__attribute__((always_inline))
void rust_isa_sse3_movsldup(const uint8_t *a, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_moveldup_ps(_mm_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse3_movshdup(const uint8_t *a, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_movehdup_ps(_mm_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse3_movddup(const uint8_t *a, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_movedup_pd(_mm_loadu_pd((const double *)(const void *)a)));
}
