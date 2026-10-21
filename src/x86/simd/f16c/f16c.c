#include <stdint.h>
#include <immintrin.h>

void rust_isa_f16c_vcvtps2ph_0(const uint8_t *a, uint8_t *out);
void rust_isa_f16c_vcvtps2ph_1(const uint8_t *a, uint8_t *out);
void rust_isa_f16c_vcvtps2ph_2(const uint8_t *a, uint8_t *out);
void rust_isa_f16c_vcvtps2ph_3(const uint8_t *a, uint8_t *out);
void rust_isa_f16c_vcvtps2ph_4(const uint8_t *a, uint8_t *out);
void rust_isa_f16c_vcvtps2ph_v256_0(const uint8_t *a, uint8_t *out);
void rust_isa_f16c_vcvtps2ph_v256_1(const uint8_t *a, uint8_t *out);
void rust_isa_f16c_vcvtps2ph_v256_2(const uint8_t *a, uint8_t *out);
void rust_isa_f16c_vcvtps2ph_v256_3(const uint8_t *a, uint8_t *out);
void rust_isa_f16c_vcvtps2ph_v256_4(const uint8_t *a, uint8_t *out);


__attribute__((always_inline))
void rust_isa_f16c_vcvtph2ps(const uint8_t *a, uint8_t *out) {
    _mm_storeu_ps((float *)(void *)out, _mm_cvtph_ps(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_f16c_vcvtph2ps_v256(const uint8_t *a, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_cvtph_ps(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

#define RUST_ISA_F16C_VCVTPS2PH(N) \
__attribute__((always_inline)) \
void rust_isa_f16c_vcvtps2ph_##N(const uint8_t *a, uint8_t *out) { \
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cvtps_ph( \
        _mm_loadu_ps((const float *)(const void *)a), N)); \
}

#define RUST_ISA_F16C_VCVTPS2PH_V256(N) \
__attribute__((always_inline)) \
void rust_isa_f16c_vcvtps2ph_v256_##N(const uint8_t *a, uint8_t *out) { \
    _mm_storeu_si128((__m128i *)(void *)out, _mm256_cvtps_ph( \
        _mm256_loadu_ps((const float *)(const void *)a), N)); \
}

RUST_ISA_F16C_VCVTPS2PH(0)
RUST_ISA_F16C_VCVTPS2PH(1)
RUST_ISA_F16C_VCVTPS2PH(2)
RUST_ISA_F16C_VCVTPS2PH(3)
RUST_ISA_F16C_VCVTPS2PH(4)

RUST_ISA_F16C_VCVTPS2PH_V256(0)
RUST_ISA_F16C_VCVTPS2PH_V256(1)
RUST_ISA_F16C_VCVTPS2PH_V256(2)
RUST_ISA_F16C_VCVTPS2PH_V256(3)
RUST_ISA_F16C_VCVTPS2PH_V256(4)
