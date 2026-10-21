#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx10_2_vaddps_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_add_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vmulps_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_mul_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vgetexpps_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_getexp_ps(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vgetexpps_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_getexp_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vgetexppd_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_getexp_pd(_mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vgetexppd_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_getexp_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vgetexpph_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, (__m256i)_mm256_getexp_ph((__m256h)_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vgetexpph_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_getexp_ph((__m512h)_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vscalefps_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_scalef_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vscalefps_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_scalef_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vscalefpd_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_scalef_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vscalefpd_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_scalef_pd(_mm512_loadu_pd((const double *)(const void *)a), _mm512_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vscalefph_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, (__m256i)_mm256_scalef_ph((__m256h)_mm256_loadu_si256((const __m256i *)(const void *)a), (__m256h)_mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vscalefph_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_scalef_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vrcp14ps_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_rcp14_ps(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vrcp14ps_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_rcp14_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vrcp14pd_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_rcp14_pd(_mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vrcp14pd_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_rcp14_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vrsqrt14ps_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_rsqrt14_ps(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vrsqrt14ps_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_rsqrt14_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vrsqrt14pd_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_rsqrt14_pd(_mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vrsqrt14pd_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_rsqrt14_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vsqrtps_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_sqrt_ps(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vsqrtps_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_sqrt_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vsqrtpd_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_sqrt_pd(_mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vsqrtpd_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_sqrt_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
uint32_t rust_isa_avx10_2_vcomisbf16(const uint8_t *a, const uint8_t *b) {
    __m128bh va = (__m128bh)_mm_loadu_si128((const __m128i *)(const void *)a);
    __m128bh vb = (__m128bh)_mm_loadu_si128((const __m128i *)(const void *)b);
    uint32_t cf, pf, zf;
    __asm__ volatile(
        "vcomisbf16 %4, %3\n\t"
        "setc %b0\n\t"
        "setp %b1\n\t"
        "sete %b2"
        : "=r"(cf), "=r"(pf), "=r"(zf)
        : "x"(va), "x"(vb)
        : "cc");
    return (cf & 1) | ((pf & 1) << 1) | ((zf & 1) << 2);
}

__attribute__((always_inline))
uint32_t rust_isa_avx10_2_vcomiss_v128(const uint8_t *a, const uint8_t *b) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    uint32_t cf, pf, zf;
    __asm__ volatile(
        "vcomiss %4, %3\n\t"
        "setc %b0\n\t"
        "setp %b1\n\t"
        "sete %b2"
        : "=r"(cf), "=r"(pf), "=r"(zf)
        : "x"(va), "x"(vb)
        : "cc");
    return (cf & 1) | ((pf & 1) << 1) | ((zf & 1) << 2);
}

__attribute__((always_inline))
uint32_t rust_isa_avx10_2_vcomisd_v128(const uint8_t *a, const uint8_t *b) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    uint32_t cf, pf, zf;
    __asm__ volatile(
        "vcomisd %4, %3\n\t"
        "setc %b0\n\t"
        "setp %b1\n\t"
        "sete %b2"
        : "=r"(cf), "=r"(pf), "=r"(zf)
        : "x"(va), "x"(vb)
        : "cc");
    return (cf & 1) | ((pf & 1) << 1) | ((zf & 1) << 2);
}
