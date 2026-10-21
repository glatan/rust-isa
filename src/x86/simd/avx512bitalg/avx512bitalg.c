#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512bitalg_vpopcntb(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_popcnt_epi8(
        _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bitalg_vpopcntw(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_popcnt_epi16(
        _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
uint64_t rust_isa_avx512bitalg_vpshufbitqmb(const uint8_t *a, const uint8_t *idx) {
    return (uint64_t)_mm512_bitshuffle_epi64_mask(
        _mm512_loadu_si512((const void *)a),
        _mm512_loadu_si512((const void *)idx));
}
