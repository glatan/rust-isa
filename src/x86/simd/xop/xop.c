#include <stdint.h>
#include <x86intrin.h>
#include <xopintrin.h>

__attribute__((always_inline))
void rust_isa_xop_vpperm(const uint8_t *a, const uint8_t *b, const uint8_t *control, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_perm_epi8(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)control)));
}

__attribute__((always_inline))
void rust_isa_xop_vpcmov(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmov_si128(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpcomb(const uint8_t *a, const uint8_t *b, uint32_t imm, uint8_t *out) {
    const __m128i va = _mm_loadu_si128((const __m128i *)(const void *)a);
    const __m128i vb = _mm_loadu_si128((const __m128i *)(const void *)b);
    __m128i r = _mm_setzero_si128();
    switch (imm) {
    case 0: r = _mm_com_epi8(va, vb, 0); break;
    case 1: r = _mm_com_epi8(va, vb, 1); break;
    case 2: r = _mm_com_epi8(va, vb, 2); break;
    case 3: r = _mm_com_epi8(va, vb, 3); break;
    case 4: r = _mm_com_epi8(va, vb, 4); break;
    case 5: r = _mm_com_epi8(va, vb, 5); break;
    case 6: r = _mm_com_epi8(va, vb, 6); break;
    case 7: r = _mm_com_epi8(va, vb, 7); break;
        default: break;
    }
    _mm_storeu_si128((__m128i *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_xop_vpcomw(const uint8_t *a, const uint8_t *b, uint32_t imm, uint8_t *out) {
    const __m128i va = _mm_loadu_si128((const __m128i *)(const void *)a);
    const __m128i vb = _mm_loadu_si128((const __m128i *)(const void *)b);
    __m128i r = _mm_setzero_si128();
    switch (imm) {
    case 0: r = _mm_com_epi16(va, vb, 0); break;
    case 1: r = _mm_com_epi16(va, vb, 1); break;
    case 2: r = _mm_com_epi16(va, vb, 2); break;
    case 3: r = _mm_com_epi16(va, vb, 3); break;
    case 4: r = _mm_com_epi16(va, vb, 4); break;
    case 5: r = _mm_com_epi16(va, vb, 5); break;
    case 6: r = _mm_com_epi16(va, vb, 6); break;
    case 7: r = _mm_com_epi16(va, vb, 7); break;
        default: break;
    }
    _mm_storeu_si128((__m128i *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_xop_vpcomd(const uint8_t *a, const uint8_t *b, uint32_t imm, uint8_t *out) {
    const __m128i va = _mm_loadu_si128((const __m128i *)(const void *)a);
    const __m128i vb = _mm_loadu_si128((const __m128i *)(const void *)b);
    __m128i r = _mm_setzero_si128();
    switch (imm) {
    case 0: r = _mm_com_epi32(va, vb, 0); break;
    case 1: r = _mm_com_epi32(va, vb, 1); break;
    case 2: r = _mm_com_epi32(va, vb, 2); break;
    case 3: r = _mm_com_epi32(va, vb, 3); break;
    case 4: r = _mm_com_epi32(va, vb, 4); break;
    case 5: r = _mm_com_epi32(va, vb, 5); break;
    case 6: r = _mm_com_epi32(va, vb, 6); break;
    case 7: r = _mm_com_epi32(va, vb, 7); break;
        default: break;
    }
    _mm_storeu_si128((__m128i *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_xop_vpcomq(const uint8_t *a, const uint8_t *b, uint32_t imm, uint8_t *out) {
    const __m128i va = _mm_loadu_si128((const __m128i *)(const void *)a);
    const __m128i vb = _mm_loadu_si128((const __m128i *)(const void *)b);
    __m128i r = _mm_setzero_si128();
    switch (imm) {
    case 0: r = _mm_com_epi64(va, vb, 0); break;
    case 1: r = _mm_com_epi64(va, vb, 1); break;
    case 2: r = _mm_com_epi64(va, vb, 2); break;
    case 3: r = _mm_com_epi64(va, vb, 3); break;
    case 4: r = _mm_com_epi64(va, vb, 4); break;
    case 5: r = _mm_com_epi64(va, vb, 5); break;
    case 6: r = _mm_com_epi64(va, vb, 6); break;
    case 7: r = _mm_com_epi64(va, vb, 7); break;
        default: break;
    }
    _mm_storeu_si128((__m128i *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_xop_vpcomub(const uint8_t *a, const uint8_t *b, uint32_t imm, uint8_t *out) {
    const __m128i va = _mm_loadu_si128((const __m128i *)(const void *)a);
    const __m128i vb = _mm_loadu_si128((const __m128i *)(const void *)b);
    __m128i r = _mm_setzero_si128();
    switch (imm) {
    case 0: r = _mm_com_epu8(va, vb, 0); break;
    case 1: r = _mm_com_epu8(va, vb, 1); break;
    case 2: r = _mm_com_epu8(va, vb, 2); break;
    case 3: r = _mm_com_epu8(va, vb, 3); break;
    case 4: r = _mm_com_epu8(va, vb, 4); break;
    case 5: r = _mm_com_epu8(va, vb, 5); break;
    case 6: r = _mm_com_epu8(va, vb, 6); break;
    case 7: r = _mm_com_epu8(va, vb, 7); break;
        default: break;
    }
    _mm_storeu_si128((__m128i *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_xop_vpcomuw(const uint8_t *a, const uint8_t *b, uint32_t imm, uint8_t *out) {
    const __m128i va = _mm_loadu_si128((const __m128i *)(const void *)a);
    const __m128i vb = _mm_loadu_si128((const __m128i *)(const void *)b);
    __m128i r = _mm_setzero_si128();
    switch (imm) {
    case 0: r = _mm_com_epu16(va, vb, 0); break;
    case 1: r = _mm_com_epu16(va, vb, 1); break;
    case 2: r = _mm_com_epu16(va, vb, 2); break;
    case 3: r = _mm_com_epu16(va, vb, 3); break;
    case 4: r = _mm_com_epu16(va, vb, 4); break;
    case 5: r = _mm_com_epu16(va, vb, 5); break;
    case 6: r = _mm_com_epu16(va, vb, 6); break;
    case 7: r = _mm_com_epu16(va, vb, 7); break;
        default: break;
    }
    _mm_storeu_si128((__m128i *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_xop_vpcomud(const uint8_t *a, const uint8_t *b, uint32_t imm, uint8_t *out) {
    const __m128i va = _mm_loadu_si128((const __m128i *)(const void *)a);
    const __m128i vb = _mm_loadu_si128((const __m128i *)(const void *)b);
    __m128i r = _mm_setzero_si128();
    switch (imm) {
    case 0: r = _mm_com_epu32(va, vb, 0); break;
    case 1: r = _mm_com_epu32(va, vb, 1); break;
    case 2: r = _mm_com_epu32(va, vb, 2); break;
    case 3: r = _mm_com_epu32(va, vb, 3); break;
    case 4: r = _mm_com_epu32(va, vb, 4); break;
    case 5: r = _mm_com_epu32(va, vb, 5); break;
    case 6: r = _mm_com_epu32(va, vb, 6); break;
    case 7: r = _mm_com_epu32(va, vb, 7); break;
        default: break;
    }
    _mm_storeu_si128((__m128i *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_xop_vpcomuq(const uint8_t *a, const uint8_t *b, uint32_t imm, uint8_t *out) {
    const __m128i va = _mm_loadu_si128((const __m128i *)(const void *)a);
    const __m128i vb = _mm_loadu_si128((const __m128i *)(const void *)b);
    __m128i r = _mm_setzero_si128();
    switch (imm) {
    case 0: r = _mm_com_epu64(va, vb, 0); break;
    case 1: r = _mm_com_epu64(va, vb, 1); break;
    case 2: r = _mm_com_epu64(va, vb, 2); break;
    case 3: r = _mm_com_epu64(va, vb, 3); break;
    case 4: r = _mm_com_epu64(va, vb, 4); break;
    case 5: r = _mm_com_epu64(va, vb, 5); break;
    case 6: r = _mm_com_epu64(va, vb, 6); break;
    case 7: r = _mm_com_epu64(va, vb, 7); break;
        default: break;
    }
    _mm_storeu_si128((__m128i *)(void *)out, r);
}

__attribute__((always_inline))
void rust_isa_xop_vprotb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_rot_epi8(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vprotw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_rot_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vprotd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_rot_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vprotq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_rot_epi64(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vpshab(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sha_epi8(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vpshaw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sha_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vpshad(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sha_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vpshaq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sha_epi64(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vpshlb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_shl_epi8(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vpshlw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_shl_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vpshld(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_shl_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vpshlq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_shl_epi64(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmacssww(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maccs_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmacsww(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_macc_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmacsswd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maccsd_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmacswd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maccd_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmacssdql(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maccslo_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmacsdql(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_macclo_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmacssdqh(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maccshi_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmacsdqh(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_macchi_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmacssdd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maccs_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmacsdd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_macc_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmadcsswd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maddsd_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vpmadcswd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_maddd_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b),
        _mm_loadu_si128((const __m128i *)(const void *)c)));
}

__attribute__((always_inline))
void rust_isa_xop_vfrczps(const uint8_t *a, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_frcz_ps(_mm_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vfrczpd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_frcz_pd(_mm_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vfrczss(const uint8_t *a, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_frcz_ss(_mm_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vfrczsd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_frcz_sd(_mm_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphaddbw(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddw_epi8(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphaddbd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddd_epi8(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphaddbq(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddq_epi8(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphaddwd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddd_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphaddwq(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddq_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphadddq(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddq_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphaddubw(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddw_epu8(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphaddubd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddd_epu8(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphaddubq(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddq_epu8(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphadduwd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddd_epu16(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphadduwq(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddq_epu16(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphaddudq(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_haddq_epu32(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphsubbw(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_hsubw_epi8(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphsubwd(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_hsubd_epi16(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_xop_vphsubdq(const uint8_t *a, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_hsubq_epi32(
        _mm_loadu_si128((const __m128i *)(const void *)a)));
}

/* LLVM does not define an encoding or intrinsic for VPMACSSDU/VPMACSDU.
   Compute the low 32 bits of the signed-by-unsigned product and add the
   accumulator lane. VPMACSSDU saturates the 32-bit sum. */
__attribute__((always_inline))
void rust_isa_xop_vpmacssdu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    int32_t av[4], bv[4], cv[4];
    _mm_storeu_si128((__m128i *)(void *)av, _mm_loadu_si128((const __m128i *)(const void *)a));
    _mm_storeu_si128((__m128i *)(void *)bv, _mm_loadu_si128((const __m128i *)(const void *)b));
    _mm_storeu_si128((__m128i *)(void *)cv, _mm_loadu_si128((const __m128i *)(const void *)c));
    for (int i = 0; i < 4; ++i) {
        const uint32_t product = (uint32_t)((int64_t)av[i] * (int64_t)(uint32_t)bv[i]);
        const int64_t sum = (int64_t)(int32_t)product + (int64_t)cv[i];
        if (sum > INT32_MAX) {
            cv[i] = INT32_MAX;
        } else if (sum < INT32_MIN) {
            cv[i] = INT32_MIN;
        } else {
            cv[i] = (int32_t)sum;
        }
    }
    _mm_storeu_si128((__m128i *)(void *)out, _mm_loadu_si128((const __m128i *)(const void *)cv));
}

__attribute__((always_inline))
void rust_isa_xop_vpmacsdu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    int32_t av[4], bv[4], cv[4];
    _mm_storeu_si128((__m128i *)(void *)av, _mm_loadu_si128((const __m128i *)(const void *)a));
    _mm_storeu_si128((__m128i *)(void *)bv, _mm_loadu_si128((const __m128i *)(const void *)b));
    _mm_storeu_si128((__m128i *)(void *)cv, _mm_loadu_si128((const __m128i *)(const void *)c));
    for (int i = 0; i < 4; ++i) {
        const uint32_t product = (uint32_t)((int64_t)av[i] * (int64_t)(uint32_t)bv[i]);
        cv[i] = (int32_t)(product + (uint32_t)cv[i]);
    }
    _mm_storeu_si128((__m128i *)(void *)out, _mm_loadu_si128((const __m128i *)(const void *)cv));
}

#define XOP_PERMIL2PS_CASE(N)                                                  \
    case N:                                                                    \
        r = _mm_permute2_ps(va, vb, vc, N);                                    \
        break;

__attribute__((always_inline))
void rust_isa_xop_vpermil2ps(const uint8_t *a, const uint8_t *b, const uint8_t *control, uint32_t imm, uint8_t *out) {
    const __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    const __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    const __m128 vc = _mm_loadu_ps((const float *)(const void *)control);
    __m128 r = _mm_setzero_ps();
    switch (imm & 0x03) {
        XOP_PERMIL2PS_CASE(0)
        XOP_PERMIL2PS_CASE(1)
        XOP_PERMIL2PS_CASE(2)
        XOP_PERMIL2PS_CASE(3)
        default:
            break;
    }
    _mm_storeu_ps((float *)(void *)out, r);
}

#undef XOP_PERMIL2PS_CASE

#define XOP_PERMIL2PD_CASE(N)                                                  \
    case N:                                                                    \
        r = _mm_permute2_pd(va, vb, vc, N);                                    \
        break;

__attribute__((always_inline))
void rust_isa_xop_vpermil2pd(const uint8_t *a, const uint8_t *b, const uint8_t *control, uint32_t imm, uint8_t *out) {
    const __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    const __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    const __m128d vc = _mm_loadu_pd((const double *)(const void *)control);
    __m128d r = _mm_setzero_pd();
    switch (imm & 0x03) {
        XOP_PERMIL2PD_CASE(0)
        XOP_PERMIL2PD_CASE(1)
        XOP_PERMIL2PD_CASE(2)
        XOP_PERMIL2PD_CASE(3)
        default:
            break;
    }
    _mm_storeu_pd((double *)(void *)out, r);
}

#undef XOP_PERMIL2PD_CASE
