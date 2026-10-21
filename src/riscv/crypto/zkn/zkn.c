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
