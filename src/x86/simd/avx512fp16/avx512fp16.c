#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512fp16_vaddph(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_add_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vsubph(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_sub_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vmulph(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_mul_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vdivph(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_div_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vsqrtph(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_sqrt_ph((__m512h)_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vmaxph(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_max_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vminph(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_min_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint32_t rust_isa_avx512fp16_vcomish(const uint8_t *a, const uint8_t *b) {
    __m128h va = (__m128h)_mm_loadu_si128((const __m128i *)(const void *)a);
    __m128h vb = (__m128h)_mm_loadu_si128((const __m128i *)(const void *)b);
    uint32_t cf, pf, zf;
    __asm__ volatile(
        "vcomish %4, %3\n\t"
        "setc %b0\n\t"
        "setp %b1\n\t"
        "sete %b2"
        : "=r"(cf), "=r"(pf), "=r"(zf)
        : "x"(va), "x"(vb)
        : "cc");
    return (cf & 1) | ((pf & 1) << 1) | ((zf & 1) << 2);
}

__attribute__((always_inline))
uint32_t rust_isa_avx512fp16_vucomish(const uint8_t *a, const uint8_t *b) {
    __m128h va = (__m128h)_mm_loadu_si128((const __m128i *)(const void *)a);
    __m128h vb = (__m128h)_mm_loadu_si128((const __m128i *)(const void *)b);
    uint32_t cf, pf, zf;
    __asm__ volatile(
        "vucomish %4, %3\n\t"
        "setc %b0\n\t"
        "setp %b1\n\t"
        "sete %b2"
        : "=r"(cf), "=r"(pf), "=r"(zf)
        : "x"(va), "x"(vb)
        : "cc");
    return (cf & 1) | ((pf & 1) << 1) | ((zf & 1) << 2);
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vgetexpph(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_getexp_ph((__m512h)_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vscalefph(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_scalef_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vrcpph(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_rcp_ph((__m512h)_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vrsqrtph(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_rsqrt_ph((__m512h)_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vcvtph2pd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_cvtph_pd((__m128h)(__m128h)_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((always_inline))
void rust_isa_avx512fp16_vcvtpd2ph(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm512_cvtpd_ph(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((always_inline))
void rust_isa_avx512fp16_vcvtsh2ss(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_cvtsh_ss(_mm_loadu_ps((const float *)(const void *)a), (__m128h)(__m128h)_mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vcvtss2sh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_cvtss_sh((__m128h)(__m128h)_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vcvtsh2sd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_cvtsh_sd(_mm_loadu_pd((const double *)(const void *)a), (__m128h)(__m128h)_mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vcvtsd2sh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_cvtsd_sh((__m128h)(__m128h)_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vcvtph2psx(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_cvtxph_ps((__m256h)(__m256h)_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vcvtps2phx(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, (__m256i)_mm512_cvtx_roundps_ph(_mm512_loadu_ps((const float *)(const void *)a), _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmadd132ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmadd_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmadd213ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmadd_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmadd231ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmadd_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmsub132ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmsub_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmsub213ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmsub_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmsub231ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmsub_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfnmadd132ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fnmadd_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfnmadd213ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fnmadd_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfnmadd231ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fnmadd_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfnmsub132ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fnmsub_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfnmsub213ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fnmsub_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfnmsub231ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fnmsub_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmaddsub132ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmaddsub_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmaddsub213ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmaddsub_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmaddsub231ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmaddsub_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmsubadd132ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmsubadd_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmsubadd213ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmsubadd_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx512fp16_vfmsubadd231ph(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_si512((void *)out, (__m512i)_mm512_fmsubadd_ph((__m512h)_mm512_loadu_si512((const void *)a), (__m512h)_mm512_loadu_si512((const void *)b), (__m512h)_mm512_loadu_si512((const void *)c)));
}
