#include <stdint.h>

static uint16_t bits16(_Float16 value) {
    uint16_t out;
    __builtin_memcpy(&out, &value, 2);
    return out;
}
static _Float16 from_bits16(uint16_t value) {
    _Float16 out;
    __builtin_memcpy(&out, &value, 2);
    return out;
}
static uint32_t bits32(float value) {
    uint32_t out;
    __builtin_memcpy(&out, &value, 4);
    return out;
}
static float from_bits32(uint32_t value) {
    float out;
    __builtin_memcpy(&out, &value, 4);
    return out;
}
static uint64_t bits64(double value) {
    uint64_t out;
    __builtin_memcpy(&out, &value, 8);
    return out;
}
static double from_bits64(uint64_t value) {
    double out;
    __builtin_memcpy(&out, &value, 8);
    return out;
}

static int is_nan16(uint16_t value) {
    return (value & 0x7c00u) == 0x7c00u && (value & 0x03ffu) != 0;
}
static int is_nan32(uint32_t value) {
    return (value & 0x7f800000u) == 0x7f800000u && (value & 0x007fffffu) != 0;
}
static int is_nan64(uint64_t value) {
    return (value & 0x7ff0000000000000ull) == 0x7ff0000000000000ull &&
           (value & 0x000fffffffffffffull) != 0;
}

static uint32_t trunc_low32(double value);

uint16_t rust_isa_riscv_zfh_fadd(uint16_t a, uint16_t b) { return bits16(from_bits16(a) + from_bits16(b)); }
uint16_t rust_isa_riscv_zfh_fsub(uint16_t a, uint16_t b) { return bits16(from_bits16(a) - from_bits16(b)); }
uint16_t rust_isa_riscv_zfh_fmul(uint16_t a, uint16_t b) { return bits16(from_bits16(a) * from_bits16(b)); }
uint16_t rust_isa_riscv_zfh_fdiv(uint16_t a, uint16_t b) { return bits16(from_bits16(a) / from_bits16(b)); }
uint16_t rust_isa_riscv_zfh_fsqrt(uint16_t a) { return bits16(__builtin_sqrtf16(from_bits16(a))); }

static uint16_t fmin16(uint16_t a, uint16_t b) {
    if (is_nan16(a) && is_nan16(b)) {
        return 0x7e00u;
    }
    if (is_nan16(a)) {
        return b;
    }
    if (is_nan16(b)) {
        return a;
    }
    _Float16 fa = from_bits16(a);
    _Float16 fb = from_bits16(b);
    if (fa == fb) {
        return (uint16_t)(a | b);
    }
    return fa < fb ? a : b;
}
static uint16_t fmax16(uint16_t a, uint16_t b) {
    if (is_nan16(a) && is_nan16(b)) {
        return 0x7e00u;
    }
    if (is_nan16(a)) {
        return b;
    }
    if (is_nan16(b)) {
        return a;
    }
    _Float16 fa = from_bits16(a);
    _Float16 fb = from_bits16(b);
    if (fa == fb) {
        return (uint16_t)(a & b);
    }
    return fa > fb ? a : b;
}

uint16_t rust_isa_riscv_zfh_fmin(uint16_t a, uint16_t b) { return fmin16(a, b); }
uint16_t rust_isa_riscv_zfh_fmax(uint16_t a, uint16_t b) { return fmax16(a, b); }
uint32_t rust_isa_riscv_zfh_feq(uint16_t a, uint16_t b) { return from_bits16(a) == from_bits16(b); }
uint32_t rust_isa_riscv_zfh_flt(uint16_t a, uint16_t b) { return from_bits16(a) < from_bits16(b); }
uint32_t rust_isa_riscv_zfh_fle(uint16_t a, uint16_t b) { return from_bits16(a) <= from_bits16(b); }
void rust_isa_riscv_zfh_flh(const uint8_t *p, uint8_t *out) {
    __builtin_memcpy(out, p, 2);
}
void rust_isa_riscv_zfh_fsh(uint8_t *p, const uint8_t *a) {
    __builtin_memcpy(p, a, 2);
}

