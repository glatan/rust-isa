#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512ifma_vpmadd52luq(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_madd52lo_epu64(_mm512_loadu_si512((const void *)acc), _mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512ifma_vpmadd52huq(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_madd52hi_epu64(_mm512_loadu_si512((const void *)acc), _mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}
