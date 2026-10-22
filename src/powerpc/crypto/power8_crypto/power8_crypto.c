#include <stdint.h>
#include <altivec.h>

typedef vector unsigned char v16u8;
typedef vector unsigned short v8u16;
typedef vector unsigned int v4u32;
typedef vector unsigned long long v2u64;

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

__attribute__((always_inline))
void rust_isa_power8_crypto_vsbox(const uint8_t *a, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_sbox_be(*(const v16u8 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_power8_crypto_vpmsumb(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_pmsum_be(
        *(const v16u8 *)(const void *)a,
        *(const v16u8 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_crypto_vpmsumh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8u16 *)(void *)out = vec_pmsum_be(
        *(const v8u16 *)(const void *)a,
        *(const v8u16 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_crypto_vpmsumw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_pmsum_be(
        *(const v4u32 *)(const void *)a,
        *(const v4u32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_crypto_vpmsumd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2u64 *)(void *)out = vec_pmsum_be(
        *(const v2u64 *)(const void *)a,
        *(const v2u64 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_power8_crypto_vpermxor(const uint8_t *a, const uint8_t *b, const uint8_t *c,
                                     uint8_t *out) {
    *(v16u8 *)(void *)out = vec_permxor(
        *(const v16u8 *)(const void *)a,
        *(const v16u8 *)(const void *)b,
        *(const v16u8 *)(const void *)c);
}

/*
 * The ST and SIX fields are encoded in the instruction, so every combination
 * is a separate constant case. The Rust wrapper only passes a constant value
 * through a const generic parameter.
 */
__attribute__((always_inline))
void rust_isa_power8_crypto_vshasigmaw(const uint8_t *a, uint32_t st_six, uint8_t *out) {
    v4u32 value = *(const v4u32 *)(const void *)a;
    switch (st_six) {
#define SHA_W_CASE(index)                                                       \
    case index:                                                                 \
        *(v4u32 *)(void *)out = vec_shasigma_be(value, index >> 4, index & 0xf); \
        break;
        SHA_W_CASE(0) SHA_W_CASE(1) SHA_W_CASE(2) SHA_W_CASE(3)
        SHA_W_CASE(4) SHA_W_CASE(5) SHA_W_CASE(6) SHA_W_CASE(7)
        SHA_W_CASE(8) SHA_W_CASE(9) SHA_W_CASE(10) SHA_W_CASE(11)
        SHA_W_CASE(12) SHA_W_CASE(13) SHA_W_CASE(14) SHA_W_CASE(15)
        SHA_W_CASE(16) SHA_W_CASE(17) SHA_W_CASE(18) SHA_W_CASE(19)
        SHA_W_CASE(20) SHA_W_CASE(21) SHA_W_CASE(22) SHA_W_CASE(23)
        SHA_W_CASE(24) SHA_W_CASE(25) SHA_W_CASE(26) SHA_W_CASE(27)
        SHA_W_CASE(28) SHA_W_CASE(29) SHA_W_CASE(30) SHA_W_CASE(31)
#undef SHA_W_CASE
    default:
        *(v4u32 *)(void *)out = (v4u32){0, 0, 0, 0};
        break;
    }
}

__attribute__((always_inline))
void rust_isa_power8_crypto_vshasigmad(const uint8_t *a, uint32_t st_six, uint8_t *out) {
    v2u64 value = *(const v2u64 *)(const void *)a;
    switch (st_six) {
#define SHA_D_CASE(index)                                                       \
    case index:                                                                 \
        *(v2u64 *)(void *)out = vec_shasigma_be(value, index >> 4, index & 0xf); \
        break;
        SHA_D_CASE(0) SHA_D_CASE(1) SHA_D_CASE(2) SHA_D_CASE(3)
        SHA_D_CASE(4) SHA_D_CASE(5) SHA_D_CASE(6) SHA_D_CASE(7)
        SHA_D_CASE(8) SHA_D_CASE(9) SHA_D_CASE(10) SHA_D_CASE(11)
        SHA_D_CASE(12) SHA_D_CASE(13) SHA_D_CASE(14) SHA_D_CASE(15)
        SHA_D_CASE(16) SHA_D_CASE(17) SHA_D_CASE(18) SHA_D_CASE(19)
        SHA_D_CASE(20) SHA_D_CASE(21) SHA_D_CASE(22) SHA_D_CASE(23)
        SHA_D_CASE(24) SHA_D_CASE(25) SHA_D_CASE(26) SHA_D_CASE(27)
        SHA_D_CASE(28) SHA_D_CASE(29) SHA_D_CASE(30) SHA_D_CASE(31)
#undef SHA_D_CASE
    default:
        *(v2u64 *)(void *)out = (v2u64){0, 0};
        break;
    }
}
