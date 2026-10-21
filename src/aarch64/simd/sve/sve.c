#include <arm_sve.h>
#include <stddef.h>

void rust_isa_sve_add_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t va = svld1_f32(pg, a + i);
        svfloat32_t vb = svld1_f32(pg, b + i);
        svst1_f32(pg, out + i, svadd_f32_z(pg, va, vb));
    }
}

void rust_isa_sve_mul_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t va = svld1_f32(pg, a + i);
        svfloat32_t vb = svld1_f32(pg, b + i);
        svst1_f32(pg, out + i, svmul_f32_z(pg, va, vb));
    }
}

void rust_isa_sve_add_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t va = svld1_s32(pg, a + i);
        svint32_t vb = svld1_s32(pg, b + i);
        svst1_s32(pg, out + i, svadd_s32_z(pg, va, vb));
    }
}

void rust_isa_sve_mul_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t va = svld1_s32(pg, a + i);
        svint32_t vb = svld1_s32(pg, b + i);
        svst1_s32(pg, out + i, svmul_s32_z(pg, va, vb));
    }
}

void rust_isa_sve_add_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t va = svld1_f64(pg, a + i);
        svfloat64_t vb = svld1_f64(pg, b + i);
        svst1_f64(pg, out + i, svadd_f64_z(pg, va, vb));
    }
}

void rust_isa_sve_mul_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t va = svld1_f64(pg, a + i);
        svfloat64_t vb = svld1_f64(pg, b + i);
        svst1_f64(pg, out + i, svmul_f64_z(pg, va, vb));
    }
}

void rust_isa_sve_sub_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t va = svld1_f32(pg, a + i);
        svfloat32_t vb = svld1_f32(pg, b + i);
        svst1_f32(pg, out + i, svsub_f32_z(pg, va, vb));
    }
}

void rust_isa_sve_sub_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t va = svld1_s32(pg, a + i);
        svint32_t vb = svld1_s32(pg, b + i);
        svst1_s32(pg, out + i, svsub_s32_z(pg, va, vb));
    }
}

void rust_isa_sve_sub_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t va = svld1_f64(pg, a + i);
        svfloat64_t vb = svld1_f64(pg, b + i);
        svst1_f64(pg, out + i, svsub_f64_z(pg, va, vb));
    }
}

void rust_isa_sve_div_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t va = svld1_f32(pg, a + i);
        svfloat32_t vb = svld1_f32(pg, b + i);
        svst1_f32(pg, out + i, svdiv_f32_z(pg, va, vb));
    }
}

void rust_isa_sve_div_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t va = svld1_s32(pg, a + i);
        svint32_t vb = svld1_s32(pg, b + i);
        svst1_s32(pg, out + i, svdiv_s32_z(pg, va, vb));
    }
}

void rust_isa_sve_div_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t va = svld1_f64(pg, a + i);
        svfloat64_t vb = svld1_f64(pg, b + i);
        svst1_f64(pg, out + i, svdiv_f64_z(pg, va, vb));
    }
}

void rust_isa_sve_min_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svmin_f32_z(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}
void rust_isa_sve_max_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svmax_f32_z(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}
void rust_isa_sve_min_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svmin_s32_z(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}
void rust_isa_sve_max_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svmax_s32_z(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}
void rust_isa_sve_min_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svmin_f64_z(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}
void rust_isa_sve_max_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svmax_f64_z(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve_abs_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svabs_f32_z(pg, svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_neg_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svneg_f32_z(pg, svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_abs_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svabs_s32_z(pg, svld1_s32(pg, a + i)));
    }
}
void rust_isa_sve_neg_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svneg_s32_z(pg, svld1_s32(pg, a + i)));
    }
}
void rust_isa_sve_abs_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svabs_f64_z(pg, svld1_f64(pg, a + i)));
    }
}
void rust_isa_sve_neg_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svneg_f64_z(pg, svld1_f64(pg, a + i)));
    }
}

void rust_isa_sve_sqrt_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svsqrt_f32_z(pg, svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_sqrt_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svsqrt_f64_z(pg, svld1_f64(pg, a + i)));
    }
}

void rust_isa_sve_and_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svand_s32_z(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}
void rust_isa_sve_or_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svorr_s32_z(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}
void rust_isa_sve_xor_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, sveor_s32_z(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_fma_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t va = svld1_f32(pg, a + i);
        svfloat32_t vb = svld1_f32(pg, b + i);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svmla_f32_z(pg, vc, va, vb));
    }
}
void rust_isa_sve_fma_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t va = svld1_f64(pg, a + i);
        svfloat64_t vb = svld1_f64(pg, b + i);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svmla_f64_z(pg, vc, va, vb));
    }
}

void rust_isa_sve_recip_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svrecpe_f32(svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_recip_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svrecpe_f64(svld1_f64(pg, a + i)));
    }
}

