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

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbkb_xperm4(uint64_t a, uint64_t b) {
    return __riscv_xperm4_64(a, b);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbkb_xperm8(uint64_t a, uint64_t b) {
    return __riscv_xperm8_64(a, b);
}
