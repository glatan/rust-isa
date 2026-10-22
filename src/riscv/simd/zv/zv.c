#include <stdint.h>

/* Vector cryptography (Zvkned, Zvknh, Zvksed, Zvksh, Zvkg, Zvbc). */

#define RV_CRYPTO __attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))

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
void rust_isa_rvv_vaesz(const uint8_t *state, const uint8_t *key, uint8_t *out) {
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
void rust_isa_rvv_vsm4k(const uint8_t *key, uint32_t round, uint8_t *out) {
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
void rust_isa_rvv_vsm4r(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vsm4r.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
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
void rust_isa_rvv_vsm3me(const uint8_t *a, const uint8_t *b, uint8_t *out) {
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
void rust_isa_rvv_vsm3c(const uint8_t *state, uint32_t round, uint8_t *out) {
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
void rust_isa_rvv_vclmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vle64.v v3, (%1)\n\t"
                     "vclmul.vv v4, v2, v3\n\t"
                     "vse64.v v4, (%2)"
                     ::"r"(a), "r"(b), "r"(out)
                     : "v2", "v3", "v4", "memory");
}
__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rvv_vclmulh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vle64.v v3, (%1)\n\t"
                     "vclmulh.vv v4, v2, v3\n\t"
                     "vse64.v v4, (%2)"
                     ::"r"(a), "r"(b), "r"(out)
                     : "v2", "v3", "v4", "memory");
}
__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_rvv_vghsh(const uint8_t *acc, const uint8_t *value, const uint8_t *key,
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
void rust_isa_rvv_vgmul(const uint8_t *acc, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vgmul.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(acc), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}
