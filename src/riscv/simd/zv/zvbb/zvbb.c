#include <stdint.h>

#define ZVBB_TARGET __attribute__((noinline, target("arch=+zvbb")))

ZVBB_TARGET
void rust_isa_zvbb_vandn_vv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vle64.v v3, (%1)\n\t"
                     "vandn.vv v4, v2, v3\n\t"
                     "vse64.v v4, (%2)" ::"r"(a), "r"(b), "r"(out) : "v2", "v3", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vandn_vx(const uint8_t *a, uint64_t x, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vandn.vx v4, v2, %1\n\t"
                     "vse64.v v4, (%2)" ::"r"(a), "r"(x), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vbrev(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vbrev.v v4, v2\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vbrev8(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vbrev8.v v4, v2\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vrev8(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vrev8.v v4, v2\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vclz(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vclz.v v4, v2\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vctz(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vctz.v v4, v2\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vcpop(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vcpop.v v4, v2\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vrol_vv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vle64.v v3, (%1)\n\t"
                     "vrol.vv v4, v2, v3\n\t"
                     "vse64.v v4, (%2)" ::"r"(a), "r"(b), "r"(out) : "v2", "v3", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vrol_vx(const uint8_t *a, uint64_t x, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vrol.vx v4, v2, %1\n\t"
                     "vse64.v v4, (%2)" ::"r"(a), "r"(x), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vle64.v v3, (%1)\n\t"
                     "vror.vv v4, v2, v3\n\t"
                     "vse64.v v4, (%2)" ::"r"(a), "r"(b), "r"(out) : "v2", "v3", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vx(const uint8_t *a, uint64_t x, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vx v4, v2, %1\n\t"
                     "vse64.v v4, (%2)" ::"r"(a), "r"(x), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vwsll.vv v4, v2, v3\n\t"
                     "vse64.v v4, (%2)" ::"r"(a), "r"(b), "r"(out) : "v2", "v3", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vx(const uint8_t *a, uint32_t x, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vx v4, v2, %1\n\t"
                     "vse64.v v4, (%2)" ::"r"(a), "r"(x), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_0(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 0\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_1(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 1\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_2(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 2\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_3(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 3\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_4(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 4\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_5(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 5\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_6(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 6\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_7(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 7\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_8(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 8\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_9(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 9\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_10(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 10\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_11(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 11\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_12(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 12\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_13(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 13\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_14(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 14\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_15(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 15\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_16(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 16\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_17(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 17\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_18(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 18\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_19(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 19\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_20(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 20\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_21(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 21\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_22(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 22\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_23(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 23\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_24(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 24\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_25(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 25\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_26(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 26\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_27(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 27\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_28(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 28\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_29(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 29\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_30(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 30\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_31(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 31\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_32(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 32\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_33(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 33\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_34(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 34\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_35(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 35\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_36(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 36\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_37(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 37\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_38(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 38\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_39(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 39\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_40(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 40\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_41(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 41\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_42(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 42\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_43(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 43\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_44(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 44\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_45(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 45\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_46(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 46\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_47(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 47\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_48(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 48\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_49(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 49\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_50(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 50\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_51(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 51\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_52(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 52\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_53(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 53\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_54(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 54\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_55(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 55\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_56(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 56\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_57(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 57\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_58(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 58\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_59(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 59\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_60(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 60\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_61(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 61\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_62(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 62\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vror_vi_63(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vror.vi v4, v2, 63\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_0(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 0\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_1(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 1\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_2(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 2\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_3(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 3\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_4(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 4\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_5(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 5\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_6(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 6\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_7(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 7\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_8(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 8\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_9(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 9\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_10(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 10\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_11(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 11\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_12(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 12\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_13(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 13\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_14(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 14\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_15(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 15\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_16(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 16\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_17(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 17\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_18(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 18\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_19(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 19\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_20(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 20\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_21(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 21\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_22(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 22\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_23(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 23\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_24(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 24\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_25(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 25\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_26(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 26\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_27(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 27\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_28(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 28\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_29(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 29\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_30(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 30\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}

ZVBB_TARGET
void rust_isa_zvbb_vwsll_vi_31(const uint8_t *a, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vwsll.vi v4, v2, 31\n\t"
                     "vse64.v v4, (%1)" ::"r"(a), "r"(out) : "v2", "v4", "memory");
}
