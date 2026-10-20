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
