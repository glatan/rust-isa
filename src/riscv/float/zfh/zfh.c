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

uint16_t rust_isa_riscv_zfh_fcvt_h_s(uint32_t value) {
    float f;
    __builtin_memcpy(&f, &value, 4);
    return bits((_Float16)f);
}
uint32_t rust_isa_riscv_zfh_fcvt_s_h(uint16_t value) {
    float f = (float)from_bits(value);
    uint32_t out;
    __builtin_memcpy(&out, &f, 4);
    return out;
}
uint16_t rust_isa_riscv_zfh_fcvt_h_d(uint64_t value) {
    double d;
    __builtin_memcpy(&d, &value, 8);
    return bits((_Float16)d);
}
uint64_t rust_isa_riscv_zfh_fcvt_d_h(uint16_t value) {
    double d = (double)from_bits(value);
    uint64_t out;
    __builtin_memcpy(&out, &d, 8);
    return out;
}
int32_t rust_isa_riscv_zfh_fcvt_w_h(uint16_t value) { return (int32_t)from_bits(value); }
uint32_t rust_isa_riscv_zfh_fcvt_wu_h(uint16_t value) { return (uint32_t)from_bits(value); }
uint16_t rust_isa_riscv_zfh_fcvt_h_w(int32_t value) { return bits((_Float16)value); }
uint16_t rust_isa_riscv_zfh_fcvt_h_wu(uint32_t value) { return bits((_Float16)value); }
int16_t rust_isa_riscv_zfh_fmv_x_h(uint16_t value) { return (int16_t)value; }
uint16_t rust_isa_riscv_zfh_fmv_h_x(int16_t value) { return (uint16_t)value; }

uint16_t rust_isa_riscv_zfh_fminm(uint16_t a, uint16_t b) {
    return bits(__builtin_fminf16(from_bits(a), from_bits(b)));
}
uint16_t rust_isa_riscv_zfh_fmaxm(uint16_t a, uint16_t b) {
    return bits(__builtin_fmaxf16(from_bits(a), from_bits(b)));
}
uint16_t rust_isa_riscv_zfh_fround(uint16_t a) {
    return bits(__builtin_rintf16(from_bits(a)));
}
uint16_t rust_isa_riscv_zfh_froundnx(uint16_t a) {
    return bits(__builtin_rintf16(from_bits(a)));
}
uint32_t rust_isa_riscv_zfh_fleq(uint16_t a, uint16_t b) {
    return from_bits(a) <= from_bits(b);
}
uint32_t rust_isa_riscv_zfh_fltq(uint16_t a, uint16_t b) {
    return from_bits(a) < from_bits(b);
}
int32_t rust_isa_riscv_zfh_fcvtmod_w_h(uint16_t a) {
    return (int32_t)from_bits(a);
}
