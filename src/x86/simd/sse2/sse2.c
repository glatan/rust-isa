#include <stdint.h>
#include <emmintrin.h>

typedef int rust_isa_v4i32 __attribute__((vector_size(16), aligned(16)));

__attribute__((always_inline))
void rust_isa_sse2_v128_load(const uint8_t *p, uint8_t out[16]) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_loadu_si128((const __m128i *)(const void *)p));
}

__attribute__((always_inline))
void rust_isa_sse2_v128_store(uint8_t *p, const uint8_t value[16]) {
    _mm_storeu_si128((__m128i *)(void *)p, _mm_loadu_si128((const __m128i *)(const void *)value));
}

__attribute__((always_inline))
void rust_isa_sse2_paddd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    rust_isa_v4i32 x = *(const rust_isa_v4i32 *)(const void *)a;
    rust_isa_v4i32 y = *(const rust_isa_v4i32 *)(const void *)b;
    *(rust_isa_v4i32 *)(void *)out = x + y;
}
