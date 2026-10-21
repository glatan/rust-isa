#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
int rust_isa_rdseed_u32(uint32_t *out) {
    return _rdseed32_step(out);
}

#if defined(__x86_64__)
__attribute__((always_inline))
int rust_isa_rdseed_u64(uint64_t *out) {
    return _rdseed64_step((unsigned long long *)(void *)out);
}
#endif
