#include <stdint.h>
#include <riscv_bitmanip.h>

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbkx_xperm4(uint64_t a, uint64_t b) {
    return __riscv_xperm4_64(a, b);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbkx_xperm8(uint64_t a, uint64_t b) {
    return __riscv_xperm8_64(a, b);
}
