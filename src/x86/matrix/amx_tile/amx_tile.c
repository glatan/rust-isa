#include <stdint.h>
#include <immintrin.h>

#if defined(__x86_64__)
#include <amxintrin.h>

__attribute__((always_inline))
void rust_isa_amx_tile_load_config(const uint8_t *config) {
    _tile_loadconfig(config);
}

__attribute__((always_inline))
void rust_isa_amx_tile_release(void) {
    _tile_release();
}

__attribute__((always_inline))
void rust_isa_amx_tile_zero_0(void) { _tile_zero(0); }
__attribute__((always_inline))
void rust_isa_amx_tile_zero_1(void) { _tile_zero(1); }
__attribute__((always_inline))
void rust_isa_amx_tile_zero_2(void) { _tile_zero(2); }
__attribute__((always_inline))
void rust_isa_amx_tile_zero_3(void) { _tile_zero(3); }
__attribute__((always_inline))
void rust_isa_amx_tile_zero_4(void) { _tile_zero(4); }
__attribute__((always_inline))
void rust_isa_amx_tile_zero_5(void) { _tile_zero(5); }
__attribute__((always_inline))
void rust_isa_amx_tile_zero_6(void) { _tile_zero(6); }
__attribute__((always_inline))
void rust_isa_amx_tile_zero_7(void) { _tile_zero(7); }
#endif
