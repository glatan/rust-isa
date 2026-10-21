#include <stdint.h>
#include <altivec.h>

typedef vector double v2f64;
typedef vector float v4f32;

__attribute__((always_inline))
void rust_isa_vsx_add_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a + *(const v2f64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_vsx_sub_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a - *(const v2f64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_vsx_mul_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a * *(const v2f64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_vsx_div_f64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a / *(const v2f64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_vsx_add_f32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a + *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_vsx_abs_f64(const uint8_t *a, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_abs(*(const v2f64 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_vsx_sqrt_f64(const uint8_t *a, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_sqrt(*(const v2f64 *)(const void *)a);
}

typedef vector unsigned char v16u8;
typedef vector unsigned long long v2u64;

__attribute__((always_inline))
void rust_isa_vsx_xvadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a + *(const v2f64 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_vsx_xvsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a - *(const v2f64 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_vsx_xvmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a * *(const v2f64 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_vsx_xvdiv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = *(const v2f64 *)(const void *)a / *(const v2f64 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_vsx_xvabs(const uint8_t *a, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_abs(*(const v2f64 *)(const void *)a);
}
__attribute__((always_inline))
void rust_isa_vsx_xvneg(const uint8_t *a, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_neg(*(const v2f64 *)(const void *)a);
}
__attribute__((always_inline))
void rust_isa_vsx_xvmax(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_max(*(const v2f64 *)(const void *)a, *(const v2f64 *)(const void *)b);
}
__attribute__((always_inline))
void rust_isa_vsx_xvmin(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_min(*(const v2f64 *)(const void *)a, *(const v2f64 *)(const void *)b);
}
__attribute__((always_inline))
void rust_isa_vsx_xvcmpeq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2u64 *)(void *)out = (v2u64)vec_cmpeq(*(const v2f64 *)(const void *)a, *(const v2f64 *)(const void *)b);
}
__attribute__((always_inline))
void rust_isa_vsx_vcmpgt(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2u64 *)(void *)out = (v2u64)vec_cmpgt(*(const v2f64 *)(const void *)a, *(const v2f64 *)(const void *)b);
}
__attribute__((always_inline))
void rust_isa_vsx_xvand(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16u8 *)(void *)out = *(const v16u8 *)(const void *)a & *(const v16u8 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_vsx_xvor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16u8 *)(void *)out = *(const v16u8 *)(const void *)a | *(const v16u8 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_vsx_xvxor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16u8 *)(void *)out = *(const v16u8 *)(const void *)a ^ *(const v16u8 *)(const void *)b;
}
__attribute__((always_inline))
void rust_isa_vsx_xvsel(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_sel(*(const v16u8 *)(const void *)a, *(const v16u8 *)(const void *)b, *(const v16u8 *)(const void *)c);
}
__attribute__((always_inline))
void rust_isa_vsx_xvperm(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(v16u8 *)(void *)out = vec_perm(*(const v16u8 *)(const void *)a, *(const v16u8 *)(const void *)b, *(const v16u8 *)(const void *)c);
}
__attribute__((always_inline))
void rust_isa_vsx_xvsl(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2u64 *)(void *)out = vec_sl(*(const v2u64 *)(const void *)a, *(const v2u64 *)(const void *)b);
}
__attribute__((always_inline))
void rust_isa_vsx_xvsr(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2u64 *)(void *)out = vec_sr(*(const v2u64 *)(const void *)a, *(const v2u64 *)(const void *)b);
}
__attribute__((always_inline))
void rust_isa_vsx_xvrdpi(const uint8_t *a, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_round(*(const v2f64 *)(const void *)a);
}
__attribute__((always_inline))
void rust_isa_vsx_xvrdpic(const uint8_t *a, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_rint(*(const v2f64 *)(const void *)a);
}
__attribute__((always_inline))
void rust_isa_vsx_xvrdpim(const uint8_t *a, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_floor(*(const v2f64 *)(const void *)a);
}
__attribute__((always_inline))
void rust_isa_vsx_xvrdpip(const uint8_t *a, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_ceil(*(const v2f64 *)(const void *)a);
}
__attribute__((always_inline))
void rust_isa_vsx_xvrdpiz(const uint8_t *a, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_trunc(*(const v2f64 *)(const void *)a);
}
__attribute__((always_inline))
void rust_isa_vsx_xvcpsgn(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2f64 *)(void *)out = vec_cpsgn(*(const v2f64 *)(const void *)a, *(const v2f64 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_vsx_xvsldoi(const uint8_t *a, const uint8_t *b, uint32_t imm, uint8_t *out) {
    const v16u8 va = *(const v16u8 *)(const void *)a;
    const v16u8 vb = *(const v16u8 *)(const void *)b;
    v16u8 r = va;
    switch (imm) {
    case 0: r = vec_sld(va, vb, 0); break;
    case 1: r = vec_sld(va, vb, 1); break;
    case 2: r = vec_sld(va, vb, 2); break;
    case 3: r = vec_sld(va, vb, 3); break;
    case 4: r = vec_sld(va, vb, 4); break;
    case 5: r = vec_sld(va, vb, 5); break;
    case 6: r = vec_sld(va, vb, 6); break;
    case 7: r = vec_sld(va, vb, 7); break;
    case 8: r = vec_sld(va, vb, 8); break;
    case 9: r = vec_sld(va, vb, 9); break;
    case 10: r = vec_sld(va, vb, 10); break;
    case 11: r = vec_sld(va, vb, 11); break;
    case 12: r = vec_sld(va, vb, 12); break;
    case 13: r = vec_sld(va, vb, 13); break;
    case 14: r = vec_sld(va, vb, 14); break;
    case 15: r = vec_sld(va, vb, 15); break;
        default: break;
    }
    *(v16u8 *)(void *)out = r;
}
