#include <stdint.h>

#include "../fp_helpers.h"

uint16_t rust_isa_riscv_scalar_fadd(uint16_t a, uint16_t b) { return bits16(from_bits16(a) + from_bits16(b)); }
uint16_t rust_isa_riscv_scalar_fsub(uint16_t a, uint16_t b) { return bits16(from_bits16(a) - from_bits16(b)); }
uint16_t rust_isa_riscv_scalar_fmul(uint16_t a, uint16_t b) { return bits16(from_bits16(a) * from_bits16(b)); }
uint16_t rust_isa_riscv_scalar_fdiv(uint16_t a, uint16_t b) { return bits16(from_bits16(a) / from_bits16(b)); }
uint16_t rust_isa_riscv_scalar_fsqrt(uint16_t a) { return bits16(__builtin_sqrtf16(from_bits16(a))); }

uint16_t rust_isa_riscv_scalar_fmin(uint16_t a, uint16_t b) { return fmin16(a, b); }
uint16_t rust_isa_riscv_scalar_fmax(uint16_t a, uint16_t b) { return fmax16(a, b); }
uint32_t rust_isa_riscv_scalar_feq(uint16_t a, uint16_t b) { return from_bits16(a) == from_bits16(b); }
uint32_t rust_isa_riscv_scalar_flt(uint16_t a, uint16_t b) { return from_bits16(a) < from_bits16(b); }
uint32_t rust_isa_riscv_scalar_fle(uint16_t a, uint16_t b) { return from_bits16(a) <= from_bits16(b); }
void rust_isa_riscv_scalar_flh(const uint8_t *p, uint8_t *out) {
    __builtin_memcpy(out, p, 2);
}
void rust_isa_riscv_scalar_fsh(uint8_t *p, const uint8_t *a) {
    __builtin_memcpy(p, a, 2);
}

uint16_t rust_isa_riscv_scalar_fcvt_h_s(uint32_t value) {
    float f;
    __builtin_memcpy(&f, &value, 4);
    return bits16((_Float16)f);
}
uint32_t rust_isa_riscv_scalar_fcvt_s_h(uint16_t value) {
    float f = (float)from_bits16(value);
    uint32_t out;
    __builtin_memcpy(&out, &f, 4);
    return out;
}
uint16_t rust_isa_riscv_scalar_fcvt_h_d(uint64_t value) {
    double d;
    __builtin_memcpy(&d, &value, 8);
    return bits16((_Float16)d);
}
uint64_t rust_isa_riscv_scalar_fcvt_d_h(uint16_t value) {
    double d = (double)from_bits16(value);
    uint64_t out;
    __builtin_memcpy(&out, &d, 8);
    return out;
}
int32_t rust_isa_riscv_scalar_fcvt_w_h(uint16_t value) { return (int32_t)from_bits16(value); }
uint32_t rust_isa_riscv_scalar_fcvt_wu_h(uint16_t value) { return (uint32_t)from_bits16(value); }
int64_t rust_isa_riscv_scalar_fcvt_l_h(uint16_t value) { return (int64_t)from_bits16(value); }
uint64_t rust_isa_riscv_scalar_fcvt_lu_h(uint16_t value) { return (uint64_t)from_bits16(value); }
uint16_t rust_isa_riscv_scalar_fcvt_h_w(int32_t value) { return bits16((_Float16)value); }
uint16_t rust_isa_riscv_scalar_fcvt_h_wu(uint32_t value) { return bits16((_Float16)value); }
uint16_t rust_isa_riscv_scalar_fcvt_h_l(int64_t value) { return bits16((_Float16)value); }
uint16_t rust_isa_riscv_scalar_fcvt_h_lu(uint64_t value) { return bits16((_Float16)value); }
int16_t rust_isa_riscv_scalar_fmv_x_h(uint16_t value) { return (int16_t)value; }
uint16_t rust_isa_riscv_scalar_fmv_h_x(int16_t value) { return (uint16_t)value; }

uint32_t rust_isa_riscv_scalar_fadd_s(uint32_t a, uint32_t b) { return bits32(from_bits32(a) + from_bits32(b)); }
uint32_t rust_isa_riscv_scalar_fsub_s(uint32_t a, uint32_t b) { return bits32(from_bits32(a) - from_bits32(b)); }
uint32_t rust_isa_riscv_scalar_fmul_s(uint32_t a, uint32_t b) { return bits32(from_bits32(a) * from_bits32(b)); }
uint32_t rust_isa_riscv_scalar_fdiv_s(uint32_t a, uint32_t b) { return bits32(from_bits32(a) / from_bits32(b)); }
uint32_t rust_isa_riscv_scalar_fsqrt_s(uint32_t a) { return bits32(__builtin_sqrtf(from_bits32(a))); }

