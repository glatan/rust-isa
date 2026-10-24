#include <stdint.h>

#include "../fp_helpers.h"

uint16_t rust_isa_riscv_zfa_fminm(uint16_t a, uint16_t b) {
    if (is_nan16(a) || is_nan16(b)) {
        return 0x7e00u;
    }
    return fmin16(a, b);
}
uint16_t rust_isa_riscv_zfa_fmaxm(uint16_t a, uint16_t b) {
    if (is_nan16(a) || is_nan16(b)) {
        return 0x7e00u;
    }
    return fmax16(a, b);
}
uint16_t rust_isa_riscv_zfa_fround(uint16_t a) {
    return bits16(__builtin_rintf16(from_bits16(a)));
}
uint16_t rust_isa_riscv_zfa_froundnx(uint16_t a) {
    return bits16(__builtin_rintf16(from_bits16(a)));
}
uint32_t rust_isa_riscv_zfa_fleq(uint16_t a, uint16_t b) {
    return from_bits16(a) <= from_bits16(b);
}
uint32_t rust_isa_riscv_zfa_fltq(uint16_t a, uint16_t b) {
    return from_bits16(a) < from_bits16(b);
}
int32_t rust_isa_riscv_zfa_fcvtmod_w_h(uint16_t a) {
    return (int32_t)trunc_low32((double)from_bits16(a));
}

uint32_t rust_isa_riscv_zfa_fminm_s(uint32_t a, uint32_t b) {
    if (is_nan32(a) || is_nan32(b)) {
        return 0x7fc00000u;
    }
    return fmin32(a, b);
}
uint32_t rust_isa_riscv_zfa_fmaxm_s(uint32_t a, uint32_t b) {
    if (is_nan32(a) || is_nan32(b)) {
        return 0x7fc00000u;
    }
    return fmax32(a, b);
}
uint32_t rust_isa_riscv_zfa_fround_s(uint32_t a) {
    return bits32(__builtin_nearbyintf(from_bits32(a)));
}
uint32_t rust_isa_riscv_zfa_froundnx_s(uint32_t a) {
    return bits32(__builtin_rintf(from_bits32(a)));
}
uint32_t rust_isa_riscv_zfa_fleq_s(uint32_t a, uint32_t b) {
    return from_bits32(a) <= from_bits32(b);
}
uint32_t rust_isa_riscv_zfa_fltq_s(uint32_t a, uint32_t b) {
    return from_bits32(a) < from_bits32(b);
}

uint64_t rust_isa_riscv_zfa_fminm_d(uint64_t a, uint64_t b) {
    if (is_nan64(a) || is_nan64(b)) {
        return 0x7ff8000000000000ull;
    }
    return fmin64(a, b);
}
uint64_t rust_isa_riscv_zfa_fmaxm_d(uint64_t a, uint64_t b) {
    if (is_nan64(a) || is_nan64(b)) {
        return 0x7ff8000000000000ull;
    }
    return fmax64(a, b);
}
uint64_t rust_isa_riscv_zfa_fround_d(uint64_t a) {
    return bits64(__builtin_nearbyint(from_bits64(a)));
}
uint64_t rust_isa_riscv_zfa_froundnx_d(uint64_t a) {
    return bits64(__builtin_rint(from_bits64(a)));
}
uint32_t rust_isa_riscv_zfa_fleq_d(uint64_t a, uint64_t b) {
    return from_bits64(a) <= from_bits64(b);
}
uint32_t rust_isa_riscv_zfa_fltq_d(uint64_t a, uint64_t b) {
    return from_bits64(a) < from_bits64(b);
}
int32_t rust_isa_riscv_zfa_fcvtmod_w_d(uint64_t value) {
    return (int32_t)trunc_low32(from_bits64(value));
}
uint32_t rust_isa_riscv_zfa_fmvh_x_d(uint64_t value) {
    return (uint32_t)(value >> 32);
}
uint64_t rust_isa_riscv_zfa_fmvp_d_x(uint32_t low, uint32_t high) {
    return ((uint64_t)high << 32) | low;
}
