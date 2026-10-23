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
void rust_isa_vsx_vsldoi(const uint8_t *a, const uint8_t *b, uint32_t imm, uint8_t *out) {
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

#define VSX_LOAD(T, p) ({ T v; __builtin_memcpy(&v, (p), sizeof(T)); v; })
#define VSX_STORE(T, p, v) do { T t = (v); __builtin_memcpy((p), &t, sizeof(T)); } while (0)

__attribute__((always_inline))
void rust_isa_vsx_xscvdpsp(const uint8_t *a, uint8_t *out) {
    VSX_STORE(float, out, (float)VSX_LOAD(double, a));
}
__attribute__((always_inline))
void rust_isa_vsx_xscvdpspn(const uint8_t *a, uint8_t *out) {
    VSX_STORE(float, out, (float)VSX_LOAD(double, a));
}
__attribute__((always_inline))
void rust_isa_vsx_xscvspdp(const uint8_t *a, uint8_t *out) {
    VSX_STORE(double, out, (double)VSX_LOAD(float, a));
}
__attribute__((always_inline))
void rust_isa_vsx_xscvspdpn(const uint8_t *a, uint8_t *out) {
    VSX_STORE(double, out, (double)VSX_LOAD(float, a));
}
__attribute__((always_inline))
void rust_isa_vsx_xvcvsxdsp(const uint8_t *a, uint8_t *out) {
    int64_t x0 = VSX_LOAD(int64_t, a);
    int64_t x1 = VSX_LOAD(int64_t, (const uint8_t *)a + 8);
    VSX_STORE(float, out, (float)x0);
    VSX_STORE(float, (uint8_t *)out + 4, (float)x1);
}
__attribute__((always_inline))
void rust_isa_vsx_xvcvuxdsp(const uint8_t *a, uint8_t *out) {
    uint64_t x0 = VSX_LOAD(uint64_t, a);
    uint64_t x1 = VSX_LOAD(uint64_t, (const uint8_t *)a + 8);
    VSX_STORE(float, out, (float)x0);
    VSX_STORE(float, (uint8_t *)out + 4, (float)x1);
}
__attribute__((always_inline))
void rust_isa_vsx_xvcvsxwsp(const uint8_t *a, uint8_t *out) {
    for (int i = 0; i < 4; i++) {
        VSX_STORE(float, (uint8_t *)out + i * 4, (float)VSX_LOAD(int32_t, (const uint8_t *)a + i * 4));
    }
}
__attribute__((always_inline))
void rust_isa_vsx_xvcvuxwsp(const uint8_t *a, uint8_t *out) {
    for (int i = 0; i < 4; i++) {
        VSX_STORE(float, (uint8_t *)out + i * 4, (float)VSX_LOAD(uint32_t, (const uint8_t *)a + i * 4));
    }
}
__attribute__((always_inline))
void rust_isa_vsx_xvcvdpsxds(const uint8_t *a, uint8_t *out) {
    VSX_STORE(int64_t, out, (int64_t)VSX_LOAD(double, a));
    VSX_STORE(int64_t, (uint8_t *)out + 8, (int64_t)VSX_LOAD(double, (const uint8_t *)a + 8));
}
__attribute__((always_inline))
void rust_isa_vsx_xvcvdpsxws(const uint8_t *a, uint8_t *out) {
    VSX_STORE(int32_t, out, (int32_t)VSX_LOAD(double, a));
    VSX_STORE(int32_t, (uint8_t *)out + 4, (int32_t)VSX_LOAD(double, (const uint8_t *)a + 8));
}
__attribute__((always_inline))
void rust_isa_vsx_xvcvdpuxds(const uint8_t *a, uint8_t *out) {
    VSX_STORE(uint64_t, out, (uint64_t)VSX_LOAD(double, a));
    VSX_STORE(uint64_t, (uint8_t *)out + 8, (uint64_t)VSX_LOAD(double, (const uint8_t *)a + 8));
}
__attribute__((always_inline))
void rust_isa_vsx_xvcvdpuxws(const uint8_t *a, uint8_t *out) {
    VSX_STORE(uint32_t, out, (uint32_t)VSX_LOAD(double, a));
    VSX_STORE(uint32_t, (uint8_t *)out + 4, (uint32_t)VSX_LOAD(double, (const uint8_t *)a + 8));
}
