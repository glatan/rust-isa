#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512f_v512_load(const uint8_t *p, uint8_t out[64]) {
    _mm512_storeu_si512((void *)out, _mm512_loadu_si512((const void *)p));
}

__attribute__((always_inline))
void rust_isa_avx512f_v512_store(uint8_t *p, const uint8_t value[64]) {
    _mm512_storeu_si512((void *)p, _mm512_loadu_si512((const void *)value));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpaddd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_epi32(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpsubd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_epi32(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpmulld(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mullo_epi32(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpaddq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_epi64(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpsubq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_epi64(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpandd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_and_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpord(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_or_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpxord(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_xor_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpermd(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_permutexvar_epi32(_mm512_loadu_si512((const void *)idx), _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpbroadcastd(uint32_t value, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_set1_epi32((int)value));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpslld(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sll_epi32(_mm512_loadu_si512((const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_avx512f_vpsrld(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_srl_epi32(_mm512_loadu_si512((const void *)a), _mm_cvtsi32_si128((int)count)));
}