uint64_t rust_isa_riscv_scalar_fadd_d(uint64_t a, uint64_t b) { return bits64(from_bits64(a) + from_bits64(b)); }
uint64_t rust_isa_riscv_scalar_fsub_d(uint64_t a, uint64_t b) { return bits64(from_bits64(a) - from_bits64(b)); }
uint64_t rust_isa_riscv_scalar_fmul_d(uint64_t a, uint64_t b) { return bits64(from_bits64(a) * from_bits64(b)); }
uint64_t rust_isa_riscv_scalar_fdiv_d(uint64_t a, uint64_t b) { return bits64(from_bits64(a) / from_bits64(b)); }
uint64_t rust_isa_riscv_scalar_fsqrt_d(uint64_t a) { return bits64(__builtin_sqrt(from_bits64(a))); }

uint32_t rust_isa_riscv_scalar_fmin_s(uint32_t a, uint32_t b) { return fmin32(a, b); }
uint32_t rust_isa_riscv_scalar_fmax_s(uint32_t a, uint32_t b) { return fmax32(a, b); }
uint64_t rust_isa_riscv_scalar_fmin_d(uint64_t a, uint64_t b) { return fmin64(a, b); }
uint64_t rust_isa_riscv_scalar_fmax_d(uint64_t a, uint64_t b) { return fmax64(a, b); }
uint32_t rust_isa_riscv_scalar_feq_d(uint64_t a, uint64_t b) { return from_bits64(a) == from_bits64(b); }
uint32_t rust_isa_riscv_scalar_flt_d(uint64_t a, uint64_t b) { return from_bits64(a) < from_bits64(b); }
uint32_t rust_isa_riscv_scalar_fle_d(uint64_t a, uint64_t b) { return from_bits64(a) <= from_bits64(b); }
uint32_t rust_isa_riscv_scalar_feq_s(uint32_t a, uint32_t b) { return from_bits32(a) == from_bits32(b); }
uint32_t rust_isa_riscv_scalar_flt_s(uint32_t a, uint32_t b) { return from_bits32(a) < from_bits32(b); }
uint32_t rust_isa_riscv_scalar_fle_s(uint32_t a, uint32_t b) { return from_bits32(a) <= from_bits32(b); }
uint32_t rust_isa_riscv_scalar_fcvt_s_d(uint64_t value) {
    return bits32((float)from_bits64(value));
}
uint64_t rust_isa_riscv_scalar_fcvt_d_s(uint32_t value) {
    return bits64((double)from_bits32(value));
}
int32_t rust_isa_riscv_scalar_fcvt_w_s(uint32_t value) { return (int32_t)from_bits32(value); }
uint32_t rust_isa_riscv_scalar_fcvt_wu_s(uint32_t value) { return (uint32_t)from_bits32(value); }
int64_t rust_isa_riscv_scalar_fcvt_l_s(uint32_t value) { return (int64_t)from_bits32(value); }
uint64_t rust_isa_riscv_scalar_fcvt_lu_s(uint32_t value) { return (uint64_t)from_bits32(value); }
uint32_t rust_isa_riscv_scalar_fcvt_s_w(int32_t value) { return bits32((float)value); }
uint32_t rust_isa_riscv_scalar_fcvt_s_wu(uint32_t value) { return bits32((float)value); }
uint32_t rust_isa_riscv_scalar_fcvt_s_l(int64_t value) { return bits32((float)value); }
uint32_t rust_isa_riscv_scalar_fcvt_s_lu(uint64_t value) { return bits32((float)value); }
int32_t rust_isa_riscv_scalar_fcvt_w_d(uint64_t value) { return (int32_t)from_bits64(value); }
uint32_t rust_isa_riscv_scalar_fcvt_wu_d(uint64_t value) { return (uint32_t)from_bits64(value); }
int64_t rust_isa_riscv_scalar_fcvt_l_d(uint64_t value) { return (int64_t)from_bits64(value); }
uint64_t rust_isa_riscv_scalar_fcvt_lu_d(uint64_t value) { return (uint64_t)from_bits64(value); }
uint64_t rust_isa_riscv_scalar_fcvt_d_w(int32_t value) { return bits64((double)value); }
uint64_t rust_isa_riscv_scalar_fcvt_d_wu(uint32_t value) { return bits64((double)value); }
uint64_t rust_isa_riscv_scalar_fcvt_d_l(int64_t value) { return bits64((double)value); }
uint64_t rust_isa_riscv_scalar_fcvt_d_lu(uint64_t value) { return bits64((double)value); }

int32_t rust_isa_riscv_scalar_fmv_x_w(uint32_t value) { return (int32_t)value; }
uint32_t rust_isa_riscv_scalar_fmv_w_x(int32_t value) { return (uint32_t)value; }
int64_t rust_isa_riscv_scalar_fmv_x_d(uint64_t value) { return (int64_t)value; }
uint64_t rust_isa_riscv_scalar_fmv_d_x(int64_t value) { return (uint64_t)value; }
