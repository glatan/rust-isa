#include <stdint.h>

__attribute__((always_inline))
void rust_isa_s390x_enh_vclzd(const uint8_t *a, uint8_t *out) {
    for (int i = 0; i < 2; i++) {
        uint64_t value;
        __builtin_memcpy(&value, a + i * 8, 8);
        uint64_t count = value == 0 ? 64 : (uint64_t)__builtin_clzll(value);
        __builtin_memcpy(out + i * 8, &count, 8);
    }
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vctzd(const uint8_t *a, uint8_t *out) {
    for (int i = 0; i < 2; i++) {
        uint64_t value;
        __builtin_memcpy(&value, a + i * 8, 8);
        uint64_t count = value == 0 ? 64 : (uint64_t)__builtin_ctzll(value);
        __builtin_memcpy(out + i * 8, &count, 8);
    }
}

__attribute__((always_inline))
void rust_isa_s390x_enh_vpopcntd(const uint8_t *a, uint8_t *out) {
    for (int i = 0; i < 2; i++) {
        uint64_t value;
        __builtin_memcpy(&value, a + i * 8, 8);
        uint64_t count = (uint64_t)__builtin_popcountll(value);
        __builtin_memcpy(out + i * 8, &count, 8);
    }
}
