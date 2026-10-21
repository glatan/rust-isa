#include <stdint.h>
#include <arm_acle.h>

__attribute__((always_inline))
uint32_t rust_isa_crc_crc32b(uint32_t crc, uint8_t value) {
    return __crc32b(crc, value);
}

__attribute__((always_inline))
uint32_t rust_isa_crc_crc32h(uint32_t crc, uint16_t value) {
    return __crc32h(crc, value);
}

__attribute__((always_inline))
uint32_t rust_isa_crc_crc32w(uint32_t crc, uint32_t value) {
    return __crc32w(crc, value);
}

__attribute__((always_inline))
uint32_t rust_isa_crc_crc32x(uint32_t crc, uint64_t value) {
    return __crc32d(crc, value);
}
