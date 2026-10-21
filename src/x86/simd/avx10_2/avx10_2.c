#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_avx10_2_vaddps_v256(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm256_storeu_ps((float *)(void *)out, _mm256_add_ps(
        _mm256_loadu_ps((const float *)(const void *)a),
        _mm256_loadu_ps((const float *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_avx10_2_vmulps_v512(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm512_storeu_ps((float *)(void *)out, _mm512_mul_ps(
        _mm512_loadu_ps((const float *)(const void *)a),
        _mm512_loadu_ps((const float *)(const void *)b)));
}
