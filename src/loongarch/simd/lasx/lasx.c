#include <stdint.h>
#include <lasxintrin.h>

__attribute__((always_inline))
void rust_isa_lasx_v256_load(const uint8_t *p, uint8_t *out) {
    *(v8i32 *)(void *)out = *(const v8i32 *)(const void *)p;
}

__attribute__((always_inline))
void rust_isa_lasx_v256_store(uint8_t *p, const uint8_t *value) {
    *(v8i32 *)(void *)p = *(const v8i32 *)(const void *)value;
}

__attribute__((always_inline))
void rust_isa_lasx_add(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i32 *)(void *)out = *(const v8i32 *)(const void *)a + *(const v8i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_mul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i32 *)(void *)out = *(const v8i32 *)(const void *)a * *(const v8i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_sub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i32 *)(void *)out = *(const v8i32 *)(const void *)a - *(const v8i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_add_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i64 *)(void *)out = *(const v4i64 *)(const void *)a + *(const v4i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_sub_i64(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i64 *)(void *)out = *(const v4i64 *)(const void *)a - *(const v4i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_mul_i32(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8i32 *)(void *)out = *(const v8i32 *)(const void *)a * *(const v8i32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_and(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i64 *)(void *)out = *(const v4i64 *)(const void *)a & *(const v4i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_or(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i64 *)(void *)out = *(const v4i64 *)(const void *)a | *(const v4i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_xor(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v4i64 *)(void *)out = *(const v4i64 *)(const void *)a ^ *(const v4i64 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_fadd(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8f32 *)(void *)out = *(const v8f32 *)(const void *)a + *(const v8f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_fsub(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8f32 *)(void *)out = *(const v8f32 *)(const void *)a - *(const v8f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_fmul(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8f32 *)(void *)out = *(const v8f32 *)(const void *)a * *(const v8f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_fdiv(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(v8f32 *)(void *)out = *(const v8f32 *)(const void *)a / *(const v8f32 *)(const void *)b;
}

__attribute__((always_inline))
void rust_isa_lasx_splat_i32(int32_t value, uint8_t *out) {
    *(v8i32 *)(void *)out = (v8i32){value, value, value, value, value, value, value, value};
}

__attribute__((always_inline))
void rust_isa_lasx_fmax_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256 *)(void *)out = __lasx_xvfmax_s(*(const __m256 *)(const void *)a,
                                             *(const __m256 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_fmin_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256 *)(void *)out = __lasx_xvfmin_s(*(const __m256 *)(const void *)a,
                                             *(const __m256 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_fcvt_s_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256 *)(void *)out = __lasx_xvfcvt_s_d(*(const __m256d *)(const void *)a,
                                               *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_ftint_w_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvftint_w_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_ffint_s_w(const uint8_t *a, uint8_t *out) {
    *(__m256 *)(void *)out = __lasx_xvffint_s_w(*(const __m256i *)(const void *)a);
}
