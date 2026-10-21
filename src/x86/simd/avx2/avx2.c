#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx2_v256_load(const uint8_t *p, uint8_t out[32]) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_loadu_si256((const __m256i *)(const void *)p));
}

__attribute__((always_inline))
void rust_isa_avx2_v256_store(uint8_t *p, const uint8_t value[32]) {
    _mm256_storeu_si256((__m256i *)(void *)p, _mm256_loadu_si256((const __m256i *)(const void *)value));
}

__attribute__((always_inline))
void rust_isa_avx2_vpaddb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpsubb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpaddw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpsubw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpaddd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpsubd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpaddq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_add_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpsubq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sub_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpmulld(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mullo_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpmullw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mullo_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpcmpeqb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_cmpeq_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpcmpeqw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_cmpeq_epi16(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpcmpeqd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_cmpeq_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpand(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_and_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_or_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpxor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_xor_si256(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpshufb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_shuffle_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpermd(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_permutevar8x32_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)idx)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpbroadcastd(uint32_t value, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_set1_epi32((int)value));
}

__attribute__((always_inline))
void rust_isa_avx2_vpslld(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sll_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpsrld(const uint8_t *a, uint32_t count, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_srl_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm_cvtsi32_si128((int)count)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpsllvd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sllv_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpsllvq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_sllv_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpsrlvd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_srlv_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpsrlvq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_srlv_epi64(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpsravd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_srav_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpblendvb(const uint8_t *a, const uint8_t *b, const uint8_t *mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_blendv_epi8(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)b), _mm256_loadu_si256((const __m256i *)(const void *)mask)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpbroadcastb(uint8_t value, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_set1_epi8((char)value));
}

__attribute__((always_inline))
void rust_isa_avx2_vpbroadcastw(uint16_t value, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_set1_epi16((short)value));
}

__attribute__((always_inline))
void rust_isa_avx2_vpbroadcastq(uint64_t value, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_set1_epi64x((long long)value));
}

__attribute__((always_inline))
void rust_isa_avx2_vpgatherdd(const uint8_t *src, const uint8_t *base, const uint8_t *index, const uint8_t *mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mask_i32gather_epi32(_mm256_loadu_si256((const __m256i *)(const void *)src), (const int *)(const void *)base, _mm256_loadu_si256((const __m256i *)(const void *)index), _mm256_loadu_si256((const __m256i *)(const void *)mask), 1));
}

__attribute__((always_inline))
void rust_isa_avx2_vpgatherqd(const uint8_t *src, const uint8_t *base, const uint8_t *index, const uint8_t *mask, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm256_mask_i64gather_epi32(_mm_loadu_si128((const __m128i *)(const void *)src), (const int *)(const void *)base, _mm256_loadu_si256((const __m256i *)(const void *)index), _mm_loadu_si128((const __m128i *)(const void *)mask), 1));
}

__attribute__((always_inline))
void rust_isa_avx2_vpgatherdq(const uint8_t *src, const uint8_t *base, const uint8_t *index, const uint8_t *mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mask_i32gather_epi64(_mm256_loadu_si256((const __m256i *)(const void *)src), (const long long *)(const void *)base, _mm_loadu_si128((const __m128i *)(const void *)index), _mm256_loadu_si256((const __m256i *)(const void *)mask), 1));
}

__attribute__((always_inline))
void rust_isa_avx2_vpgatherqq(const uint8_t *src, const uint8_t *base, const uint8_t *index, const uint8_t *mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_mask_i64gather_epi64(_mm256_loadu_si256((const __m256i *)(const void *)src), (const long long *)(const void *)base, _mm256_loadu_si256((const __m256i *)(const void *)index), _mm256_loadu_si256((const __m256i *)(const void *)mask), 1));
}

__attribute__((always_inline))
void rust_isa_avx2_vpmaskmovd_load(const uint8_t *p, const uint8_t *mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_maskload_epi32((const int *)(const void *)p, _mm256_loadu_si256((const __m256i *)(const void *)mask)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpmaskmovd_store(uint8_t *p, const uint8_t *mask, const uint8_t *value) {
    _mm256_maskstore_epi32((int *)(void *)p, _mm256_loadu_si256((const __m256i *)(const void *)mask), _mm256_loadu_si256((const __m256i *)(const void *)value));
}

__attribute__((always_inline))
void rust_isa_avx2_vpmaskmovq_load(const uint8_t *p, const uint8_t *mask, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_maskload_epi64((const long long *)(const void *)p, _mm256_loadu_si256((const __m256i *)(const void *)mask)));
}

__attribute__((always_inline))
void rust_isa_avx2_vpmaskmovq_store(uint8_t *p, const uint8_t *mask, const uint8_t *value) {
    _mm256_maskstore_epi64((long long *)(void *)p, _mm256_loadu_si256((const __m256i *)(const void *)mask), _mm256_loadu_si256((const __m256i *)(const void *)value));
}

__attribute__((always_inline))
void rust_isa_avx2_vpermps(const uint8_t *a, const uint8_t *idx, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_permutevar8x32_epi32(_mm256_loadu_si256((const __m256i *)(const void *)a), _mm256_loadu_si256((const __m256i *)(const void *)idx)));
}
