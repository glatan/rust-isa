#include <stdint.h>
#include <wasm_simd128.h>

typedef int __attribute__((vector_size(16), aligned(16))) rust_isa_v128;
#define RUST_ISA_LOAD(p) (*(const rust_isa_v128 *)(const void *)(p))
#define RUST_ISA_STORE(p, v) (*(rust_isa_v128 *)(void *)(p) = (rust_isa_v128)(v))

__attribute__((always_inline))
void rust_isa_relaxed_simd_f32x4_relaxed_madd(const uint8_t a[16], const uint8_t b[16], const uint8_t c[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_relaxed_madd((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b), (v128_t)RUST_ISA_LOAD(c)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_f32x4_relaxed_nmadd(const uint8_t a[16], const uint8_t b[16], const uint8_t c[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_relaxed_nmadd((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b), (v128_t)RUST_ISA_LOAD(c)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_f64x2_relaxed_madd(const uint8_t a[16], const uint8_t b[16], const uint8_t c[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_relaxed_madd((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b), (v128_t)RUST_ISA_LOAD(c)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_f64x2_relaxed_nmadd(const uint8_t a[16], const uint8_t b[16], const uint8_t c[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_relaxed_nmadd((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b), (v128_t)RUST_ISA_LOAD(c)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i8x16_relaxed_laneselect(const uint8_t a[16], const uint8_t b[16], const uint8_t c[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_relaxed_laneselect((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b), (v128_t)RUST_ISA_LOAD(c)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i16x8_relaxed_laneselect(const uint8_t a[16], const uint8_t b[16], const uint8_t c[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_relaxed_laneselect((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b), (v128_t)RUST_ISA_LOAD(c)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i32x4_relaxed_laneselect(const uint8_t a[16], const uint8_t b[16], const uint8_t c[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_relaxed_laneselect((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b), (v128_t)RUST_ISA_LOAD(c)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i64x2_relaxed_laneselect(const uint8_t a[16], const uint8_t b[16], const uint8_t c[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_relaxed_laneselect((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b), (v128_t)RUST_ISA_LOAD(c)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i8x16_relaxed_swizzle(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_relaxed_swizzle((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_f32x4_relaxed_min(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_relaxed_min((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_f32x4_relaxed_max(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_relaxed_max((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_f64x2_relaxed_min(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_relaxed_min((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_f64x2_relaxed_max(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_relaxed_max((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i32x4_relaxed_trunc_f32x4_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_relaxed_trunc_f32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i32x4_relaxed_trunc_f32x4_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_relaxed_trunc_f32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i32x4_relaxed_trunc_f64x2_s_zero(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_relaxed_trunc_f64x2_zero((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i32x4_relaxed_trunc_f64x2_u_zero(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_relaxed_trunc_f64x2_zero((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i16x8_relaxed_q15mulr_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_relaxed_q15mulr((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i16x8_relaxed_dot_i8x16_i7x16_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_relaxed_dot_i8x16_i7x16((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_relaxed_simd_i32x4_relaxed_dot_i8x16_i7x16_add_s(const uint8_t a[16], const uint8_t b[16], const uint8_t c[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_relaxed_dot_i8x16_i7x16_add((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b), (v128_t)RUST_ISA_LOAD(c)));
}
