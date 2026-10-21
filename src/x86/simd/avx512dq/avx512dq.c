#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512dq_vpmullq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mullo_epi64(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vpmovm2d(uint16_t mask, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_movm_epi32((__mmask16)mask));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vpmovm2q(uint8_t mask, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_movm_epi64((__mmask8)mask));
}

__attribute__((always_inline))
uint16_t rust_isa_avx512dq_vpmovd2m(const uint8_t *a) {
    return (uint16_t)(_mm512_movepi32_mask(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx512dq_vpmovq2m(const uint8_t *a) {
    return (uint8_t)(_mm512_movepi64_mask(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vandpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_and_pd(_mm512_loadu_pd((const double *)(const void *)a), _mm512_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vandnpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_andnot_pd(_mm512_loadu_pd((const double *)(const void *)a), _mm512_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vorpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_or_pd(_mm512_loadu_pd((const double *)(const void *)a), _mm512_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vxorpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_xor_pd(_mm512_loadu_pd((const double *)(const void *)a), _mm512_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vandps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_and_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vandnps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_andnot_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vorps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_or_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vxorps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_xor_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vgetexpps(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_getexp_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vgetexppd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_getexp_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vgetexpss(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_getexp_ss(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vgetexpsd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_getexp_sd(_mm_loadu_pd((const double *)(const void *)a), _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vscalefps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_scalef_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vscalefpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_scalef_pd(_mm512_loadu_pd((const double *)(const void *)a), _mm512_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vscalefss(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_scalef_ss(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vscalefsd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_scalef_sd(_mm_loadu_pd((const double *)(const void *)a), _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vrcp14ps(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_rcp14_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vrcp14pd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_rcp14_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vrcp14ss(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_rcp14_ss(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vrcp14sd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_rcp14_sd(_mm_loadu_pd((const double *)(const void *)a), _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vrsqrt14ps(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_rsqrt14_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vrsqrt14pd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_rsqrt14_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vrsqrt14ss(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_rsqrt14_ss(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vrsqrt14sd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_rsqrt14_sd(_mm_loadu_pd((const double *)(const void *)a), _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvtpd2qq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvtpd_epi64(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvtpd2uqq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvtpd_epu64(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvtps2qq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvtps_epi64(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvtps2uqq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvtps_epu64(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvtqq2pd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_cvtepi64_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvtqq2ps(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm512_cvtepi64_ps(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvtuqq2pd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_cvtepu64_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvtuqq2ps(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm512_cvtepu64_ps(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvttpd2qq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvttpd_epi64(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvttpd2uqq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvttpd_epu64(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvttps2qq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvttps_epi64(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512dq_vcvttps2uqq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvttps_epu64(_mm256_loadu_ps((const float *)(const void *)a)));
}