uint16_t rust_isa_riscv_zfh_fcvt_h_s(uint32_t value) {
    float f;
    __builtin_memcpy(&f, &value, 4);
    return bits16((_Float16)f);
}
uint32_t rust_isa_riscv_zfh_fcvt_s_h(uint16_t value) {
    float f = (float)from_bits16(value);
    uint32_t out;
    __builtin_memcpy(&out, &f, 4);
    return out;
}
uint16_t rust_isa_riscv_zfh_fcvt_h_d(uint64_t value) {
    double d;
    __builtin_memcpy(&d, &value, 8);
    return bits16((_Float16)d);
}
uint64_t rust_isa_riscv_zfh_fcvt_d_h(uint16_t value) {
    double d = (double)from_bits16(value);
    uint64_t out;
    __builtin_memcpy(&out, &d, 8);
    return out;
}
int32_t rust_isa_riscv_zfh_fcvt_w_h(uint16_t value) { return (int32_t)from_bits16(value); }
uint32_t rust_isa_riscv_zfh_fcvt_wu_h(uint16_t value) { return (uint32_t)from_bits16(value); }
int64_t rust_isa_riscv_zfh_fcvt_l_h(uint16_t value) { return (int64_t)from_bits16(value); }
uint64_t rust_isa_riscv_zfh_fcvt_lu_h(uint16_t value) { return (uint64_t)from_bits16(value); }
uint16_t rust_isa_riscv_zfh_fcvt_h_w(int32_t value) { return bits16((_Float16)value); }
uint16_t rust_isa_riscv_zfh_fcvt_h_wu(uint32_t value) { return bits16((_Float16)value); }
uint16_t rust_isa_riscv_zfh_fcvt_h_l(int64_t value) { return bits16((_Float16)value); }
uint16_t rust_isa_riscv_zfh_fcvt_h_lu(uint64_t value) { return bits16((_Float16)value); }
int16_t rust_isa_riscv_zfh_fmv_x_h(uint16_t value) { return (int16_t)value; }
uint16_t rust_isa_riscv_zfh_fmv_h_x(int16_t value) { return (uint16_t)value; }

uint16_t rust_isa_riscv_zfh_fminm(uint16_t a, uint16_t b) {
    if (is_nan16(a) || is_nan16(b)) {
        return 0x7e00u;
    }
    return fmin16(a, b);
}
uint16_t rust_isa_riscv_zfh_fmaxm(uint16_t a, uint16_t b) {
    if (is_nan16(a) || is_nan16(b)) {
        return 0x7e00u;
    }
    return fmax16(a, b);
}
uint16_t rust_isa_riscv_zfh_fround(uint16_t a) {
    return bits16(__builtin_rintf16(from_bits16(a)));
}
uint16_t rust_isa_riscv_zfh_froundnx(uint16_t a) {
    return bits16(__builtin_rintf16(from_bits16(a)));
}
uint32_t rust_isa_riscv_zfh_fleq(uint16_t a, uint16_t b) {
    return from_bits16(a) <= from_bits16(b);
}
uint32_t rust_isa_riscv_zfh_fltq(uint16_t a, uint16_t b) {
    return from_bits16(a) < from_bits16(b);
}
int32_t rust_isa_riscv_zfh_fcvtmod_w_h(uint16_t a) {
    return (int32_t)trunc_low32((double)from_bits16(a));
}

uint32_t rust_isa_riscv_zfh_fadd_s(uint32_t a, uint32_t b) { return bits32(from_bits32(a) + from_bits32(b)); }
uint32_t rust_isa_riscv_zfh_fsub_s(uint32_t a, uint32_t b) { return bits32(from_bits32(a) - from_bits32(b)); }
uint32_t rust_isa_riscv_zfh_fmul_s(uint32_t a, uint32_t b) { return bits32(from_bits32(a) * from_bits32(b)); }
uint32_t rust_isa_riscv_zfh_fdiv_s(uint32_t a, uint32_t b) { return bits32(from_bits32(a) / from_bits32(b)); }
uint32_t rust_isa_riscv_zfh_fsqrt_s(uint32_t a) { return bits32(__builtin_sqrtf(from_bits32(a))); }

uint64_t rust_isa_riscv_zfh_fadd_d(uint64_t a, uint64_t b) { return bits64(from_bits64(a) + from_bits64(b)); }
uint64_t rust_isa_riscv_zfh_fsub_d(uint64_t a, uint64_t b) { return bits64(from_bits64(a) - from_bits64(b)); }
uint64_t rust_isa_riscv_zfh_fmul_d(uint64_t a, uint64_t b) { return bits64(from_bits64(a) * from_bits64(b)); }
uint64_t rust_isa_riscv_zfh_fdiv_d(uint64_t a, uint64_t b) { return bits64(from_bits64(a) / from_bits64(b)); }
uint64_t rust_isa_riscv_zfh_fsqrt_d(uint64_t a) { return bits64(__builtin_sqrt(from_bits64(a))); }

