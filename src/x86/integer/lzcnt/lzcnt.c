#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
uint32_t rust_isa_lzcnt_u32(uint32_t value) {
    return _lzcnt_u32(value);
}

#if defined(__x86_64__)
__attribute__((always_inline))
uint64_t rust_isa_lzcnt_u64(uint64_t value) {
    return _lzcnt_u64(value);
}
#endif
