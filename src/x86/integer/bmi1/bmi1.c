#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
uint32_t rust_isa_bmi1_andn(uint32_t a, uint32_t b) { return _andn_u32(a, b); }

__attribute__((always_inline))
uint32_t rust_isa_bmi1_bextr(uint32_t a, uint32_t start, uint32_t len) { return _bextr_u32(a, start, len); }

__attribute__((always_inline))
uint32_t rust_isa_bmi1_blsi(uint32_t a) { return _blsi_u32(a); }

__attribute__((always_inline))
uint32_t rust_isa_bmi1_blsmsk(uint32_t a) { return _blsmsk_u32(a); }

__attribute__((always_inline))
uint32_t rust_isa_bmi1_blsr(uint32_t a) { return _blsr_u32(a); }
