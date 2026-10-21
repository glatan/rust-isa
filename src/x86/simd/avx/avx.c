#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx_vaddps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_add_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vaddpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_add_pd(
        _mm256_loadu_pd((const double *)(const void *)a),
        _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vsubps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_sub_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vsubpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_sub_pd(
        _mm256_loadu_pd((const double *)(const void *)a),
        _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vmulps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_mul_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vmulpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_mul_pd(
        _mm256_loadu_pd((const double *)(const void *)a),
        _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vdivps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_div_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vdivpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_div_pd(
        _mm256_loadu_pd((const double *)(const void *)a),
        _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vandps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_and_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vorps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_or_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vxorps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_xor_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vaddss(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_add_ss(
        _mm_loadu_ps((const float *)(const void *)a),
        _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vaddsd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_add_sd(
        _mm_loadu_pd((const double *)(const void *)a),
        _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vbroadcastss(float value, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_set1_ps(value));
}

__attribute__((always_inline))
void rust_isa_avx_vbroadcastsd(double value, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_set1_pd(value));
}

__attribute__((always_inline))
uint32_t rust_isa_avx_vmovmskps(const uint8_t *a) {
    return (uint32_t)_mm256_movemask_ps(_mm256_loadu_ps((const float *)(const void *)a));
}

__attribute__((always_inline))
void rust_isa_avx_vzeroupper(void) {
    _mm256_zeroupper();
}

__attribute__((always_inline))
void rust_isa_avx_vunpcklps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_unpacklo_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vunpckhps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_unpackhi_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vunpcklpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_unpacklo_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vunpckhpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_unpackhi_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vpsadbw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_sad_epu8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vblendvps(const uint8_t *a, const uint8_t *b, const uint8_t *mask, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_blendv_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b), _mm256_loadu_ps((const float *)(const void *)mask)));
}

__attribute__((always_inline))
void rust_isa_avx_vblendvpd(const uint8_t *a, const uint8_t *b, const uint8_t *mask, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_blendv_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b), _mm256_loadu_pd((const double *)(const void *)mask)));
}

__attribute__((always_inline))
void rust_isa_avx_vbroadcastf128(const uint8_t *p, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_castps_si256(_mm256_broadcast_ps((const __m128 *)(const void *)p)));
}

__attribute__((always_inline))
void rust_isa_avx_vmaskmovps_load(const uint8_t *p, const uint8_t *mask, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_maskload_ps((const float *)(const void *)p, _mm256_loadu_ps((const float *)(const void *)mask)));
}

__attribute__((always_inline))
void rust_isa_avx_vmaskmovps_store(uint8_t *p, const uint8_t *mask, const uint8_t *value) {
    _mm256_maskstore_ps((float *)(void *)p, _mm256_loadu_ps((const float *)(const void *)mask), _mm256_loadu_ps((const float *)(const void *)value));
}

__attribute__((always_inline))
void rust_isa_avx_vmaskmovpd_load(const uint8_t *p, const uint8_t *mask, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_maskload_pd((const double *)(const void *)p, _mm256_loadu_pd((const double *)(const void *)mask)));
}

__attribute__((always_inline))
void rust_isa_avx_vmaskmovpd_store(uint8_t *p, const uint8_t *mask, const uint8_t *value) {
    _mm256_maskstore_pd((double *)(void *)p, _mm256_loadu_pd((const double *)(const void *)mask), _mm256_loadu_pd((const double *)(const void *)value));
}

__attribute__((always_inline))
void rust_isa_avx_vmovd(uint32_t value, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cvtsi32_si128((int)value));
}

__attribute__((always_inline))
uint32_t rust_isa_avx_vmovd_to_u32(const uint8_t *a) {
    return (uint32_t)_mm_cvtsi128_si32(_mm_loadu_si128((const __m128i *)(const void *)a));
}

__attribute__((always_inline))
uint32_t rust_isa_avx_vmovmskpd(const uint8_t *a) {
    return (uint32_t)_mm256_movemask_pd(_mm256_loadu_pd((const double *)(const void *)a));
}

