#include <stdint.h>

__attribute__((always_inline))
uint32_t rust_isa_popcnt_popcnt_u32(uint32_t a) { return (uint32_t)__builtin_popcount(a); }

__attribute__((always_inline))
uint64_t rust_isa_popcnt_popcnt_u64(uint64_t a) { return (uint64_t)__builtin_popcountll(a); }
