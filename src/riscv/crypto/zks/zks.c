#include <stdint.h>
#include <riscv_crypto.h>

uint32_t rust_isa_riscv_zks_sm4ed_0(uint32_t a, uint32_t b);
uint32_t rust_isa_riscv_zks_sm4ed_1(uint32_t a, uint32_t b);
uint32_t rust_isa_riscv_zks_sm4ed_2(uint32_t a, uint32_t b);
uint32_t rust_isa_riscv_zks_sm4ed_3(uint32_t a, uint32_t b);
uint32_t rust_isa_riscv_zks_sm4ks_0(uint32_t a, uint32_t b);
uint32_t rust_isa_riscv_zks_sm4ks_1(uint32_t a, uint32_t b);
uint32_t rust_isa_riscv_zks_sm4ks_2(uint32_t a, uint32_t b);
uint32_t rust_isa_riscv_zks_sm4ks_3(uint32_t a, uint32_t b);


__attribute__((always_inline))
uint32_t rust_isa_riscv_zks_sm3p0(uint32_t value) {
    return __riscv_sm3p0(value);
}

__attribute__((always_inline))
uint32_t rust_isa_riscv_zks_sm3p1(uint32_t value) {
    return __riscv_sm3p1(value);
}

#define RUST_ISA_SM4ED(N) \
__attribute__((always_inline)) \
uint32_t rust_isa_riscv_zks_sm4ed_##N(uint32_t a, uint32_t b) { \
    return __riscv_sm4ed(a, b, N); \
}

#define RUST_ISA_SM4KS(N) \
__attribute__((always_inline)) \
uint32_t rust_isa_riscv_zks_sm4ks_##N(uint32_t a, uint32_t b) { \
    return __riscv_sm4ks(a, b, N); \
}

RUST_ISA_SM4ED(0)
RUST_ISA_SM4ED(1)
RUST_ISA_SM4ED(2)
RUST_ISA_SM4ED(3)

RUST_ISA_SM4KS(0)
RUST_ISA_SM4KS(1)
RUST_ISA_SM4KS(2)
RUST_ISA_SM4KS(3)
