#include <stdint.h>

__attribute__((always_inline))
int32_t rust_isa_jsconv_fjcvtzs(double value) {
    int32_t out;
    __asm__("fjcvtzs %w0, %d1" : "=r"(out) : "w"(value));
    return out;
}
