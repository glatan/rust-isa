#include <stdint.h>
#include <immintrin.h>

void rust_isa_sm3_vsm3rnds2_0(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out);
void rust_isa_sm3_vsm3rnds2_1(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out);
void rust_isa_sm3_vsm3rnds2_2(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out);
void rust_isa_sm3_vsm3rnds2_3(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out);


__attribute__((always_inline))
void rust_isa_sm3_vsm3msg1(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sm3msg1_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_sm3_vsm3msg2(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sm3msg2_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

#define RUST_ISA_SM3_VSM3RNDS2(N) \
__attribute__((always_inline)) \
void rust_isa_sm3_vsm3rnds2_##N(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) { \
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sm3rnds2_epi32( \
        _mm_loadu_si128((const __m128i *)(const void *)a), \
        _mm_loadu_si128((const __m128i *)(const void *)b), \
        _mm_loadu_si128((const __m128i *)(const void *)c), N)); \
}

RUST_ISA_SM3_VSM3RNDS2(0)
RUST_ISA_SM3_VSM3RNDS2(1)
RUST_ISA_SM3_VSM3RNDS2(2)
RUST_ISA_SM3_VSM3RNDS2(3)
