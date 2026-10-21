#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512vbmi2_vpcompressb(const uint8_t *a, uint64_t mask, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_maskz_compress_epi8((__mmask64)mask, _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vbmi2_vpexpandb(const uint8_t *a, uint64_t mask, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_maskz_expand_epi8((__mmask64)mask, _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vbmi2_vpcompressw(const uint8_t *a, uint32_t mask, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_maskz_compress_epi16((__mmask32)mask, _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vbmi2_vpexpandw(const uint8_t *a, uint32_t mask, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_maskz_expand_epi16((__mmask32)mask, _mm512_loadu_si512((const void *)a)));
}
