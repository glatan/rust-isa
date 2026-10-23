#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512f_v512_load(const uint8_t *p, uint8_t out[64]) {
    _mm512_storeu_si512((void *)out, _mm512_loadu_si512((const void *)p));
}

__attribute__((always_inline))
void rust_isa_avx512f_v512_store(uint8_t *p, const uint8_t value[64]) {
    _mm512_storeu_si512((void *)p, _mm512_loadu_si512((const void *)value));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpaddd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_epi32(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpsubd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_epi32(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpmulld(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mullo_epi32(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpaddq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_epi64(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpsubq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_epi64(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpandd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_and_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpandq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_and_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpandnd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_andnot_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpandnq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_andnot_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpord(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_or_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vporq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_or_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpxord(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_xor_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpxorq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_xor_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint16_t rust_isa_avx512f_vptestmd(const uint8_t *a, const uint8_t *b) {
    return (uint16_t)(_mm512_test_epi32_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx512f_vptestmq(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm512_test_epi64_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint16_t rust_isa_avx512f_vptestnmd(const uint8_t *a, const uint8_t *b) {
    return (uint16_t)(_mm512_testn_epi32_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx512f_vptestnmq(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm512_testn_epi64_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpbroadcastd(uint32_t value, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_set1_epi32((int)value));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpbroadcastq(uint64_t value, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_set1_epi64((long long)value));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpermd(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_permutexvar_epi32(_mm512_loadu_si512((const void *)idx), _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpermq(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_permutexvar_epi64(_mm512_loadu_si512((const void *)idx), _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpslld(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sll_epi32(_mm512_loadu_si512((const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpsrld(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_srl_epi32(_mm512_loadu_si512((const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vcompressps(const uint8_t *a, uint16_t mask, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_maskz_compress_ps((__mmask16)mask, _mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vcompresspd(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_maskz_compress_pd((__mmask8)mask, _mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vexpandps(const uint8_t *a, uint16_t mask, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_maskz_expand_ps((__mmask16)mask, _mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vexpandpd(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_maskz_expand_pd((__mmask8)mask, _mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vblendmps(const uint8_t *a, const uint8_t *b, uint16_t mask, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_mask_blend_ps((__mmask16)mask, _mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vblendmpd(const uint8_t *a, const uint8_t *b, uint8_t mask, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_mask_blend_pd((__mmask8)mask, _mm512_loadu_pd((const double *)(const void *)a), _mm512_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vmovaps(const uint8_t *a, const uint8_t *src, uint16_t mask, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_mask_mov_ps(_mm512_loadu_ps((const float *)(const void *)a), (__mmask16)mask, _mm512_loadu_ps((const float *)(const void *)src)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vmovapd(const uint8_t *a, const uint8_t *src, uint8_t mask, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_mask_mov_pd(_mm512_loadu_pd((const double *)(const void *)a), (__mmask8)mask, _mm512_loadu_pd((const double *)(const void *)src)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vcvtps2pd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_cvtps_pd(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vcvtpd2ps(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm512_cvtpd_ps(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vcvtss2sd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_cvtss_sd(_mm_loadu_pd((const double *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vcvtsd2ss(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_cvtsd_ss(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vcvtdq2ps(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_cvtepi32_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vcvtps2dq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvtps_epi32(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vcvttps2dq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvttps_epi32(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vcvtpd2dq(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm512_cvtpd_epi32(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vcvttpd2dq(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm512_cvttpd_epi32(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vsqrtps(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_sqrt_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vsqrtpd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_sqrt_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vgetexpps(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_getexp_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vgetexppd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_getexp_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vscalefps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_scalef_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vscalefpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_scalef_pd(_mm512_loadu_pd((const double *)(const void *)a), _mm512_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vrcpps(const uint8_t *a, uint8_t *out) {
    __m512 v = _mm512_loadu_ps((const float *)(const void *)a);
    __m128 r0 = _mm_rcp_ps(_mm512_extractf32x4_ps(v, 0));
    __m128 r1 = _mm_rcp_ps(_mm512_extractf32x4_ps(v, 1));
    __m128 r2 = _mm_rcp_ps(_mm512_extractf32x4_ps(v, 2));
    __m128 r3 = _mm_rcp_ps(_mm512_extractf32x4_ps(v, 3));
    __m512 outv = _mm512_insertf32x4(_mm512_castps128_ps512(r0), r1, 1);
    outv = _mm512_insertf32x4(outv, r2, 2);
    outv = _mm512_insertf32x4(outv, r3, 3);
    _mm512_storeu_ps((float *)(void *)out, outv);
}

__attribute__((always_inline))
void rust_isa_avx512f_vrsqrtps(const uint8_t *a, uint8_t *out) {
    __m512 v = _mm512_loadu_ps((const float *)(const void *)a);
    __m128 r0 = _mm_rsqrt_ps(_mm512_extractf32x4_ps(v, 0));
    __m128 r1 = _mm_rsqrt_ps(_mm512_extractf32x4_ps(v, 1));
    __m128 r2 = _mm_rsqrt_ps(_mm512_extractf32x4_ps(v, 2));
    __m128 r3 = _mm_rsqrt_ps(_mm512_extractf32x4_ps(v, 3));
    __m512 outv = _mm512_insertf32x4(_mm512_castps128_ps512(r0), r1, 1);
    outv = _mm512_insertf32x4(outv, r2, 2);
    outv = _mm512_insertf32x4(outv, r3, 3);
    _mm512_storeu_ps((float *)(void *)out, outv);
}

__attribute__((always_inline))
void rust_isa_avx512f_vgatherdps(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint16_t mask, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_mask_i32gather_ps(_mm512_loadu_ps((const float *)(const void *)src), (__mmask16)mask, _mm512_loadu_si512((const void *)indices), (const void *)base, 4));
}

__attribute__((always_inline))
void rust_isa_avx512f_vgatherdpd(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint8_t mask, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_mask_i32gather_pd(_mm512_loadu_pd((const double *)(const void *)src), (__mmask8)mask, _mm256_loadu_si256((const __m256i *)(const void *)indices), (const void *)base, 8));
}

__attribute__((always_inline))
void rust_isa_avx512f_vgatherqps(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint8_t mask, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm512_mask_i64gather_ps(_mm256_loadu_ps((const float *)(const void *)src), (__mmask8)mask, _mm512_loadu_si512((const void *)indices), (const void *)base, 4));
}

__attribute__((always_inline))
void rust_isa_avx512f_vgatherqpd(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint8_t mask, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_mask_i64gather_pd(_mm512_loadu_pd((const double *)(const void *)src), (__mmask8)mask, _mm512_loadu_si512((const void *)indices), (const void *)base, 8));
}

__attribute__((always_inline))
void rust_isa_avx512f_vscatterdps(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint16_t mask) {
    _mm512_mask_i32scatter_ps((void *)base, (__mmask16)mask, _mm512_loadu_si512((const void *)indices), _mm512_loadu_ps((const float *)(const void *)a), 4);
}

__attribute__((always_inline))
void rust_isa_avx512f_vscatterdpd(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint8_t mask) {
    _mm512_mask_i32scatter_pd((void *)base, (__mmask8)mask, _mm256_loadu_si256((const __m256i *)(const void *)indices), _mm512_loadu_pd((const double *)(const void *)a), 8);
}

__attribute__((always_inline))
void rust_isa_avx512f_vscatterqps(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint8_t mask) {
    _mm512_mask_i64scatter_ps((void *)base, (__mmask8)mask, _mm512_loadu_si512((const void *)indices), _mm256_loadu_ps((const float *)(const void *)a), 4);
}

__attribute__((always_inline))
void rust_isa_avx512f_vscatterqpd(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint8_t mask) {
    _mm512_mask_i64scatter_pd((void *)base, (__mmask8)mask, _mm512_loadu_si512((const void *)indices), _mm512_loadu_pd((const double *)(const void *)a), 8);
}
