#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_fma_vfmadd132ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    _mm256_storeu_si256((__m256i *)(void *)out, _mm256_fmadd_ps(_mm256_loadu_ps((const float *)(const void *)a), _mm256_loadu_ps((const float *)(const void *)b), _mm256_loadu_ps((const float *)(const void *)c)));
}
