#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512vpopcntdq_vpopcntd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_popcnt_epi32(
        _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512vpopcntdq_vpopcntq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_popcnt_epi64(
        _mm512_loadu_si512((const void *)a)));
}
