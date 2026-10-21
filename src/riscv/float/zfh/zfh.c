#include <stdint.h>

static uint16_t bits(_Float16 value) {
    uint16_t out;
    __builtin_memcpy(&out, &value, 2);
    return out;
}
static _Float16 from_bits(uint16_t value) {
    _Float16 out;
    __builtin_memcpy(&out, &value, 2);
    return out;
}

uint16_t rust_isa_riscv_zfh_fadd(uint16_t a, uint16_t b) { return bits(from_bits(a) + from_bits(b)); }
uint16_t rust_isa_riscv_zfh_fsub(uint16_t a, uint16_t b) { return bits(from_bits(a) - from_bits(b)); }
uint16_t rust_isa_riscv_zfh_fmul(uint16_t a, uint16_t b) { return bits(from_bits(a) * from_bits(b)); }
uint16_t rust_isa_riscv_zfh_fdiv(uint16_t a, uint16_t b) { return bits(from_bits(a) / from_bits(b)); }
uint16_t rust_isa_riscv_zfh_fsqrt(uint16_t a) { return bits(__builtin_sqrtf16(from_bits(a))); }
uint16_t rust_isa_riscv_zfh_fmin(uint16_t a, uint16_t b) { return bits(__builtin_fminf16(from_bits(a), from_bits(b))); }
uint16_t rust_isa_riscv_zfh_fmax(uint16_t a, uint16_t b) { return bits(__builtin_fmaxf16(from_bits(a), from_bits(b))); }
uint32_t rust_isa_riscv_zfh_feq(uint16_t a, uint16_t b) { return from_bits(a) == from_bits(b); }
uint32_t rust_isa_riscv_zfh_flt(uint16_t a, uint16_t b) { return from_bits(a) < from_bits(b); }
uint32_t rust_isa_riscv_zfh_fle(uint16_t a, uint16_t b) { return from_bits(a) <= from_bits(b); }
void rust_isa_riscv_zfh_flh(const uint8_t *p, uint8_t *out) {
    __builtin_memcpy(out, p, 2);
}
void rust_isa_riscv_zfh_fsh(uint8_t *p, const uint8_t *a) {
    __builtin_memcpy(p, a, 2);
}
