#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx512bw_vpaddb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpsubb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpaddw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_add_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpsubw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sub_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpaddsb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_adds_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpaddsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_adds_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpaddusb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_adds_epu8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpaddusw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_adds_epu16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpsubsb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_subs_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpsubsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_subs_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpsubusb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_subs_epu8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpsubusw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_subs_epu16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpavgb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_avg_epu8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpavgw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_avg_epu16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpminub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_min_epu8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpminsb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_min_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpminsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_min_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmaxub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_max_epu8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmaxsb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_max_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmaxsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_max_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmulhw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mulhi_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmulhuw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mulhi_epu16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmullw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_mullo_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmaddwd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_madd_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmaddubsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_maddubs_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpacksswb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_packs_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpackssdw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_packs_epi32(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpackuswb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_packus_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpackusdw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_packus_epi32(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpunpcklbw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_unpacklo_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpunpckhbw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_unpackhi_epi8(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpunpcklwd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_unpacklo_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpunpckhwd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_unpackhi_epi16(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpsllw(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sll_epi16(_mm512_loadu_si512((const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpsrlw(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_srl_epi16(_mm512_loadu_si512((const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpsraw(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_sra_epi16(_mm512_loadu_si512((const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovwb(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm512_cvtepi16_epi8(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovswb(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm512_cvtsepi16_epi8(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovuswb(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm512_cvtusepi16_epi8(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovdb(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm512_cvtepi32_epi8(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovsdb(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm512_cvtsepi32_epi8(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovusdb(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm512_cvtusepi32_epi8(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovdw(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm512_cvtepi32_epi16(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovsdw(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm512_cvtsepi32_epi16(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovusdw(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm512_cvtusepi32_epi16(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovqw(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm512_cvtepi64_epi16(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovsqw(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm512_cvtsepi64_epi16(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovusqw(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm512_cvtusepi64_epi16(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovqb(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm512_cvtepi64_epi8(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovsqb(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm512_cvtsepi64_epi8(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovusqb(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm512_cvtusepi64_epi8(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
uint64_t rust_isa_avx512bw_vptestmb(const uint8_t *a, const uint8_t *b) {
    return (uint64_t)(_mm512_test_epi8_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint32_t rust_isa_avx512bw_vptestmw(const uint8_t *a, const uint8_t *b) {
    return (uint32_t)(_mm512_test_epi16_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint64_t rust_isa_avx512bw_vptestnmb(const uint8_t *a, const uint8_t *b) {
    return (uint64_t)(_mm512_testn_epi8_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
uint32_t rust_isa_avx512bw_vptestnmw(const uint8_t *a, const uint8_t *b) {
    return (uint32_t)(_mm512_testn_epi16_mask(_mm512_loadu_si512((const void *)a), _mm512_loadu_si512((const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovm2b(uint64_t mask, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_movm_epi8((__mmask64)mask));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpmovm2w(uint32_t mask, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_movm_epi16((__mmask32)mask));
}

__attribute__((always_inline))
uint64_t rust_isa_avx512bw_vpmovb2m(const uint8_t *a) {
    return (uint64_t)(_mm512_movepi8_mask(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
uint32_t rust_isa_avx512bw_vpmovw2m(const uint8_t *a) {
    return (uint32_t)(_mm512_movepi16_mask(_mm512_loadu_si512((const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpbroadcastb(uint8_t value, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_set1_epi8((char)value));
}

__attribute__((always_inline))
void rust_isa_avx512bw_vpbroadcastw(uint16_t value, uint8_t *out) {
    _mm512_storeu_si512((void *)out, _mm512_set1_epi16((short)value));
}
