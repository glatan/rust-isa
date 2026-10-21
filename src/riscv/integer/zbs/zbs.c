#include <stdint.h>

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbs_bset(uint64_t value, uint64_t index) {
    return value | ((uint64_t)1 << (index & 63));
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbs_bclr(uint64_t value, uint64_t index) {
    return value & ~((uint64_t)1 << (index & 63));
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbs_binv(uint64_t value, uint64_t index) {
    return value ^ ((uint64_t)1 << (index & 63));
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbs_bext(uint64_t value, uint64_t index) {
    return (value >> (index & 63)) & 1;
}
