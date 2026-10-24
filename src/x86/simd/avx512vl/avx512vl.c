#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddd_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_add_epi32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddd_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddq_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_add_epi64(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubd_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sub_epi32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubd_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubq_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sub_epi64(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpandd_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_and_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpandd_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_and_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpandq_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_and_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpandq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_and_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpandnd_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_andnot_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpandnd_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_andnot_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpandnq_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_andnot_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpandnq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_andnot_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpord_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_or_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpord_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_or_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vporq_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_or_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vporq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_or_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpxord_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_xor_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpxord_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_xor_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpxorq_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_xor_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpxorq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_xor_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vptestmd_v128(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm_test_epi32_mask(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vptestmd_v256(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm256_test_epi32_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vptestnmd_v128(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm_testn_epi32_mask(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vptestnmd_v256(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm256_testn_epi32_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vptestmq_v128(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm_test_epi64_mask(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vptestmq_v256(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm256_test_epi64_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vptestnmq_v128(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm_testn_epi64_mask(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vptestnmq_v256(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm256_testn_epi64_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint16_t rust_isa_avx512vl_vptestmb_v128(const uint8_t *a, const uint8_t *b) {
    return (uint16_t)(_mm_test_epi8_mask(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint32_t rust_isa_avx512vl_vptestmb_v256(const uint8_t *a, const uint8_t *b) {
    return (uint32_t)(_mm256_test_epi8_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint16_t rust_isa_avx512vl_vptestnmb_v128(const uint8_t *a, const uint8_t *b) {
    return (uint16_t)(_mm_testn_epi8_mask(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint32_t rust_isa_avx512vl_vptestnmb_v256(const uint8_t *a, const uint8_t *b) {
    return (uint32_t)(_mm256_testn_epi8_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vptestmw_v128(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm_test_epi16_mask(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint16_t rust_isa_avx512vl_vptestmw_v256(const uint8_t *a, const uint8_t *b) {
    return (uint16_t)(_mm256_test_epi16_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vptestnmw_v128(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm_testn_epi16_mask(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint16_t rust_isa_avx512vl_vptestnmw_v256(const uint8_t *a, const uint8_t *b) {
    return (uint16_t)(_mm256_testn_epi16_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
void rust_isa_avx512vl_vpmovm2b_v128(uint16_t mask, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_movm_epi8((__mmask16)mask));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
void rust_isa_avx512vl_vpmovm2b_v256(uint32_t mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_movm_epi8((__mmask32)mask));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
void rust_isa_avx512vl_vpmovm2w_v128(uint8_t mask, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_movm_epi16((__mmask8)mask));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
void rust_isa_avx512vl_vpmovm2w_v256(uint16_t mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_movm_epi16((__mmask16)mask));
}

__attribute__((target("avx512dq")))
__attribute__((always_inline))
void rust_isa_avx512vl_vpmovm2d_v128(uint8_t mask, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_movm_epi32((__mmask8)mask));
}

__attribute__((target("avx512dq")))
__attribute__((always_inline))
void rust_isa_avx512vl_vpmovm2d_v256(uint8_t mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_movm_epi32((__mmask8)mask));
}

__attribute__((target("avx512dq")))
__attribute__((always_inline))
void rust_isa_avx512vl_vpmovm2q_v128(uint8_t mask, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_movm_epi64((__mmask8)mask));
}

__attribute__((target("avx512dq")))
__attribute__((always_inline))
void rust_isa_avx512vl_vpmovm2q_v256(uint8_t mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_movm_epi64((__mmask8)mask));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint16_t rust_isa_avx512vl_vpmovb2m_v128(const uint8_t *a) {
    return (uint16_t)(_mm_movepi8_mask(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint32_t rust_isa_avx512vl_vpmovb2m_v256(const uint8_t *a) {
    return (uint32_t)(_mm256_movepi8_mask(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vpmovw2m_v128(const uint8_t *a) {
    return (uint8_t)(_mm_movepi16_mask(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((target("avx512bw")))
__attribute__((always_inline))
uint16_t rust_isa_avx512vl_vpmovw2m_v256(const uint8_t *a) {
    return (uint16_t)(_mm256_movepi16_mask(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((target("avx512dq")))
__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vpmovd2m_v128(const uint8_t *a) {
    return (uint8_t)(_mm_movepi32_mask(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((target("avx512dq")))
__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vpmovd2m_v256(const uint8_t *a) {
    return (uint8_t)(_mm256_movepi32_mask(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((target("avx512dq")))
__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vpmovq2m_v128(const uint8_t *a) {
    return (uint8_t)(_mm_movepi64_mask(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((target("avx512dq")))
__attribute__((always_inline))
uint8_t rust_isa_avx512vl_vpmovq2m_v256(const uint8_t *a) {
    return (uint8_t)(_mm256_movepi64_mask(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vcompressps_v128(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_maskz_compress_ps((__mmask8)mask, _mm_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vcompressps_v256(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_maskz_compress_ps((__mmask8)mask, _mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vcompresspd_v128(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_maskz_compress_pd((__mmask8)mask, _mm_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vcompresspd_v256(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_maskz_compress_pd((__mmask8)mask, _mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vexpandps_v128(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_maskz_expand_ps((__mmask8)mask, _mm_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vexpandps_v256(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_maskz_expand_ps((__mmask8)mask, _mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vexpandpd_v128(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_maskz_expand_pd((__mmask8)mask, _mm_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vexpandpd_v256(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_maskz_expand_pd((__mmask8)mask, _mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vblendmps_v128(const uint8_t *a, const uint8_t *b, uint8_t mask, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_mask_blend_ps((__mmask8)mask, _mm_loadu_ps((const float *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vblendmps_v256(const uint8_t *a, const uint8_t *b, uint8_t mask, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_mask_blend_ps((__mmask8)mask, _mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vblendmpd_v128(const uint8_t *a, const uint8_t *b, uint8_t mask, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_mask_blend_pd((__mmask8)mask, _mm_loadu_pd((const double *)(const void *)a), _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vblendmpd_v256(const uint8_t *a, const uint8_t *b, uint8_t mask, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_mask_blend_pd((__mmask8)mask, _mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vmovaps_v128(const uint8_t *a, const uint8_t *src, uint8_t mask, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_mask_mov_ps(_mm_loadu_ps((const float *)(const void *)a), (__mmask8)mask, _mm_loadu_ps((const float *)(const void *)src)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vmovaps_v256(const uint8_t *a, const uint8_t *src, uint8_t mask, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_mask_mov_ps(_mm256_loadu_ps((const float *)(const void *)a), (__mmask8)mask, _mm256_loadu_ps((const float *)(const void *)src)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vmovapd_v128(const uint8_t *a, const uint8_t *src, uint8_t mask, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_mask_mov_pd(_mm_loadu_pd((const double *)(const void *)a), (__mmask8)mask, _mm_loadu_pd((const double *)(const void *)src)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vmovapd_v256(const uint8_t *a, const uint8_t *src, uint8_t mask, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_mask_mov_pd(_mm256_loadu_pd((const double *)(const void *)a), (__mmask8)mask, _mm256_loadu_pd((const double *)(const void *)src)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vgatherdps_v128(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint8_t mask, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_mmask_i32gather_ps(_mm_loadu_ps((const float *)(const void *)src), (__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)indices), (const void *)base, 4));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vgatherdps_v256(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint8_t mask, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_mmask_i32gather_ps(_mm256_loadu_ps((const float *)(const void *)src), (__mmask8)mask, _mm256_loadu_si256((const __m256i *)(const void *)indices), (const void *)base, 4));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vgatherdpd_v128(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint8_t mask, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_mmask_i32gather_pd(_mm_loadu_pd((const double *)(const void *)src), (__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)indices), (const void *)base, 8));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vgatherdpd_v256(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint8_t mask, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_mmask_i32gather_pd(_mm256_loadu_pd((const double *)(const void *)src), (__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)indices), (const void *)base, 8));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vgatherqps_v128(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint8_t mask, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_mmask_i64gather_ps(_mm_loadu_ps((const float *)(const void *)src), (__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)indices), (const void *)base, 4));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vgatherqps_v256(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint8_t mask, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm256_mmask_i64gather_ps(_mm_loadu_ps((const float *)(const void *)src), (__mmask8)mask, _mm256_loadu_si256((const __m256i *)(const void *)indices), (const void *)base, 4));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vgatherqpd_v128(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint8_t mask, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_mmask_i64gather_pd(_mm_loadu_pd((const double *)(const void *)src), (__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)indices), (const void *)base, 8));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vgatherqpd_v256(const uint8_t *src, const uint8_t *base, const uint8_t *indices, uint8_t mask, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_mmask_i64gather_pd(_mm256_loadu_pd((const double *)(const void *)src), (__mmask8)mask, _mm256_loadu_si256((const __m256i *)(const void *)indices), (const void *)base, 8));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vscatterdps_v128(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint8_t mask) {
    _mm_mask_i32scatter_ps((void *)base, (__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)indices), _mm_loadu_ps((const float *)(const void *)a), 4);
}

__attribute__((always_inline))
void rust_isa_avx512vl_vscatterdps_v256(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint8_t mask) {
    _mm256_mask_i32scatter_ps((void *)base, (__mmask8)mask, _mm256_loadu_si256((const __m256i *)(const void *)indices), _mm256_loadu_ps((const float *)(const void *)a), 4);
}

__attribute__((always_inline))
void rust_isa_avx512vl_vscatterdpd_v128(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint8_t mask) {
    _mm_mask_i32scatter_pd((void *)base, (__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)indices), _mm_loadu_pd((const double *)(const void *)a), 8);
}

__attribute__((always_inline))
void rust_isa_avx512vl_vscatterdpd_v256(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint8_t mask) {
    _mm256_mask_i32scatter_pd((void *)base, (__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)indices), _mm256_loadu_pd((const double *)(const void *)a), 8);
}

__attribute__((always_inline))
void rust_isa_avx512vl_vscatterqps_v128(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint8_t mask) {
    _mm_mask_i64scatter_ps((void *)base, (__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)indices), _mm_loadu_ps((const float *)(const void *)a), 4);
}

__attribute__((always_inline))
void rust_isa_avx512vl_vscatterqps_v256(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint8_t mask) {
    _mm256_mask_i64scatter_ps((void *)base, (__mmask8)mask, _mm256_loadu_si256((const __m256i *)(const void *)indices), _mm_loadu_ps((const float *)(const void *)a), 4);
}

__attribute__((always_inline))
void rust_isa_avx512vl_vscatterqpd_v128(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint8_t mask) {
    _mm_mask_i64scatter_pd((void *)base, (__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)indices), _mm_loadu_pd((const double *)(const void *)a), 8);
}

__attribute__((always_inline))
void rust_isa_avx512vl_vscatterqpd_v256(uint8_t *base, const uint8_t *indices, const uint8_t *a, uint8_t mask) {
    _mm256_mask_i64scatter_pd((void *)base, (__mmask8)mask, _mm256_loadu_si256((const __m256i *)(const void *)indices), _mm256_loadu_pd((const double *)(const void *)a), 8);
}

__attribute__((target("avx512dq")))
__attribute__((always_inline))
void rust_isa_avx512vl_vpmullq_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mullo_epi64(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((target("avx512dq")))
__attribute__((always_inline))
void rust_isa_avx512vl_vpmullq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mullo_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512ifma")))
void rust_isa_avx512vl_vpmadd52luq_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_madd52lo_epu64(_mm_loadu_si128((const __m128i *)(const void *)acc), _mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512ifma")))
void rust_isa_avx512vl_vpmadd52huq_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_madd52hi_epu64(_mm_loadu_si128((const __m128i *)(const void *)acc), _mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512ifma")))
void rust_isa_avx512vl_vpmadd52luq_v256(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_madd52lo_epu64(_mm256_loadu_si256((const __m256i *)(const void *)acc), _mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512ifma")))
void rust_isa_avx512vl_vpmadd52huq_v256(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_madd52hi_epu64(_mm256_loadu_si256((const __m256i *)(const void *)acc), _mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi")))
void rust_isa_avx512vl_vpermb_v128(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_permutexvar_epi8(_mm_loadu_si128((const __m128i *)(const void *)idx), _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi")))
void rust_isa_avx512vl_vpermi2b_v128(const uint8_t *a, const uint8_t *b, const uint8_t *idx, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_permutex2var_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)idx), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi")))
void rust_isa_avx512vl_vpermt2b_v128(const uint8_t *a, const uint8_t *b, const uint8_t *idx, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_permutex2var_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)idx), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi")))
void rust_isa_avx512vl_vpermb_v256(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_permutexvar_epi8(_mm256_loadu_si256((const __m256i *)(const void *)idx), _mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi")))
void rust_isa_avx512vl_vpermi2b_v256(const uint8_t *a, const uint8_t *b, const uint8_t *idx, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_permutex2var_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)idx), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi")))
void rust_isa_avx512vl_vpermt2b_v256(const uint8_t *a, const uint8_t *b, const uint8_t *idx, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_permutex2var_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)idx), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi2")))
void rust_isa_avx512vl_vpcompressb_v128(const uint8_t *a, uint16_t mask, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maskz_compress_epi8((__mmask16)mask, _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi2")))
void rust_isa_avx512vl_vpcompressb_v256(const uint8_t *a, uint32_t mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_maskz_compress_epi8((__mmask32)mask, _mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi2")))
void rust_isa_avx512vl_vpcompressw_v128(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maskz_compress_epi16((__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi2")))
void rust_isa_avx512vl_vpcompressw_v256(const uint8_t *a, uint16_t mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_maskz_compress_epi16((__mmask16)mask, _mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi2")))
void rust_isa_avx512vl_vpexpandb_v128(const uint8_t *a, uint16_t mask, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maskz_expand_epi8((__mmask16)mask, _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi2")))
void rust_isa_avx512vl_vpexpandb_v256(const uint8_t *a, uint32_t mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_maskz_expand_epi8((__mmask32)mask, _mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi2")))
void rust_isa_avx512vl_vpexpandw_v128(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maskz_expand_epi8((__mmask8)mask, _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vbmi2")))
void rust_isa_avx512vl_vpexpandw_v256(const uint8_t *a, uint16_t mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_maskz_expand_epi8((__mmask16)mask, _mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vnni")))
void rust_isa_avx512vl_vpdpbusd_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpbusd_epi32(_mm_loadu_si128((const __m128i *)(const void *)acc), _mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vnni")))
void rust_isa_avx512vl_vpdpbusd_v256(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpbusd_epi32(_mm256_loadu_si256((const __m256i *)(const void *)acc), _mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vnni")))
void rust_isa_avx512vl_vpdpbusds_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpbusds_epi32(_mm_loadu_si128((const __m128i *)(const void *)acc), _mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vnni")))
void rust_isa_avx512vl_vpdpbusds_v256(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpbusds_epi32(_mm256_loadu_si256((const __m256i *)(const void *)acc), _mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vnni")))
void rust_isa_avx512vl_vpdpwssd_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpwssd_epi32(_mm_loadu_si128((const __m128i *)(const void *)acc), _mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vnni")))
void rust_isa_avx512vl_vpdpwssd_v256(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpwssd_epi32(_mm256_loadu_si256((const __m256i *)(const void *)acc), _mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vnni")))
void rust_isa_avx512vl_vpdpwssds_v128(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_dpwssds_epi32(_mm_loadu_si128((const __m128i *)(const void *)acc), _mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512vnni")))
void rust_isa_avx512vl_vpdpwssds_v256(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpwssds_epi32(_mm256_loadu_si256((const __m256i *)(const void *)acc), _mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
__attribute__((target("avx512bitalg")))
void rust_isa_avx512vl_vpopcntb_v128(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_popcnt_epi8(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512bitalg")))
void rust_isa_avx512vl_vpopcntb_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_popcnt_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512bitalg")))
void rust_isa_avx512vl_vpopcntw_v128(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_popcnt_epi16(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512bitalg")))
void rust_isa_avx512vl_vpopcntw_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_popcnt_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vpopcntdq")))
void rust_isa_avx512vl_vpopcntd_v128(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_popcnt_epi32(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vpopcntdq")))
void rust_isa_avx512vl_vpopcntd_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_popcnt_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vpopcntdq")))
void rust_isa_avx512vl_vpopcntq_v128(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_popcnt_epi64(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
__attribute__((target("avx512vpopcntdq")))
void rust_isa_avx512vl_vpopcntq_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_popcnt_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((target("avx512bitalg")))
__attribute__((always_inline))
uint16_t rust_isa_avx512vl_vpshufbitqmb_v128(const uint8_t *a, const uint8_t *idx) {
    return (uint16_t)(_mm_bitshuffle_epi64_mask(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)idx)));
}

__attribute__((target("avx512bitalg")))
__attribute__((always_inline))
uint32_t rust_isa_avx512vl_vpshufbitqmb_v256(const uint8_t *a, const uint8_t *idx) {
    return (uint32_t)(_mm256_bitshuffle_epi64_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)idx)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddb_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_add_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddb_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_add_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubb_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sub_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubb_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sub_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmullw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mullo_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmullw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mullo_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmulhw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mulhi_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmulhw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mulhi_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmulhuw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mulhi_epu16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmulhuw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mulhi_epu16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmaddwd_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_madd_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmaddwd_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_madd_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmaddubsw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maddubs_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmaddubsw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_maddubs_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddsb_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_adds_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddsb_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_adds_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddsw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_adds_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddsw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_adds_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddusb_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_adds_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddusb_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_adds_epu8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddusw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_adds_epu16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpaddusw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_adds_epu16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubsb_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_subs_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubsb_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_subs_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubsw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_subs_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubsw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_subs_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubusb_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_subs_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubusb_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_subs_epu8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubusw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_subs_epu16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpsubusw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_subs_epu16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpavgb_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_avg_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpavgb_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_avg_epu8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpavgw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_avg_epu16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpavgw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_avg_epu16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpminsb_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_min_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpminsb_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_min_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpminsw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_min_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpminsw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_min_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpminub_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_min_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpminub_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_min_epu8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmaxsb_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_max_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmaxsb_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_max_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmaxsw_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_max_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmaxsw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_max_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmaxub_v128(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_max_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vl_vpmaxub_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_max_epu8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}


__attribute__((always_inline))
__attribute__((target("avx512vp2intersect")))
void rust_isa_avx512vl_vp2intersectd_v128(const uint8_t *a, const uint8_t *b, uint32_t *a_mask, uint32_t *b_mask) {
    __mmask8 first = 0;
    __mmask8 second = 0;
    _mm_2intersect_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        &first,
        &second);
    *a_mask = (uint32_t)first;
    *b_mask = (uint32_t)second;
}

__attribute__((always_inline))
__attribute__((target("avx512vp2intersect")))
void rust_isa_avx512vl_vp2intersectq_v128(const uint8_t *a, const uint8_t *b, uint32_t *a_mask, uint32_t *b_mask) {
    __mmask8 first = 0;
    __mmask8 second = 0;
    _mm_2intersect_epi64(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        &first,
        &second);
    *a_mask = (uint32_t)first;
    *b_mask = (uint32_t)second;
}

__attribute__((always_inline))
__attribute__((target("avx512vp2intersect")))
void rust_isa_avx512vl_vp2intersectd_v256(const uint8_t *a, const uint8_t *b, uint32_t *a_mask, uint32_t *b_mask) {
    __mmask8 first = 0;
    __mmask8 second = 0;
    _mm256_2intersect_epi32(
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b),
        &first,
        &second);
    *a_mask = (uint32_t)first;
    *b_mask = (uint32_t)second;
}

__attribute__((always_inline))
__attribute__((target("avx512vp2intersect")))
void rust_isa_avx512vl_vp2intersectq_v256(const uint8_t *a, const uint8_t *b, uint32_t *a_mask, uint32_t *b_mask) {
    __mmask8 first = 0;
    __mmask8 second = 0;
    _mm256_2intersect_epi64(
        _mm256_loadu_si256((const __m256i *)(const void *)a),
        _mm256_loadu_si256((const __m256i *)(const void *)b),
        &first,
        &second);
    *a_mask = (uint32_t)first;
    *b_mask = (uint32_t)second;
}
