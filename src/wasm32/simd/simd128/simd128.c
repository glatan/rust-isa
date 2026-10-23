#include <stdbool.h>
#include <stdint.h>
#include <wasm_simd128.h>

typedef int __attribute__((vector_size(16), aligned(16))) rust_isa_v128;
#define RUST_ISA_LOAD(p) (*(const rust_isa_v128 *)(const void *)(p))
#define RUST_ISA_STORE(p, v) (*(rust_isa_v128 *)(void *)(p) = (rust_isa_v128)(v))

__attribute__((always_inline))
void rust_isa_simd128_v128_load(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_splat(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_splat(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load16_splat(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load16_splat(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load32_splat(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load32_splat(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load64_splat(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load64_splat(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8x8_s(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_load8x8(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8x8_u(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_load8x8(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load16x4_s(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_load16x4(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load16x4_u(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_load16x4(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load32x2_s(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_load32x2(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load32x2_u(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u64x2_load32x2(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load32_zero(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load32_zero(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load64_zero(const uint8_t *p, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load64_zero(p));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_0(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 0));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_1(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 1));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_2(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 2));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_3(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 3));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_4(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 4));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_5(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 5));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_6(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 6));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_7(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 7));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_8(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 8));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_9(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 9));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_10(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 10));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_11(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 11));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_12(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 12));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_13(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 13));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_14(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 14));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load8_lane_15(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 15));
}


__attribute__((always_inline))
void rust_isa_simd128_v128_load16_lane_0(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 0));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load16_lane_1(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 1));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load16_lane_2(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 2));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load16_lane_3(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 3));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load16_lane_4(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 4));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load16_lane_5(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 5));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load16_lane_6(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 6));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load16_lane_7(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 7));
}


__attribute__((always_inline))
void rust_isa_simd128_v128_load32_lane_0(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load32_lane(p, (v128_t)RUST_ISA_LOAD(vec), 0));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load32_lane_1(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load32_lane(p, (v128_t)RUST_ISA_LOAD(vec), 1));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load32_lane_2(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load32_lane(p, (v128_t)RUST_ISA_LOAD(vec), 2));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load32_lane_3(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load32_lane(p, (v128_t)RUST_ISA_LOAD(vec), 3));
}


__attribute__((always_inline))
void rust_isa_simd128_v128_load64_lane_0(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load64_lane(p, (v128_t)RUST_ISA_LOAD(vec), 0));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_load64_lane_1(const uint8_t *p, const uint8_t vec[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_load64_lane(p, (v128_t)RUST_ISA_LOAD(vec), 1));
}


