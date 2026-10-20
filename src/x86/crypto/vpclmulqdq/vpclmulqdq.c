#include <stdint.h>
#include <immintrin.h>

typedef struct { __m256i v; } rust_isa_v256;
typedef struct { __m512i v; } rust_isa_v512;

__attribute__((always_inline))
static __m256i clmul256(__m256i a, __m256i b, uint8_t imm) {
    switch (imm & 0x11) {
    case 0x00: return _mm256_clmulepi64_epi128(a, b, 0x00);
    case 0x01: return _mm256_clmulepi64_epi128(a, b, 0x01);
    case 0x10: return _mm256_clmulepi64_epi128(a, b, 0x10);
    default: return _mm256_clmulepi64_epi128(a, b, 0x11);
    }
}

__attribute__((always_inline))
void rust_isa_vpclmulqdq_vpclmulqdq_v256(const uint8_t *a, const uint8_t *b, uint8_t imm, uint8_t *out) {
    __m256i x = _mm256_loadu_si256((const __m256i *)(const void *)a);
    __m256i y = _mm256_loadu_si256((const __m256i *)(const void *)b);
    _mm256_storeu_si256((__m256i *)(void *)out, clmul256(x, y, imm));
}

__attribute__((always_inline))
static __m512i clmul512(__m512i a, __m512i b, uint8_t imm) {
    switch (imm & 0x11) {
    case 0x00: return _mm512_clmulepi64_epi128(a, b, 0x00);
    case 0x01: return _mm512_clmulepi64_epi128(a, b, 0x01);
    case 0x10: return _mm512_clmulepi64_epi128(a, b, 0x10);
    default: return _mm512_clmulepi64_epi128(a, b, 0x11);
    }
}

__attribute__((always_inline))
void rust_isa_vpclmulqdq_vpclmulqdq_v512(const uint8_t *a, const uint8_t *b, uint8_t imm, uint8_t *out) {
    __m512i x = _mm512_loadu_si512((const void *)a);
    __m512i y = _mm512_loadu_si512((const void *)b);
    _mm512_storeu_si512((void *)out, clmul512(x, y, imm));
}
