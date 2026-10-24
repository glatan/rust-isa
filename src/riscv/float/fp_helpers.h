#ifndef RUST_ISA_RISCV_FP_HELPERS_H
#define RUST_ISA_RISCV_FP_HELPERS_H

#include <stdint.h>

static inline uint16_t bits16(_Float16 value) {
    uint16_t out;
    __builtin_memcpy(&out, &value, 2);
    return out;
}
static inline _Float16 from_bits16(uint16_t value) {
    _Float16 out;
    __builtin_memcpy(&out, &value, 2);
    return out;
}
static inline uint32_t bits32(float value) {
    uint32_t out;
    __builtin_memcpy(&out, &value, 4);
    return out;
}
static inline float from_bits32(uint32_t value) {
    float out;
    __builtin_memcpy(&out, &value, 4);
    return out;
}
static inline uint64_t bits64(double value) {
    uint64_t out;
    __builtin_memcpy(&out, &value, 8);
    return out;
}
static inline double from_bits64(uint64_t value) {
    double out;
    __builtin_memcpy(&out, &value, 8);
    return out;
}

static inline int is_nan16(uint16_t value) {
    return (value & 0x7c00u) == 0x7c00u && (value & 0x03ffu) != 0;
}
static inline int is_nan32(uint32_t value) {
    return (value & 0x7f800000u) == 0x7f800000u && (value & 0x007fffffu) != 0;
}
static inline int is_nan64(uint64_t value) {
    return (value & 0x7ff0000000000000ull) == 0x7ff0000000000000ull &&
           (value & 0x000fffffffffffffull) != 0;
}

static inline uint16_t fmin16(uint16_t a, uint16_t b) {
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
static inline uint16_t fmax16(uint16_t a, uint16_t b) {
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

static inline uint32_t fmin32(uint32_t a, uint32_t b) {
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
static inline uint32_t fmax32(uint32_t a, uint32_t b) {
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
static inline uint64_t fmin64(uint64_t a, uint64_t b) {
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
static inline uint64_t fmax64(uint64_t a, uint64_t b) {
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

static inline uint32_t trunc_low32(double value) {
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

#endif
