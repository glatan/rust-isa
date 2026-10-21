#include <stdint.h>
#include <arm_neon.h>

__attribute__((always_inline))
void rust_isa_sha3_veor3q_u8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u8(out, veor3q_u8(vld1q_u8(a), vld1q_u8(b), vld1q_u8(c)));
}

__attribute__((always_inline))
void rust_isa_sha3_vbcaxq_u8(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    vst1q_u8(out, vbcaxq_u8(vld1q_u8(a), vld1q_u8(b), vld1q_u8(c)));
}

__attribute__((always_inline))
void rust_isa_sha3_vrax1q_u64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    vst1q_u64((uint64_t *)(void *)out, vrax1q_u64(
        vld1q_u64((const uint64_t *)(const void *)a),
        vld1q_u64((const uint64_t *)(const void *)b)));
}

/* XAR: rotate-xor with a 64-bit lane rotation. */

__attribute__((always_inline))
void rust_isa_sha3_vxarq_u64(const uint8_t *a, const uint8_t *b, int imm, uint8_t *out) {
    uint64x2_t va = vld1q_u64((const uint64_t *)(const void *)a);
    uint64x2_t vb = vld1q_u64((const uint64_t *)(const void *)b);
    uint64x2_t r;
    switch (imm) {
        case 0: __asm__("xar %0.2d, %1.2d, %2.2d, #0" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 1: __asm__("xar %0.2d, %1.2d, %2.2d, #1" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 2: __asm__("xar %0.2d, %1.2d, %2.2d, #2" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 3: __asm__("xar %0.2d, %1.2d, %2.2d, #3" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 4: __asm__("xar %0.2d, %1.2d, %2.2d, #4" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 5: __asm__("xar %0.2d, %1.2d, %2.2d, #5" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 6: __asm__("xar %0.2d, %1.2d, %2.2d, #6" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 7: __asm__("xar %0.2d, %1.2d, %2.2d, #7" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 8: __asm__("xar %0.2d, %1.2d, %2.2d, #8" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 9: __asm__("xar %0.2d, %1.2d, %2.2d, #9" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 10: __asm__("xar %0.2d, %1.2d, %2.2d, #10" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 11: __asm__("xar %0.2d, %1.2d, %2.2d, #11" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 12: __asm__("xar %0.2d, %1.2d, %2.2d, #12" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 13: __asm__("xar %0.2d, %1.2d, %2.2d, #13" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 14: __asm__("xar %0.2d, %1.2d, %2.2d, #14" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 15: __asm__("xar %0.2d, %1.2d, %2.2d, #15" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 16: __asm__("xar %0.2d, %1.2d, %2.2d, #16" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 17: __asm__("xar %0.2d, %1.2d, %2.2d, #17" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 18: __asm__("xar %0.2d, %1.2d, %2.2d, #18" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 19: __asm__("xar %0.2d, %1.2d, %2.2d, #19" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 20: __asm__("xar %0.2d, %1.2d, %2.2d, #20" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 21: __asm__("xar %0.2d, %1.2d, %2.2d, #21" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 22: __asm__("xar %0.2d, %1.2d, %2.2d, #22" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 23: __asm__("xar %0.2d, %1.2d, %2.2d, #23" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 24: __asm__("xar %0.2d, %1.2d, %2.2d, #24" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 25: __asm__("xar %0.2d, %1.2d, %2.2d, #25" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 26: __asm__("xar %0.2d, %1.2d, %2.2d, #26" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 27: __asm__("xar %0.2d, %1.2d, %2.2d, #27" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 28: __asm__("xar %0.2d, %1.2d, %2.2d, #28" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 29: __asm__("xar %0.2d, %1.2d, %2.2d, #29" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 30: __asm__("xar %0.2d, %1.2d, %2.2d, #30" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 31: __asm__("xar %0.2d, %1.2d, %2.2d, #31" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 32: __asm__("xar %0.2d, %1.2d, %2.2d, #32" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 33: __asm__("xar %0.2d, %1.2d, %2.2d, #33" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 34: __asm__("xar %0.2d, %1.2d, %2.2d, #34" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 35: __asm__("xar %0.2d, %1.2d, %2.2d, #35" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 36: __asm__("xar %0.2d, %1.2d, %2.2d, #36" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 37: __asm__("xar %0.2d, %1.2d, %2.2d, #37" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 38: __asm__("xar %0.2d, %1.2d, %2.2d, #38" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 39: __asm__("xar %0.2d, %1.2d, %2.2d, #39" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 40: __asm__("xar %0.2d, %1.2d, %2.2d, #40" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 41: __asm__("xar %0.2d, %1.2d, %2.2d, #41" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 42: __asm__("xar %0.2d, %1.2d, %2.2d, #42" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 43: __asm__("xar %0.2d, %1.2d, %2.2d, #43" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 44: __asm__("xar %0.2d, %1.2d, %2.2d, #44" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 45: __asm__("xar %0.2d, %1.2d, %2.2d, #45" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 46: __asm__("xar %0.2d, %1.2d, %2.2d, #46" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 47: __asm__("xar %0.2d, %1.2d, %2.2d, #47" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 48: __asm__("xar %0.2d, %1.2d, %2.2d, #48" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 49: __asm__("xar %0.2d, %1.2d, %2.2d, #49" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 50: __asm__("xar %0.2d, %1.2d, %2.2d, #50" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 51: __asm__("xar %0.2d, %1.2d, %2.2d, #51" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 52: __asm__("xar %0.2d, %1.2d, %2.2d, #52" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 53: __asm__("xar %0.2d, %1.2d, %2.2d, #53" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 54: __asm__("xar %0.2d, %1.2d, %2.2d, #54" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 55: __asm__("xar %0.2d, %1.2d, %2.2d, #55" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 56: __asm__("xar %0.2d, %1.2d, %2.2d, #56" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 57: __asm__("xar %0.2d, %1.2d, %2.2d, #57" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 58: __asm__("xar %0.2d, %1.2d, %2.2d, #58" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 59: __asm__("xar %0.2d, %1.2d, %2.2d, #59" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 60: __asm__("xar %0.2d, %1.2d, %2.2d, #60" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 61: __asm__("xar %0.2d, %1.2d, %2.2d, #61" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 62: __asm__("xar %0.2d, %1.2d, %2.2d, #62" : "=w"(r) : "w"(va), "w"(vb)); break;
        case 63: __asm__("xar %0.2d, %1.2d, %2.2d, #63" : "=w"(r) : "w"(va), "w"(vb)); break;
        default: r = vdupq_n_u64(0); break;
    }
    vst1q_u64((uint64_t *)(void *)out, r);
}