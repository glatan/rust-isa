#include <stdint.h>
#include <riscv_bitmanip.h>

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_andn(uint64_t a, uint64_t b) {
    return a & ~b;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_orn(uint64_t a, uint64_t b) {
    return a | ~b;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_xnor(uint64_t a, uint64_t b) {
    return ~(a ^ b);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_clz(uint64_t value) {
    return (uint64_t)__riscv_clz_64(value);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_ctz(uint64_t value) {
    return value == 0 ? 64 : (uint64_t)__builtin_ctzll(value);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_cpop(uint64_t value) {
    return (uint64_t)__riscv_cpop_64(value);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_min(uint64_t a, uint64_t b) {
    return a < b ? a : b;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_max(uint64_t a, uint64_t b) {
    return a > b ? a : b;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_sext_b(uint64_t value) {
    return (uint64_t)(int64_t)(int8_t)value;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_sext_h(uint64_t value) {
    return (uint64_t)(int64_t)(int16_t)value;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_zext_h(uint64_t value) {
    return value & 0xffff;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_rol(uint64_t value, uint64_t shift) {
    return __riscv_rol_64(value, (uint32_t)shift);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_ror(uint64_t value, uint64_t shift) {
    return __riscv_ror_64(value, (uint32_t)shift);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_max_signed(uint64_t a, uint64_t b) {
    return (int64_t)a > (int64_t)b ? a : b;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_min_signed(uint64_t a, uint64_t b) {
    return (int64_t)a < (int64_t)b ? a : b;
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zbb_orc_b(uint64_t value) {
    uint64_t out = 0;
    for (unsigned shift = 0; shift < 64; shift += 8) {
        if ((value >> shift) & 0xff) {
            out |= 0xffULL << shift;
        }
    }
    return out;
}
