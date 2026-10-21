#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512vnni_vpdpbusd(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_dpbusd_epi32(_mm512_loadu_si512((const void *)acc), _mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vnni_vpdpbusds(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_dpbusds_epi32(_mm512_loadu_si512((const void *)acc), _mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vnni_vpdpwssd(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_dpwssd_epi32(_mm512_loadu_si512((const void *)acc), _mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512vnni_vpdpwssds(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_dpwssds_epi32(_mm512_loadu_si512((const void *)acc), _mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}
