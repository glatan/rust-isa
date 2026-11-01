#include <stdint.h>
#include <lsxintrin.h>
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

void rust_isa_lasx_xvfcmp_caf_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_caf_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_saf_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_saf_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_clt_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_clt_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_slt_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_slt_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_ceq_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_ceq_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_seq_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_seq_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cle_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cle_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sle_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sle_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cun_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cun_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sun_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sun_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cult_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cult_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sult_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sult_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cueq_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cueq_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sueq_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sueq_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cule_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cule_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sule_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sule_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cne_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cne_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sne_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sne_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cor_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cor_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sor_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sor_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cune_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cune_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sune_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sune_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_caf_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_caf_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_saf_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_saf_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_clt_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_clt_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_slt_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_slt_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_ceq_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_ceq_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_seq_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_seq_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cle_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cle_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sle_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sle_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cun_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cun_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sun_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sun_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cult_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cult_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sult_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sult_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cueq_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cueq_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sueq_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sueq_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cule_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cule_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sule_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sule_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cne_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cne_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sne_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sne_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cor_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cor_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sor_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sor_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_cune_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_cune_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfcmp_sune_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfcmp_sune_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

void rust_isa_lasx_xvfmadd_s(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfmadd_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

void rust_isa_lasx_xvfmadd_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfmadd_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

void rust_isa_lasx_xvfmsub_s(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfmsub_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

void rust_isa_lasx_xvfmsub_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfmsub_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

void rust_isa_lasx_xvfnmadd_s(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfnmadd_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

void rust_isa_lasx_xvfnmadd_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfnmadd_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

void rust_isa_lasx_xvfnmsub_s(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfnmsub_s(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

void rust_isa_lasx_xvfnmsub_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvfnmsub_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitsel_v(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitsel_v(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvshuf_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvshuf_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvldx(const void *p, long offset, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvldx(p, offset);
}

__attribute__((always_inline))
void rust_isa_lasx_xvstx(const uint8_t *value, void *p, long offset) {
    __lasx_xvstx(*(const __m256i *)(const void *)value, p, offset);
}

__attribute__((always_inline))
void rust_isa_lasx_xvseq_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvseq_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvseq_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvseq_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvseq_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvseq_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvseq_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvseq_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsle_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsle_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsle_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsle_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsle_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsle_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsle_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsle_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsle_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsle_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsle_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsle_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsle_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsle_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsle_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsle_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvslt_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvslt_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvslt_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvslt_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvslt_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvslt_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvslt_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvslt_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvslt_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvslt_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvslt_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvslt_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvslt_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvslt_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvslt_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvslt_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_h_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_w_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_d_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_q_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwev_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwev_h_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwev_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwev_w_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwev_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwev_d_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwev_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwev_q_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_h_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_w_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_d_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_q_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwod_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwod_h_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwod_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwod_w_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwod_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwod_d_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwod_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwod_q_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_h_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_w_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_d_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_q_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwev_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwev_h_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwev_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwev_w_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwev_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwev_d_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwev_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwev_q_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_h_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_w_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_d_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_q_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwod_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwod_h_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwod_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwod_w_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwod_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwod_d_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsubwod_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsubwod_q_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_h_bu_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_h_bu_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_w_hu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_w_hu_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_d_wu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_d_wu_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwev_q_du_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwev_q_du_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_h_bu_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_h_bu_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_w_hu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_w_hu_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_d_wu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_d_wu_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvaddwod_q_du_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvaddwod_q_du_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsadd_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsadd_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsadd_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsadd_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsadd_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsadd_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsadd_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsadd_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssub_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssub_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssub_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssub_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssub_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssub_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssub_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssub_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsadd_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsadd_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsadd_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsadd_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsadd_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsadd_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsadd_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsadd_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssub_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssub_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssub_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssub_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssub_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssub_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssub_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssub_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhaddw_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhaddw_h_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhaddw_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhaddw_w_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhaddw_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhaddw_d_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhaddw_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhaddw_q_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhsubw_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhsubw_h_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhsubw_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhsubw_w_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhsubw_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhsubw_d_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhsubw_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhsubw_q_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhaddw_hu_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhaddw_hu_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhaddw_wu_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhaddw_wu_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhaddw_du_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhaddw_du_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhaddw_qu_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhaddw_qu_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhsubw_hu_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhsubw_hu_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhsubw_wu_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhsubw_wu_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhsubw_du_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhsubw_du_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvhsubw_qu_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvhsubw_qu_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvadda_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvadda_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvadda_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvadda_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvadda_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvadda_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvadda_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvadda_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvabsd_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvabsd_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvabsd_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvabsd_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvabsd_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvabsd_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvabsd_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvabsd_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvabsd_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvabsd_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvabsd_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvabsd_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvabsd_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvabsd_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvabsd_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvabsd_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavg_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavg_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavg_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavg_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavg_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavg_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavg_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavg_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavg_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavg_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavg_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavg_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavg_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavg_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavg_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavg_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavgr_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavgr_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavgr_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavgr_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavgr_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavgr_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavgr_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavgr_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavgr_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavgr_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavgr_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavgr_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavgr_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavgr_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvavgr_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvavgr_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmax_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmax_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmax_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmax_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmax_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmax_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmax_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmax_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmin_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmin_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmin_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmin_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmin_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmin_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmin_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmin_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmax_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmax_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmax_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmax_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmax_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmax_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmax_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmax_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmin_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmin_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmin_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmin_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmin_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmin_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmin_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmin_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmul_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmul_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmul_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmul_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmuh_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmuh_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmuh_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmuh_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmuh_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmuh_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmuh_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmuh_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmuh_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmuh_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmuh_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmuh_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmuh_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmuh_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmuh_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmuh_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_h_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_w_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_d_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_q_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_h_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_h_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_w_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_w_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_d_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_d_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_q_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_q_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_h_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_w_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_d_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_q_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_h_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_h_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_w_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_w_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_d_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_d_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_q_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_q_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_h_bu_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_h_bu_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_w_hu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_w_hu_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_d_wu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_d_wu_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwev_q_du_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwev_q_du_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_h_bu_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_h_bu_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_w_hu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_w_hu_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_d_wu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_d_wu_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmulwod_q_du_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmulwod_q_du_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmadd_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmadd_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmadd_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmadd_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmadd_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmadd_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmadd_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmadd_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmsub_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmsub_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmsub_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmsub_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmsub_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmsub_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmsub_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmsub_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_h_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_h_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_w_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_w_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_d_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_d_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_q_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_q_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_h_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_h_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_w_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_w_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_d_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_d_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_q_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_q_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_h_bu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_h_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_w_hu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_w_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_d_wu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_d_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_q_du(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_q_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_h_bu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_h_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_w_hu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_w_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_d_wu(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_d_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_q_du(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_q_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_h_bu_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_h_bu_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_w_hu_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_w_hu_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_d_wu_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_d_wu_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwev_q_du_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwev_q_du_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_h_bu_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_h_bu_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_w_hu_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_w_hu_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_d_wu_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_d_wu_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmaddwod_q_du_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmaddwod_q_du_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvdiv_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvdiv_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvdiv_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvdiv_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvdiv_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvdiv_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvdiv_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvdiv_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmod_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmod_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmod_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmod_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmod_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmod_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmod_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmod_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvdiv_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvdiv_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvdiv_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvdiv_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvdiv_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvdiv_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvdiv_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvdiv_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmod_bu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmod_bu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmod_hu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmod_hu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmod_wu(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmod_wu(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmod_du(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmod_du(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsll_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsll_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsll_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsll_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsll_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsll_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsll_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsll_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrl_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrl_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrl_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrl_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrl_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrl_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrl_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrl_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsra_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsra_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsra_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsra_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsra_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsra_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsra_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsra_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvrotr_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvrotr_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvrotr_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvrotr_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvrotr_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvrotr_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvrotr_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvrotr_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrlr_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrlr_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrlr_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrlr_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrlr_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrlr_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrlr_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrlr_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrar_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrar_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrar_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrar_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrar_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrar_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrar_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrar_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrln_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrln_b_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrln_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrln_h_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrln_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrln_w_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsran_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsran_b_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsran_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsran_h_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsran_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsran_w_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrlrn_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrlrn_b_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrlrn_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrlrn_h_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrlrn_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrlrn_w_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrarn_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrarn_b_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrarn_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrarn_h_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsrarn_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsrarn_w_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrln_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrln_b_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrln_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrln_h_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrln_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrln_w_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssran_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssran_b_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssran_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssran_h_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssran_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssran_w_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrlrn_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrlrn_b_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrlrn_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrlrn_h_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrlrn_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrlrn_w_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrarn_b_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrarn_b_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrarn_h_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrarn_h_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrarn_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrarn_w_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrln_bu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrln_bu_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrln_hu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrln_hu_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrln_wu_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrln_wu_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssran_bu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssran_bu_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssran_hu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssran_hu_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssran_wu_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssran_wu_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrlrn_bu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrlrn_bu_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrlrn_hu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrlrn_hu_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrlrn_wu_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrlrn_wu_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrarn_bu_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrarn_bu_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrarn_hu_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrarn_hu_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvssrarn_wu_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvssrarn_wu_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitclr_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitclr_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitclr_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitclr_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitclr_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitclr_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitclr_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitclr_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitset_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitset_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitset_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitset_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitset_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitset_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitset_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitset_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitrev_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitrev_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitrev_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitrev_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitrev_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitrev_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvbitrev_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvbitrev_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpackev_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpackev_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpackev_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpackev_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpackev_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpackev_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpackev_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpackev_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpackod_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpackod_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpackod_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpackod_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpackod_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpackod_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpackod_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpackod_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvilvl_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvilvl_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvilvl_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvilvl_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvilvl_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvilvl_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvilvl_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvilvl_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvilvh_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvilvh_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvilvh_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvilvh_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvilvh_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvilvh_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvilvh_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvilvh_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpickev_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpickev_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpickev_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpickev_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpickev_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpickev_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpickev_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpickev_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpickod_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpickod_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpickod_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpickod_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpickod_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpickod_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpickod_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpickod_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplve_b(const uint8_t *a, int b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvreplve_b(*(const __m256i *)(const void *)a, b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplve_h(const uint8_t *a, int b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvreplve_h(*(const __m256i *)(const void *)a, b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplve_w(const uint8_t *a, int b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvreplve_w(*(const __m256i *)(const void *)a, b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplve_d(const uint8_t *a, int b, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvreplve_d(*(const __m256i *)(const void *)a, b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvand_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvand_v(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvor_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvor_v(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvxor_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvxor_v(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvnor_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvnor_v(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvandn_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvandn_v(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvorn_v(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvorn_v(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstp_b(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrstp_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstp_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrstp_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvadd_q(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvadd_q(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsub_q(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsub_q(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsigncov_b(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsigncov_b(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsigncov_h(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsigncov_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsigncov_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsigncov_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvsigncov_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvsigncov_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfcvt_h_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfcvt_h_s(*(const __m256 *)(const void *)a, *(const __m256 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftint_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftint_w_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrm_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrm_w_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrp_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrp_w_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrz_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrz_w_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrne_w_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrne_w_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvshuf_h(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvshuf_h(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvshuf_w(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvshuf_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvshuf_d(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvshuf_d(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b, *(const __m256i *)(const void *)c);
}

__attribute__((always_inline))
void rust_isa_lasx_xvperm_w(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvperm_w(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvclo_b(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvclo_b(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvclo_h(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvclo_h(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvclo_w(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvclo_w(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvclo_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvclo_d(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvclz_b(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvclz_b(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvclz_h(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvclz_h(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvclz_w(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvclz_w(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvclz_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvclz_d(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpcnt_b(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpcnt_b(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpcnt_h(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpcnt_h(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpcnt_w(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpcnt_w(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvpcnt_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvpcnt_d(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvneg_w(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvneg_w(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmskltz_b(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmskltz_b(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmskltz_h(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmskltz_h(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmskltz_w(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmskltz_w(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmskltz_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmskltz_d(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmskgez_b(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmskgez_b(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvmsknz_b(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvmsknz_b(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfclass_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfclass_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfclass_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfclass_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftint_w_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftint_w_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftint_l_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftint_l_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrm_w_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrm_w_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrm_l_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrm_l_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrp_w_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrp_w_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrp_l_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrp_l_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrz_w_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrz_w_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrz_l_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrz_l_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrne_w_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrne_w_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrne_l_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrne_l_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftint_wu_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftint_wu_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftint_lu_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftint_lu_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrz_wu_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrz_wu_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrz_lu_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrz_lu_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintl_l_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintl_l_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftinth_l_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftinth_l_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrml_l_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrml_l_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrmh_l_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrmh_l_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrpl_l_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrpl_l_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrph_l_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrph_l_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrzl_l_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrzl_l_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrzh_l_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrzh_l_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrnel_l_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrnel_l_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvftintrneh_l_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvftintrneh_l_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvexth_h_b(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvexth_h_b(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvexth_w_h(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvexth_w_h(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvexth_d_w(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvexth_d_w(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvexth_q_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvexth_q_d(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvexth_hu_bu(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvexth_hu_bu(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvexth_wu_hu(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvexth_wu_hu(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvexth_du_wu(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvexth_du_wu(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvexth_qu_du(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvexth_qu_du(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplgr2vr_b(int a, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvreplgr2vr_b(a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplgr2vr_h(int a, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvreplgr2vr_h(a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplgr2vr_w(int a, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvreplgr2vr_w(a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplgr2vr_d(long a, uint8_t *out) {
    *(__m256i *)(void *)out = __lasx_xvreplgr2vr_d(a);
}

__attribute__((always_inline))
void rust_isa_lasx_vext2xv_w_b(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_vext2xv_w_b(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_vext2xv_d_b(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_vext2xv_d_b(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_vext2xv_w_h(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_vext2xv_w_h(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_vext2xv_d_h(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_vext2xv_d_h(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_vext2xv_d_w(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_vext2xv_d_w(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_vext2xv_hu_bu(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_vext2xv_hu_bu(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_vext2xv_wu_bu(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_vext2xv_wu_bu(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_vext2xv_du_bu(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_vext2xv_du_bu(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_vext2xv_wu_hu(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_vext2xv_wu_hu(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_vext2xv_du_hu(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_vext2xv_du_hu(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_vext2xv_du_wu(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_vext2xv_du_wu(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplve0_b(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvreplve0_b(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplve0_h(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvreplve0_h(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplve0_w(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvreplve0_w(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplve0_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvreplve0_d(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvreplve0_q(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvreplve0_q(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvextl_q_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvextl_q_d(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvextl_qu_du(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvextl_qu_du(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfadd_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfadd_s(*(const __m256 *)(const void *)a, *(const __m256 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfadd_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfadd_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfsub_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfsub_s(*(const __m256 *)(const void *)a, *(const __m256 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfsub_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfsub_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfmul_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfmul_s(*(const __m256 *)(const void *)a, *(const __m256 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfmul_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfmul_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfdiv_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfdiv_s(*(const __m256 *)(const void *)a, *(const __m256 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfdiv_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfdiv_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfmax_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfmax_s(*(const __m256 *)(const void *)a, *(const __m256 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfmax_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfmax_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfmin_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfmin_s(*(const __m256 *)(const void *)a, *(const __m256 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfmin_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfmin_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfmaxa_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfmaxa_s(*(const __m256 *)(const void *)a, *(const __m256 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfmaxa_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfmaxa_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfmina_s(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfmina_s(*(const __m256 *)(const void *)a, *(const __m256 *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfmina_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfmina_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfcvt_s_d(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfcvt_s_d(*(const __m256d *)(const void *)a, *(const __m256d *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvffint_s_l(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvffint_s_l(*(const __m256i *)(const void *)a, *(const __m256i *)(const void *)b);
}

__attribute__((always_inline))
void rust_isa_lasx_xvflogb_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvflogb_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvflogb_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvflogb_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfsqrt_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfsqrt_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfsqrt_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfsqrt_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrecip_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrecip_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrecip_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrecip_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrsqrt_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrsqrt_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrsqrt_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrsqrt_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))

__attribute__((always_inline))

__attribute__((always_inline))

__attribute__((always_inline))

__attribute__((always_inline))
void rust_isa_lasx_xvfrint_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrint_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrint_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrint_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrintrm_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrintrm_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrintrm_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrintrm_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrintrp_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrintrp_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrintrp_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrintrp_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrintrz_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrintrz_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrintrz_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrintrz_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrintrne_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrintrne_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrintrne_d(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfrintrne_d(*(const __m256d *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfcvtl_s_h(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfcvtl_s_h(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfcvth_s_h(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfcvth_s_h(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfcvtl_d_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfcvtl_d_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvfcvth_d_s(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvfcvth_d_s(*(const __m256 *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvffint_s_w(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvffint_s_w(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvffint_s_wu(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvffint_s_wu(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvffint_d_l(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvffint_d_l(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvffint_d_lu(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvffint_d_lu(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvffintl_d_w(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvffintl_d_w(*(const __m256i *)(const void *)a);
}

__attribute__((always_inline))
void rust_isa_lasx_xvffinth_d_w(const uint8_t *a, uint8_t *out) {
    *(__m256i *)(void *)out = (__m256i)__lasx_xvffinth_d_w(*(const __m256i *)(const void *)a);
}
__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_0(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 0);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 0);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_1(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 1);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 1);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_2(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 2);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 2);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_3(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 3);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 3);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_4(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 4);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 4);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_5(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 5);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 5);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_6(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 6);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 6);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_7(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 7);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 7);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 8);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 8);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_9(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 9);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 9);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_10(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 10);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 10);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_11(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 11);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 11);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_12(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 12);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 12);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_13(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 13);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 13);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_14(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 14);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 14);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_15(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 15);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 15);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 16);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 16);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_17(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 17);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 17);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_18(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 18);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 18);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_19(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 19);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 19);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_20(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 20);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 20);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_21(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 21);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 21);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_22(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 22);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 22);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_23(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 23);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 23);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_24(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 24);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 24);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_25(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 25);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 25);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_26(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 26);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 26);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_27(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 27);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 27);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_28(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 28);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 28);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_29(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 29);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 29);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_30(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 30);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 30);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_b_31(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_b(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 31);
    __m128i hi = __lsx_vfrstpi_b(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 31);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_0(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 0);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 0);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_1(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 1);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 1);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_2(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 2);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 2);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_3(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 3);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 3);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_4(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 4);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 4);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_5(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 5);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 5);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_6(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 6);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 6);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_7(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 7);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 7);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_8(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 8);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 8);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_9(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 9);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 9);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_10(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 10);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 10);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_11(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 11);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 11);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_12(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 12);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 12);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_13(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 13);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 13);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_14(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 14);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 14);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_15(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 15);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 15);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_16(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 16);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 16);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_17(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 17);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 17);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_18(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 18);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 18);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_19(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 19);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 19);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_20(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 20);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 20);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_21(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 21);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 21);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_22(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 22);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 22);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_23(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 23);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 23);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_24(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 24);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 24);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_25(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 25);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 25);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_26(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 26);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 26);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_27(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 27);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 27);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_28(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 28);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 28);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_29(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 29);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 29);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_30(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 30);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 30);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}

__attribute__((always_inline))
void rust_isa_lasx_xvfrstpi_h_31(const uint8_t *a, const uint8_t *b, uint8_t *out) {
    __m128i lo = __lsx_vfrstpi_h(*(const __m128i *)(const void *)a, *(const __m128i *)(const void *)b, 31);
    __m128i hi = __lsx_vfrstpi_h(*(const __m128i *)(const void *)(a + 16), *(const __m128i *)(const void *)(b + 16), 31);
    *(__m128i *)(void *)out = lo;
    *(__m128i *)(void *)(out + 16) = hi;
}
