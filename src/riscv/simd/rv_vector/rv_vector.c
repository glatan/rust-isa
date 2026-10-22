#include <riscv_vector.h>
#include <stddef.h>
#include <stdint.h>

typedef int v4i32 __attribute__((vector_size(16)));

__attribute__((always_inline))
void rust_isa_rv_vector_v128_load(const uint8_t *p, uint8_t out[16]) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)p;
}

__attribute__((always_inline))
void rust_isa_rv_vector_v128_store(uint8_t *p, const uint8_t value[16]) {
    *(v4i32 *)(void *)p = *(const v4i32 *)(const void *)value;
}

__attribute__((always_inline))
void rust_isa_rv_vector_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a + *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a * *(const v4i32 *)(const void *)b;
}

typedef signed char v16i8 __attribute__((vector_size(16)));
typedef short v8i16 __attribute__((vector_size(16)));
typedef long long v2i64 __attribute__((vector_size(16)));
typedef float v4f32 __attribute__((vector_size(16)));

__attribute__((always_inline))
void rust_isa_rv_vector_add_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16i8 *)(void *)out = *(const v16i8 *)(const void *)a + *(const v16i8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_add_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a + *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_add_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a + *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_sub_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16i8 *)(void *)out = *(const v16i8 *)(const void *)a - *(const v16i8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_sub_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a - *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_sub_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a - *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_sub_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a - *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_mul_i8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v16i8 *)(void *)out = *(const v16i8 *)(const void *)a * *(const v16i8 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_mul_i16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i16 *)(void *)out = *(const v8i16 *)(const void *)a * *(const v8i16 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_and(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a & *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_or(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a | *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_xor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v2i64 *)(void *)out = *(const v2i64 *)(const void *)a ^ *(const v2i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_cmpeq_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a == *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_cmpgt_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i32 *)(void *)out = *(const v4i32 *)(const void *)a > *(const v4i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_fadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a + *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_fsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a - *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_fmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a * *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_fdiv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4f32 *)(void *)out = *(const v4f32 *)(const void *)a / *(const v4f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_rv_vector_splat_i32(int32_t value, uint8_t *out) {
    *(v4i32 *)(void *)out = (v4i32){value, value, value, value};
}

/* Fixed 128-bit helpers for the RVV intrinsic implementations. */

static inline vint32m1_t rv_load_i32(const uint8_t *p) {
    return __riscv_vle32_v_i32m1((const int32_t *)(const void *)p, 4);
}

static inline void rv_store_i32(uint8_t *p, vint32m1_t value) {
    __riscv_vse32_v_i32m1((int32_t *)(void *)p, value, 4);
}

static inline vuint32m1_t rv_load_u32(const uint8_t *p) {
    return __riscv_vle32_v_u32m1((const uint32_t *)(const void *)p, 4);
}

static inline void rv_store_u32(uint8_t *p, vuint32m1_t value) {
    __riscv_vse32_v_u32m1((uint32_t *)(void *)p, value, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vdiv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vdiv_vv_i32m1(va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vrem(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vrem_vv_i32m1(va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vmin(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vmin_vv_i32m1(va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vmax(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vmax_vv_i32m1(va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vdivu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    rv_store_u32(out, __riscv_vdivu_vv_u32m1(va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vremu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    rv_store_u32(out, __riscv_vremu_vv_u32m1(va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vminu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    rv_store_u32(out, __riscv_vminu_vv_u32m1(va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vmaxu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    rv_store_u32(out, __riscv_vmaxu_vv_u32m1(va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vsll(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vuint32m1_t shift = rv_load_u32(b);
    rv_store_i32(out, __riscv_vsll_vv_i32m1(va, shift, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsrl(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t shift = rv_load_u32(b);
    rv_store_u32(out, __riscv_vsrl_vv_u32m1(va, shift, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsra(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vuint32m1_t shift = rv_load_u32(b);
    rv_store_i32(out, __riscv_vsra_vv_i32m1(va, shift, 4));
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmseq_mask(const uint8_t *a, const uint8_t *b) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    vbool32_t mask = __riscv_vmseq_vv_i32m1_b32(va, vb, 4);
    uint16_t out = 0;
    __riscv_vsm_v_b32((uint8_t *)&out, mask, 4);
    return out & 0x0f;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmsne_mask(const uint8_t *a, const uint8_t *b) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    vbool32_t mask = __riscv_vmsne_vv_i32m1_b32(va, vb, 4);
    uint16_t out = 0;
    __riscv_vsm_v_b32((uint8_t *)&out, mask, 4);
    return out & 0x0f;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmslt_mask(const uint8_t *a, const uint8_t *b) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    vbool32_t mask = __riscv_vmslt_vv_i32m1_b32(va, vb, 4);
    uint16_t out = 0;
    __riscv_vsm_v_b32((uint8_t *)&out, mask, 4);
    return out & 0x0f;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmsle_mask(const uint8_t *a, const uint8_t *b) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    vbool32_t mask = __riscv_vmsle_vv_i32m1_b32(va, vb, 4);
    uint16_t out = 0;
    __riscv_vsm_v_b32((uint8_t *)&out, mask, 4);
    return out & 0x0f;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmsgt_mask(const uint8_t *a, const uint8_t *b) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    vbool32_t mask = __riscv_vmsgt_vv_i32m1_b32(va, vb, 4);
    uint16_t out = 0;
    __riscv_vsm_v_b32((uint8_t *)&out, mask, 4);
    return out & 0x0f;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmsge_mask(const uint8_t *a, const uint8_t *b) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    vbool32_t mask = __riscv_vmsge_vv_i32m1_b32(va, vb, 4);
    uint16_t out = 0;
    __riscv_vsm_v_b32((uint8_t *)&out, mask, 4);
    return out & 0x0f;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmsltu_mask(const uint8_t *a, const uint8_t *b) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    vbool32_t mask = __riscv_vmsltu_vv_u32m1_b32(va, vb, 4);
    uint16_t out = 0;
    __riscv_vsm_v_b32((uint8_t *)&out, mask, 4);
    return out & 0x0f;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmsleu_mask(const uint8_t *a, const uint8_t *b) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    vbool32_t mask = __riscv_vmsleu_vv_u32m1_b32(va, vb, 4);
    uint16_t out = 0;
    __riscv_vsm_v_b32((uint8_t *)&out, mask, 4);
    return out & 0x0f;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmsgtu_mask(const uint8_t *a, const uint8_t *b) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    vbool32_t mask = __riscv_vmsgtu_vv_u32m1_b32(va, vb, 4);
    uint16_t out = 0;
    __riscv_vsm_v_b32((uint8_t *)&out, mask, 4);
    return out & 0x0f;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmsgeu_mask(const uint8_t *a, const uint8_t *b) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    vbool32_t mask = __riscv_vmsgeu_vv_u32m1_b32(va, vb, 4);
    uint16_t out = 0;
    __riscv_vsm_v_b32((uint8_t *)&out, mask, 4);
    return out & 0x0f;
}
__attribute__((always_inline))
void rust_isa_rv_vector_vmacc(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vint32m1_t vd = rv_load_i32(acc);
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vmacc_vv_i32m1(vd, va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vnmsac(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vint32m1_t vd = rv_load_i32(acc);
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vnmsac_vv_i32m1(vd, va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vmadd(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vint32m1_t vd = rv_load_i32(acc);
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vmadd_vv_i32m1(vd, va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vnmsub(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vint32m1_t vd = rv_load_i32(acc);
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vnmsub_vv_i32m1(vd, va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vmv(
    const uint8_t *a, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    rv_store_i32(out, __riscv_vmv_v_v_i32m1(va, 4));
}

__attribute__((always_inline))
int32_t rust_isa_rv_vector_vmv_x_s(
    const uint8_t *a) {
    vint32m1_t va = rv_load_i32(a);
    return __riscv_vmv_x_s_i32m1_i32(va);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vmv_s_x(
    int32_t value, uint8_t *out) {
    vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, 4);
    rv_store_i32(out, __riscv_vmv_s_x_i32m1_tu(zero, value, 4));
}

/* Saturating, averaging, widening, and narrowing integer operations. */

__attribute__((always_inline))
void rust_isa_rv_vector_vsadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vsadd_vv_i32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vssub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vssub_vv_i32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsaddu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    rv_store_u32(out, __riscv_vsaddu_vv_u32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vssubu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    rv_store_u32(out, __riscv_vssubu_vv_u32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vaadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vaadd_vv_i32m1(va, vb, __RISCV_VXRM_RNU, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vasub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    rv_store_i32(out, __riscv_vasub_vv_i32m1(va, vb, __RISCV_VXRM_RNU, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vaaddu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    rv_store_u32(out, __riscv_vaaddu_vv_u32m1(va, vb, __RISCV_VXRM_RNU, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vasubu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t vb = rv_load_u32(b);
    rv_store_u32(out, __riscv_vasubu_vv_u32m1(va, vb, __RISCV_VXRM_RNU, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vwmacc(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vint32m1_t vd = rv_load_i32(acc);
    vint16mf2_t va = __riscv_vle16_v_i16mf2((const int16_t *)(const void *)a, 4);
    vint16mf2_t vb = __riscv_vle16_v_i16mf2((const int16_t *)(const void *)b, 4);
    rv_store_i32(out, __riscv_vwmacc_vv_i32m1(vd, va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vwmaccu(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                                uint8_t *out) {
    vuint32m1_t vd = rv_load_u32(acc);
    vuint16mf2_t va = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)a, 4);
    vuint16mf2_t vb = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)b, 4);
    rv_store_u32(out, __riscv_vwmaccu_vv_u32m1(vd, va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vwmaccsu(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                                 uint8_t *out) {
    vint32m1_t vd = rv_load_i32(acc);
    vint16mf2_t va = __riscv_vle16_v_i16mf2((const int16_t *)(const void *)a, 4);
    vuint16mf2_t vb = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)b, 4);
    rv_store_i32(out, __riscv_vwmaccsu_vv_i32m1(vd, va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vwmaccus(const uint8_t *acc, uint32_t scalar, const uint8_t *b,
                                 uint8_t *out) {
    vint32m1_t vd = rv_load_i32(acc);
    vint16mf2_t vb = __riscv_vle16_v_i16mf2((const int16_t *)(const void *)b, 4);
    rv_store_i32(out, __riscv_vwmaccus_vx_i32m1(vd, scalar, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vzext(const uint8_t *a, uint8_t *out) {
    vuint8mf2_t va = __riscv_vle8_v_u8mf2((const uint8_t *)(const void *)a, 8);
    __riscv_vse16_v_u16m1((uint16_t *)(void *)out, __riscv_vzext_vf2_u16m1(va, 8), 8);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsext(const uint8_t *a, uint8_t *out) {
    vint8mf2_t va = __riscv_vle8_v_i8mf2((const int8_t *)(const void *)a, 8);
    __riscv_vse16_v_i16m1((int16_t *)(void *)out, __riscv_vsext_vf2_i16m1(va, 8), 8);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vwadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint16mf2_t va = __riscv_vle16_v_i16mf2((const int16_t *)(const void *)a, 4);
    vint16mf2_t vb = __riscv_vle16_v_i16mf2((const int16_t *)(const void *)b, 4);
    rv_store_i32(out, __riscv_vwadd_vv_i32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vwaddu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint16mf2_t va = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)a, 4);
    vuint16mf2_t vb = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)b, 4);
    rv_store_u32(out, __riscv_vwaddu_vv_u32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vwsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint16mf2_t va = __riscv_vle16_v_i16mf2((const int16_t *)(const void *)a, 4);
    vint16mf2_t vb = __riscv_vle16_v_i16mf2((const int16_t *)(const void *)b, 4);
    rv_store_i32(out, __riscv_vwsub_vv_i32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vwsubu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint16mf2_t va = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)a, 4);
    vuint16mf2_t vb = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)b, 4);
    rv_store_u32(out, __riscv_vwsubu_vv_u32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vwmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint16mf2_t va = __riscv_vle16_v_i16mf2((const int16_t *)(const void *)a, 4);
    vint16mf2_t vb = __riscv_vle16_v_i16mf2((const int16_t *)(const void *)b, 4);
    rv_store_i32(out, __riscv_vwmul_vv_i32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vwmulu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vuint16mf2_t va = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)a, 4);
    vuint16mf2_t vb = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)b, 4);
    rv_store_u32(out, __riscv_vwmulu_vv_u32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vwmulsu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vint16mf2_t va = __riscv_vle16_v_i16mf2((const int16_t *)(const void *)a, 4);
    vuint16mf2_t vb = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)b, 4);
    rv_store_i32(out, __riscv_vwmulsu_vv_i32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vnsrl(const uint8_t *a, uint32_t shift, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vuint16mf2_t result =
        __riscv_vnsrl_wx_u16mf2(__riscv_vreinterpret_u32m1(va), shift, 4);
    uint8_t zero[16] = {0};
    __riscv_vse16_v_u16mf2((uint16_t *)(void *)zero, result, 4);
    __builtin_memcpy(out, zero, 16);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vnsra(const uint8_t *a, uint32_t shift, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vint16mf2_t result = __riscv_vnsra_wx_i16mf2(va, shift, 4);
    uint8_t zero[16] = {0};
    __riscv_vse16_v_i16mf2((int16_t *)(void *)zero, result, 4);
    __builtin_memcpy(out, zero, 16);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vnclip(const uint8_t *a, uint32_t shift, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vint16mf2_t result =
        __riscv_vnclip_wx_i16mf2(va, shift, __RISCV_VXRM_RNU, 4);
    uint8_t zero[16] = {0};
    __riscv_vse16_v_i16mf2((int16_t *)(void *)zero, result, 4);
    __builtin_memcpy(out, zero, 16);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vnclipu(const uint8_t *a, uint32_t shift, uint8_t *out) {
    vuint32m1_t va = rv_load_u32(a);
    vuint16mf2_t result =
        __riscv_vnclipu_wx_u16mf2(va, shift, __RISCV_VXRM_RNU, 4);
    uint8_t zero[16] = {0};
    __riscv_vse16_v_u16mf2((uint16_t *)(void *)zero, result, 4);
    __builtin_memcpy(out, zero, 16);
}
