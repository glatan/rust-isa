#include <stdint.h>
#include <riscv_bitmanip.h>

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbc_clmul(uint64_t a, uint64_t b) {
    return __riscv_clmul_64(a, b);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbc_clmulh(uint64_t a, uint64_t b) {
    return __riscv_clmulh_64(a, b);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbc_clmulr(uint64_t a, uint64_t b) {
    return __riscv_clmulr_64(a, b);
}
