#include <stdint.h>

#include <lsxintrin.h>

void rust_isa_lsx_vfrecipe_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrecipe_s(*(const __m128 *)(const void *)a);
}

void rust_isa_lsx_vfrecipe_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrecipe_d(*(const __m128d *)(const void *)a);
}

void rust_isa_lsx_vfrsqrte_s(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrsqrte_s(*(const __m128 *)(const void *)a);
}

void rust_isa_lsx_vfrsqrte_d(const uint8_t *a, uint8_t *out) {
    *(__m128i *)(void *)out = (__m128i)__lsx_vfrsqrte_d(*(const __m128d *)(const void *)a);
}
