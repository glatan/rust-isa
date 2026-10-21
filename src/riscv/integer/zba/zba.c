#include <stdint.h>

__attribute__((always_inline))
uint64_t rust_isa_riscv_zba_sh1add(uint64_t a, uint64_t b) {
    return (a << 1) + b;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zba_sh2add(uint64_t a, uint64_t b) {
    return (a << 2) + b;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zba_sh3add(uint64_t a, uint64_t b) {
    return (a << 3) + b;
}