static uint32_t fmin32(uint32_t a, uint32_t b) {
    if (is_nan32(a) && is_nan32(b)) {
        return 0x7fc00000u;
    }
    if (is_nan32(a)) {
        return b;
    }
    if (is_nan32(b)) {
        return a;
    }
    float fa = from_bits32(a);
    float fb = from_bits32(b);
    if (fa == fb) {
        return a | b;
    }
    return fa < fb ? a : b;
}
static uint32_t fmax32(uint32_t a, uint32_t b) {
    if (is_nan32(a) && is_nan32(b)) {
        return 0x7fc00000u;
    }
    if (is_nan32(a)) {
        return b;
    }
    if (is_nan32(b)) {
        return a;
    }
    float fa = from_bits32(a);
    float fb = from_bits32(b);
    if (fa == fb) {
        return a & b;
    }
    return fa > fb ? a : b;
}
static uint64_t fmin64(uint64_t a, uint64_t b) {
    if (is_nan64(a) && is_nan64(b)) {
        return 0x7ff8000000000000ull;
    }
    if (is_nan64(a)) {
        return b;
    }
    if (is_nan64(b)) {
        return a;
    }
    double fa = from_bits64(a);
    double fb = from_bits64(b);
    if (fa == fb) {
        return a | b;
    }
    return fa < fb ? a : b;
}
static uint64_t fmax64(uint64_t a, uint64_t b) {
    if (is_nan64(a) && is_nan64(b)) {
        return 0x7ff8000000000000ull;
    }
    if (is_nan64(a)) {
        return b;
    }
    if (is_nan64(b)) {
        return a;
    }
    double fa = from_bits64(a);
    double fb = from_bits64(b);
    if (fa == fb) {
        return a & b;
    }
    return fa > fb ? a : b;
}

uint32_t rust_isa_riscv_zfh_fmin_s(uint32_t a, uint32_t b) { return fmin32(a, b); }
uint32_t rust_isa_riscv_zfh_fmax_s(uint32_t a, uint32_t b) { return fmax32(a, b); }
uint32_t rust_isa_riscv_zfh_fminm_s(uint32_t a, uint32_t b) {
    if (is_nan32(a) || is_nan32(b)) {
        return 0x7fc00000u;
    }
    return fmin32(a, b);
}
uint32_t rust_isa_riscv_zfh_fmaxm_s(uint32_t a, uint32_t b) {
    if (is_nan32(a) || is_nan32(b)) {
        return 0x7fc00000u;
    }
    return fmax32(a, b);
}
uint64_t rust_isa_riscv_zfh_fmin_d(uint64_t a, uint64_t b) { return fmin64(a, b); }
uint64_t rust_isa_riscv_zfh_fmax_d(uint64_t a, uint64_t b) { return fmax64(a, b); }
uint64_t rust_isa_riscv_zfh_fminm_d(uint64_t a, uint64_t b) {
    if (is_nan64(a) || is_nan64(b)) {
        return 0x7ff8000000000000ull;
    }
    return fmin64(a, b);
}
uint64_t rust_isa_riscv_zfh_fmaxm_d(uint64_t a, uint64_t b) {
    if (is_nan64(a) || is_nan64(b)) {
        return 0x7ff8000000000000ull;
    }
    return fmax64(a, b);
}

uint32_t rust_isa_riscv_zfh_feq_s(uint32_t a, uint32_t b) { return from_bits32(a) == from_bits32(b); }
uint32_t rust_isa_riscv_zfh_flt_s(uint32_t a, uint32_t b) { return from_bits32(a) < from_bits32(b); }
uint32_t rust_isa_riscv_zfh_fle_s(uint32_t a, uint32_t b) { return from_bits32(a) <= from_bits32(b); }
uint32_t rust_isa_riscv_zfh_fleq_s(uint32_t a, uint32_t b) { return from_bits32(a) <= from_bits32(b); }
uint32_t rust_isa_riscv_zfh_fltq_s(uint32_t a, uint32_t b) { return from_bits32(a) < from_bits32(b); }
uint32_t rust_isa_riscv_zfh_feq_d(uint64_t a, uint64_t b) { return from_bits64(a) == from_bits64(b); }
uint32_t rust_isa_riscv_zfh_flt_d(uint64_t a, uint64_t b) { return from_bits64(a) < from_bits64(b); }
uint32_t rust_isa_riscv_zfh_fle_d(uint64_t a, uint64_t b) { return from_bits64(a) <= from_bits64(b); }
uint32_t rust_isa_riscv_zfh_fleq_d(uint64_t a, uint64_t b) { return from_bits64(a) <= from_bits64(b); }
uint32_t rust_isa_riscv_zfh_fltq_d(uint64_t a, uint64_t b) { return from_bits64(a) < from_bits64(b); }

