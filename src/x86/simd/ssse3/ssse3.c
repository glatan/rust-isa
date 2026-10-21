#include <stdint.h>
#include <tmmintrin.h>

void rust_isa_ssse3_palignr_0(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_1(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_2(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_3(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_4(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_5(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_6(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_7(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_8(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_9(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_10(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_11(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_12(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_13(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_14(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_15(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_16(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_17(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_18(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_19(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_20(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_21(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_22(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_23(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_24(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_25(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_26(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_27(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_28(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_29(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_30(const uint8_t *a, const uint8_t *b, uint8_t *out);
void rust_isa_ssse3_palignr_31(const uint8_t *a, const uint8_t *b, uint8_t *out);


__attribute__((always_inline))
void rust_isa_ssse3_pabsb(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_abs_epi8(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_ssse3_pabsw(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_abs_epi16(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_ssse3_pabsd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_abs_epi32(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_ssse3_pshufb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_shuffle_epi8(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_ssse3_phaddw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_hadd_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_ssse3_phaddd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_hadd_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_ssse3_phaddsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_hadds_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_ssse3_phsubw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_hsub_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_ssse3_phsubd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_hsub_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_ssse3_phsubsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_hsubs_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_ssse3_pmaddubsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maddubs_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_ssse3_pmulhrsw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_mulhrs_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_ssse3_psignb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sign_epi8(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_ssse3_psignw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sign_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_ssse3_psignd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sign_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

#define RUST_ISA_SSSE3_PALIGNR(N) \
__attribute__((always_inline)) \
void rust_isa_ssse3_palignr_##N(const uint8_t *a, const uint8_t *b, uint8_t *out) { \
    _mm_storeu_si128((__m128i *)(void *)out, _mm_alignr_epi8( \
        _mm_loadu_si128((const __m128i *)(const void *)a), \
        _mm_loadu_si128((const __m128i *)(const void *)b), N)); \
}

RUST_ISA_SSSE3_PALIGNR(0)
RUST_ISA_SSSE3_PALIGNR(1)
RUST_ISA_SSSE3_PALIGNR(2)
RUST_ISA_SSSE3_PALIGNR(3)
RUST_ISA_SSSE3_PALIGNR(4)
RUST_ISA_SSSE3_PALIGNR(5)
RUST_ISA_SSSE3_PALIGNR(6)
RUST_ISA_SSSE3_PALIGNR(7)
RUST_ISA_SSSE3_PALIGNR(8)
RUST_ISA_SSSE3_PALIGNR(9)
RUST_ISA_SSSE3_PALIGNR(10)
RUST_ISA_SSSE3_PALIGNR(11)
RUST_ISA_SSSE3_PALIGNR(12)
RUST_ISA_SSSE3_PALIGNR(13)
RUST_ISA_SSSE3_PALIGNR(14)
RUST_ISA_SSSE3_PALIGNR(15)
RUST_ISA_SSSE3_PALIGNR(16)
RUST_ISA_SSSE3_PALIGNR(17)
RUST_ISA_SSSE3_PALIGNR(18)
RUST_ISA_SSSE3_PALIGNR(19)
RUST_ISA_SSSE3_PALIGNR(20)
RUST_ISA_SSSE3_PALIGNR(21)
RUST_ISA_SSSE3_PALIGNR(22)
RUST_ISA_SSSE3_PALIGNR(23)
RUST_ISA_SSSE3_PALIGNR(24)
RUST_ISA_SSSE3_PALIGNR(25)
RUST_ISA_SSSE3_PALIGNR(26)
RUST_ISA_SSSE3_PALIGNR(27)
RUST_ISA_SSSE3_PALIGNR(28)
RUST_ISA_SSSE3_PALIGNR(29)
RUST_ISA_SSSE3_PALIGNR(30)
RUST_ISA_SSSE3_PALIGNR(31)