__attribute__((always_inline))
void rust_isa_simd128_v128_store(uint8_t *p, const uint8_t a[16]) {
    wasm_v128_store(p, (v128_t)RUST_ISA_LOAD(a));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_0(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 0);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_1(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 1);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_2(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 2);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_3(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 3);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_4(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 4);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_5(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 5);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_6(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 6);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_7(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 7);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_8(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 8);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_9(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 9);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_10(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 10);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_11(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 11);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_12(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 12);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_13(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 13);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_14(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 14);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store8_lane_15(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store8_lane(p, (v128_t)RUST_ISA_LOAD(vec), 15);
}


__attribute__((always_inline))
void rust_isa_simd128_v128_store16_lane_0(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 0);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store16_lane_1(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 1);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store16_lane_2(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 2);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store16_lane_3(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 3);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store16_lane_4(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 4);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store16_lane_5(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 5);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store16_lane_6(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 6);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store16_lane_7(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store16_lane(p, (v128_t)RUST_ISA_LOAD(vec), 7);
}


__attribute__((always_inline))
void rust_isa_simd128_v128_store32_lane_0(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store32_lane(p, (v128_t)RUST_ISA_LOAD(vec), 0);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store32_lane_1(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store32_lane(p, (v128_t)RUST_ISA_LOAD(vec), 1);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store32_lane_2(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store32_lane(p, (v128_t)RUST_ISA_LOAD(vec), 2);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store32_lane_3(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store32_lane(p, (v128_t)RUST_ISA_LOAD(vec), 3);
}


__attribute__((always_inline))
void rust_isa_simd128_v128_store64_lane_0(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store64_lane(p, (v128_t)RUST_ISA_LOAD(vec), 0);
}

__attribute__((always_inline))
void rust_isa_simd128_v128_store64_lane_1(uint8_t *p, const uint8_t vec[16]) {
    wasm_v128_store64_lane(p, (v128_t)RUST_ISA_LOAD(vec), 1);
}


__attribute__((always_inline))
void rust_isa_simd128_i8x16_splat(int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_splat((int8_t)value));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_0(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 0);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_1(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 1);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_2(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 2);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_3(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 3);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_4(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 4);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_5(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 5);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_6(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 6);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_7(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 7);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_8(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 8);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_9(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 9);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_10(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 10);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_11(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 11);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_12(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 12);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_13(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 13);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_14(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 14);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_extract_lane_s_15(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 15);
}


__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_0(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 0);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_1(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 1);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_2(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 2);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_3(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 3);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_4(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 4);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_5(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 5);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_6(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 6);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_7(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 7);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_8(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 8);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_9(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 9);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_10(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 10);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_11(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 11);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_12(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 12);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_13(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 13);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_14(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 14);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i8x16_extract_lane_u_15(const uint8_t a[16]) {
    return (uint32_t)wasm_u8x16_extract_lane((v128_t)RUST_ISA_LOAD(a), 15);
}


__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_0(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 0, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_1(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 1, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_2(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 2, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_3(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 3, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_4(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 4, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_5(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 5, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_6(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 6, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_7(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 7, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_8(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 8, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_9(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 9, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_10(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 10, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_11(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 11, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_12(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 12, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_13(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 13, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_14(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 14, (int8_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_replace_lane_15(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_replace_lane((v128_t)RUST_ISA_LOAD(a), 15, (int8_t)value));
}


__attribute__((always_inline))
void rust_isa_simd128_i16x8_splat(int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_splat((int16_t)value));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i16x8_extract_lane_s_0(const uint8_t a[16]) {
    return (int32_t)wasm_i16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 0);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i16x8_extract_lane_s_1(const uint8_t a[16]) {
    return (int32_t)wasm_i16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 1);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i16x8_extract_lane_s_2(const uint8_t a[16]) {
    return (int32_t)wasm_i16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 2);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i16x8_extract_lane_s_3(const uint8_t a[16]) {
    return (int32_t)wasm_i16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 3);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i16x8_extract_lane_s_4(const uint8_t a[16]) {
    return (int32_t)wasm_i16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 4);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i16x8_extract_lane_s_5(const uint8_t a[16]) {
    return (int32_t)wasm_i16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 5);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i16x8_extract_lane_s_6(const uint8_t a[16]) {
    return (int32_t)wasm_i16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 6);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i16x8_extract_lane_s_7(const uint8_t a[16]) {
    return (int32_t)wasm_i16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 7);
}


__attribute__((always_inline))
uint32_t rust_isa_simd128_i16x8_extract_lane_u_0(const uint8_t a[16]) {
    return (uint32_t)wasm_u16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 0);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i16x8_extract_lane_u_1(const uint8_t a[16]) {
    return (uint32_t)wasm_u16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 1);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i16x8_extract_lane_u_2(const uint8_t a[16]) {
    return (uint32_t)wasm_u16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 2);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i16x8_extract_lane_u_3(const uint8_t a[16]) {
    return (uint32_t)wasm_u16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 3);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i16x8_extract_lane_u_4(const uint8_t a[16]) {
    return (uint32_t)wasm_u16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 4);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i16x8_extract_lane_u_5(const uint8_t a[16]) {
    return (uint32_t)wasm_u16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 5);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i16x8_extract_lane_u_6(const uint8_t a[16]) {
    return (uint32_t)wasm_u16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 6);
}

__attribute__((always_inline))
uint32_t rust_isa_simd128_i16x8_extract_lane_u_7(const uint8_t a[16]) {
    return (uint32_t)wasm_u16x8_extract_lane((v128_t)RUST_ISA_LOAD(a), 7);
}


__attribute__((always_inline))
void rust_isa_simd128_i16x8_replace_lane_0(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_replace_lane((v128_t)RUST_ISA_LOAD(a), 0, (int16_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_replace_lane_1(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_replace_lane((v128_t)RUST_ISA_LOAD(a), 1, (int16_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_replace_lane_2(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_replace_lane((v128_t)RUST_ISA_LOAD(a), 2, (int16_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_replace_lane_3(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_replace_lane((v128_t)RUST_ISA_LOAD(a), 3, (int16_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_replace_lane_4(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_replace_lane((v128_t)RUST_ISA_LOAD(a), 4, (int16_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_replace_lane_5(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_replace_lane((v128_t)RUST_ISA_LOAD(a), 5, (int16_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_replace_lane_6(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_replace_lane((v128_t)RUST_ISA_LOAD(a), 6, (int16_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_replace_lane_7(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_replace_lane((v128_t)RUST_ISA_LOAD(a), 7, (int16_t)value));
}


__attribute__((always_inline))
void rust_isa_simd128_i32x4_splat(int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_splat((int32_t)value));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i32x4_extract_lane_0(const uint8_t a[16]) {
    return (int32_t)wasm_i32x4_extract_lane((v128_t)RUST_ISA_LOAD(a), 0);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i32x4_extract_lane_1(const uint8_t a[16]) {
    return (int32_t)wasm_i32x4_extract_lane((v128_t)RUST_ISA_LOAD(a), 1);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i32x4_extract_lane_2(const uint8_t a[16]) {
    return (int32_t)wasm_i32x4_extract_lane((v128_t)RUST_ISA_LOAD(a), 2);
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i32x4_extract_lane_3(const uint8_t a[16]) {
    return (int32_t)wasm_i32x4_extract_lane((v128_t)RUST_ISA_LOAD(a), 3);
}


__attribute__((always_inline))
void rust_isa_simd128_i32x4_replace_lane_0(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_replace_lane((v128_t)RUST_ISA_LOAD(a), 0, (int32_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_replace_lane_1(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_replace_lane((v128_t)RUST_ISA_LOAD(a), 1, (int32_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_replace_lane_2(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_replace_lane((v128_t)RUST_ISA_LOAD(a), 2, (int32_t)value));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_replace_lane_3(const uint8_t a[16], int32_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_replace_lane((v128_t)RUST_ISA_LOAD(a), 3, (int32_t)value));
}


__attribute__((always_inline))
void rust_isa_simd128_i64x2_splat(int64_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_splat(value));
}

__attribute__((always_inline))
int64_t rust_isa_simd128_i64x2_extract_lane_0(const uint8_t a[16]) {
    return (int64_t)wasm_i64x2_extract_lane((v128_t)RUST_ISA_LOAD(a), 0);
}

__attribute__((always_inline))
int64_t rust_isa_simd128_i64x2_extract_lane_1(const uint8_t a[16]) {
    return (int64_t)wasm_i64x2_extract_lane((v128_t)RUST_ISA_LOAD(a), 1);
}


__attribute__((always_inline))
void rust_isa_simd128_i64x2_replace_lane_0(const uint8_t a[16], int64_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_replace_lane((v128_t)RUST_ISA_LOAD(a), 0, value));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_replace_lane_1(const uint8_t a[16], int64_t value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_replace_lane((v128_t)RUST_ISA_LOAD(a), 1, value));
}


__attribute__((always_inline))
void rust_isa_simd128_f32x4_splat(float value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_splat(value));
}

__attribute__((always_inline))
float rust_isa_simd128_f32x4_extract_lane_0(const uint8_t a[16]) {
    return (float)wasm_f32x4_extract_lane((v128_t)RUST_ISA_LOAD(a), 0);
}

__attribute__((always_inline))
float rust_isa_simd128_f32x4_extract_lane_1(const uint8_t a[16]) {
    return (float)wasm_f32x4_extract_lane((v128_t)RUST_ISA_LOAD(a), 1);
}

__attribute__((always_inline))
float rust_isa_simd128_f32x4_extract_lane_2(const uint8_t a[16]) {
    return (float)wasm_f32x4_extract_lane((v128_t)RUST_ISA_LOAD(a), 2);
}

__attribute__((always_inline))
float rust_isa_simd128_f32x4_extract_lane_3(const uint8_t a[16]) {
    return (float)wasm_f32x4_extract_lane((v128_t)RUST_ISA_LOAD(a), 3);
}


__attribute__((always_inline))
void rust_isa_simd128_f32x4_replace_lane_0(const uint8_t a[16], float value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_replace_lane((v128_t)RUST_ISA_LOAD(a), 0, value));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_replace_lane_1(const uint8_t a[16], float value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_replace_lane((v128_t)RUST_ISA_LOAD(a), 1, value));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_replace_lane_2(const uint8_t a[16], float value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_replace_lane((v128_t)RUST_ISA_LOAD(a), 2, value));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_replace_lane_3(const uint8_t a[16], float value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_replace_lane((v128_t)RUST_ISA_LOAD(a), 3, value));
}


__attribute__((always_inline))
void rust_isa_simd128_f64x2_splat(double value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_splat(value));
}

__attribute__((always_inline))
double rust_isa_simd128_f64x2_extract_lane_0(const uint8_t a[16]) {
    return (double)wasm_f64x2_extract_lane((v128_t)RUST_ISA_LOAD(a), 0);
}

__attribute__((always_inline))
double rust_isa_simd128_f64x2_extract_lane_1(const uint8_t a[16]) {
    return (double)wasm_f64x2_extract_lane((v128_t)RUST_ISA_LOAD(a), 1);
}


__attribute__((always_inline))
void rust_isa_simd128_f64x2_replace_lane_0(const uint8_t a[16], double value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_replace_lane((v128_t)RUST_ISA_LOAD(a), 0, value));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_replace_lane_1(const uint8_t a[16], double value, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_replace_lane((v128_t)RUST_ISA_LOAD(a), 1, value));
}


__attribute__((always_inline))
void rust_isa_simd128_i8x16_eq(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_eq((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_ne(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_ne((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_lt_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_lt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_lt_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u8x16_lt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_gt_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_gt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_gt_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u8x16_gt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_le_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_le((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_le_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u8x16_le((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_ge_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_ge((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_ge_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u8x16_ge((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_eq(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_eq((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_ne(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_ne((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_lt_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_lt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_lt_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_lt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_gt_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_gt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_gt_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_gt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_le_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_le((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_le_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_le((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_ge_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_ge((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_ge_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_ge((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_eq(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_eq((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_ne(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_ne((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_lt_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_lt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_lt_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_lt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_gt_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_gt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_gt_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_gt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_le_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_le((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_le_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_le((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_ge_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_ge((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_ge_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_ge((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_eq(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_eq((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_ne(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_ne((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_lt_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_lt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_gt_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_gt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_le_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_le((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_ge_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_ge((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_eq(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_eq((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_ne(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_ne((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_lt(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_lt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_gt(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_gt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_le(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_le((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_ge(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_ge((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_eq(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_eq((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_ne(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_ne((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_lt(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_lt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_gt(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_gt((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_le(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_le((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_ge(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_ge((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_not(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_not((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_and(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_and((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_or(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_or((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_xor(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_xor((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_andnot(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_andnot((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_v128_any_true(const uint8_t a[16]) {
    return (int32_t)wasm_v128_any_true((v128_t)RUST_ISA_LOAD(a));
}

__attribute__((always_inline))
void rust_isa_simd128_v128_bitselect(const uint8_t a[16], const uint8_t b[16], const uint8_t c[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_v128_bitselect((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b), (v128_t)RUST_ISA_LOAD(c)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_abs(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_abs((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_neg(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_neg((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_all_true(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_all_true((v128_t)RUST_ISA_LOAD(a));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i8x16_bitmask(const uint8_t a[16]) {
    return (int32_t)wasm_i8x16_bitmask((v128_t)RUST_ISA_LOAD(a));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_popcnt(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_popcnt((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_shl(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_shl((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_shr_s(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_shr((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_shr_u(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u8x16_shr((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_add(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_add((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_add_sat_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_add_sat((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_add_sat_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u8x16_add_sat((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_sub(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_sub((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_sub_sat_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_sub_sat((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_sub_sat_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u8x16_sub_sat((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_min_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_min((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_min_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u8x16_min((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_max_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_max((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_max_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u8x16_max((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_avgr_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u8x16_avgr((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_abs(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_abs((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_neg(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_neg((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i16x8_all_true(const uint8_t a[16]) {
    return (int32_t)wasm_i16x8_all_true((v128_t)RUST_ISA_LOAD(a));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i16x8_bitmask(const uint8_t a[16]) {
    return (int32_t)wasm_i16x8_bitmask((v128_t)RUST_ISA_LOAD(a));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_shl(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_shl((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_shr_s(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_shr((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_shr_u(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_shr((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_add(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_add((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_add_sat_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_add_sat((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_add_sat_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_add_sat((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_sub(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_sub((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_sub_sat_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_sub_sat((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_sub_sat_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_sub_sat((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_mul(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_mul((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_min_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_min((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_min_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_min((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_max_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_max((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_max_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_max((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_avgr_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_avgr((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_abs(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_abs((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_neg(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_neg((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i32x4_all_true(const uint8_t a[16]) {
    return (int32_t)wasm_i32x4_all_true((v128_t)RUST_ISA_LOAD(a));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i32x4_bitmask(const uint8_t a[16]) {
    return (int32_t)wasm_i32x4_bitmask((v128_t)RUST_ISA_LOAD(a));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_shl(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_shl((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_shr_s(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_shr((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_shr_u(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_shr((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_add(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_add((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_sub(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_sub((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_mul(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_mul((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_min_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_min((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_min_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_min((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_max_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_max((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_max_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_max((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_dot_i16x8_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_dot_i16x8((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_abs(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_abs((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_neg(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_neg((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i64x2_all_true(const uint8_t a[16]) {
    return (int32_t)wasm_i64x2_all_true((v128_t)RUST_ISA_LOAD(a));
}

__attribute__((always_inline))
int32_t rust_isa_simd128_i64x2_bitmask(const uint8_t a[16]) {
    return (int32_t)wasm_i64x2_bitmask((v128_t)RUST_ISA_LOAD(a));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_shl(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_shl((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_shr_s(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_shr((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_shr_u(const uint8_t a[16], uint32_t count, uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u64x2_shr((v128_t)RUST_ISA_LOAD(a), count));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_add(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_add((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_sub(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_sub((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_mul(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_mul((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_abs(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_abs((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_neg(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_neg((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_sqrt(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_sqrt((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_ceil(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_ceil((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_floor(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_floor((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_trunc(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_trunc((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_nearest(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_nearest((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_add(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_add((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_sub(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_sub((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_mul(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_mul((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_div(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_div((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_min(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_min((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_max(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_max((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_pmin(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_pmin((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_pmax(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_pmax((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_abs(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_abs((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_neg(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_neg((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_sqrt(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_sqrt((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_ceil(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_ceil((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_floor(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_floor((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_trunc(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_trunc((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_nearest(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_nearest((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_add(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_add((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_sub(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_sub((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_mul(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_mul((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_div(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_div((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_min(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_min((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_max(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_max((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_pmin(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_pmin((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_pmax(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_pmax((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_trunc_sat_f32x4_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_trunc_sat_f32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_trunc_sat_f32x4_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_trunc_sat_f32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_convert_i32x4_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_convert_i32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_convert_i32x4_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_convert_u32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_convert_low_i32x4_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_convert_low_i32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_convert_low_i32x4_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_convert_low_u32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_trunc_sat_f64x2_s_zero(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_trunc_sat_f64x2_zero((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_trunc_sat_f64x2_u_zero(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_trunc_sat_f64x2_zero((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f32x4_demote_f64x2_zero(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f32x4_demote_f64x2_zero((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_f64x2_promote_low_f32x4(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_f64x2_promote_low_f32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_swizzle(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_swizzle((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_narrow_i16x8_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i8x16_narrow_i16x8((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i8x16_narrow_i16x8_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u8x16_narrow_i16x8((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_narrow_i32x4_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_narrow_i32x4((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_narrow_i32x4_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_narrow_i32x4((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_extend_low_i8x16_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_extend_low_i8x16((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_extend_high_i8x16_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_extend_high_i8x16((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_extend_low_i8x16_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_extend_low_u8x16((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_extend_high_i8x16_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_extend_high_u8x16((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_extend_low_i16x8_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_extend_low_i16x8((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_extend_high_i16x8_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_extend_high_i16x8((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_extend_low_i16x8_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_extend_low_u16x8((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_extend_high_i16x8_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_extend_high_u16x8((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_extend_low_i32x4_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_extend_low_i32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_extend_high_i32x4_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_extend_high_i32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_extend_low_i32x4_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u64x2_extend_low_u32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_extend_high_i32x4_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u64x2_extend_high_u32x4((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_extadd_pairwise_i8x16_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_extadd_pairwise_i8x16((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_extadd_pairwise_i8x16_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_extadd_pairwise_u8x16((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_extadd_pairwise_i16x8_s(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_extadd_pairwise_i16x8((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_extadd_pairwise_i16x8_u(const uint8_t a[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_extadd_pairwise_u16x8((v128_t)RUST_ISA_LOAD(a)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_extmul_low_i8x16_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_extmul_low_i8x16((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_extmul_high_i8x16_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_extmul_high_i8x16((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_extmul_low_i8x16_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_extmul_low_u8x16((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_extmul_high_i8x16_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u16x8_extmul_high_u8x16((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_extmul_low_i16x8_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_extmul_low_i16x8((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_extmul_high_i16x8_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i32x4_extmul_high_i16x8((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_extmul_low_i16x8_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_extmul_low_u16x8((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i32x4_extmul_high_i16x8_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u32x4_extmul_high_u16x8((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_extmul_low_i32x4_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_extmul_low_i32x4((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_extmul_high_i32x4_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i64x2_extmul_high_i32x4((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_extmul_low_i32x4_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u64x2_extmul_low_u32x4((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i64x2_extmul_high_i32x4_u(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_u64x2_extmul_high_u32x4((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

__attribute__((always_inline))
void rust_isa_simd128_i16x8_q15mulr_sat_s(const uint8_t a[16], const uint8_t b[16], uint8_t out[16]) {
    RUST_ISA_STORE(out, wasm_i16x8_q15mulr_sat((v128_t)RUST_ISA_LOAD(a), (v128_t)RUST_ISA_LOAD(b)));
}