void rust_isa_sve_rsqrt_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svrsqrte_f32(svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_rsqrt_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svrsqrte_f64(svld1_f64(pg, a + i)));
    }
}

void rust_isa_sve_floor_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svrintm_f32_z(pg, svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_ceil_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svrintp_f32_z(pg, svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_floor_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svrintm_f64_z(pg, svld1_f64(pg, a + i)));
    }
}
void rust_isa_sve_ceil_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svrintp_f64_z(pg, svld1_f64(pg, a + i)));
    }
}

void rust_isa_sve_trunc_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svrintz_f32_z(pg, svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_rint_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svrintn_f32_z(pg, svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_trunc_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svrintz_f64_z(pg, svld1_f64(pg, a + i)));
    }
}
void rust_isa_sve_rint_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svrintn_f64_z(pg, svld1_f64(pg, a + i)));
    }
}

void rust_isa_sve_minnm_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svminnm_f32_z(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}
void rust_isa_sve_maxnm_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svmaxnm_f32_z(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}
void rust_isa_sve_minnm_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svminnm_f64_z(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}
void rust_isa_sve_maxnm_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svmaxnm_f64_z(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

float rust_isa_sve_addv_f32(const float *a, size_t len) {
    svfloat32_t acc = svdup_f32(0.0f);
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svadd_f32_m(pg, acc, svld1_f32(pg, a + i));
    }
    return svaddv_f32(svptrue_b32(), acc);
}
int32_t rust_isa_sve_addv_i32(const int32_t *a, size_t len) {
    svint32_t acc = svdup_s32(0);
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svadd_s32_m(pg, acc, svld1_s32(pg, a + i));
    }
    return svaddv_s32(svptrue_b32(), acc);
}
double rust_isa_sve_addv_f64(const double *a, size_t len) {
    svfloat64_t acc = svdup_f64(0.0);
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        acc = svadd_f64_m(pg, acc, svld1_f64(pg, a + i));
    }
    return svaddv_f64(svptrue_b64(), acc);
}

float rust_isa_sve_minv_f32(const float *a, size_t len) {
    svfloat32_t acc = svdup_f32(__builtin_inff());
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svmin_f32_m(pg, acc, svld1_f32(pg, a + i));
    }
    return svminv_f32(svptrue_b32(), acc);
}
float rust_isa_sve_maxv_f32(const float *a, size_t len) {
    svfloat32_t acc = svdup_f32(-__builtin_inff());
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svmax_f32_m(pg, acc, svld1_f32(pg, a + i));
    }
    return svmaxv_f32(svptrue_b32(), acc);
}
int32_t rust_isa_sve_minv_i32(const int32_t *a, size_t len) {
    svint32_t acc = svdup_s32(INT32_MAX);
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svmin_s32_m(pg, acc, svld1_s32(pg, a + i));
    }
    return svminv_s32(svptrue_b32(), acc);
}
int32_t rust_isa_sve_maxv_i32(const int32_t *a, size_t len) {
    svint32_t acc = svdup_s32(INT32_MIN);
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svmax_s32_m(pg, acc, svld1_s32(pg, a + i));
    }
    return svmaxv_s32(svptrue_b32(), acc);
}
double rust_isa_sve_minv_f64(const double *a, size_t len) {
    svfloat64_t acc = svdup_f64(__builtin_inf());
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        acc = svmin_f64_m(pg, acc, svld1_f64(pg, a + i));
    }
    return svminv_f64(svptrue_b64(), acc);
}
double rust_isa_sve_maxv_f64(const double *a, size_t len) {
    svfloat64_t acc = svdup_f64(-__builtin_inf());
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        acc = svmax_f64_m(pg, acc, svld1_f64(pg, a + i));
    }
    return svmaxv_f64(svptrue_b64(), acc);
}

void rust_isa_sve_mls_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t va = svld1_f32(pg, a + i);
        svfloat32_t vb = svld1_f32(pg, b + i);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svmls_f32_z(pg, vc, va, vb));
    }
}
void rust_isa_sve_mls_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t va = svld1_f64(pg, a + i);
        svfloat64_t vb = svld1_f64(pg, b + i);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svmls_f64_z(pg, vc, va, vb));
    }
}

void rust_isa_sve_clz_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, (uint32_t *)(void *)(out + i), svclz_u32_z(pg, svld1_u32(pg, (const uint32_t *)(const void *)(a + i))));
    }
}
void rust_isa_sve_cnt_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, (uint32_t *)(void *)(out + i), svcnt_u32_z(pg, svld1_u32(pg, (const uint32_t *)(const void *)(a + i))));
    }
}
void rust_isa_sve_not_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, (uint32_t *)(void *)(out + i), svnot_u32_z(pg, svld1_u32(pg, (const uint32_t *)(const void *)(a + i))));
    }
}
