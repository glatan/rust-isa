#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx10_1_vaddps_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_add_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vaddps_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_add_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vmulps_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_mul_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vmulps_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_mul_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpaddb_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpaddb_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpaddw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpaddw_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpaddd_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpaddd_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_epi32(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpaddq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpaddq_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_epi64(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpsubb_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpsubb_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpsubw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpsubw_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpsubd_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpsubd_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_epi32(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpsubq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpsubq_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_epi64(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmullw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mullo_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmullw_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mullo_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmulld_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mullo_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmulld_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mullo_epi32(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmullq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mullo_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmullq_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mullo_epi64(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmulhw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mulhi_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmulhw_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mulhi_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmulhuw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mulhi_epu16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmulhuw_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mulhi_epu16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmaddwd_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_madd_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmaddwd_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_madd_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmaddubsw_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_maddubs_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmaddubsw_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_maddubs_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpandd_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_and_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpandd_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_and_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpandq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_and_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpandq_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_and_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpandnd_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_andnot_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpandnd_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_andnot_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpandnq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_andnot_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpandnq_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_andnot_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpord_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_or_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpord_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_or_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vporq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_or_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vporq_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_or_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpxord_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_xor_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpxord_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_xor_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpxorq_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_xor_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpxorq_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_xor_si512(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint32_t rust_isa_avx10_1_vptestmb_v256(const uint8_t *a, const uint8_t *b) {
    return (uint32_t)(_mm256_test_epi8_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
uint64_t rust_isa_avx10_1_vptestmb_v512(const uint8_t *a, const uint8_t *b) {
    return (uint64_t)(_mm512_test_epi8_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint16_t rust_isa_avx10_1_vptestmw_v256(const uint8_t *a, const uint8_t *b) {
    return (uint16_t)(_mm256_test_epi16_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
uint32_t rust_isa_avx10_1_vptestmw_v512(const uint8_t *a, const uint8_t *b) {
    return (uint32_t)(_mm512_test_epi16_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx10_1_vptestmd_v256(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm256_test_epi32_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
uint16_t rust_isa_avx10_1_vptestmd_v512(const uint8_t *a, const uint8_t *b) {
    return (uint16_t)(_mm512_test_epi32_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx10_1_vptestmq_v256(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm256_test_epi64_mask(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
uint8_t rust_isa_avx10_1_vptestmq_v512(const uint8_t *a, const uint8_t *b) {
    return (uint8_t)(_mm512_test_epi64_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpermd_v256(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_permutexvar_epi32(_mm256_loadu_si256((const __m256i *)(const void *)idx), _mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpermd_v512(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_permutexvar_epi32(_mm512_loadu_si512((const void *)idx), _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpermq_v256(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_permutexvar_epi64(_mm256_loadu_si256((const __m256i *)(const void *)idx), _mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpermq_v512(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_permutexvar_epi64(_mm512_loadu_si512((const void *)idx), _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpermb_v256(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_permutexvar_epi8(_mm256_loadu_si256((const __m256i *)(const void *)idx), _mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpermb_v512(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_permutexvar_epi8(_mm512_loadu_si512((const void *)idx), _mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpermi2b_v256(const uint8_t *a, const uint8_t *b, const uint8_t *idx, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_permutex2var_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)idx), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpermi2b_v512(const uint8_t *a, const uint8_t *b, const uint8_t *idx, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_permutex2var_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)idx), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpermt2b_v256(const uint8_t *a, const uint8_t *b, const uint8_t *idx, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_permutex2var_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)idx), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpermt2b_v512(const uint8_t *a, const uint8_t *b, const uint8_t *idx, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_permutex2var_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)idx), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcompressps_v256(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_maskz_compress_ps((__mmask8)mask, _mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcompressps_v512(const uint8_t *a, uint16_t mask, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_maskz_compress_ps((__mmask16)mask, _mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcompresspd_v256(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_maskz_compress_pd((__mmask8)mask, _mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcompresspd_v512(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_maskz_compress_pd((__mmask8)mask, _mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vexpandps_v256(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_maskz_expand_ps((__mmask8)mask, _mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vexpandps_v512(const uint8_t *a, uint16_t mask, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_maskz_expand_ps((__mmask16)mask, _mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vexpandpd_v256(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_maskz_expand_pd((__mmask8)mask, _mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vexpandpd_v512(const uint8_t *a, uint8_t mask, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_maskz_expand_pd((__mmask8)mask, _mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcvtdq2ps_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_cvtepi32_ps(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcvtps2dq_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_cvtps_epi32(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcvttps2dq_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_cvttps_epi32(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vrcp14ps_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_rcp14_ps(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vrcp14pd_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_rcp14_pd(_mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vrsqrt14ps_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_rsqrt14_ps(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vrsqrt14pd_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_rsqrt14_pd(_mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vsqrtps_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_sqrt_ps(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vsqrtpd_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_sqrt_pd(_mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcvtdq2ps_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_cvtepi32_ps(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcvtps2dq_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvtps_epi32(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcvttps2dq_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_cvttps_epi32(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vrcp14ps_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_rcp14_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vrcp14pd_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_rcp14_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vrsqrt14ps_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_rsqrt14_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vrsqrt14pd_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_rsqrt14_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vsqrtps_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_sqrt_ps(_mm512_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vsqrtpd_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_sqrt_pd(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcvtps2pd_v256(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_cvtps_pd(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcvtpd2ps_v512(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm512_cvtpd_ps(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcvtpd2dq_v512(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm512_cvtpd_epi32(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vcvttpd2dq_v512(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm512_cvttpd_epi32(_mm512_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfmadd132ps_v256(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_fmadd_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b), _mm256_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfmadd132ps_v512(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_fmadd_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b), _mm512_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfmadd132pd_v256(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_fmadd_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b), _mm256_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfmadd132pd_v512(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_fmadd_pd(_mm512_loadu_pd((const double *)(const void *)a), _mm512_loadu_pd((const double *)(const void *)b), _mm512_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfmsub132ps_v256(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_fmsub_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b), _mm256_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfmsub132ps_v512(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_fmsub_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b), _mm512_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfmsub132pd_v256(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_fmsub_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b), _mm256_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfmsub132pd_v512(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_fmsub_pd(_mm512_loadu_pd((const double *)(const void *)a), _mm512_loadu_pd((const double *)(const void *)b), _mm512_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfnmadd132ps_v256(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_fnmadd_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b), _mm256_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfnmadd132ps_v512(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_fnmadd_ps(_mm512_loadu_ps((const float *)(const void *)a), _mm512_loadu_ps((const float *)(const void *)b), _mm512_loadu_ps((const float *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfnmadd132pd_v256(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_fnmadd_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b), _mm256_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vfnmadd132pd_v512(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm512_storeu_pd((double *)(void *)out, _mm512_fnmadd_pd(_mm512_loadu_pd((const double *)(const void *)a), _mm512_loadu_pd((const double *)(const void *)b), _mm512_loadu_pd((const double *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpdpbusd_v256(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpbusd_epi32(_mm256_loadu_si256((const __m256i *)(const void *)acc), _mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpdpbusd_v512(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_dpbusd_epi32(_mm512_loadu_si512((const void *)acc), _mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpdpwssd_v256(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_dpwssd_epi32(_mm256_loadu_si256((const __m256i *)(const void *)acc), _mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpdpwssd_v512(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_dpwssd_epi32(_mm512_loadu_si512((const void *)acc), _mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmadd52luq_v256(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_madd52lo_epu64(_mm256_loadu_si256((const __m256i *)(const void *)acc), _mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmadd52luq_v512(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_madd52lo_epu64(_mm512_loadu_si512((const void *)acc), _mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmadd52huq_v256(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_madd52hi_epu64(_mm256_loadu_si256((const __m256i *)(const void *)acc), _mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpmadd52huq_v512(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_madd52hi_epu64(_mm512_loadu_si512((const void *)acc), _mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpopcntb_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_popcnt_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpopcntb_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_popcnt_epi8(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpopcntw_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_popcnt_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpopcntw_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_popcnt_epi16(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpopcntd_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_popcnt_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpopcntd_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_popcnt_epi32(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpopcntq_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_popcnt_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpopcntq_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_popcnt_epi64(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vplzcntd_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_lzcnt_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vplzcntd_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_lzcnt_epi32(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vplzcntq_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_lzcnt_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vplzcntq_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_lzcnt_epi64(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpconflictd_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_conflict_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpconflictd_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_conflict_epi32(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpconflictq_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_conflict_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx10_1_vpconflictq_v512(const uint8_t *a, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_conflict_epi64(_mm512_loadu_si512((const void *)a)));
}
