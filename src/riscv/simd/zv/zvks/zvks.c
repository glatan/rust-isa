#include <stdint.h>

#define RV_SM4K_CASE(r)                                                        \
    case r:                                                                    \
        __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"               \
                         "vle32.v v2, (%0)\n\t"                                \
                         "vsm4k.vi v2, v2, " #r "\n\t"                         \
                         "vse32.v v2, (%1)"                                    \
                         ::"r"(key), "r"(out)                                  \
                         : "v2", "memory");                                    \
        break;

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
void rust_isa_zvks_vsm3c(const uint8_t *state, uint32_t round, uint8_t *out) {
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
void rust_isa_zvks_vsm3me(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 8, e32, m2, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v4, (%1)\n\t"
                     "vsm3me.vv v6, v2, v4\n\t"
                     "vse32.v v6, (%2)"
                     ::"r"(a), "r"(b), "r"(out)
                     : "v2", "v3", "v4", "v5", "v6", "v7", "memory");
}

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_zvks_vsm4k(const uint8_t *key, uint32_t round, uint8_t *out) {
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
void rust_isa_zvks_vsm4r(const uint8_t *state, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vsm4r.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(state), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}
