#include <stdint.h>

#include <lasxintrin.h>

void rust_isa_lasx_xvfrecipe_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrecipe_s(*(const __m256 *)(const void *)a);
}

void rust_isa_lasx_xvfrecipe_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrecipe_d(*(const __m256d *)(const void *)a);
}

void rust_isa_lasx_xvfrsqrte_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrsqrte_s(*(const __m256 *)(const void *)a);
}

void rust_isa_lasx_xvfrsqrte_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrsqrte_d(*(const __m256d *)(const void *)a);
}
