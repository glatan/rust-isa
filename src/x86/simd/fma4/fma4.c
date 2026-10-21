#include <stdint.h>
#include <x86intrin.h>
#include <fma4intrin.h>

__attribute__((always_inline))
void rust_isa_fma4_macc_ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_macc_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_macc_pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_macc_pd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_msub_ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_msub_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}
