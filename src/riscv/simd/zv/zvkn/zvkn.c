#include <stdint.h>

#define RV_AES_KEY1_CASE(r)                                                    \
    case r:                                                                    \
        __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"               \
                         "vle32.v v2, (%0)\n\t"                                \
                         "vaeskf1.vi v2, v2, " #r "\n\t"                       \
                         "vse32.v v2, (%1)"                                    \
                         ::"r"(key), "r"(out)                                  \
                         : "v2", "memory");                                    \
        break;

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
void rust_isa_rvv_vaesdf(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vaesdf.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rvv_vaesdm(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vaesdm.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rvv_vaesef(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vaesef.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rvv_vaesem(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vaesem.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rvv_vaeskf1(const uint8_t *key, uint32_t round, uint8_t *out) {
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

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rvv_vaeskf2(const uint8_t *key, uint32_t round, uint8_t *out) {
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

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rvv_vaesz(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vaesz.vs v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rvv_vsha2ch(const uint8_t *a, const uint8_t *b, const uint8_t *c,
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
void rust_isa_rvv_vsha2cl(const uint8_t *a, const uint8_t *b, const uint8_t *c,
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
void rust_isa_rvv_vsha2ms(const uint8_t *a, const uint8_t *b, const uint8_t *c,
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
