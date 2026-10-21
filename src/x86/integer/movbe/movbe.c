#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
uint16_t rust_isa_movbe_load_u16(const void *p) {
    return (uint16_t)_loadbe_i16(p);
}

__attribute__((always_inline))
void rust_isa_movbe_store_u16(void *p, uint16_t value) {
    _storebe_i16(p, (short)value);
}

__attribute__((always_inline))
uint32_t rust_isa_movbe_load_u32(const void *p) {
    return (uint32_t)_loadbe_i32(p);
}

__attribute__((always_inline))
void rust_isa_movbe_store_u32(void *p, uint32_t value) {
    _storebe_i32(p, (int)value);
}

#if defined(__x86_64__)
__attribute__((always_inline))
uint64_t rust_isa_movbe_load_u64(const void *p) {
    return (uint64_t)_loadbe_i64(p);
}

__attribute__((always_inline))
void rust_isa_movbe_store_u64(void *p, uint64_t value) {
    _storebe_i64(p, (long long)value);
}
#endif
