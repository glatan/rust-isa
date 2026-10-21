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
