#include <stdint.h>
#include <ammintrin.h>

__attribute__((always_inline))
void rust_isa_sse4a_extrq(const uint8_t *a, const uint8_t *control, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_extract_si64(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)control)));
}

__attribute__((always_inline))
void rust_isa_sse4a_insertq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_insert_si64(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_sse4a_movntsd(float *p, const uint8_t *a) {
    _mm_stream_sd((double *)(void *)p, _mm_castsi128_pd(_mm_loadu_si128((const __m128i *)(const void *)a)));
}

__attribute__((always_inline))
void rust_isa_sse4a_movntss(float *p, const uint8_t *a) {
    _mm_stream_ss(p, _mm_loadu_si128((const __m128i *)(const void *)a));
}
