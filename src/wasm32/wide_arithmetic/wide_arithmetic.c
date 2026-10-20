#include <stdint.h>

__attribute__((always_inline))
void rust_isa_wide_arithmetic_add128(uint64_t alo, uint64_t ahi, uint64_t blo, uint64_t bhi, uint64_t out[2]) {
    unsigned __int128 a = ((unsigned __int128)ahi << 64) | alo;
    unsigned __int128 b = ((unsigned __int128)bhi << 64) | blo;
    unsigned __int128 r = a + b;
    out[0] = (uint64_t)r;
    out[1] = (uint64_t)(r >> 64);
}

__attribute__((always_inline))
void rust_isa_wide_arithmetic_sub128(uint64_t alo, uint64_t ahi, uint64_t blo, uint64_t bhi, uint64_t out[2]) {
    unsigned __int128 a = ((unsigned __int128)ahi << 64) | alo;
    unsigned __int128 b = ((unsigned __int128)bhi << 64) | blo;
    unsigned __int128 r = a - b;
    out[0] = (uint64_t)r;
    out[1] = (uint64_t)(r >> 64);
}

__attribute__((always_inline))
void rust_isa_wide_arithmetic_mul_wide_u(uint64_t a, uint64_t b, uint64_t out[2]) {
    unsigned __int128 r = (unsigned __int128)a * b;
    out[0] = (uint64_t)r;
    out[1] = (uint64_t)(r >> 64);
}

__attribute__((always_inline))
void rust_isa_wide_arithmetic_mul_wide_s(int64_t a, int64_t b, int64_t out[2]) {
    __int128 r = (__int128)a * b;
    out[0] = (int64_t)r;
    out[1] = (int64_t)(r >> 64);
}
