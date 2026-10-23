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
void rust_isa_sse2_pminsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_min_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pmaxsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
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

__attribute__((always_inline))
void rust_isa_sse2_paddsb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_adds_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_paddsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_adds_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_paddusb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_adds_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_paddusw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_adds_epu16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_psubsb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_subs_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_psubsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_subs_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_psubusb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_subs_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_psubusw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_subs_epu16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_packsswb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_packs_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_packssdw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_packs_epi32(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_packuswb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_packus_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pmulhw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mulhi_epi16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_pmulhuw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mulhi_epu16(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
int rust_isa_sse2_pmovmskb(const uint8_t *a) {
    return _mm_movemask_epi8(_mm_loadu_si128((const __m128i *)(const void *)a));
}

__attribute__((always_inline))
int rust_isa_sse2_movmskps(const uint8_t *a) {
    return _mm_movemask_ps(_mm_loadu_ps((const float *)(const void *)a));
}

__attribute__((always_inline))
int rust_isa_sse2_movmskpd(const uint8_t *a) {
    return _mm_movemask_pd(_mm_loadu_pd((const double *)(const void *)a));
}

__attribute__((always_inline))
void rust_isa_sse2_andps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_and_ps(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_andnps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_andnot_ps(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_orps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_or_ps(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_xorps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_xor_ps(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_unpcklps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_unpacklo_ps(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_unpckhps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_unpackhi_ps(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_unpcklpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_unpacklo_pd(_mm_loadu_pd((const double *)(const void *)a), _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_unpckhpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_unpackhi_pd(_mm_loadu_pd((const double *)(const void *)a), _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse2_cvtsi2ss(int32_t value, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_cvtsi32_ss(_mm_setzero_ps(), value));
}

__attribute__((always_inline))
void rust_isa_sse2_cvtsi2sd(int32_t value, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_cvtsi32_sd(_mm_setzero_pd(), value));
}

__attribute__((always_inline))
int32_t rust_isa_sse2_cvtss2si(const uint8_t *a) {
    return _mm_cvtss_si32(_mm_loadu_ps((const float *)(const void *)a));
}

__attribute__((always_inline))
int32_t rust_isa_sse2_cvtsd2si(const uint8_t *a) {
    return _mm_cvtsd_si32(_mm_loadu_pd((const double *)(const void *)a));
}

__attribute__((always_inline))
int32_t rust_isa_sse2_cvttss2si(const uint8_t *a) {
    return _mm_cvttss_si32(_mm_loadu_ps((const float *)(const void *)a));
}

__attribute__((always_inline))
int32_t rust_isa_sse2_cvttsd2si(const uint8_t *a) {
    return _mm_cvttsd_si32(_mm_loadu_pd((const double *)(const void *)a));
}

__attribute__((always_inline))
void rust_isa_sse2_cvtps2pd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_cvtps_pd(_mm_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse2_cvtpd2ps(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_cvtpd_ps(_mm_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse2_cvtdq2ps(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, (__m128i)_mm_cvtepi32_ps(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse2_cvtps2dq(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cvtps_epi32(_mm_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse2_cvttps2dq(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cvttps_epi32(_mm_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse2_maskmovdqu(const uint8_t *a, const uint8_t *mask, uint8_t *p) {
    _mm_maskmoveu_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)mask), (char *)(void *)p);
}

#define RUST_ISA_SSE2_PSLLDQ(N) \
__attribute__((always_inline)) \
void rust_isa_sse2_pslldq_##N(const uint8_t *a, uint8_t *out) { \
    _mm_storeu_si128((__m128i *)(void *)out, _mm_slli_si128(_mm_loadu_si128((const __m128i *)(const void *)a), N)); \
}

#define RUST_ISA_SSE2_PSRLDQ(N) \
__attribute__((always_inline)) \
void rust_isa_sse2_psrldq_##N(const uint8_t *a, uint8_t *out) { \
    _mm_storeu_si128((__m128i *)(void *)out, _mm_srli_si128(_mm_loadu_si128((const __m128i *)(const void *)a), N)); \
}

RUST_ISA_SSE2_PSLLDQ(0)
RUST_ISA_SSE2_PSLLDQ(1)
RUST_ISA_SSE2_PSLLDQ(2)
RUST_ISA_SSE2_PSLLDQ(3)
RUST_ISA_SSE2_PSLLDQ(4)
RUST_ISA_SSE2_PSLLDQ(5)
RUST_ISA_SSE2_PSLLDQ(6)
RUST_ISA_SSE2_PSLLDQ(7)
RUST_ISA_SSE2_PSLLDQ(8)
RUST_ISA_SSE2_PSLLDQ(9)
RUST_ISA_SSE2_PSLLDQ(10)
RUST_ISA_SSE2_PSLLDQ(11)
RUST_ISA_SSE2_PSLLDQ(12)
RUST_ISA_SSE2_PSLLDQ(13)
RUST_ISA_SSE2_PSLLDQ(14)
RUST_ISA_SSE2_PSLLDQ(15)
RUST_ISA_SSE2_PSRLDQ(0)
RUST_ISA_SSE2_PSRLDQ(1)
RUST_ISA_SSE2_PSRLDQ(2)
RUST_ISA_SSE2_PSRLDQ(3)
RUST_ISA_SSE2_PSRLDQ(4)
RUST_ISA_SSE2_PSRLDQ(5)
RUST_ISA_SSE2_PSRLDQ(6)
RUST_ISA_SSE2_PSRLDQ(7)
RUST_ISA_SSE2_PSRLDQ(8)
RUST_ISA_SSE2_PSRLDQ(9)
RUST_ISA_SSE2_PSRLDQ(10)
RUST_ISA_SSE2_PSRLDQ(11)
RUST_ISA_SSE2_PSRLDQ(12)
RUST_ISA_SSE2_PSRLDQ(13)
RUST_ISA_SSE2_PSRLDQ(14)
RUST_ISA_SSE2_PSRLDQ(15)

__attribute__((always_inline))
uint32_t rust_isa_sse2_movd(const uint8_t *a) {
    return (uint32_t)_mm_cvtsi128_si32(_mm_loadu_si128((const __m128i *)(const void *)a));
}

__attribute__((always_inline))
uint64_t rust_isa_sse2_movq(const uint8_t *a) {
    return (uint64_t)_mm_cvtsi128_si64(_mm_loadu_si128((const __m128i *)(const void *)a));
}

__attribute__((always_inline))
void rust_isa_sse2_pandn(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_andnot_si128(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
int32_t rust_isa_sse2_comiss(const uint8_t *a, const uint8_t *b) {
    const __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    const __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    if (_mm_comilt_ss(va, vb)) return -1;
    if (_mm_comigt_ss(va, vb)) return 1;
    if (_mm_comieq_ss(va, vb)) return 0;
    return 2;
}

__attribute__((always_inline))
int32_t rust_isa_sse2_comisd(const uint8_t *a, const uint8_t *b) {
    const __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    const __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    if (_mm_comilt_sd(va, vb)) return -1;
    if (_mm_comigt_sd(va, vb)) return 1;
    if (_mm_comieq_sd(va, vb)) return 0;
    return 2;
}

__attribute__((always_inline))
int32_t rust_isa_sse2_ucomiss(const uint8_t *a, const uint8_t *b) {
    const __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    const __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    if (_mm_ucomilt_ss(va, vb)) return -1;
    if (_mm_ucomigt_ss(va, vb)) return 1;
    if (_mm_ucomieq_ss(va, vb)) return 0;
    return 2;
}

__attribute__((always_inline))
int32_t rust_isa_sse2_ucomisd(const uint8_t *a, const uint8_t *b) {
    const __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    const __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    if (_mm_ucomilt_sd(va, vb)) return -1;
    if (_mm_ucomigt_sd(va, vb)) return 1;
    if (_mm_ucomieq_sd(va, vb)) return 0;
    return 2;
}

__attribute__((always_inline))
void rust_isa_sse2_movntps(float *p, const uint8_t *a) {
    _mm_stream_ps(p, _mm_loadu_ps((const float *)(const void *)a));
}

__attribute__((always_inline))
void rust_isa_sse2_movntpd(double *p, const uint8_t *a) {
    _mm_stream_pd(p, _mm_loadu_pd((const double *)(const void *)a));
}

__attribute__((always_inline))
void rust_isa_sse2_movntdq(uint8_t *p, const uint8_t *a) {
    _mm_stream_si128((__m128i *)(void *)p, _mm_loadu_si128((const __m128i *)(const void *)a));
}

__attribute__((always_inline))
void rust_isa_sse2_movnti(int32_t *p, int32_t value) {
    _mm_stream_si32(p, value);
}

#if defined(__x86_64__)
__attribute__((always_inline))
void rust_isa_sse2_movntq(uint8_t *p, const uint8_t *a) {
    _mm_stream_si64((long long *)(void *)p, _mm_cvtsi128_si64(_mm_loadu_si128((const __m128i *)(const void *)a)));
}
#endif

__attribute__((always_inline))
void rust_isa_sse2_cvtdq2pd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_cvtepi32_pd(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse2_cvtpd2dq(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cvtpd_epi32(_mm_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse2_cvttpd2dq(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cvttpd_epi32(_mm_loadu_pd((const double *)(const void *)a)));
}