uint32_t rust_isa_riscv_zfh_fround_s(uint32_t a) {
    return bits32(__builtin_nearbyintf(from_bits32(a)));
}
uint32_t rust_isa_riscv_zfh_froundnx_s(uint32_t a) {
    return bits32(__builtin_rintf(from_bits32(a)));
}
uint64_t rust_isa_riscv_zfh_fround_d(uint64_t a) {
    return bits64(__builtin_nearbyint(from_bits64(a)));
}
uint64_t rust_isa_riscv_zfh_froundnx_d(uint64_t a) {
    return bits64(__builtin_rint(from_bits64(a)));
}

uint32_t rust_isa_riscv_zfh_fcvt_s_d(uint64_t value) {
    return bits32((float)from_bits64(value));
}
uint64_t rust_isa_riscv_zfh_fcvt_d_s(uint32_t value) {
    return bits64((double)from_bits32(value));
}
int32_t rust_isa_riscv_zfh_fcvt_w_s(uint32_t value) { return (int32_t)from_bits32(value); }
uint32_t rust_isa_riscv_zfh_fcvt_wu_s(uint32_t value) { return (uint32_t)from_bits32(value); }
int64_t rust_isa_riscv_zfh_fcvt_l_s(uint32_t value) { return (int64_t)from_bits32(value); }
uint64_t rust_isa_riscv_zfh_fcvt_lu_s(uint32_t value) { return (uint64_t)from_bits32(value); }
uint32_t rust_isa_riscv_zfh_fcvt_s_w(int32_t value) { return bits32((float)value); }
uint32_t rust_isa_riscv_zfh_fcvt_s_wu(uint32_t value) { return bits32((float)value); }
uint32_t rust_isa_riscv_zfh_fcvt_s_l(int64_t value) { return bits32((float)value); }
uint32_t rust_isa_riscv_zfh_fcvt_s_lu(uint64_t value) { return bits32((float)value); }
int32_t rust_isa_riscv_zfh_fcvt_w_d(uint64_t value) { return (int32_t)from_bits64(value); }
uint32_t rust_isa_riscv_zfh_fcvt_wu_d(uint64_t value) { return (uint32_t)from_bits64(value); }
int64_t rust_isa_riscv_zfh_fcvt_l_d(uint64_t value) { return (int64_t)from_bits64(value); }
uint64_t rust_isa_riscv_zfh_fcvt_lu_d(uint64_t value) { return (uint64_t)from_bits64(value); }
uint64_t rust_isa_riscv_zfh_fcvt_d_w(int32_t value) { return bits64((double)value); }
uint64_t rust_isa_riscv_zfh_fcvt_d_wu(uint32_t value) { return bits64((double)value); }
uint64_t rust_isa_riscv_zfh_fcvt_d_l(int64_t value) { return bits64((double)value); }
uint64_t rust_isa_riscv_zfh_fcvt_d_lu(uint64_t value) { return bits64((double)value); }

static uint32_t trunc_low32(double value) {
    uint64_t raw = bits64(value);
    uint64_t sign = raw >> 63;
    uint64_t exponent = (raw >> 52) & 0x7ffu;
    uint64_t fraction = raw & 0x000fffffffffffffull;
    if (exponent == 0x7ffu) {
        return 0;
    }
    int unbiased = (int)exponent - 1023;
    if (unbiased < 0) {
        return 0;
    }
    uint64_t magnitude;
    if (unbiased >= 52) {
        unsigned shift = (unsigned)(unbiased - 52);
        if (shift >= 32) {
            return 0;
        }
        magnitude = (0x0010000000000000ull | fraction) << shift;
    } else {
        magnitude = (0x0010000000000000ull | fraction) >> (52 - unbiased);
    }
    uint32_t low = (uint32_t)magnitude;
    return sign != 0 ? 0u - low : low;
}

int32_t rust_isa_riscv_zfh_fcvtmod_w_d(uint64_t value) {
    return (int32_t)trunc_low32(from_bits64(value));
}

int32_t rust_isa_riscv_zfh_fmv_x_w(uint32_t value) { return (int32_t)value; }
uint32_t rust_isa_riscv_zfh_fmv_w_x(int32_t value) { return (uint32_t)value; }
int64_t rust_isa_riscv_zfh_fmv_x_d(uint64_t value) { return (int64_t)value; }
uint64_t rust_isa_riscv_zfh_fmv_d_x(int64_t value) { return (uint64_t)value; }
uint32_t rust_isa_riscv_zfh_fmvh_x_d(uint64_t value) { return (uint32_t)(value >> 32); }
uint64_t rust_isa_riscv_zfh_fmvp_d_x(uint32_t low, uint32_t high) {
    return ((uint64_t)high << 32) | low;
}
