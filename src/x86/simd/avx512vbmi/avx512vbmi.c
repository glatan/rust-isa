#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512vbmi_vpermb(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_permutexvar_epi8(_mm512_loadu_si512((const void *)idx), _mm512_loadu_si512((const void *)a)));
}
