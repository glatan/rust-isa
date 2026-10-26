#include <stdint.h>

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_zvbc_vclmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vle64.v v3, (%1)\n\t"
                     "vclmul.vv v4, v2, v3\n\t"
                     "vse64.v v4, (%2)"
                     ::"r"(a), "r"(b), "r"(out)
                     : "v2", "v3", "v4", "memory");
}

__attribute__((noinline, target("arch=+zvkn,+zvks,+zvkg,+zvbc")))
void rust_isa_zvbc_vclmulh(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __asm__ volatile("vsetivli zero, 2, e64, m1, ta, ma\n\t"
                     "vle64.v v2, (%0)\n\t"
                     "vle64.v v3, (%1)\n\t"
                     "vclmulh.vv v4, v2, v3\n\t"
                     "vse64.v v4, (%2)"
                     ::"r"(a), "r"(b), "r"(out)
                     : "v2", "v3", "v4", "memory");
}
