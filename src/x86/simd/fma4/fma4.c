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

__attribute__((always_inline))
void rust_isa_fma4_vfmaddps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_macc_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfmaddpd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_macc_pd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfmaddss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_macc_ss(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfmaddsd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_macc_sd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfmsubps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_msub_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfmsubpd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_msub_pd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfmsubss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_msub_ss(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfmsubsd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_msub_sd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfnmaddps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_nmacc_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfnmaddpd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_nmacc_pd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfnmaddss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_nmacc_ss(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfnmaddsd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_nmacc_sd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfnmsubps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_nmsub_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfnmsubpd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_nmsub_pd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfnmsubss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_nmsub_ss(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfnmsubsd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_nmsub_sd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfmaddsubps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_maddsub_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfmaddsubpd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_maddsub_pd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfmsubaddps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_msubadd_ps(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b),
        _mm_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_fma4_vfmsubaddpd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_msubadd_pd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b),
        _mm_loadu_pd((const double *)(const void *)c)));
}
