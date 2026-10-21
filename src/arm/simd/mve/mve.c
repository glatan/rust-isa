#include <stdint.h>
#include <arm_mve.h>

__attribute__((always_inline))
static inline int32x4_t load_s32(const uint8_t *p) {
    return vld1q_s32((const int32_t *)(const void *)p);
}

__attribute__((always_inline))
static inline void store_s32(uint8_t *out, int32x4_t value) {
    vst1q_s32((int32_t *)(void *)out, value);
}

__attribute__((always_inline))
static inline uint32x4_t load_u32(const uint8_t *p) {
    return vld1q_u32((const uint32_t *)(const void *)p);
}

__attribute__((always_inline))
static inline void store_u32(uint8_t *out, uint32x4_t value) {
    vst1q_u32((uint32_t *)(void *)out, value);
}

__attribute__((always_inline))
void rust_isa_arm_mve_ldr(const uint8_t *p, uint8_t *out) {
    store_u32(out, vld1q_u32((const uint32_t *)(const void *)p));
}

__attribute__((always_inline))
void rust_isa_arm_mve_ldrb(const uint8_t *p, uint8_t *out) {
    vst1q_u8(out, vld1q_u8(p));
}

__attribute__((always_inline))
void rust_isa_arm_mve_str(uint8_t *p, const uint8_t *value) {
    vst1q_u32((uint32_t *)(void *)p, vld1q_u32((const uint32_t *)(const void *)value));
}

__attribute__((always_inline))
void rust_isa_arm_mve_strb(uint8_t *p, const uint8_t *value) {
    vst1q_u8(p, vld1q_u8(value));
}

