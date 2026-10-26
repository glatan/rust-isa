#include <stdint.h>

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_zvkg_vghsh(const uint8_t *acc, const uint8_t *value, const uint8_t *key,
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
void rust_isa_zvkg_vgmul(const uint8_t *acc, const uint8_t *key, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 4, e32, m1, ta, ma\n\t"
                     "vle32.v v2, (%0)\n\t"
                     "vle32.v v3, (%1)\n\t"
                     "vgmul.vv v2, v3\n\t"
                     "vse32.v v2, (%2)"
                     ::"r"(acc), "r"(key), "r"(out)
                     : "v2", "v3", "memory");
}
