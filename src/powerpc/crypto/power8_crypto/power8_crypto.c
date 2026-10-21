#include <stdint.h>
#include <altivec.h>

typedef vector unsigned char v16u8;

__attribute__((always_inline))
void rust_isa_power8_crypto_cipher(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_cipher_be(
        *(const v16u8 *)(const void *)a,
        *(const v16u8 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_crypto_ncipher(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_ncipher_be(
        *(const v16u8 *)(const void *)a,
        *(const v16u8 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_crypto_cipherlast(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_cipherlast_be(
        *(const v16u8 *)(const void *)a,
        *(const v16u8 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_crypto_ncipherlast(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_ncipherlast_be(
        *(const v16u8 *)(const void *)a,
        *(const v16u8 *)(const void *)b);
}