__attribute__((always_inline))
void rust_isa_arm_mve_abs(const uint8_t *a, uint8_t *out) {
    store_s32(out, __arm_vabsq_s32(load_s32(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, vaddq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
int64_t rust_isa_arm_mve_addlv(const uint8_t *a) {
    return __arm_vaddlvq_s32(load_s32(a));
}

__attribute__((always_inline))
int32_t rust_isa_arm_mve_addv(const uint8_t *a) {
    return __arm_vaddvq_s32(load_s32(a));
}

__attribute__((always_inline))
void rust_isa_arm_mve_and(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_u32(out, vandq_u32(load_u32(a), load_u32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_abd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vabdq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
uint32_t rust_isa_arm_mve_abav(uint32_t acc, const uint8_t *a, const uint8_t *b) {
    return __arm_vabavq_s8(acc, vld1q_s8((const int8_t *)(const void *)a),
                           vld1q_s8((const int8_t *)(const void *)b));
}

__attribute__((always_inline))
void rust_isa_arm_mve_bic(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_u32(out, vbicq_u32(load_u32(a), load_u32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_brsr(const uint8_t *a, uint32_t bits, uint8_t *out) {
    store_s32(out, __arm_vbrsrq_n_s32(load_s32(a), (int32_t)bits));
}

__attribute__((always_inline))
void rust_isa_arm_mve_clz(const uint8_t *a, uint8_t *out) {
    store_s32(out, __arm_vclzq_s32(load_s32(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_cmp(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    const mve_pred16_t predicate = __arm_vcmpeqq_s32(load_s32(a), load_s32(b));
    store_s32(out, __arm_vpselq_s32(vdupq_n_s32(-1), vdupq_n_s32(0), predicate));
}

__attribute__((always_inline))
uint16_t rust_isa_arm_mve_ctp(uint32_t count) {
    return (uint16_t)__arm_vctp8q(count);
}

__attribute__((always_inline))
void rust_isa_arm_mve_dup(int32_t value, uint8_t *out) {
    store_s32(out, vdupq_n_s32(value));
}

__attribute__((always_inline))
void rust_isa_arm_mve_eor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_u32(out, veorq_u32(load_u32(a), load_u32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_hadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vhaddq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_hsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vhsubq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_idup(uint32_t base, uint32_t step, uint8_t *out) {
    uint32x4_t value;
    switch (step) {
    case 1:
        value = __arm_vidupq_n_u32(base, 1);
        break;
    case 2:
        value = __arm_vidupq_n_u32(base, 2);
        break;
    case 4:
        value = __arm_vidupq_n_u32(base, 4);
        break;
    case 8:
        value = __arm_vidupq_n_u32(base, 8);
        break;
    default:
        value = vdupq_n_u32(0);
        break;
    }
    store_u32(out, value);
}

__attribute__((always_inline))
void rust_isa_arm_mve_iwdup(uint32_t base, uint32_t wrap, uint32_t step, uint8_t *out) {
    uint32x4_t value;
    switch (step) {
    case 1:
        value = __arm_viwdupq_n_u32(base, wrap, 1);
        break;
    case 2:
        value = __arm_viwdupq_n_u32(base, wrap, 2);
        break;
    case 4:
        value = __arm_viwdupq_n_u32(base, wrap, 4);
        break;
    case 8:
        value = __arm_viwdupq_n_u32(base, wrap, 8);
        break;
    default:
        value = vdupq_n_u32(0);
        break;
    }
    store_u32(out, value);
}

__attribute__((always_inline))
void rust_isa_arm_mve_max(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vmaxq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
int32_t rust_isa_arm_mve_maxv(const uint8_t *a) {
    return __arm_vmaxvq_s32(-2147483647 - 1, load_s32(a));
}

__attribute__((always_inline))
void rust_isa_arm_mve_min(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vminq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
int32_t rust_isa_arm_mve_minv(const uint8_t *a) {
    return __arm_vminvq_s32(2147483647, load_s32(a));
}

__attribute__((always_inline))
void rust_isa_arm_mve_mla(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, vaddq_s32(load_s32(acc), vmulq_s32(load_s32(a), load_s32(b))));
}

__attribute__((always_inline))
void rust_isa_arm_mve_mlal(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    const int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    const int16x8_t vb = vld1q_s16((const int16_t *)(const void *)b);
    store_s32(out, vaddq_s32(load_s32(acc), __arm_vmullbq_int_s16(va, vb)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_mls(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, vsubq_s32(load_s32(acc), vmulq_s32(load_s32(a), load_s32(b))));
}

__attribute__((always_inline))
void rust_isa_arm_mve_mlsl(const uint8_t *acc, const uint8_t *a, const uint8_t *b, uint8_t *out) {
    const int16x8_t va = vld1q_s16((const int16_t *)(const void *)a);
    const int16x8_t vb = vld1q_s16((const int16_t *)(const void *)b);
    store_s32(out, vsubq_s32(load_s32(acc), __arm_vmullbq_int_s16(va, vb)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_mov(const uint8_t *a, uint8_t *out) {
    store_u32(out, load_u32(a));
}

__attribute__((always_inline))
void rust_isa_arm_mve_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, vmulq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_mull(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vmullbq_int_s16(vld1q_s16((const int16_t *)(const void *)a),
                                         vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_mvn(const uint8_t *a, uint8_t *out) {
    store_u32(out, vmvnq_u32(load_u32(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_neg(const uint8_t *a, uint8_t *out) {
    store_s32(out, __arm_vnegq_s32(load_s32(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_orn(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_u32(out, vornq_u32(load_u32(a), load_u32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_orr(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_u32(out, vorrq_u32(load_u32(a), load_u32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_qabs(const uint8_t *a, uint8_t *out) {
    store_s32(out, __arm_vqabsq_s32(load_s32(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_qadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vqaddq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_qdmulh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vqdmulhq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_qdmull(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vqdmullbq_s16(vld1q_s16((const int16_t *)(const void *)a),
                                       vld1q_s16((const int16_t *)(const void *)b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_qmovn(const uint8_t *a, uint8_t *out) {
    vst1q_s16((int16_t *)(void *)out,
              __arm_vqmovnbq_s32(vdupq_n_s16(0), load_s32(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_qneg(const uint8_t *a, uint8_t *out) {
    store_s32(out, __arm_vqnegq_s32(load_s32(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_qrdmulh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vqrdmulhq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_qshl(const uint8_t *a, uint32_t count, uint8_t *out) {
    store_s32(out, __arm_vqshlq_s32(load_s32(a), vdupq_n_s32((int32_t)count)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_qsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vqsubq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_rev16(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, __arm_vrev16q_u8(vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_rev32(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, __arm_vrev32q_u8(vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_rev64(const uint8_t *a, uint8_t *out) {
    vst1q_u8(out, __arm_vrev64q_u8(vld1q_u8(a)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_rmulh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, __arm_vrmulhq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_shl(const uint8_t *a, uint32_t count, uint8_t *out) {
    store_s32(out, __arm_vshlq_s32(load_s32(a), vdupq_n_s32((int32_t)count)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_shr(const uint8_t *a, uint32_t count, uint8_t *out) {
    store_s32(out, __arm_vshlq_s32(load_s32(a), vdupq_n_s32(-(int32_t)count)));
}

#define VSLI_CASES \
    case 0: return __arm_vsliq_n_s32(x, y, 0); \
    case 1: return __arm_vsliq_n_s32(x, y, 1); \
    case 2: return __arm_vsliq_n_s32(x, y, 2); \
    case 3: return __arm_vsliq_n_s32(x, y, 3); \
    case 4: return __arm_vsliq_n_s32(x, y, 4); \
    case 5: return __arm_vsliq_n_s32(x, y, 5); \
    case 6: return __arm_vsliq_n_s32(x, y, 6); \
    case 7: return __arm_vsliq_n_s32(x, y, 7); \
    case 8: return __arm_vsliq_n_s32(x, y, 8); \
    case 9: return __arm_vsliq_n_s32(x, y, 9); \
    case 10: return __arm_vsliq_n_s32(x, y, 10); \
    case 11: return __arm_vsliq_n_s32(x, y, 11); \
    case 12: return __arm_vsliq_n_s32(x, y, 12); \
    case 13: return __arm_vsliq_n_s32(x, y, 13); \
    case 14: return __arm_vsliq_n_s32(x, y, 14); \
    case 15: return __arm_vsliq_n_s32(x, y, 15); \
    case 16: return __arm_vsliq_n_s32(x, y, 16); \
    case 17: return __arm_vsliq_n_s32(x, y, 17); \
    case 18: return __arm_vsliq_n_s32(x, y, 18); \
    case 19: return __arm_vsliq_n_s32(x, y, 19); \
    case 20: return __arm_vsliq_n_s32(x, y, 20); \
    case 21: return __arm_vsliq_n_s32(x, y, 21); \
    case 22: return __arm_vsliq_n_s32(x, y, 22); \
    case 23: return __arm_vsliq_n_s32(x, y, 23); \
    case 24: return __arm_vsliq_n_s32(x, y, 24); \
    case 25: return __arm_vsliq_n_s32(x, y, 25); \
    case 26: return __arm_vsliq_n_s32(x, y, 26); \
    case 27: return __arm_vsliq_n_s32(x, y, 27); \
    case 28: return __arm_vsliq_n_s32(x, y, 28); \
    case 29: return __arm_vsliq_n_s32(x, y, 29); \
    case 30: return __arm_vsliq_n_s32(x, y, 30); \
    case 31: return __arm_vsliq_n_s32(x, y, 31);

__attribute__((always_inline))
static inline int32x4_t shift_left_insert(int32x4_t x, int32x4_t y, uint32_t count) {
    switch (count & 31) {
    VSLI_CASES
    }
    return x;
}

__attribute__((always_inline))
static inline int32x4_t shift_right_insert(int32x4_t x, int32x4_t y, uint32_t count) {
    switch (count) {
    case 1: return __arm_vsriq_n_s32(x, y, 1);
    case 2: return __arm_vsriq_n_s32(x, y, 2);
    case 3: return __arm_vsriq_n_s32(x, y, 3);
    case 4: return __arm_vsriq_n_s32(x, y, 4);
    case 5: return __arm_vsriq_n_s32(x, y, 5);
    case 6: return __arm_vsriq_n_s32(x, y, 6);
    case 7: return __arm_vsriq_n_s32(x, y, 7);
    case 8: return __arm_vsriq_n_s32(x, y, 8);
    case 9: return __arm_vsriq_n_s32(x, y, 9);
    case 10: return __arm_vsriq_n_s32(x, y, 10);
    case 11: return __arm_vsriq_n_s32(x, y, 11);
    case 12: return __arm_vsriq_n_s32(x, y, 12);
    case 13: return __arm_vsriq_n_s32(x, y, 13);
    case 14: return __arm_vsriq_n_s32(x, y, 14);
    case 15: return __arm_vsriq_n_s32(x, y, 15);
    case 16: return __arm_vsriq_n_s32(x, y, 16);
    case 17: return __arm_vsriq_n_s32(x, y, 17);
    case 18: return __arm_vsriq_n_s32(x, y, 18);
    case 19: return __arm_vsriq_n_s32(x, y, 19);
    case 20: return __arm_vsriq_n_s32(x, y, 20);
    case 21: return __arm_vsriq_n_s32(x, y, 21);
    case 22: return __arm_vsriq_n_s32(x, y, 22);
    case 23: return __arm_vsriq_n_s32(x, y, 23);
    case 24: return __arm_vsriq_n_s32(x, y, 24);
    case 25: return __arm_vsriq_n_s32(x, y, 25);
    case 26: return __arm_vsriq_n_s32(x, y, 26);
    case 27: return __arm_vsriq_n_s32(x, y, 27);
    case 28: return __arm_vsriq_n_s32(x, y, 28);
    case 29: return __arm_vsriq_n_s32(x, y, 29);
    case 30: return __arm_vsriq_n_s32(x, y, 30);
    case 31: return __arm_vsriq_n_s32(x, y, 31);
    }
    return x;
}

__attribute__((always_inline))
void rust_isa_arm_mve_sli(const uint8_t *a, const uint8_t *b, uint32_t count, uint8_t *out) {
    store_s32(out, shift_left_insert(load_s32(a), load_s32(b), count));
}

__attribute__((always_inline))
void rust_isa_arm_mve_sri(const uint8_t *a, const uint8_t *b, uint32_t count, uint8_t *out) {
    store_s32(out, shift_right_insert(load_s32(a), load_s32(b), count));
}

__attribute__((always_inline))
void rust_isa_arm_mve_sub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    store_s32(out, vsubq_s32(load_s32(a), load_s32(b)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_subn(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    const int32x4_t diff = vsubq_s32(load_s32(a), load_s32(b));
    int16_t *lanes = (int16_t *)(void *)out;
    lanes[0] = (int16_t)(diff[0] >> 16);
    lanes[1] = (int16_t)(diff[1] >> 16);
    lanes[2] = (int16_t)(diff[2] >> 16);
    lanes[3] = (int16_t)(diff[3] >> 16);
    lanes[4] = 0;
    lanes[5] = 0;
    lanes[6] = 0;
    lanes[7] = 0;
}

__attribute__((always_inline))
void rust_isa_arm_mve_subw(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    const int16x8_t vb = vld1q_s16((const int16_t *)(const void *)b);
    store_s32(out, vsubq_s32(load_s32(a), __arm_vmovlbq_s16(vb)));
}

__attribute__((always_inline))
void rust_isa_arm_mve_uzp(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    const uint8_t *va = a;
    const uint8_t *vb = b;
    for (unsigned index = 0; index < 8; index++) {
        out[index] = va[index * 2];
        out[index + 8] = vb[index * 2];
    }
}

__attribute__((always_inline))
void rust_isa_arm_mve_zip(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    for (unsigned index = 0; index < 8; index++) {
        out[index * 2] = a[index];
        out[index * 2 + 1] = b[index];
    }
}
