#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
uint32_t rust_isa_bmi2_bzhi(uint32_t a, uint32_t index) { return _bzhi_u32(a, index); }

__attribute__((always_inline))
uint32_t rust_isa_bmi2_pdep(uint32_t a, uint32_t mask) { return _pdep_u32(a, mask); }

__attribute__((always_inline))
uint32_t rust_isa_bmi2_pext(uint32_t a, uint32_t mask) { return _pext_u32(a, mask); }
