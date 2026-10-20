#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_gfni_gf2p8mulb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_gf2p8mul_epi8(_mm_loadu_si128((const __m128i *)(const void *)a), _mm_loadu_si128((const __m128i *)(const void *)b)));
}
