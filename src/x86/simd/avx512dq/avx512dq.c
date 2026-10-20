#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512dq_vpmullq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mullo_epi64(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}
