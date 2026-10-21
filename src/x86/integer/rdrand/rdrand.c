#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
int rust_isa_rdrand_u32(uint32_t *out) {
    return _rdrand32_step(out);
}

#if defined(__x86_64__)
__attribute__((always_inline))
int rust_isa_rdrand_u64(uint64_t *out) {
    return _rdrand64_step((unsigned long long *)(void *)out);
}
#endif
