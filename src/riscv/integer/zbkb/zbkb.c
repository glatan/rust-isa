#include <stdint.h>
#include <riscv_bitmanip.h>

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbkb_pack(uint64_t a, uint64_t b) {
    uint64_t out;
    __asm__("pack %0, %1, %2" : "=r"(out) : "r"(a), "r"(b));
    return out;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbkb_packh(uint64_t a, uint64_t b) {
    uint64_t out;
    __asm__("packh %0, %1, %2" : "=r"(out) : "r"(a), "r"(b));
    return out;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbkb_brev8(uint64_t a) {
    return __riscv_brev8_64(a);
}

/* RV32-only instruction; implemented as a bit loop on RV64. */
__attribute__((always_inline))
uint64_t rust_isa_riscv_zbkb_zip(uint64_t value) {
    uint32_t input = (uint32_t)value;
    uint32_t output = 0;
    for (unsigned i = 0; i < 16; ++i) {
        output |= ((input >> i) & 1u) << (2u * i);
        output |= ((input >> (i + 16u)) & 1u) << (2u * i + 1u);
    }
    return output;
}

/* RV32-only instruction; implemented as a bit loop on RV64. */
__attribute__((always_inline))
uint64_t rust_isa_riscv_zbkb_unzip(uint64_t value) {
    uint32_t input = (uint32_t)value;
    uint32_t output = 0;
    for (unsigned i = 0; i < 16; ++i) {
        output |= ((input >> (2u * i)) & 1u) << i;
        output |= ((input >> (2u * i + 1u)) & 1u) << (i + 16u);
    }
    return output;
}
