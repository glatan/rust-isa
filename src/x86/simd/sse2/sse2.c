#include <stdint.h>
#include <emmintrin.h>

typedef int rust_isa_v4i32 __attribute__((vector_size(16), aligned(16)));

__attribute__((always_inline))
void rust_isa_sse2_v128_load(const uint8_t *p, uint8_t out[16]) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_loadu_si128((const __m128i *)(const void *)p));
}

__attribute__((always_inline))
void rust_isa_sse2_v128_store(uint8_t *p, const uint8_t value[16]) {
    _mm_storeu_si128((__m128i *)(void *)p, _mm_loadu_si128((const __m128i *)(const void *)value));
}

__attribute__((always_inline))
void rust_isa_sse2_paddd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    rust_isa_v4i32 x = *(const rust_isa_v4i32 *)(const void *)a;
    rust_isa_v4i32 y = *(const rust_isa_v4i32 *)(const void *)b;
    *(rust_isa_v4i32 *)(void *)out = x + y;
}

__attribute__((always_inline))
void rust_isa_sse2_paddb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_add_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_psubb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sub_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pcmpeqb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmpeq_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_paddw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_add_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_psubw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sub_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pcmpeqw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmpeq_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pmullw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mullo_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_psubd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sub_epi32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pcmpeqd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmpeq_epi32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_paddq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_add_epi64(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_psubq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sub_epi64(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pand(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_and_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_por(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_or_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pxor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_xor_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pcmpgtb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmpgt_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pminub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_min_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pmaxub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_max_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pavgb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_avg_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pcmpgtw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmpgt_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pmaddwd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_madd_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pminw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_min_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pmaxw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_max_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pavgw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_avg_epu16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pcmpgtd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmpgt_epi32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pmuludq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mul_epu32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_psadbw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sad_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_addps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_add_ps(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_subps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sub_ps(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_mulps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mul_ps(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_divps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_div_ps(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_minps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_min_ps(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_maxps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_max_ps(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_cmpeqps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmpeq_ps(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_cmpltps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmplt_ps(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_cmpleps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmple_ps(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_addpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_add_pd(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_subpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sub_pd(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_mulpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mul_pd(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_divpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_div_pd(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_minpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_min_pd(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_maxpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_max_pd(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_cmpeqpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmpeq_pd(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_cmpltpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmplt_pd(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_cmplepd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmple_pd(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_sqrtps(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sqrt_ps(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse2_sqrtpd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sqrt_pd(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse2_punpcklbw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_unpacklo_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_punpckhbw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_unpackhi_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_punpcklwd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_unpacklo_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_punpckhwd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_unpackhi_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_punpckldq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_unpacklo_epi32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_punpckhdq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_unpackhi_epi32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_punpcklqdq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_punpckhqdq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_psllw(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sll_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_sse2_pslld(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sll_epi32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_sse2_psllq(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sll_epi64(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_sse2_psrlw(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_srl_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_sse2_psrld(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_srl_epi32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_sse2_psrlq(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_srl_epi64(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_sse2_psraw(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sra_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_sse2_psrad(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sra_epi32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_cvtsi32_si128((int)count)));
}
