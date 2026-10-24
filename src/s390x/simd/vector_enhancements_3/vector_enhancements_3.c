#include <stdint.h>
#include <vecintrin.h>

typedef __vector signed __int128 v1i128;
typedef __vector unsigned __int128 v1u128;
typedef __vector signed long long v2i64;
typedef __vector unsigned long long v2u64;
typedef __vector signed char v16i8;
typedef __vector unsigned char v16u8;
typedef __vector signed short v8i16;
typedef __vector unsigned short v8u16;
typedef __vector signed int v4i32;
typedef __vector unsigned int v4u32;

#define LOAD(type, p) (*(const type *)(const void *)(p))
#define STORE(type, p, value) (*(type *)(void *)(p) = (value))

void rust_isa_s390x_ve3_vaq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1u128, out, LOAD(v1u128, a) + LOAD(v1u128, b));
}

void rust_isa_s390x_ve3_vsq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1u128, out, LOAD(v1u128, a) - LOAD(v1u128, b));
}

void rust_isa_s390x_ve3_vmalq(const uint8_t *acc, const uint8_t *a,
                              const uint8_t *b, uint8_t *out) {
    STORE(v1u128, out, LOAD(v1u128, acc) + LOAD(v1u128, a) * LOAD(v1u128, b));
}

void rust_isa_s390x_ve3_vmxq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1i128, out, vec_max(LOAD(v1i128, a), LOAD(v1i128, b)));
}

void rust_isa_s390x_ve3_vmnq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1i128, out, vec_min(LOAD(v1i128, a), LOAD(v1i128, b)));
}

void rust_isa_s390x_ve3_vmxlq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1u128, out, vec_max(LOAD(v1u128, a), LOAD(v1u128, b)));
}

void rust_isa_s390x_ve3_vmnlq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1u128, out, vec_min(LOAD(v1u128, a), LOAD(v1u128, b)));
}

void rust_isa_s390x_ve3_vavgq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1i128, out, vec_avg(LOAD(v1i128, a), LOAD(v1i128, b)));
}

void rust_isa_s390x_ve3_vavglq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1u128, out, vec_avg(LOAD(v1u128, a), LOAD(v1u128, b)));
}

void rust_isa_s390x_ve3_vclzq(const uint8_t *a, uint8_t *out) {
    STORE(v1u128, out, vec_cntlz(LOAD(v1u128, a)));
}

void rust_isa_s390x_ve3_vctzq(const uint8_t *a, uint8_t *out) {
    STORE(v1u128, out, vec_cnttz(LOAD(v1u128, a)));
}

void rust_isa_s390x_ve3_vlcq(const uint8_t *a, uint8_t *out) {
    STORE(v1u128, out, -LOAD(v1u128, a));
}

void rust_isa_s390x_ve3_vlpq(const uint8_t *a, uint8_t *out) {
    STORE(v1i128, out, vec_abs(LOAD(v1i128, a)));
}

void rust_isa_s390x_ve3_vuphg(const uint8_t *a, uint8_t *out) {
    STORE(v1i128, out, vec_unpackh(LOAD(v2i64, a)));
}

void rust_isa_s390x_ve3_vuplg(const uint8_t *a, uint8_t *out) {
    STORE(v1i128, out, vec_unpackl(LOAD(v2i64, a)));
}

void rust_isa_s390x_ve3_vuplhg(const uint8_t *a, uint8_t *out) {
    STORE(v1u128, out, vec_unpackh(LOAD(v2u64, a)));
}

void rust_isa_s390x_ve3_vupllg(const uint8_t *a, uint8_t *out) {
    STORE(v1u128, out, vec_unpackl(LOAD(v2u64, a)));
}

void rust_isa_s390x_ve3_vblendb(const uint8_t *a, const uint8_t *b,
                                const uint8_t *mask, uint8_t *out) {
    STORE(v16u8, out, vec_blend(LOAD(v16u8, a), LOAD(v16u8, b), LOAD(v16i8, mask)));
}

void rust_isa_s390x_ve3_vblendh(const uint8_t *a, const uint8_t *b,
                                const uint8_t *mask, uint8_t *out) {
    STORE(v8u16, out, vec_blend(LOAD(v8u16, a), LOAD(v8u16, b), LOAD(v8i16, mask)));
}

void rust_isa_s390x_ve3_vblendf(const uint8_t *a, const uint8_t *b,
                                const uint8_t *mask, uint8_t *out) {
    STORE(v4u32, out, vec_blend(LOAD(v4u32, a), LOAD(v4u32, b), LOAD(v4i32, mask)));
}

void rust_isa_s390x_ve3_vblendg(const uint8_t *a, const uint8_t *b,
                                const uint8_t *mask, uint8_t *out) {
    STORE(v2u64, out, vec_blend(LOAD(v2u64, a), LOAD(v2u64, b), LOAD(v2i64, mask)));
}

void rust_isa_s390x_ve3_vgemb(const uint8_t *mask, uint8_t *out) {
    STORE(v16u8, out, vec_gen_element_masks_8(LOAD(v8u16, mask)));
}

void rust_isa_s390x_ve3_vgemh(const uint8_t *mask, uint8_t *out) {
    STORE(v8u16, out, vec_gen_element_masks_16(LOAD(v16u8, mask)));
}

void rust_isa_s390x_ve3_vgemf(const uint8_t *mask, uint8_t *out) {
    STORE(v4u32, out, vec_gen_element_masks_32(LOAD(v16u8, mask)));
}

void rust_isa_s390x_ve3_vgemg(const uint8_t *mask, uint8_t *out) {
    STORE(v2u64, out, vec_gen_element_masks_64(LOAD(v16u8, mask)));
}

void rust_isa_s390x_ve3_vgemq(const uint8_t *mask, uint8_t *out) {
    STORE(v1u128, out, vec_gen_element_masks_128(LOAD(v16u8, mask)));
}

void rust_isa_s390x_ve3_vdq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1i128, out, LOAD(v1i128, a) / LOAD(v1i128, b));
}

void rust_isa_s390x_ve3_vrq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1i128, out, LOAD(v1i128, a) % LOAD(v1i128, b));
}

void rust_isa_s390x_ve3_vdlq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1u128, out, LOAD(v1u128, a) / LOAD(v1u128, b));
}

void rust_isa_s390x_ve3_vrlq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v1u128, out, LOAD(v1u128, a) % LOAD(v1u128, b));
}

void rust_isa_s390x_ve3_vdf(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v4i32, out, LOAD(v4i32, a) / LOAD(v4i32, b));
}

void rust_isa_s390x_ve3_vdg(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v2i64, out, LOAD(v2i64, a) / LOAD(v2i64, b));
}

void rust_isa_s390x_ve3_vdlf(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v4u32, out, LOAD(v4u32, a) / LOAD(v4u32, b));
}

void rust_isa_s390x_ve3_vdlg(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v2u64, out, LOAD(v2u64, a) / LOAD(v2u64, b));
}

void rust_isa_s390x_ve3_vrf(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v4i32, out, LOAD(v4i32, a) % LOAD(v4i32, b));
}

void rust_isa_s390x_ve3_vrg(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v2i64, out, LOAD(v2i64, a) % LOAD(v2i64, b));
}

void rust_isa_s390x_ve3_vrlf(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v4u32, out, LOAD(v4u32, a) % LOAD(v4u32, b));
}

void rust_isa_s390x_ve3_vrlg(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    STORE(v2u64, out, LOAD(v2u64, a) % LOAD(v2u64, b));
}