__attribute__((always_inline))
void rust_isa_avx_vmovshdup(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_movehdup_ps(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx_vmovsldup(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_moveldup_ps(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx_vmovddup(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_movedup_pd(_mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx_vlddqu(const uint8_t *p, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_lddqu_si256((const __m256i *)(const void *)p));
}

__attribute__((always_inline))
void rust_isa_avx_vaddsubps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_addsub_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vaddsubpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_addsub_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vandpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_and_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vandnps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_andnot_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vandnpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_andnot_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vxorpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_xor_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vmaxps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_max_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vmaxpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_max_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vminps(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_min_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vminpd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_min_pd(_mm256_loadu_pd((const double *)(const void *)a), _mm256_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vsqrtps(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_sqrt_ps(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx_vsqrtpd(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_sqrt_pd(_mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx_vrcpps(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_rcp_ps(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx_vcvtps2pd(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_pd((double *)(void *)out, _mm256_cvtps_pd(_mm_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx_vcvtpd2ps(const uint8_t *a, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm256_cvtpd_ps(_mm256_loadu_pd((const double *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx_vcvtss2sd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_pd((double *)(void *)out, _mm_cvtss_sd(_mm_loadu_pd((const double *)(const void *)a), _mm_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vcvtsd2ss(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_cvtsd_ss(_mm_loadu_ps((const float *)(const void *)a), _mm_loadu_pd((const double *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx_vcvtdq2ps(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_cvtepi32_ps(_mm256_loadu_si256((const __m256i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx_vcvtps2dq(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_cvtps_epi32(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_avx_vcvttps2dq(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_cvttps_epi32(_mm256_loadu_ps((const float *)(const void *)a)));
}

__attribute__((always_inline))
uint32_t rust_isa_avx_vcomiss(const uint8_t *a, const uint8_t *b) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    uint32_t cf, pf, zf;
    __asm__ volatile(
        "vcomiss %4, %3\n\t"
        "setc %b0\n\t"
        "setp %b1\n\t"
        "sete %b2"
        : "=r"(cf), "=r"(pf), "=r"(zf)
        : "x"(va), "x"(vb)
        : "cc");
    return (cf & 1) | ((pf & 1) << 1) | ((zf & 1) << 2);
}

__attribute__((always_inline))
uint32_t rust_isa_avx_vcomisd(const uint8_t *a, const uint8_t *b) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    uint32_t cf, pf, zf;
    __asm__ volatile(
        "vcomisd %4, %3\n\t"
        "setc %b0\n\t"
        "setp %b1\n\t"
        "sete %b2"
        : "=r"(cf), "=r"(pf), "=r"(zf)
        : "x"(va), "x"(vb)
        : "cc");
    return (cf & 1) | ((pf & 1) << 1) | ((zf & 1) << 2);
}

__attribute__((always_inline))
uint32_t rust_isa_avx_vucomiss(const uint8_t *a, const uint8_t *b) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    uint32_t cf, pf, zf;
    __asm__ volatile(
        "vucomiss %4, %3\n\t"
        "setc %b0\n\t"
        "setp %b1\n\t"
        "sete %b2"
        : "=r"(cf), "=r"(pf), "=r"(zf)
        : "x"(va), "x"(vb)
        : "cc");
    return (cf & 1) | ((pf & 1) << 1) | ((zf & 1) << 2);
}

__attribute__((always_inline))
uint32_t rust_isa_avx_vucomisd(const uint8_t *a, const uint8_t *b) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    uint32_t cf, pf, zf;
    __asm__ volatile(
        "vucomisd %4, %3\n\t"
        "setc %b0\n\t"
        "setp %b1\n\t"
        "sete %b2"
        : "=r"(cf), "=r"(pf), "=r"(zf)
        : "x"(va), "x"(vb)
        : "cc");
    return (cf & 1) | ((pf & 1) << 1) | ((zf & 1) << 2);
}

__attribute__((always_inline))
uint32_t rust_isa_avx_vptest(const uint8_t *a, const uint8_t *b) {
    __m256i va = _mm256_loadu_si256((const __m256i *)(const void *)a);
    __m256i vb = _mm256_loadu_si256((const __m256i *)(const void *)b);
    uint32_t flags = 0;
    if (_mm256_testz_si256(va, vb)) { flags |= 4; }
    if (_mm256_testc_si256(va, vb)) { flags |= 1; }
    return flags;
}

__attribute__((always_inline))
uint32_t rust_isa_avx_vtestps(const uint8_t *a, const uint8_t *b) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    uint32_t flags = 0;
    if (_mm256_testz_ps(va, vb)) { flags |= 4; }
    if (_mm256_movemask_ps(_mm256_and_ps(va, vb)) == 0xff) { flags |= 1; }
    return flags;
}

__attribute__((always_inline))
uint32_t rust_isa_avx_vtestpd(const uint8_t *a, const uint8_t *b) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    uint32_t flags = 0;
    if (_mm256_testz_pd(va, vb)) { flags |= 4; }
    if (_mm256_movemask_pd(_mm256_and_pd(va, vb)) == 0x0f) { flags |= 1; }
    return flags;
}

__attribute__((always_inline))
void rust_isa_avx_vmovntps(uint8_t *p, const uint8_t *value) {
    _mm256_stream_ps((float *)(void *)p, _mm256_loadu_ps((const float *)(const void *)value));
}

__attribute__((always_inline))
void rust_isa_avx_vmovntpd(uint8_t *p, const uint8_t *value) {
    _mm256_stream_pd((double *)(void *)p, _mm256_loadu_pd((const double *)(const void *)value));
}

__attribute__((always_inline))
void rust_isa_avx_vmovq(uint64_t value, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cvtsi64_si128((long long)value));
}

__attribute__((always_inline))
uint64_t rust_isa_avx_vmovq_to_u64(const uint8_t *a) {
    return (uint64_t)_mm_cvtsi128_si64(_mm_loadu_si128((const __m128i *)(const void *)a));
}

