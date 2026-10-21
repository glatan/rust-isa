#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512vp2intersect_vp2intersectd(const uint8_t *a, const uint8_t *b, uint32_t *a_mask, uint32_t *b_mask) {
    __mmask16 first = 0;
    __mmask16 second = 0;
    _mm512_2intersect_epi32(
        _mm512_loadu_si512((const void *)a),
        _mm512_loadu_si512((const void *)b),
        &first,
        &second);
    *a_mask = (uint32_t)first;
    *b_mask = (uint32_t)second;
}

__attribute__((always_inline))
void rust_isa_avx512vp2intersect_vp2intersectq(const uint8_t *a, const uint8_t *b, uint32_t *a_mask, uint32_t *b_mask) {
    __mmask8 first = 0;
    __mmask8 second = 0;
    _mm512_2intersect_epi64(
        _mm512_loadu_si512((const void *)a),
        _mm512_loadu_si512((const void *)b),
        &first,
        &second);
    *a_mask = (uint32_t)first;
    *b_mask = (uint32_t)second;
}
