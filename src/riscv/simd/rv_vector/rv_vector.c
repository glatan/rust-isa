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

/* Mask, slide, gather, compress, and reduction operations. */

__attribute__((always_inline))
void rust_isa_rv_vector_vmerge(const uint8_t *a, const uint8_t *b, uint16_t mask,
                               uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t vb = rv_load_i32(b);
    vbool32_t m = __riscv_vlm_v_b32((const uint8_t *)&mask, 4);
    rv_store_i32(out, __riscv_vmerge_vvm_i32m1(va, vb, m, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vslideup(const uint8_t *dest, const uint8_t *src, uint32_t amount,
                                 uint8_t *out) {
    vint32m1_t vd = rv_load_i32(dest);
    vint32m1_t vs = rv_load_i32(src);
    rv_store_i32(out, __riscv_vslideup_vx_i32m1(vd, vs, amount, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vslidedown(const uint8_t *src, uint32_t amount, uint8_t *out) {
    vint32m1_t vs = rv_load_i32(src);
    rv_store_i32(out, __riscv_vslidedown_vx_i32m1(vs, amount, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vslide1up(const uint8_t *src, int32_t value, uint8_t *out) {
    vint32m1_t vs = rv_load_i32(src);
    rv_store_i32(out, __riscv_vslide1up_vx_i32m1(vs, value, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vslide1down(const uint8_t *src, int32_t value, uint8_t *out) {
    vint32m1_t vs = rv_load_i32(src);
    rv_store_i32(out, __riscv_vslide1down_vx_i32m1(vs, value, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vrgather(const uint8_t *a, const uint8_t *indices, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vuint32m1_t vi = rv_load_u32(indices);
    rv_store_i32(out, __riscv_vrgather_vv_i32m1(va, vi, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vrgatherei16(const uint8_t *a, const uint8_t *indices, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vuint16mf2_t vi =
        __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)indices, 4);
    rv_store_i32(out, __riscv_vrgatherei16_vv_i32m1(va, vi, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vcompress(const uint8_t *a, uint16_t mask, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    vbool32_t m = __riscv_vlm_v_b32((const uint8_t *)&mask, 4);
    vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, 4);
    rv_store_i32(out, __riscv_vcompress_vm_i32m1_tu(zero, va, m, 4));
}

__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmand(uint16_t a, uint16_t b) {
    vbool8_t va = __riscv_vlm_v_b8((const uint8_t *)&a, 16);
    vbool8_t vb = __riscv_vlm_v_b8((const uint8_t *)&b, 16);
    vbool8_t vd = __riscv_vmand_mm_b8(va, vb, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, vd, 16);
    return out;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmnand(uint16_t a, uint16_t b) {
    vbool8_t va = __riscv_vlm_v_b8((const uint8_t *)&a, 16);
    vbool8_t vb = __riscv_vlm_v_b8((const uint8_t *)&b, 16);
    vbool8_t vd = __riscv_vmnand_mm_b8(va, vb, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, vd, 16);
    return out;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmandn(uint16_t a, uint16_t b) {
    vbool8_t va = __riscv_vlm_v_b8((const uint8_t *)&a, 16);
    vbool8_t vb = __riscv_vlm_v_b8((const uint8_t *)&b, 16);
    vbool8_t vd = __riscv_vmandn_mm_b8(va, vb, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, vd, 16);
    return out;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmxor(uint16_t a, uint16_t b) {
    vbool8_t va = __riscv_vlm_v_b8((const uint8_t *)&a, 16);
    vbool8_t vb = __riscv_vlm_v_b8((const uint8_t *)&b, 16);
    vbool8_t vd = __riscv_vmxor_mm_b8(va, vb, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, vd, 16);
    return out;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmor(uint16_t a, uint16_t b) {
    vbool8_t va = __riscv_vlm_v_b8((const uint8_t *)&a, 16);
    vbool8_t vb = __riscv_vlm_v_b8((const uint8_t *)&b, 16);
    vbool8_t vd = __riscv_vmor_mm_b8(va, vb, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, vd, 16);
    return out;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmnor(uint16_t a, uint16_t b) {
    vbool8_t va = __riscv_vlm_v_b8((const uint8_t *)&a, 16);
    vbool8_t vb = __riscv_vlm_v_b8((const uint8_t *)&b, 16);
    vbool8_t vd = __riscv_vmnor_mm_b8(va, vb, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, vd, 16);
    return out;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmorn(uint16_t a, uint16_t b) {
    vbool8_t va = __riscv_vlm_v_b8((const uint8_t *)&a, 16);
    vbool8_t vb = __riscv_vlm_v_b8((const uint8_t *)&b, 16);
    vbool8_t vd = __riscv_vmorn_mm_b8(va, vb, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, vd, 16);
    return out;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmxnor(uint16_t a, uint16_t b) {
    vbool8_t va = __riscv_vlm_v_b8((const uint8_t *)&a, 16);
    vbool8_t vb = __riscv_vlm_v_b8((const uint8_t *)&b, 16);
    vbool8_t vd = __riscv_vmxnor_mm_b8(va, vb, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, vd, 16);
    return out;
}
uint64_t rust_isa_rv_vector_vcpop(uint16_t mask) {
    vbool8_t m = __riscv_vlm_v_b8((const uint8_t *)&mask, 16);
    return (uint64_t)__riscv_vcpop_m_b8(m, 16);
}

int64_t rust_isa_rv_vector_vfirst(uint16_t mask) {
    vbool8_t m = __riscv_vlm_v_b8((const uint8_t *)&mask, 16);
    return (int64_t)__riscv_vfirst_m_b8(m, 16);
}

__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmsbf(uint16_t a) {
    vbool8_t va = __riscv_vlm_v_b8((const uint8_t *)&a, 16);
    vbool8_t vd = __riscv_vmsbf_m_b8(va, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, vd, 16);
    return out;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmsof(uint16_t a) {
    vbool8_t va = __riscv_vlm_v_b8((const uint8_t *)&a, 16);
    vbool8_t vd = __riscv_vmsof_m_b8(va, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, vd, 16);
    return out;
}
__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vmsif(uint16_t a) {
    vbool8_t va = __riscv_vlm_v_b8((const uint8_t *)&a, 16);
    vbool8_t vd = __riscv_vmsif_m_b8(va, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, vd, 16);
    return out;
}
void rust_isa_rv_vector_viota(uint16_t mask, uint8_t *out) {
    vbool8_t m = __riscv_vlm_v_b8((const uint8_t *)&mask, 16);
    __riscv_vse8_v_u8m1((uint8_t *)(void *)out, __riscv_viota_m_u8m1(m, 16), 16);
}

void rust_isa_rv_vector_vid(uint8_t *out) {
    __riscv_vse8_v_u8m1((uint8_t *)(void *)out, __riscv_vid_v_u8m1(16), 16);
}

uint32_t rust_isa_rv_vector_vredsum(const uint8_t *a) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t init = __riscv_vmv_v_x_i32m1(0, 1);
    vint32m1_t result = __riscv_vredsum_vs_i32m1_i32m1(va, init, 4);
    return (uint32_t)__riscv_vmv_x_s_i32m1_i32(result);
}

int32_t rust_isa_rv_vector_vredand(const uint8_t *a) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t init = __riscv_vmv_v_x_i32m1(-1, 1);
    vint32m1_t result = __riscv_vredand_vs_i32m1_i32m1(va, init, 4);
    return __riscv_vmv_x_s_i32m1_i32(result);
}

int32_t rust_isa_rv_vector_vredor(const uint8_t *a) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t init = __riscv_vmv_v_x_i32m1(0, 1);
    vint32m1_t result = __riscv_vredor_vs_i32m1_i32m1(va, init, 4);
    return __riscv_vmv_x_s_i32m1_i32(result);
}

int32_t rust_isa_rv_vector_vredxor(const uint8_t *a) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t init = __riscv_vmv_v_x_i32m1(0, 1);
    vint32m1_t result = __riscv_vredxor_vs_i32m1_i32m1(va, init, 4);
    return __riscv_vmv_x_s_i32m1_i32(result);
}

int32_t rust_isa_rv_vector_vredmin(const uint8_t *a) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t init = __riscv_vmv_v_x_i32m1(INT32_MAX, 1);
    vint32m1_t result = __riscv_vredmin_vs_i32m1_i32m1(va, init, 4);
    return __riscv_vmv_x_s_i32m1_i32(result);
}

int32_t rust_isa_rv_vector_vredmax(const uint8_t *a) {
    vint32m1_t va = rv_load_i32(a);
    vint32m1_t init = __riscv_vmv_v_x_i32m1(INT32_MIN, 1);
    vint32m1_t result = __riscv_vredmax_vs_i32m1_i32m1(va, init, 4);
    return __riscv_vmv_x_s_i32m1_i32(result);
}

uint32_t rust_isa_rv_vector_vredminu(const uint8_t *a) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t init = __riscv_vmv_v_x_u32m1(UINT32_MAX, 1);
    vuint32m1_t result = __riscv_vredminu_vs_u32m1_u32m1(va, init, 4);
    return __riscv_vmv_x_s_u32m1_u32(result);
}

uint32_t rust_isa_rv_vector_vredmaxu(const uint8_t *a) {
    vuint32m1_t va = rv_load_u32(a);
    vuint32m1_t init = __riscv_vmv_v_x_u32m1(0, 1);
    vuint32m1_t result = __riscv_vredmaxu_vs_u32m1_u32m1(va, init, 4);
    return __riscv_vmv_x_s_u32m1_u32(result);
}

int64_t rust_isa_rv_vector_vwredsum(const uint8_t *a) {
    vint32m1_t va = rv_load_i32(a);
    vint64m1_t init = __riscv_vmv_v_x_i64m1(0, 1);
    vint64m1_t result = __riscv_vwredsum_vs_i32m1_i64m1(va, init, 4);
    return __riscv_vmv_x_s_i64m1_i64(result);
}

uint64_t rust_isa_rv_vector_vwredsumu(const uint8_t *a) {
    vuint32m1_t va = rv_load_u32(a);
    vuint64m1_t init = __riscv_vmv_v_x_u64m1(0, 1);
    vuint64m1_t result = __riscv_vwredsumu_vs_u32m1_u64m1(va, init, 4);
    return __riscv_vmv_x_s_u64m1_u64(result);
}

/* Vector memory operations. */
__attribute__((always_inline))
void rust_isa_rv_vector_vle8(const uint8_t *p, uint8_t *out) {
    __riscv_vse8_v_i8m1((int8_t *)(void *)out,
                                __riscv_vle8_v_i8m1((const int8_t *)(const void *)p, 16), 16);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vse8(uint8_t *p, const uint8_t *value) {
    __riscv_vse8_v_i8m1((int8_t *)(void *)p,
                                __riscv_vle8_v_i8m1((const int8_t *)(const void *)value, 16), 16);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlse8(const uint8_t *p, ptrdiff_t stride, uint8_t *out) {
    __riscv_vse8_v_i8m1((int8_t *)(void *)out,
                                __riscv_vlse8_v_i8m1((const int8_t *)(const void *)p, stride, 16), 16);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsse8(uint8_t *p, ptrdiff_t stride, const uint8_t *value) {
    __riscv_vsse8_v_i8m1((int8_t *)(void *)p, stride,
                                 __riscv_vle8_v_i8m1((const int8_t *)(const void *)value, 16), 16);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vle16(const uint8_t *p, uint8_t *out) {
    __riscv_vse16_v_i16m1((int16_t *)(void *)out,
                                __riscv_vle16_v_i16m1((const int16_t *)(const void *)p, 8), 8);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vse16(uint8_t *p, const uint8_t *value) {
    __riscv_vse16_v_i16m1((int16_t *)(void *)p,
                                __riscv_vle16_v_i16m1((const int16_t *)(const void *)value, 8), 8);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlse16(const uint8_t *p, ptrdiff_t stride, uint8_t *out) {
    __riscv_vse16_v_i16m1((int16_t *)(void *)out,
                                __riscv_vlse16_v_i16m1((const int16_t *)(const void *)p, stride, 8), 8);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsse16(uint8_t *p, ptrdiff_t stride, const uint8_t *value) {
    __riscv_vsse16_v_i16m1((int16_t *)(void *)p, stride,
                                 __riscv_vle16_v_i16m1((const int16_t *)(const void *)value, 8), 8);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vle32(const uint8_t *p, uint8_t *out) {
    __riscv_vse32_v_i32m1((int32_t *)(void *)out,
                                __riscv_vle32_v_i32m1((const int32_t *)(const void *)p, 4), 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vse32(uint8_t *p, const uint8_t *value) {
    __riscv_vse32_v_i32m1((int32_t *)(void *)p,
                                __riscv_vle32_v_i32m1((const int32_t *)(const void *)value, 4), 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlse32(const uint8_t *p, ptrdiff_t stride, uint8_t *out) {
    __riscv_vse32_v_i32m1((int32_t *)(void *)out,
                                __riscv_vlse32_v_i32m1((const int32_t *)(const void *)p, stride, 4), 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsse32(uint8_t *p, ptrdiff_t stride, const uint8_t *value) {
    __riscv_vsse32_v_i32m1((int32_t *)(void *)p, stride,
                                 __riscv_vle32_v_i32m1((const int32_t *)(const void *)value, 4), 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vle64(const uint8_t *p, uint8_t *out) {
    __riscv_vse64_v_i64m1((int64_t *)(void *)out,
                                __riscv_vle64_v_i64m1((const int64_t *)(const void *)p, 2), 2);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vse64(uint8_t *p, const uint8_t *value) {
    __riscv_vse64_v_i64m1((int64_t *)(void *)p,
                                __riscv_vle64_v_i64m1((const int64_t *)(const void *)value, 2), 2);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlse64(const uint8_t *p, ptrdiff_t stride, uint8_t *out) {
    __riscv_vse64_v_i64m1((int64_t *)(void *)out,
                                __riscv_vlse64_v_i64m1((const int64_t *)(const void *)p, stride, 2), 2);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsse64(uint8_t *p, ptrdiff_t stride, const uint8_t *value) {
    __riscv_vsse64_v_i64m1((int64_t *)(void *)p, stride,
                                 __riscv_vle64_v_i64m1((const int64_t *)(const void *)value, 2), 2);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vloxei8(const uint8_t *base, const uint8_t *indices,
                                    uint8_t *out) {
    vuint8mf4_t vi = __riscv_vle8_v_u8mf4((const uint8_t *)(const void *)indices, 4);
    vint32m1_t value = __riscv_vloxei8_v_i32m1((const int32_t *)(const void *)base, vi, 4);
    __riscv_vse32_v_i32m1((int32_t *)(void *)out, value, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vluxei8(const uint8_t *base, const uint8_t *indices,
                                    uint8_t *out) {
    vuint8mf4_t vi = __riscv_vle8_v_u8mf4((const uint8_t *)(const void *)indices, 4);
    vint32m1_t value = __riscv_vluxei8_v_i32m1((const int32_t *)(const void *)base, vi, 4);
    __riscv_vse32_v_i32m1((int32_t *)(void *)out, value, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsoxei8(uint8_t *base, const uint8_t *indices,
                                    const uint8_t *value) {
    vuint8mf4_t vi = __riscv_vle8_v_u8mf4((const uint8_t *)(const void *)indices, 4);
    vint32m1_t v = rv_load_i32(value);
    __riscv_vsoxei8_v_i32m1((int32_t *)(void *)base, vi, v, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsuxei8(uint8_t *base, const uint8_t *indices,
                                    const uint8_t *value) {
    vuint8mf4_t vi = __riscv_vle8_v_u8mf4((const uint8_t *)(const void *)indices, 4);
    vint32m1_t v = rv_load_i32(value);
    __riscv_vsuxei8_v_i32m1((int32_t *)(void *)base, vi, v, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vloxei16(const uint8_t *base, const uint8_t *indices,
                                    uint8_t *out) {
    vuint16mf2_t vi = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)indices, 4);
    vint32m1_t value = __riscv_vloxei16_v_i32m1((const int32_t *)(const void *)base, vi, 4);
    __riscv_vse32_v_i32m1((int32_t *)(void *)out, value, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vluxei16(const uint8_t *base, const uint8_t *indices,
                                    uint8_t *out) {
    vuint16mf2_t vi = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)indices, 4);
    vint32m1_t value = __riscv_vluxei16_v_i32m1((const int32_t *)(const void *)base, vi, 4);
    __riscv_vse32_v_i32m1((int32_t *)(void *)out, value, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsoxei16(uint8_t *base, const uint8_t *indices,
                                    const uint8_t *value) {
    vuint16mf2_t vi = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)indices, 4);
    vint32m1_t v = rv_load_i32(value);
    __riscv_vsoxei16_v_i32m1((int32_t *)(void *)base, vi, v, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsuxei16(uint8_t *base, const uint8_t *indices,
                                    const uint8_t *value) {
    vuint16mf2_t vi = __riscv_vle16_v_u16mf2((const uint16_t *)(const void *)indices, 4);
    vint32m1_t v = rv_load_i32(value);
    __riscv_vsuxei16_v_i32m1((int32_t *)(void *)base, vi, v, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vloxei32(const uint8_t *base, const uint8_t *indices,
                                    uint8_t *out) {
    vuint32m1_t vi = __riscv_vle32_v_u32m1((const uint32_t *)(const void *)indices, 4);
    vint32m1_t value = __riscv_vloxei32_v_i32m1((const int32_t *)(const void *)base, vi, 4);
    __riscv_vse32_v_i32m1((int32_t *)(void *)out, value, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vluxei32(const uint8_t *base, const uint8_t *indices,
                                    uint8_t *out) {
    vuint32m1_t vi = __riscv_vle32_v_u32m1((const uint32_t *)(const void *)indices, 4);
    vint32m1_t value = __riscv_vluxei32_v_i32m1((const int32_t *)(const void *)base, vi, 4);
    __riscv_vse32_v_i32m1((int32_t *)(void *)out, value, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsoxei32(uint8_t *base, const uint8_t *indices,
                                    const uint8_t *value) {
    vuint32m1_t vi = __riscv_vle32_v_u32m1((const uint32_t *)(const void *)indices, 4);
    vint32m1_t v = rv_load_i32(value);
    __riscv_vsoxei32_v_i32m1((int32_t *)(void *)base, vi, v, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsuxei32(uint8_t *base, const uint8_t *indices,
                                    const uint8_t *value) {
    vuint32m1_t vi = __riscv_vle32_v_u32m1((const uint32_t *)(const void *)indices, 4);
    vint32m1_t v = rv_load_i32(value);
    __riscv_vsuxei32_v_i32m1((int32_t *)(void *)base, vi, v, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vloxei64(const uint8_t *base, const uint8_t *indices,
                                    uint8_t *out) {
    vuint64m2_t vi = __riscv_vle64_v_u64m2((const uint64_t *)(const void *)indices, 2);
    vint32m1_t value = __riscv_vloxei64_v_i32m1((const int32_t *)(const void *)base, vi, 2);
    __riscv_vse32_v_i32m1((int32_t *)(void *)out, value, 2);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vluxei64(const uint8_t *base, const uint8_t *indices,
                                    uint8_t *out) {
    vuint64m2_t vi = __riscv_vle64_v_u64m2((const uint64_t *)(const void *)indices, 2);
    vint32m1_t value = __riscv_vluxei64_v_i32m1((const int32_t *)(const void *)base, vi, 2);
    __riscv_vse32_v_i32m1((int32_t *)(void *)out, value, 2);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsoxei64(uint8_t *base, const uint8_t *indices,
                                    const uint8_t *value) {
    vuint64m2_t vi = __riscv_vle64_v_u64m2((const uint64_t *)(const void *)indices, 2);
    vint32m1_t v = rv_load_i32(value);
    __riscv_vsoxei64_v_i32m1((int32_t *)(void *)base, vi, v, 2);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsuxei64(uint8_t *base, const uint8_t *indices,
                                    const uint8_t *value) {
    vuint64m2_t vi = __riscv_vle64_v_u64m2((const uint64_t *)(const void *)indices, 2);
    vint32m1_t v = rv_load_i32(value);
    __riscv_vsuxei64_v_i32m1((int32_t *)(void *)base, vi, v, 2);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlseg2(const uint8_t *p, uint8_t *out) {
    vint32m1x2_t pair = __riscv_vlseg2e32_v_i32m1x2((const int32_t *)(const void *)p, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x2_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x2_i32m1(pair, 1));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsseg2(uint8_t *p, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1x2_t pair = __riscv_vcreate_v_i32m1x2(v0, v1);
    __riscv_vsseg2e32_v_i32m1x2((int32_t *)(void *)p, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlsseg2(const uint8_t *p, ptrdiff_t stride, uint8_t *out) {
    vint32m1x2_t pair = __riscv_vlsseg2e32_v_i32m1x2((const int32_t *)(const void *)p, stride, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x2_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x2_i32m1(pair, 1));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vssseg2(uint8_t *p, ptrdiff_t stride, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1x2_t pair = __riscv_vcreate_v_i32m1x2(v0, v1);
    __riscv_vssseg2e32_v_i32m1x2((int32_t *)(void *)p, stride, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlseg3(const uint8_t *p, uint8_t *out) {
    vint32m1x3_t pair = __riscv_vlseg3e32_v_i32m1x3((const int32_t *)(const void *)p, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x3_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x3_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x3_i32m1(pair, 2));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsseg3(uint8_t *p, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1x3_t pair = __riscv_vcreate_v_i32m1x3(v0, v1, v2);
    __riscv_vsseg3e32_v_i32m1x3((int32_t *)(void *)p, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlsseg3(const uint8_t *p, ptrdiff_t stride, uint8_t *out) {
    vint32m1x3_t pair = __riscv_vlsseg3e32_v_i32m1x3((const int32_t *)(const void *)p, stride, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x3_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x3_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x3_i32m1(pair, 2));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vssseg3(uint8_t *p, ptrdiff_t stride, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1x3_t pair = __riscv_vcreate_v_i32m1x3(v0, v1, v2);
    __riscv_vssseg3e32_v_i32m1x3((int32_t *)(void *)p, stride, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlseg4(const uint8_t *p, uint8_t *out) {
    vint32m1x4_t pair = __riscv_vlseg4e32_v_i32m1x4((const int32_t *)(const void *)p, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x4_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x4_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x4_i32m1(pair, 2));
    rv_store_i32(out + 48, __riscv_vget_v_i32m1x4_i32m1(pair, 3));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsseg4(uint8_t *p, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1_t v3 = rv_load_i32(values + 48);
    vint32m1x4_t pair = __riscv_vcreate_v_i32m1x4(v0, v1, v2, v3);
    __riscv_vsseg4e32_v_i32m1x4((int32_t *)(void *)p, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlsseg4(const uint8_t *p, ptrdiff_t stride, uint8_t *out) {
    vint32m1x4_t pair = __riscv_vlsseg4e32_v_i32m1x4((const int32_t *)(const void *)p, stride, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x4_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x4_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x4_i32m1(pair, 2));
    rv_store_i32(out + 48, __riscv_vget_v_i32m1x4_i32m1(pair, 3));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vssseg4(uint8_t *p, ptrdiff_t stride, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1_t v3 = rv_load_i32(values + 48);
    vint32m1x4_t pair = __riscv_vcreate_v_i32m1x4(v0, v1, v2, v3);
    __riscv_vssseg4e32_v_i32m1x4((int32_t *)(void *)p, stride, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlseg5(const uint8_t *p, uint8_t *out) {
    vint32m1x5_t pair = __riscv_vlseg5e32_v_i32m1x5((const int32_t *)(const void *)p, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x5_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x5_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x5_i32m1(pair, 2));
    rv_store_i32(out + 48, __riscv_vget_v_i32m1x5_i32m1(pair, 3));
    rv_store_i32(out + 64, __riscv_vget_v_i32m1x5_i32m1(pair, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsseg5(uint8_t *p, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1_t v3 = rv_load_i32(values + 48);
    vint32m1_t v4 = rv_load_i32(values + 64);
    vint32m1x5_t pair = __riscv_vcreate_v_i32m1x5(v0, v1, v2, v3, v4);
    __riscv_vsseg5e32_v_i32m1x5((int32_t *)(void *)p, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlsseg5(const uint8_t *p, ptrdiff_t stride, uint8_t *out) {
    vint32m1x5_t pair = __riscv_vlsseg5e32_v_i32m1x5((const int32_t *)(const void *)p, stride, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x5_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x5_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x5_i32m1(pair, 2));
    rv_store_i32(out + 48, __riscv_vget_v_i32m1x5_i32m1(pair, 3));
    rv_store_i32(out + 64, __riscv_vget_v_i32m1x5_i32m1(pair, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vssseg5(uint8_t *p, ptrdiff_t stride, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1_t v3 = rv_load_i32(values + 48);
    vint32m1_t v4 = rv_load_i32(values + 64);
    vint32m1x5_t pair = __riscv_vcreate_v_i32m1x5(v0, v1, v2, v3, v4);
    __riscv_vssseg5e32_v_i32m1x5((int32_t *)(void *)p, stride, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlseg6(const uint8_t *p, uint8_t *out) {
    vint32m1x6_t pair = __riscv_vlseg6e32_v_i32m1x6((const int32_t *)(const void *)p, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x6_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x6_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x6_i32m1(pair, 2));
    rv_store_i32(out + 48, __riscv_vget_v_i32m1x6_i32m1(pair, 3));
    rv_store_i32(out + 64, __riscv_vget_v_i32m1x6_i32m1(pair, 4));
    rv_store_i32(out + 80, __riscv_vget_v_i32m1x6_i32m1(pair, 5));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsseg6(uint8_t *p, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1_t v3 = rv_load_i32(values + 48);
    vint32m1_t v4 = rv_load_i32(values + 64);
    vint32m1_t v5 = rv_load_i32(values + 80);
    vint32m1x6_t pair = __riscv_vcreate_v_i32m1x6(v0, v1, v2, v3, v4, v5);
    __riscv_vsseg6e32_v_i32m1x6((int32_t *)(void *)p, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlsseg6(const uint8_t *p, ptrdiff_t stride, uint8_t *out) {
    vint32m1x6_t pair = __riscv_vlsseg6e32_v_i32m1x6((const int32_t *)(const void *)p, stride, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x6_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x6_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x6_i32m1(pair, 2));
    rv_store_i32(out + 48, __riscv_vget_v_i32m1x6_i32m1(pair, 3));
    rv_store_i32(out + 64, __riscv_vget_v_i32m1x6_i32m1(pair, 4));
    rv_store_i32(out + 80, __riscv_vget_v_i32m1x6_i32m1(pair, 5));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vssseg6(uint8_t *p, ptrdiff_t stride, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1_t v3 = rv_load_i32(values + 48);
    vint32m1_t v4 = rv_load_i32(values + 64);
    vint32m1_t v5 = rv_load_i32(values + 80);
    vint32m1x6_t pair = __riscv_vcreate_v_i32m1x6(v0, v1, v2, v3, v4, v5);
    __riscv_vssseg6e32_v_i32m1x6((int32_t *)(void *)p, stride, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlseg7(const uint8_t *p, uint8_t *out) {
    vint32m1x7_t pair = __riscv_vlseg7e32_v_i32m1x7((const int32_t *)(const void *)p, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x7_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x7_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x7_i32m1(pair, 2));
    rv_store_i32(out + 48, __riscv_vget_v_i32m1x7_i32m1(pair, 3));
    rv_store_i32(out + 64, __riscv_vget_v_i32m1x7_i32m1(pair, 4));
    rv_store_i32(out + 80, __riscv_vget_v_i32m1x7_i32m1(pair, 5));
    rv_store_i32(out + 96, __riscv_vget_v_i32m1x7_i32m1(pair, 6));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsseg7(uint8_t *p, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1_t v3 = rv_load_i32(values + 48);
    vint32m1_t v4 = rv_load_i32(values + 64);
    vint32m1_t v5 = rv_load_i32(values + 80);
    vint32m1_t v6 = rv_load_i32(values + 96);
    vint32m1x7_t pair = __riscv_vcreate_v_i32m1x7(v0, v1, v2, v3, v4, v5, v6);
    __riscv_vsseg7e32_v_i32m1x7((int32_t *)(void *)p, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlsseg7(const uint8_t *p, ptrdiff_t stride, uint8_t *out) {
    vint32m1x7_t pair = __riscv_vlsseg7e32_v_i32m1x7((const int32_t *)(const void *)p, stride, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x7_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x7_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x7_i32m1(pair, 2));
    rv_store_i32(out + 48, __riscv_vget_v_i32m1x7_i32m1(pair, 3));
    rv_store_i32(out + 64, __riscv_vget_v_i32m1x7_i32m1(pair, 4));
    rv_store_i32(out + 80, __riscv_vget_v_i32m1x7_i32m1(pair, 5));
    rv_store_i32(out + 96, __riscv_vget_v_i32m1x7_i32m1(pair, 6));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vssseg7(uint8_t *p, ptrdiff_t stride, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1_t v3 = rv_load_i32(values + 48);
    vint32m1_t v4 = rv_load_i32(values + 64);
    vint32m1_t v5 = rv_load_i32(values + 80);
    vint32m1_t v6 = rv_load_i32(values + 96);
    vint32m1x7_t pair = __riscv_vcreate_v_i32m1x7(v0, v1, v2, v3, v4, v5, v6);
    __riscv_vssseg7e32_v_i32m1x7((int32_t *)(void *)p, stride, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlseg8(const uint8_t *p, uint8_t *out) {
    vint32m1x8_t pair = __riscv_vlseg8e32_v_i32m1x8((const int32_t *)(const void *)p, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x8_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x8_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x8_i32m1(pair, 2));
    rv_store_i32(out + 48, __riscv_vget_v_i32m1x8_i32m1(pair, 3));
    rv_store_i32(out + 64, __riscv_vget_v_i32m1x8_i32m1(pair, 4));
    rv_store_i32(out + 80, __riscv_vget_v_i32m1x8_i32m1(pair, 5));
    rv_store_i32(out + 96, __riscv_vget_v_i32m1x8_i32m1(pair, 6));
    rv_store_i32(out + 112, __riscv_vget_v_i32m1x8_i32m1(pair, 7));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsseg8(uint8_t *p, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1_t v3 = rv_load_i32(values + 48);
    vint32m1_t v4 = rv_load_i32(values + 64);
    vint32m1_t v5 = rv_load_i32(values + 80);
    vint32m1_t v6 = rv_load_i32(values + 96);
    vint32m1_t v7 = rv_load_i32(values + 112);
    vint32m1x8_t pair = __riscv_vcreate_v_i32m1x8(v0, v1, v2, v3, v4, v5, v6, v7);
    __riscv_vsseg8e32_v_i32m1x8((int32_t *)(void *)p, pair, 4);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vlsseg8(const uint8_t *p, ptrdiff_t stride, uint8_t *out) {
    vint32m1x8_t pair = __riscv_vlsseg8e32_v_i32m1x8((const int32_t *)(const void *)p, stride, 4);
    rv_store_i32(out + 0, __riscv_vget_v_i32m1x8_i32m1(pair, 0));
    rv_store_i32(out + 16, __riscv_vget_v_i32m1x8_i32m1(pair, 1));
    rv_store_i32(out + 32, __riscv_vget_v_i32m1x8_i32m1(pair, 2));
    rv_store_i32(out + 48, __riscv_vget_v_i32m1x8_i32m1(pair, 3));
    rv_store_i32(out + 64, __riscv_vget_v_i32m1x8_i32m1(pair, 4));
    rv_store_i32(out + 80, __riscv_vget_v_i32m1x8_i32m1(pair, 5));
    rv_store_i32(out + 96, __riscv_vget_v_i32m1x8_i32m1(pair, 6));
    rv_store_i32(out + 112, __riscv_vget_v_i32m1x8_i32m1(pair, 7));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vssseg8(uint8_t *p, ptrdiff_t stride, const uint8_t *values) {
    vint32m1_t v0 = rv_load_i32(values + 0);
    vint32m1_t v1 = rv_load_i32(values + 16);
    vint32m1_t v2 = rv_load_i32(values + 32);
    vint32m1_t v3 = rv_load_i32(values + 48);
    vint32m1_t v4 = rv_load_i32(values + 64);
    vint32m1_t v5 = rv_load_i32(values + 80);
    vint32m1_t v6 = rv_load_i32(values + 96);
    vint32m1_t v7 = rv_load_i32(values + 112);
    vint32m1x8_t pair = __riscv_vcreate_v_i32m1x8(v0, v1, v2, v3, v4, v5, v6, v7);
    __riscv_vssseg8e32_v_i32m1x8((int32_t *)(void *)p, stride, pair, 4);
}

__attribute__((always_inline))
uint16_t rust_isa_rv_vector_vlm(const uint8_t *p) {
    vbool8_t m = __riscv_vlm_v_b8(p, 16);
    uint16_t out = 0;
    __riscv_vsm_v_b8((uint8_t *)&out, m, 16);
    return out;
}

__attribute__((always_inline))
void rust_isa_rv_vector_vsm(uint8_t *p, uint16_t mask) {
    vbool8_t m = __riscv_vlm_v_b8((const uint8_t *)&mask, 16);
    __riscv_vsm_v_b8(p, m, 16);
}

/* Floating-point vector operations. */

static inline vfloat32m1_t rv_load_f32(const uint8_t *p) {
    return __riscv_vle32_v_f32m1((const float *)(const void *)p, 4);
}

static inline void rv_store_f32(uint8_t *p, vfloat32m1_t value) {
    __riscv_vse32_v_f32m1((float *)(void *)p, value, 4);
}

static inline vfloat64m1_t rv_load_f64(const uint8_t *p) {
    return __riscv_vle64_v_f64m1((const double *)(const void *)p, 2);
}

static inline void rv_store_f64(uint8_t *p, vfloat64m1_t value) {
    __riscv_vse64_v_f64m1((double *)(void *)p, value, 2);
}

static inline vfloat32mf2_t rv_load_f32_half(const uint8_t *p) {
    return __riscv_vle32_v_f32mf2((const float *)(const void *)p, 2);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfrdiv(const uint8_t *a, float scalar, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    rv_store_f32(out, __riscv_vfrdiv_vf_f32m1(va, scalar, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfmin(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t vb = rv_load_f32(b);
    rv_store_f32(out, __riscv_vfmin_vv_f32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfmax(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t vb = rv_load_f32(b);
    rv_store_f32(out, __riscv_vfmax_vv_f32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfsgnj(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t vb = rv_load_f32(b);
    rv_store_f32(out, __riscv_vfsgnj_vv_f32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfsgnjn(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t vb = rv_load_f32(b);
    rv_store_f32(out, __riscv_vfsgnjn_vv_f32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfsgnjx(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t vb = rv_load_f32(b);
    rv_store_f32(out, __riscv_vfsgnjx_vv_f32m1(va, vb, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfabs(const uint8_t *a, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    rv_store_f32(out, __riscv_vfabs_v_f32m1(va, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfneg(const uint8_t *a, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    rv_store_f32(out, __riscv_vfneg_v_f32m1(va, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfsqrt(const uint8_t *a, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    rv_store_f32(out, __riscv_vfsqrt_v_f32m1(va, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfrsqrt7(const uint8_t *a, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    rv_store_f32(out, __riscv_vfrsqrt7_v_f32m1(va, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfrec7(const uint8_t *a, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    rv_store_f32(out, __riscv_vfrec7_v_f32m1(va, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfmerge(const uint8_t *a, float scalar, uint16_t mask, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    vbool32_t m = __riscv_vlm_v_b32((const uint8_t *)&mask, 4);
    rv_store_f32(out, __riscv_vfmerge_vfm_f32m1(va, scalar, m, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfmv(float scalar, uint8_t *out) {
    rv_store_f32(out, __riscv_vfmv_v_f_f32m1(scalar, 4));
}

__attribute__((always_inline))
float rust_isa_rv_vector_vfmv_f_s(const uint8_t *a) {
    vfloat32m1_t va = rv_load_f32(a);
    return __riscv_vfmv_f_s_f32m1_f32(va);
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfma(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vfloat32m1_t vd = rv_load_f32(acc);
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t vb = rv_load_f32(b);
    rv_store_f32(out, __riscv_vfmadd_vv_f32m1(vd, va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vfms(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vfloat32m1_t vd = rv_load_f32(acc);
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t vb = rv_load_f32(b);
    rv_store_f32(out, __riscv_vfmsub_vv_f32m1(vd, va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vfnma(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vfloat32m1_t vd = rv_load_f32(acc);
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t vb = rv_load_f32(b);
    rv_store_f32(out, __riscv_vfnmadd_vv_f32m1(vd, va, vb, 4));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vfnms(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vfloat32m1_t vd = rv_load_f32(acc);
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t vb = rv_load_f32(b);
    rv_store_f32(out, __riscv_vfnmsub_vv_f32m1(vd, va, vb, 4));
}
__attribute__((always_inline))
float rust_isa_rv_vector_vfredsum(const uint8_t *a) {
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t init = __riscv_vfmv_v_f_f32m1(0.0f, 1);
    vfloat32m1_t result = __riscv_vfredusum_vs_f32m1_f32m1(va, init, 4);
    return __riscv_vfmv_f_s_f32m1_f32(result);
}
__attribute__((always_inline))
float rust_isa_rv_vector_vfredusum(const uint8_t *a) {
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t init = __riscv_vfmv_v_f_f32m1(0.0f, 1);
    vfloat32m1_t result = __riscv_vfredusum_vs_f32m1_f32m1(va, init, 4);
    return __riscv_vfmv_f_s_f32m1_f32(result);
}
__attribute__((always_inline))
float rust_isa_rv_vector_vfredosum(const uint8_t *a) {
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t init = __riscv_vfmv_v_f_f32m1(0.0f, 1);
    vfloat32m1_t result = __riscv_vfredosum_vs_f32m1_f32m1(va, init, 4);
    return __riscv_vfmv_f_s_f32m1_f32(result);
}
__attribute__((always_inline))
float rust_isa_rv_vector_vfredmax(const uint8_t *a) {
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t init = __riscv_vfmv_v_f_f32m1(-__builtin_inff(), 1);
    vfloat32m1_t result = __riscv_vfredmax_vs_f32m1_f32m1(va, init, 4);
    return __riscv_vfmv_f_s_f32m1_f32(result);
}
__attribute__((always_inline))
float rust_isa_rv_vector_vfredmin(const uint8_t *a) {
    vfloat32m1_t va = rv_load_f32(a);
    vfloat32m1_t init = __riscv_vfmv_v_f_f32m1(__builtin_inff(), 1);
    vfloat32m1_t result = __riscv_vfredmin_vs_f32m1_f32m1(va, init, 4);
    return __riscv_vfmv_f_s_f32m1_f32(result);
}
__attribute__((always_inline))
void rust_isa_rv_vector_vfwadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vfloat32mf2_t va = rv_load_f32_half(a);
    vfloat32mf2_t vb = rv_load_f32_half(b);
    rv_store_f64(out, __riscv_vfwadd_vv_f64m1(va, vb, 2));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfwsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vfloat32mf2_t va = rv_load_f32_half(a);
    vfloat32mf2_t vb = rv_load_f32_half(b);
    rv_store_f64(out, __riscv_vfwsub_vv_f64m1(va, vb, 2));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfwmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vfloat32mf2_t va = rv_load_f32_half(a);
    vfloat32mf2_t vb = rv_load_f32_half(b);
    rv_store_f64(out, __riscv_vfwmul_vv_f64m1(va, vb, 2));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfwmacc(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vfloat64m1_t vd = rv_load_f64(acc);
    vfloat32mf2_t va = rv_load_f32_half(a);
    vfloat32mf2_t vb = rv_load_f32_half(b);
    rv_store_f64(out, __riscv_vfwmacc_vv_f64m1(vd, va, vb, 2));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vfwnmacc(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vfloat64m1_t vd = rv_load_f64(acc);
    vfloat32mf2_t va = rv_load_f32_half(a);
    vfloat32mf2_t vb = rv_load_f32_half(b);
    rv_store_f64(out, __riscv_vfwnmacc_vv_f64m1(vd, va, vb, 2));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vfwmsac(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vfloat64m1_t vd = rv_load_f64(acc);
    vfloat32mf2_t va = rv_load_f32_half(a);
    vfloat32mf2_t vb = rv_load_f32_half(b);
    rv_store_f64(out, __riscv_vfwmsac_vv_f64m1(vd, va, vb, 2));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vfwnmsac(const uint8_t *acc, const uint8_t *a, const uint8_t *b,
                               uint8_t *out) {
    vfloat64m1_t vd = rv_load_f64(acc);
    vfloat32mf2_t va = rv_load_f32_half(a);
    vfloat32mf2_t vb = rv_load_f32_half(b);
    rv_store_f64(out, __riscv_vfwnmsac_vv_f64m1(vd, va, vb, 2));
}
__attribute__((always_inline))
void rust_isa_rv_vector_vfclass(const uint8_t *a, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    rv_store_u32(out, __riscv_vfclass_v_u32m1(va, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfcvt_xu_f(const uint8_t *a, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    rv_store_u32(out, __riscv_vfcvt_xu_f_v_u32m1(va, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfcvt_x_f(const uint8_t *a, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    rv_store_i32(out, __riscv_vfcvt_x_f_v_i32m1(va, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfcvt_rtz_xu_f(const uint8_t *a, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    rv_store_u32(out, __riscv_vfcvt_rtz_xu_f_v_u32m1(va, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfcvt_rtz_x_f(const uint8_t *a, uint8_t *out) {
    vfloat32m1_t va = rv_load_f32(a);
    rv_store_i32(out, __riscv_vfcvt_rtz_x_f_v_i32m1(va, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfcvt_f_xu(const uint8_t *a, uint8_t *out) {
    vuint32m1_t va = rv_load_u32(a);
    rv_store_f32(out, __riscv_vfcvt_f_xu_v_f32m1(va, 4));
}

__attribute__((always_inline))
void rust_isa_rv_vector_vfcvt_f_x(const uint8_t *a, uint8_t *out) {
    vint32m1_t va = rv_load_i32(a);
    rv_store_f32(out, __riscv_vfcvt_f_x_v_f32m1(va, 4));
}

/* Vector cryptography (Zvkned, Zvknh, Zvksed, Zvksh, Zvkg, Zvbc). */

#define RV_CRYPTO __attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vaesef(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vaesef.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}
__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vaesem(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vaesem.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}
__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vaesdf(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vaesdf.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}
__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vaesdm(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vaesdm.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}
__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vaesz(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vaesz.vs v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}

#define RV_AES_KEY1_CASE(r)                                                    \
    case r:                                                                    \
        __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"               \
                         "vle32.v v2, (%0)\n\t"                                \
                         "vaeskf1.vi v2, v2, " #r "\n\t"                       \
                         "vse32.v v2, (%1)"                                    \
                         ::"r"(key), "r"(out)                                  \
                         : "v2", "memory");                                    \
        break;

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vaeskf1(const uint8_t *key, uint32_t round, uint8_t *out) {
    switch (round) {
        RV_AES_KEY1_CASE(0)
        RV_AES_KEY1_CASE(1)
        RV_AES_KEY1_CASE(2)
        RV_AES_KEY1_CASE(3)
        RV_AES_KEY1_CASE(4)
        RV_AES_KEY1_CASE(5)
        RV_AES_KEY1_CASE(6)
        RV_AES_KEY1_CASE(7)
        RV_AES_KEY1_CASE(8)
        RV_AES_KEY1_CASE(9)
        RV_AES_KEY1_CASE(10)
    default:
        break;
    }
}

#define RV_AES_KEY2_CASE(r)                                                    \
    case r:                                                                    \
        __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"               \
                         "vle32.v v2, (%0)\n\t"                                \
                         "vaeskf2.vi v2, v2, " #r "\n\t"                       \
                         "vse32.v v2, (%1)"                                    \
                         ::"r"(key), "r"(out)                                  \
                         : "v2", "memory");                                    \
        break;

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vaeskf2(const uint8_t *key, uint32_t round, uint8_t *out) {
    switch (round) {
        RV_AES_KEY2_CASE(2)
        RV_AES_KEY2_CASE(3)
        RV_AES_KEY2_CASE(4)
        RV_AES_KEY2_CASE(5)
        RV_AES_KEY2_CASE(6)
        RV_AES_KEY2_CASE(7)
        RV_AES_KEY2_CASE(8)
        RV_AES_KEY2_CASE(9)
        RV_AES_KEY2_CASE(10)
    default:
        break;
    }
}

#define RV_SM4K_CASE(r)                                                        \
    case r:                                                                    \
        __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"               \
                         "vle32.v v2, (%0)\n\t"                                \
                         "vsm4k.vi v2, v2, " #r "\n\t"                         \
                         "vse32.v v2, (%1)"                                    \
                         ::"r"(key), "r"(out)                                  \
                         : "v2", "memory");                                    \
        break;

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vsm4k(const uint8_t *key, uint32_t round, uint8_t *out) {
    switch (round) {
        RV_SM4K_CASE(0)
        RV_SM4K_CASE(1)
        RV_SM4K_CASE(2)
        RV_SM4K_CASE(3)
        RV_SM4K_CASE(4)
        RV_SM4K_CASE(5)
        RV_SM4K_CASE(6)
        RV_SM4K_CASE(7)
    default:
        break;
    }
}

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vsm4r(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vsm4r.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vsha2ms(const uint8_t *a, const uint8_t *b, const uint8_t *c,
                               uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vle32.v v4, (%2)\n\t"
                     "vsha2ms.vv v2, v3, v4\n\t"
                     "vse32.v v2, (%3)"
                     ::"r"(a), "r"(b), "r"(c), "r"(out)
                     : "v2", "v3", "v4", "memory");
}
__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vsha2ch(const uint8_t *a, const uint8_t *b, const uint8_t *c,
                               uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vle32.v v4, (%2)\n\t"
                     "vsha2ch.vv v2, v3, v4\n\t"
                     "vse32.v v2, (%3)"
                     ::"r"(a), "r"(b), "r"(c), "r"(out)
                     : "v2", "v3", "v4", "memory");
}
__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vsha2cl(const uint8_t *a, const uint8_t *b, const uint8_t *c,
                               uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vle32.v v4, (%2)\n\t"
                     "vsha2cl.vv v2, v3, v4\n\t"
                     "vse32.v v2, (%3)"
                     ::"r"(a), "r"(b), "r"(c), "r"(out)
                     : "v2", "v3", "v4", "memory");
}
__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vsm3me(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 8, e32, m2, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v4, (%1)\n\t"
                     "vsm3me.vv v6, v2, v4\n\t"
                     "vse32.v v6, (%2)"
                     ::"r"(a), "r"(b), "r"(out)
                     : "v2", "v3", "v4", "v5", "v6", "v7", "memory");
}

#define RV_SM3C_CASE(r)                                                        \
    case r:                                                                    \
        __asm__ volatile("vsetivli zero, 8, e32, m2, ta, ma\n\t"               \
                         "vle32.v v2, (%0)\n\t"                                \
                         "vsm3c.vi v4, v2, " #r "\n\t"                         \
                         "vse32.v v4, (%1)"                                    \
                         ::"r"(state), "r"(out)                                \
                         : "v2", "v3", "v4", "v5", "memory");                  \
        break;

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vsm3c(const uint8_t *state, uint32_t round, uint8_t *out) {
    switch (round) {
        RV_SM3C_CASE(0)
        RV_SM3C_CASE(1)
        RV_SM3C_CASE(2)
        RV_SM3C_CASE(3)
        RV_SM3C_CASE(4)
        RV_SM3C_CASE(5)
        RV_SM3C_CASE(6)
        RV_SM3C_CASE(7)
        RV_SM3C_CASE(8)
        RV_SM3C_CASE(9)
        RV_SM3C_CASE(10)
        RV_SM3C_CASE(11)
        RV_SM3C_CASE(12)
        RV_SM3C_CASE(13)
        RV_SM3C_CASE(14)
        RV_SM3C_CASE(15)
        RV_SM3C_CASE(16)
        RV_SM3C_CASE(17)
        RV_SM3C_CASE(18)
        RV_SM3C_CASE(19)
        RV_SM3C_CASE(20)
        RV_SM3C_CASE(21)
        RV_SM3C_CASE(22)
        RV_SM3C_CASE(23)
        RV_SM3C_CASE(24)
        RV_SM3C_CASE(25)
        RV_SM3C_CASE(26)
        RV_SM3C_CASE(27)
        RV_SM3C_CASE(28)
        RV_SM3C_CASE(29)
        RV_SM3C_CASE(30)
        RV_SM3C_CASE(31)
    default:
        break;
    }
}

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vclmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vle64.v v3, (%1)\n\t"
                     "vclmul.vv v4, v2, v3\n\t"
                     "vse64.v v4, (%2)"
                     ::"r"(a), "r"(b), "r"(out)
                     : "v2", "v3", "v4", "memory");
}
__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vclmulh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vle64.v v3, (%1)\n\t"
                     "vclmulh.vv v4, v2, v3\n\t"
                     "vse64.v v4, (%2)"
                     ::"r"(a), "r"(b), "r"(out)
                     : "v2", "v3", "v4", "memory");
}
__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vghsh(const uint8_t *acc, const uint8_t *value, const uint8_t *key,
                              uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vle32.v v4, (%2)\n\t"
                     "vghsh.vv v2, v3, v4\n\t"
                     "vse32.v v2, (%3)"
                     ::"r"(acc), "r"(value), "r"(key), "r"(out)
                     : "v2", "v3", "v4", "memory");
}

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rv_vector_vgmul(const uint8_t *acc, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vgmul.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(acc), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}
