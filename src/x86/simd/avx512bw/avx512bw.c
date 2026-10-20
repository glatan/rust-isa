#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512bw_vpaddb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpsubb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmullw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mullo_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}
