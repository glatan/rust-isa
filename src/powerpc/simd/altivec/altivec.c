#include <stdint.h>
#include <altivec.h>

typedef vector unsigned int v4u32;
typedef vector signed int v4i32;

__attribute__((always_inline))
void rust_isa_altivec_v128_load(const uint8_t *p, uint8_t out[16]) {
    *(v4u32 *)(void *)out = *(const v4u32 *)(const void *)p;
}

__attribute__((always_inline))
void rust_isa_altivec_v128_store(uint8_t *p, const uint8_t value[16]) {
    *(v4u32 *)(void *)p = *(const v4u32 *)(const void *)value;
}

__attribute__((always_inline))
void rust_isa_altivec_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_add(*(const v4u32 *)(const void *)a, *(const v4u32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = vec_mul(*(const v4u32 *)(const void *)a, *(const v4u32 *)(const void *)b);
}

typedef signed char v16i8 __attribute__((vector_size(16)));
typedef unsigned char v16u8 __attribute__((vector_size(16)));
typedef short v8i16 __attribute__((vector_size(16)));
typedef long long v2i64 __attribute__((vector_size(16)));
typedef float v4f32 __attribute__((vector_size(16)));

__attribute__((always_inline))
void rust_isa_altivec_add_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16i8 *)(void *)out = *(const v16i8 *)(const void *)a + *(const v16i8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_add_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a + *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_add_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a + *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_sub_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16i8 *)(void *)out = *(const v16i8 *)(const void *)a - *(const v16i8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_sub_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a - *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_sub_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = *(const v4u32 *)(const void *)a - *(const v4u32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_sub_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a - *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_mul_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a * *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_mul_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = *(const v4u32 *)(const void *)a * *(const v4u32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_and(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a & *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_or(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a | *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_xor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a ^ *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_cmpeq_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = (v4u32)vec_cmpeq(
        *(const v4u32 *)(const void *)a,
        *(const v4u32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_cmpgt_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4u32 *)(void *)out = (v4u32)vec_cmpgt(
        *(const v4i32 *)(const void *)a,
        *(const v4i32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_fadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a + *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_fsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a - *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_fmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a * *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_fdiv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a / *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_permute(const uint8_t *a, const uint8_t *b, const uint8_t *index, uint8_t *out) {
    *(v16i8 *)(void *)out = (v16i8)vec_perm(
        *(const v16u8 *)(const void *)a,
        *(const v16u8 *)(const void *)b,
        *(const v16u8 *)(const void *)index);
}

__attribute__((always_inline))
void rust_isa_altivec_splat_i32(int32_t value, uint8_t *out) {
    *(v4u32 *)(void *)out = (v4u32){value, value, value, value};
}

typedef vector unsigned char av16u8;
typedef vector unsigned short av8u16;
typedef vector unsigned int av4u32;
typedef vector unsigned long long av2u64;
typedef vector signed int av4i32;

__attribute__((always_inline))
void rust_isa_altivec_vaddubm(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned char *)(void *)out = *(const vector unsigned char *)(const void *)a + *(const vector unsigned char *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vadduhm(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned short *)(void *)out = *(const vector unsigned short *)(const void *)a + *(const vector unsigned short *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vadduwm(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned int *)(void *)out = *(const vector unsigned int *)(const void *)a + *(const vector unsigned int *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vaddudm(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned long long *)(void *)out = *(const vector unsigned long long *)(const void *)a + *(const vector unsigned long long *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vsububm(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned char *)(void *)out = *(const vector unsigned char *)(const void *)a - *(const vector unsigned char *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vsubuhm(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned char *)(void *)out = *(const vector unsigned char *)(const void *)a - *(const vector unsigned char *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vsubuwm(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned char *)(void *)out = *(const vector unsigned char *)(const void *)a - *(const vector unsigned char *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vsubudm(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned char *)(void *)out = *(const vector unsigned char *)(const void *)a - *(const vector unsigned char *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vand(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned char *)(void *)out = *(const vector unsigned char *)(const void *)a & *(const vector unsigned char *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned char *)(void *)out = *(const vector unsigned char *)(const void *)a | *(const vector unsigned char *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vxor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned char *)(void *)out = *(const vector unsigned char *)(const void *)a ^ *(const vector unsigned char *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vandc(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(av16u8 *)(void *)out = *(const av16u8 *)(const void *)a & ~*(const av16u8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_altivec_vnor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(av16u8 *)(void *)out = ~(*(const av16u8 *)(const void *)a | *(const av16u8 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vnot(const uint8_t *a, uint8_t *out) {
    *(av16u8 *)(void *)out = ~*(const av16u8 *)(const void *)a;
}

__attribute__((always_inline))
void rust_isa_altivec_vsel(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(av16u8 *)(void *)out = vec_sel(
        *(const av16u8 *)(const void *)a,
        *(const av16u8 *)(const void *)b,
        *(const av16u8 *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_altivec_vperm(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(av16u8 *)(void *)out = vec_perm(
        *(const av16u8 *)(const void *)a,
        *(const av16u8 *)(const void *)b,
        *(const av16u8 *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_altivec_vmuleub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned short *)(void *)out = vec_mule(
        *(const vector unsigned char *)(const void *)a,
        *(const vector unsigned char *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vmuleuh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned int *)(void *)out = vec_mule(
        *(const vector unsigned short *)(const void *)a,
        *(const vector unsigned short *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vmuleuw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned long long *)(void *)out = vec_mule(
        *(const vector unsigned int *)(const void *)a,
        *(const vector unsigned int *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vmuloub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned short *)(void *)out = vec_mulo(
        *(const vector unsigned char *)(const void *)a,
        *(const vector unsigned char *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vmulouh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned int *)(void *)out = vec_mulo(
        *(const vector unsigned short *)(const void *)a,
        *(const vector unsigned short *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vmulouw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned long long *)(void *)out = vec_mulo(
        *(const vector unsigned int *)(const void *)a,
        *(const vector unsigned int *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vmin(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned int *)(void *)out = vec_min(
        *(const vector unsigned int *)(const void *)a,
        *(const vector unsigned int *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vmax(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned int *)(void *)out = vec_max(
        *(const vector unsigned int *)(const void *)a,
        *(const vector unsigned int *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vavg(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned char *)(void *)out = vec_avg(
        *(const vector unsigned char *)(const void *)a,
        *(const vector unsigned char *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vavgl(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned short *)(void *)out = vec_avg(
        *(const vector unsigned short *)(const void *)a,
        *(const vector unsigned short *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vcmpeq(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(av4u32 *)(void *)out = (av4u32)vec_cmpeq(
        *(const av4i32 *)(const void *)a,
        *(const av4i32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vcmpgt(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(av4u32 *)(void *)out = (av4u32)vec_cmpgt(
        *(const av4i32 *)(const void *)a,
        *(const av4i32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vcmp(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(av4u32 *)(void *)out = (av4u32)vec_cmpgt(
        *(const av4i32 *)(const void *)a,
        *(const av4i32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vsl(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned int *)(void *)out = vec_sl(
        *(const vector unsigned int *)(const void *)a,
        *(const vector unsigned int *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vsr(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(vector unsigned int *)(void *)out = vec_sr(
        *(const vector unsigned int *)(const void *)a,
        *(const vector unsigned int *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vslo(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(av16u8 *)(void *)out = vec_slo(
        *(const av16u8 *)(const void *)a,
        *(const av16u8 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vsro(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(av16u8 *)(void *)out = vec_sro(
        *(const av16u8 *)(const void *)a,
        *(const av16u8 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vsldoi(const uint8_t *a, const uint8_t *b, uint32_t imm, uint8_t *out) {
    const av16u8 va = *(const av16u8 *)(const void *)a;
    const av16u8 vb = *(const av16u8 *)(const void *)b;
    av16u8 r = va;
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
    *(av16u8 *)(void *)out = r;
}

typedef vector signed short av8i16;
typedef vector short av8s16;

__attribute__((always_inline))
void rust_isa_altivec_lvx(const uint8_t *p, uint8_t *out) {
    *(av16u8 *)(void *)out = *(const av16u8 *)(const void *)p;
}

__attribute__((always_inline))
void rust_isa_altivec_stvx(uint8_t *p, const uint8_t *a) {
    *(av16u8 *)(void *)p = *(const av16u8 *)(const void *)a;
}

__attribute__((always_inline))
void rust_isa_altivec_vmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(av8i16 *)(void *)out = vec_mul(
        *(const av8i16 *)(const void *)a,
        *(const av8i16 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vmla(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = vec_madd(
        *(const v4f32 *)(const void *)a,
        *(const v4f32 *)(const void *)b,
        *(const v4f32 *)(const void *)acc);
}

__attribute__((always_inline))
void rust_isa_altivec_vmls(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = vec_msub(
        *(const v4f32 *)(const void *)a,
        *(const v4f32 *)(const void *)b,
        *(const v4f32 *)(const void *)acc);
}

__attribute__((always_inline))
void rust_isa_altivec_vmadd(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = vec_madd(
        *(const v4f32 *)(const void *)acc,
        *(const v4f32 *)(const void *)a,
        *(const v4f32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vnmsub(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = vec_nmsub(
        *(const v4f32 *)(const void *)a,
        *(const v4f32 *)(const void *)b,
        *(const v4f32 *)(const void *)acc);
}

__attribute__((always_inline))
void rust_isa_altivec_vpk(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(av8i16 *)(void *)out = vec_pack(
        *(const v4i32 *)(const void *)a,
        *(const v4i32 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_altivec_vupk(const uint8_t *a, uint8_t *out) {
    *(v4i32 *)(void *)out = vec_unpackh(*(const av8i16 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_altivec_lvebx(const uint8_t *p, uint32_t index, uint8_t *out) {
    *(av16u8 *)(void *)out = vec_lde((long)index, p);
}

__attribute__((always_inline))
void rust_isa_altivec_lvehx(const uint8_t *p, uint32_t index, uint8_t *out) {
    *(av8u16 *)(void *)out = vec_lde((long)index, (const unsigned short *)(const void *)p);
}

__attribute__((always_inline))
void rust_isa_altivec_lvewx(const uint8_t *p, uint32_t index, uint8_t *out) {
    *(av4u32 *)(void *)out = vec_lde((long)index, (const unsigned int *)(const void *)p);
}

__attribute__((always_inline))
void rust_isa_altivec_stvebx(uint8_t *p, uint32_t index, const uint8_t *a) {
    vec_ste(*(const av16u8 *)(const void *)a, (long)index, (unsigned char *)(void *)p);
}

__attribute__((always_inline))
void rust_isa_altivec_stvehx(uint8_t *p, uint32_t index, const uint8_t *a) {
    vec_ste(*(const av8u16 *)(const void *)a, (long)index, (unsigned short *)(void *)p);
}

__attribute__((always_inline))
void rust_isa_altivec_stvewx(uint8_t *p, uint32_t index, const uint8_t *a) {
    vec_ste(*(const av4u32 *)(const void *)a, (long)index, (unsigned int *)(void *)p);
}
