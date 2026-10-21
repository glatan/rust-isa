#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx_vaddps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_add_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vaddpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_add_pd(
        _mm256_loadu_pd((const double *)(const void *)a),
        _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vsubps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_sub_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vsubpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_sub_pd(
        _mm256_loadu_pd((const double *)(const void *)a),
        _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vmulps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_mul_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vmulpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_mul_pd(
        _mm256_loadu_pd((const double *)(const void *)a),
        _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vdivps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_div_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vdivpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_div_pd(
        _mm256_loadu_pd((const double *)(const void *)a),
        _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vandps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_and_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vorps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_or_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vxorps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_xor_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vaddss(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_add_ss(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vaddsd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_add_sd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vbroadcastss(float value, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_set1_ps(value));
}

__attribute__((always_inline))
void rust_isa_avx_vbroadcastsd(double value, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_set1_pd(value));
}

__attribute__((always_inline))
uint32_t rust_isa_avx_vmovmskps(const uint8_t *a) {
    return (uint32_t)_mm256_movemask_ps(_mm256_loadu_ps((const float *)(const void *)a));
}

__attribute__((always_inline))
void rust_isa_avx_vzeroupper(void) {
    _mm256_zeroupper();
}
