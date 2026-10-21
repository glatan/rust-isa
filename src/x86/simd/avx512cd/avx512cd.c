#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512cd_vplzcntd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_lzcnt_epi32(
        _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512cd_vplzcntq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_lzcnt_epi64(
        _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512cd_vpconflictd(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_conflict_epi32(
        _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512cd_vpconflictq(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_conflict_epi64(
        _mm512_loadu_si512((const void *)a)));
}
