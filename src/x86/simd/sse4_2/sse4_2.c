#include <stdint.h>
#include <nmmintrin.h>

__attribute__((always_inline))
void rust_isa_sse42_pcmpgtq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    _mm_storeu_si128((__m128i *)(void *)out, _mm_cmpgt_epi64(
        _mm_loadu_si128((const __m128i *)(const void *)a),
        _mm_loadu_si128((const __m128i *)(const void *)b)));
}

__attribute__((always_inline))
uint32_t rust_isa_sse42_crc32_u8(uint32_t crc, uint8_t value) {
    return _mm_crc32_u8(crc, value);
}

__attribute__((always_inline))
uint32_t rust_isa_sse42_crc32_u16(uint32_t crc, uint16_t value) {
    return _mm_crc32_u16(crc, value);
}

__attribute__((always_inline))
uint32_t rust_isa_sse42_crc32_u32(uint32_t crc, uint32_t value) {
    return _mm_crc32_u32(crc, value);
}

#if defined(__x86_64__)
__attribute__((always_inline))
uint64_t rust_isa_sse42_crc32_u64(uint64_t crc, uint64_t value) {
    return _mm_crc32_u64(crc, value);
}
#endif
