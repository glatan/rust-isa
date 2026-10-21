#include <stdint.h>
#include <riscv_crypto.h>

__attribute__((always_inline))
uint32_t rust_isa_riscv_zkn_sha256sig0(uint32_t value) {
    return __riscv_sha256sig0(value);
}

__attribute__((always_inline))
uint32_t rust_isa_riscv_zkn_sha256sig1(uint32_t value) {
    return __riscv_sha256sig1(value);
}

__attribute__((always_inline))
uint32_t rust_isa_riscv_zkn_sha256sum0(uint32_t value) {
    return __riscv_sha256sum0(value);
}

__attribute__((always_inline))
uint32_t rust_isa_riscv_zkn_sha256sum1(uint32_t value) {
    return __riscv_sha256sum1(value);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zkn_sha512sig0(uint64_t value) {
    return __riscv_sha512sig0(value);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zkn_sha512sig1(uint64_t value) {
    return __riscv_sha512sig1(value);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zkn_sha512sum0(uint64_t value) {
    return __riscv_sha512sum0(value);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zkn_sha512sum1(uint64_t value) {
    return __riscv_sha512sum1(value);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zkn_aes64es(uint64_t a, uint64_t b) {
    return __riscv_aes64es(a, b);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zkn_aes64ds(uint64_t a, uint64_t b) {
    return __riscv_aes64ds(a, b);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zkn_aes64esm(uint64_t a, uint64_t b) {
    return __riscv_aes64esm(a, b);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zkn_aes64dsm(uint64_t a, uint64_t b) {
    return __riscv_aes64dsm(a, b);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zkn_aes64im(uint64_t value) {
    return __riscv_aes64im(value);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zkn_aes64ks2(uint64_t a, uint64_t b) {
    return __riscv_aes64ks2(a, b);
}

__attribute__((always_inline))
uint64_t rust_isa_riscv_zkn_aes64ks1i(uint64_t value, uint32_t round) {
    switch (round) {
    case 0: return __riscv_aes64ks1i(value, 0);
    case 1: return __riscv_aes64ks1i(value, 1);
    case 2: return __riscv_aes64ks1i(value, 2);
    case 3: return __riscv_aes64ks1i(value, 3);
    case 4: return __riscv_aes64ks1i(value, 4);
    case 5: return __riscv_aes64ks1i(value, 5);
    case 6: return __riscv_aes64ks1i(value, 6);
    case 7: return __riscv_aes64ks1i(value, 7);
    case 8: return __riscv_aes64ks1i(value, 8);
    case 9: return __riscv_aes64ks1i(value, 9);
    case 10: return __riscv_aes64ks1i(value, 10);
    default: return value;
    }
}
