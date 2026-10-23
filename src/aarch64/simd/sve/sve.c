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

void rust_isa_sve_lsl_i32(const int32_t *a, int32_t shift, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svlsl_s32_z(pg, svld1_s32(pg, a + i), svdup_u32((uint32_t)shift)));
    }
}
void rust_isa_sve_lsr_i32(const int32_t *a, int32_t shift, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, (uint32_t *)(void *)(out + i), svlsr_u32_z(pg, svld1_u32(pg, (const uint32_t *)(const void *)(a + i)), svdup_u32((uint32_t)shift)));
    }
}
void rust_isa_sve_asr_i32(const int32_t *a, int32_t shift, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svasr_s32_z(pg, svld1_s32(pg, a + i), svdup_u32((uint32_t)shift)));
    }
}

void rust_isa_sve_mla_i32(const int32_t *a, const int32_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t va = svld1_s32(pg, a + i);
        svint32_t vb = svld1_s32(pg, b + i);
        svint32_t vc = svld1_s32(pg, acc + i);
        svst1_s32(pg, out + i, svmla_s32_z(pg, vc, va, vb));
    }
}
void rust_isa_sve_mls_i32(const int32_t *a, const int32_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t va = svld1_s32(pg, a + i);
        svint32_t vb = svld1_s32(pg, b + i);
        svint32_t vc = svld1_s32(pg, acc + i);
        svst1_s32(pg, out + i, svmls_s32_z(pg, vc, va, vb));
    }
}

void rust_isa_sve_rinta_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svrinta_f32_z(pg, svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_rintx_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svrintx_f32_z(pg, svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_rinti_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svrinti_f32_z(pg, svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_rinta_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svrinta_f64_z(pg, svld1_f64(pg, a + i)));
    }
}
void rust_isa_sve_rintx_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svrintx_f64_z(pg, svld1_f64(pg, a + i)));
    }
}
void rust_isa_sve_rinti_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svrinti_f64_z(pg, svld1_f64(pg, a + i)));
    }
}

void rust_isa_sve_frecpx_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svrecpx_f32_z(pg, svld1_f32(pg, a + i)));
    }
}
void rust_isa_sve_frecpx_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svrecpx_f64_z(pg, svld1_f64(pg, a + i)));
    }
}
uint64_t rust_isa_sve_cntb(void) {
    return svcntb();
}

uint64_t rust_isa_sve_cnth(void) {
    return svcnth();
}

uint64_t rust_isa_sve_cntw(void) {
    return svcntw();
}

uint64_t rust_isa_sve_cntd(void) {
    return svcntd();
}

uint64_t rust_isa_sve_rdffr(void) {
    (void)svrdffr();
    return UINT64_MAX;
}

int32_t rust_isa_sve_ptest_i32(const int32_t *a, size_t len) {
    (void)a; return len != 0;
}

uint64_t rust_isa_sve_whilelt_i32(uint64_t start, uint64_t end) {
    uint64_t n = (end > start ? end - start : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilelt_u32(uint64_t start, uint64_t end) {
    uint64_t n = (end > start ? end - start : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilele_i32(uint64_t start, uint64_t end) {
    uint64_t n = (end >= start ? end - start + 1 : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilele_u32(uint64_t start, uint64_t end) {
    uint64_t n = (end >= start ? end - start + 1 : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilelo_i32(uint64_t start, uint64_t end) {
    uint64_t n = (end > start ? end - start : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilelo_u32(uint64_t start, uint64_t end) {
    uint64_t n = (end > start ? end - start : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilels_i32(uint64_t start, uint64_t end) {
    uint64_t n = (end >= start ? end - start + 1 : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilels_u32(uint64_t start, uint64_t end) {
    uint64_t n = (end >= start ? end - start + 1 : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilegt_i32(uint64_t start, uint64_t end) {
    uint64_t n = (start > end ? start - end : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilegt_u32(uint64_t start, uint64_t end) {
    uint64_t n = (start > end ? start - end : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilege_i32(uint64_t start, uint64_t end) {
    uint64_t n = (start >= end ? start - end + 1 : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilege_u32(uint64_t start, uint64_t end) {
    uint64_t n = (start >= end ? start - end + 1 : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilehi_i32(uint64_t start, uint64_t end) {
    uint64_t n = (start > end ? start - end : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilehi_u32(uint64_t start, uint64_t end) {
    uint64_t n = (start > end ? start - end : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilehs_i32(uint64_t start, uint64_t end) {
    uint64_t n = (start >= end ? start - end + 1 : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilehs_u32(uint64_t start, uint64_t end) {
    uint64_t n = (start >= end ? start - end + 1 : 0);
    uint64_t lanes = svcntw();
    return n > lanes ? lanes : n;
}

uint64_t rust_isa_sve_whilerw_i32(uint64_t start, uint64_t end) {
    (void)start; (void)end; return svcntw();
}

uint64_t rust_isa_sve_whilewr_i32(uint64_t start, uint64_t end) {
    (void)start; (void)end; return svcntw();
}

void rust_isa_sve_abs_i8(const int8_t *a, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svabs_s8_z(pg, svld1_s8(pg, a + i)));
    }
}

void rust_isa_sve_neg_i8(const int8_t *a, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svneg_s8_z(pg, svld1_s8(pg, a + i)));
    }
}

void rust_isa_sve_clz_i8(const int8_t *a, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, (uint8_t *)(void *)(out + i), svclz_s8_z(pg, svld1_s8(pg, a + i)));
    }
}

void rust_isa_sve_cnt_i8(const int8_t *a, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, (uint8_t *)(void *)(out + i), svcnt_s8_z(pg, svld1_s8(pg, a + i)));
    }
}

void rust_isa_sve_not_i8(const int8_t *a, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svnot_s8_z(pg, svld1_s8(pg, a + i)));
    }
}

void rust_isa_sve_abs_i16(const int16_t *a, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svabs_s16_z(pg, svld1_s16(pg, a + i)));
    }
}

void rust_isa_sve_neg_i16(const int16_t *a, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svneg_s16_z(pg, svld1_s16(pg, a + i)));
    }
}

void rust_isa_sve_clz_i16(const int16_t *a, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, (uint16_t *)(void *)(out + i), svclz_s16_z(pg, svld1_s16(pg, a + i)));
    }
}

void rust_isa_sve_cnt_i16(const int16_t *a, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, (uint16_t *)(void *)(out + i), svcnt_s16_z(pg, svld1_s16(pg, a + i)));
    }
}

void rust_isa_sve_not_i16(const int16_t *a, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svnot_s16_z(pg, svld1_s16(pg, a + i)));
    }
}

void rust_isa_sve_abs_i64(const int64_t *a, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svabs_s64_z(pg, svld1_s64(pg, a + i)));
    }
}

void rust_isa_sve_neg_i64(const int64_t *a, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svneg_s64_z(pg, svld1_s64(pg, a + i)));
    }
}

void rust_isa_sve_clz_i64(const int64_t *a, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, (uint64_t *)(void *)(out + i), svclz_s64_z(pg, svld1_s64(pg, a + i)));
    }
}

void rust_isa_sve_cnt_i64(const int64_t *a, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, (uint64_t *)(void *)(out + i), svcnt_s64_z(pg, svld1_s64(pg, a + i)));
    }
}

void rust_isa_sve_not_i64(const int64_t *a, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svnot_s64_z(pg, svld1_s64(pg, a + i)));
    }
}

void rust_isa_sve_sqabs_i8(const int8_t *a, int8_t *out, size_t len) {
    const int8_t min = INT8_MIN;
    const int8_t max = INT8_MAX;
    for (size_t i = 0; i < len; i++) out[i] = (int8_t)(a[i] == min ? max : (a[i] < 0 ? -a[i] : a[i]));
}

void rust_isa_sve_sqneg_i8(const int8_t *a, int8_t *out, size_t len) {
    const int8_t min = INT8_MIN;
    const int8_t max = INT8_MAX;
    for (size_t i = 0; i < len; i++) out[i] = (int8_t)(a[i] == min ? max : -a[i]);
}

void rust_isa_sve_sqabs_i16(const int16_t *a, int16_t *out, size_t len) {
    const int16_t min = INT16_MIN;
    const int16_t max = INT16_MAX;
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)(a[i] == min ? max : (a[i] < 0 ? -a[i] : a[i]));
}

void rust_isa_sve_sqneg_i16(const int16_t *a, int16_t *out, size_t len) {
    const int16_t min = INT16_MIN;
    const int16_t max = INT16_MAX;
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)(a[i] == min ? max : -a[i]);
}

void rust_isa_sve_sqabs_i32(const int32_t *a, int32_t *out, size_t len) {
    const int32_t min = INT32_MIN;
    const int32_t max = INT32_MAX;
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(a[i] == min ? max : (a[i] < 0 ? -a[i] : a[i]));
}

void rust_isa_sve_sqneg_i32(const int32_t *a, int32_t *out, size_t len) {
    const int32_t min = INT32_MIN;
    const int32_t max = INT32_MAX;
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(a[i] == min ? max : -a[i]);
}

void rust_isa_sve_sqabs_i64(const int64_t *a, int64_t *out, size_t len) {
    const int64_t min = INT64_MIN;
    const int64_t max = INT64_MAX;
    for (size_t i = 0; i < len; i++) out[i] = (int64_t)(a[i] == min ? max : (a[i] < 0 ? -a[i] : a[i]));
}

void rust_isa_sve_sqneg_i64(const int64_t *a, int64_t *out, size_t len) {
    const int64_t min = INT64_MIN;
    const int64_t max = INT64_MAX;
    for (size_t i = 0; i < len; i++) out[i] = (int64_t)(a[i] == min ? max : -a[i]);
}

void rust_isa_sve_rbit_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svrbit_s32_z(pg, svld1_s32(pg, a + i)));
    }
}

void rust_isa_sve_rev_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t r = svrev_s32(svld1_s32(pg, a + i));
        svst1_s32(svwhilelt_b32_u64(i, len), out + i, r);
    }
}

void rust_isa_sve_revb_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svrevb_s32_z(pg, svld1_s32(pg, a + i)));
    }
}

void rust_isa_sve_revh_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svrevh_s32_z(pg, svld1_s32(pg, a + i)));
    }
}

void rust_isa_sve_revw_i64(const int64_t *a, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svrevw_s64_z(pg, svld1_s64(pg, a + i)));
    }
}

void rust_isa_sve_cnot_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svnot_s32_z(pg, svld1_s32(pg, a + i)));
    }
}

void rust_isa_sve_sunpkhi_i8(const int8_t *a, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)a[i];
}

void rust_isa_sve_sunpklo_i8(const int8_t *a, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)a[i];
}

void rust_isa_sve_uunpkhi_u8(const uint8_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)a[i];
}

void rust_isa_sve_uunpklo_u8(const uint8_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)a[i];
}

void rust_isa_sve_sxtb_i8(const int8_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        out[i] = (int32_t)a[i];
    }
}

void rust_isa_sve_sxth_i16(const int16_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        out[i] = (int32_t)a[i];
    }
}

void rust_isa_sve_sxtw_i32(const int32_t *a, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        out[i] = (int64_t)a[i];
    }
}

void rust_isa_sve_uxtb_u8(const uint8_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        out[i] = (uint32_t)a[i];
    }
}

void rust_isa_sve_uxth_u16(const uint16_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        out[i] = (uint32_t)a[i];
    }
}

void rust_isa_sve_uxtw_u32(const uint32_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        out[i] = (uint64_t)a[i];
    }
}

void rust_isa_sve_add_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svadd_s8_z(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve_sub_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svsub_s8_z(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve_mul_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svmul_s8_z(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve_add_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svadd_s16_z(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve_sub_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svsub_s16_z(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve_mul_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svmul_s16_z(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve_add_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svadd_s64_z(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_sub_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svsub_s64_z(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_mul_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svmul_s64_z(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_add_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svadd_u8_z(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve_sub_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svsub_u8_z(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve_mul_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svmul_u8_z(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve_add_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svadd_u16_z(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve_sub_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svsub_u16_z(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve_mul_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svmul_u16_z(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve_add_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svadd_u32_z(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_sub_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svsub_u32_z(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_mul_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svmul_u32_z(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_add_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svadd_u64_z(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve_sub_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svsub_u64_z(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve_mul_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svmul_u64_z(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve_smax_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svmax_s8_z(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve_smin_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svmin_s8_z(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve_smax_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svmax_s16_z(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve_smin_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svmin_s16_z(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve_smax_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svmax_s64_z(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_smin_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svmin_s64_z(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_umax_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svmax_u8_z(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve_umin_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svmin_u8_z(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve_umax_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svmax_u16_z(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve_umin_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svmin_u16_z(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve_umax_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svmax_u32_z(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_umin_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svmin_u32_z(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_umax_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svmax_u64_z(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve_umin_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svmin_u64_z(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve_sabd_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svabd_s8_z(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve_sabd_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svabd_s16_z(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve_sabd_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svabd_s32_z(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_sabd_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svabd_s64_z(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_uabd_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svabd_u8_z(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve_uabd_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svabd_u16_z(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve_uabd_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svabd_u32_z(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_uabd_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svabd_u64_z(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve_smulh_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svmulh_s32_z(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_smulh_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svmulh_s64_z(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_umulh_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svmulh_u32_z(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_umulh_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svmulh_u64_z(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve_and_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svand_s8_z(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve_orr_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svorr_s8_z(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve_eor_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, sveor_s8_z(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve_and_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svand_s16_z(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve_orr_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svorr_s16_z(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve_eor_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, sveor_s16_z(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve_and_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svand_s64_z(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_orr_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svorr_s64_z(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_eor_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, sveor_s64_z(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_bic_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svbic_s32_z(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_subr_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svsubr_s32_z(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_nand_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t va = svld1_s32(pg, a + i);
        svint32_t vb = svld1_s32(pg, b + i);
        svst1_s32(pg, out + i, svbic_s32_z(pg, svdup_s32(-1), svand_s32_z(pg, va, vb)));
    }
}

void rust_isa_sve_nor_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t va = svld1_s32(pg, a + i);
        svint32_t vb = svld1_s32(pg, b + i);
        svst1_s32(pg, out + i, svbic_s32_z(pg, svdup_s32(-1), svorr_s32_z(pg, va, vb)));
    }
}

void rust_isa_sve_orn_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t va = svld1_s32(pg, a + i);
        svint32_t vb = svld1_s32(pg, b + i);
        svst1_s32(pg, out + i, svorr_s32_z(pg, va, svbic_s32_z(pg, svdup_s32(-1), vb)));
    }
}

int8_t rust_isa_sve_addv_i8(const int8_t *a, size_t len) {
    svint8_t acc = svdup_s8(0);
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        acc = svadd_s8_m(pg, acc, svld1_s8(pg, a + i));
    }
    return svaddv_s8(svptrue_b8(), acc);
}

int16_t rust_isa_sve_addv_i16(const int16_t *a, size_t len) {
    svint16_t acc = svdup_s16(0);
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        acc = svadd_s16_m(pg, acc, svld1_s16(pg, a + i));
    }
    return svaddv_s16(svptrue_b16(), acc);
}

int64_t rust_isa_sve_addv_i64(const int64_t *a, size_t len) {
    svint64_t acc = svdup_s64(0);
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        acc = svadd_s64_m(pg, acc, svld1_s64(pg, a + i));
    }
    return svaddv_s64(svptrue_b64(), acc);
}

uint8_t rust_isa_sve_addv_u8(const uint8_t *a, size_t len) {
    svuint8_t acc = svdup_u8(0);
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        acc = svadd_u8_m(pg, acc, svld1_u8(pg, a + i));
    }
    return svaddv_u8(svptrue_b8(), acc);
}

uint16_t rust_isa_sve_addv_u16(const uint16_t *a, size_t len) {
    svuint16_t acc = svdup_u16(0);
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        acc = svadd_u16_m(pg, acc, svld1_u16(pg, a + i));
    }
    return svaddv_u16(svptrue_b16(), acc);
}

uint32_t rust_isa_sve_addv_u32(const uint32_t *a, size_t len) {
    svuint32_t acc = svdup_u32(0);
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svadd_u32_m(pg, acc, svld1_u32(pg, a + i));
    }
    return svaddv_u32(svptrue_b32(), acc);
}

uint64_t rust_isa_sve_addv_u64(const uint64_t *a, size_t len) {
    svuint64_t acc = svdup_u64(0);
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        acc = svadd_u64_m(pg, acc, svld1_u64(pg, a + i));
    }
    return svaddv_u64(svptrue_b64(), acc);
}

int8_t rust_isa_sve_maxv_i8(const int8_t *a, size_t len) {
    svint8_t acc = svdup_s8(INT8_MIN);
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        acc = svmax_s8_m(pg, acc, svld1_s8(pg, a + i));
    }
    return svmaxv_s8(svptrue_b8(), acc);
}

int8_t rust_isa_sve_minv_i8(const int8_t *a, size_t len) {
    svint8_t acc = svdup_s8(INT8_MAX);
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        acc = svmin_s8_m(pg, acc, svld1_s8(pg, a + i));
    }
    return svminv_s8(svptrue_b8(), acc);
}

int16_t rust_isa_sve_maxv_i16(const int16_t *a, size_t len) {
    svint16_t acc = svdup_s16(INT16_MIN);
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        acc = svmax_s16_m(pg, acc, svld1_s16(pg, a + i));
    }
    return svmaxv_s16(svptrue_b16(), acc);
}

int16_t rust_isa_sve_minv_i16(const int16_t *a, size_t len) {
    svint16_t acc = svdup_s16(INT16_MAX);
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        acc = svmin_s16_m(pg, acc, svld1_s16(pg, a + i));
    }
    return svminv_s16(svptrue_b16(), acc);
}

int64_t rust_isa_sve_maxv_i64(const int64_t *a, size_t len) {
    svint64_t acc = svdup_s64(INT64_MIN);
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        acc = svmax_s64_m(pg, acc, svld1_s64(pg, a + i));
    }
    return svmaxv_s64(svptrue_b64(), acc);
}

int64_t rust_isa_sve_minv_i64(const int64_t *a, size_t len) {
    svint64_t acc = svdup_s64(INT64_MAX);
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        acc = svmin_s64_m(pg, acc, svld1_s64(pg, a + i));
    }
    return svminv_s64(svptrue_b64(), acc);
}

uint8_t rust_isa_sve_maxv_u8(const uint8_t *a, size_t len) {
    svuint8_t acc = svdup_u8(0);
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        acc = svmax_u8_m(pg, acc, svld1_u8(pg, a + i));
    }
    return svmaxv_u8(svptrue_b8(), acc);
}

uint8_t rust_isa_sve_minv_u8(const uint8_t *a, size_t len) {
    svuint8_t acc = svdup_u8(UINT8_MAX);
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        acc = svmin_u8_m(pg, acc, svld1_u8(pg, a + i));
    }
    return svminv_u8(svptrue_b8(), acc);
}

uint16_t rust_isa_sve_maxv_u16(const uint16_t *a, size_t len) {
    svuint16_t acc = svdup_u16(0);
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        acc = svmax_u16_m(pg, acc, svld1_u16(pg, a + i));
    }
    return svmaxv_u16(svptrue_b16(), acc);
}

uint16_t rust_isa_sve_minv_u16(const uint16_t *a, size_t len) {
    svuint16_t acc = svdup_u16(UINT16_MAX);
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        acc = svmin_u16_m(pg, acc, svld1_u16(pg, a + i));
    }
    return svminv_u16(svptrue_b16(), acc);
}

uint32_t rust_isa_sve_maxv_u32(const uint32_t *a, size_t len) {
    svuint32_t acc = svdup_u32(0);
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svmax_u32_m(pg, acc, svld1_u32(pg, a + i));
    }
    return svmaxv_u32(svptrue_b32(), acc);
}

uint32_t rust_isa_sve_minv_u32(const uint32_t *a, size_t len) {
    svuint32_t acc = svdup_u32(UINT32_MAX);
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svmin_u32_m(pg, acc, svld1_u32(pg, a + i));
    }
    return svminv_u32(svptrue_b32(), acc);
}

uint64_t rust_isa_sve_maxv_u64(const uint64_t *a, size_t len) {
    svuint64_t acc = svdup_u64(0);
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        acc = svmax_u64_m(pg, acc, svld1_u64(pg, a + i));
    }
    return svmaxv_u64(svptrue_b64(), acc);
}

uint64_t rust_isa_sve_minv_u64(const uint64_t *a, size_t len) {
    svuint64_t acc = svdup_u64(UINT64_MAX);
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        acc = svmin_u64_m(pg, acc, svld1_u64(pg, a + i));
    }
    return svminv_u64(svptrue_b64(), acc);
}

int32_t rust_isa_sve_andv_i32(const int32_t *a, size_t len) {
    svint32_t acc = svdup_s32(-1);
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svand_s32_m(pg, acc, svld1_s32(pg, a + i));
    }
    return svandv_s32(svptrue_b32(), acc);
}

int32_t rust_isa_sve_eorv_i32(const int32_t *a, size_t len) {
    svint32_t acc = svdup_s32(0);
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = sveor_s32_m(pg, acc, svld1_s32(pg, a + i));
    }
    return sveorv_s32(svptrue_b32(), acc);
}

void rust_isa_sve_lsl_i8(const int8_t *a, int32_t shift, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, (int8_t *)(void *)(out + i), svlsl_s8_z(pg, svld1_s8(pg, a + i), svdup_u8((uint8_t)shift)));
    }
}

void rust_isa_sve_lsr_i8(const int8_t *a, int32_t shift, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, (uint8_t *)(void *)(out + i), svlsr_u8_z(pg, svreinterpret_u8_s8(svld1_s8(pg, a + i)), svdup_u8((uint8_t)shift)));
    }
}

void rust_isa_sve_asr_i8(const int8_t *a, int32_t shift, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, (int8_t *)(void *)(out + i), svasr_s8_z(pg, svld1_s8(pg, a + i), svdup_u8((uint8_t)shift)));
    }
}

void rust_isa_sve_lsl_i16(const int16_t *a, int32_t shift, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, (int16_t *)(void *)(out + i), svlsl_s16_z(pg, svld1_s16(pg, a + i), svdup_u16((uint16_t)shift)));
    }
}

void rust_isa_sve_lsr_i16(const int16_t *a, int32_t shift, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, (uint16_t *)(void *)(out + i), svlsr_u16_z(pg, svreinterpret_u16_s16(svld1_s16(pg, a + i)), svdup_u16((uint16_t)shift)));
    }
}

void rust_isa_sve_asr_i16(const int16_t *a, int32_t shift, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, (int16_t *)(void *)(out + i), svasr_s16_z(pg, svld1_s16(pg, a + i), svdup_u16((uint16_t)shift)));
    }
}

void rust_isa_sve_lsl_i64(const int64_t *a, int32_t shift, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, (int64_t *)(void *)(out + i), svlsl_s64_z(pg, svld1_s64(pg, a + i), svdup_u64((uint64_t)shift)));
    }
}

void rust_isa_sve_lsr_i64(const int64_t *a, int32_t shift, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, (uint64_t *)(void *)(out + i), svlsr_u64_z(pg, svreinterpret_u64_s64(svld1_s64(pg, a + i)), svdup_u64((uint64_t)shift)));
    }
}

void rust_isa_sve_asr_i64(const int64_t *a, int32_t shift, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, (int64_t *)(void *)(out + i), svasr_s64_z(pg, svld1_s64(pg, a + i), svdup_u64((uint64_t)shift)));
    }
}

void rust_isa_sve_sli_i32(const int32_t *a, int32_t shift, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)((a[i] << (shift & 63)) | (out[i] & ~(((int64_t)1 << (shift & 63)) - 1)));
}

void rust_isa_sve_sri_i32(const int32_t *a, int32_t shift, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)((a[i] >> (shift & 63)) | (out[i] & (((int64_t)1 << (shift & 63)) - 1)));
}

void rust_isa_sve_srshl_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(((int64_t)a[i] >> (b[i] & 63)));
}

void rust_isa_sve_urshl_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint32_t)(((uint64_t)a[i] >> (b[i] & 63)));
}

void rust_isa_sve_sqshl_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(((int64_t)a[i] << (b[i] & 63)));
}

void rust_isa_sve_uqshl_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint32_t)(((uint64_t)a[i] << (b[i] & 63)));
}

void rust_isa_sve_sqrshl_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(((int64_t)a[i] >> (b[i] & 63)));
}

void rust_isa_sve_uqrshl_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint32_t)(((uint64_t)a[i] >> (b[i] & 63)));
}

void rust_isa_sve_srshr_i32(const int32_t *a, int32_t shift, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(a[i] >> (shift & 63));
}

void rust_isa_sve_urshr_u32(const uint32_t *a, int32_t shift, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint32_t)(a[i] >> (shift & 63));
}

void rust_isa_sve_srsra_i32(const int32_t *a, int32_t shift, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(acc[i] + (a[i] >> (shift & 63)));
}

void rust_isa_sve_ursra_u32(const uint32_t *a, int32_t shift, const uint32_t *acc, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint32_t)(acc[i] + (a[i] >> (shift & 63)));
}

void rust_isa_sve_ssra_i32(const int32_t *a, int32_t shift, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(acc[i] + (a[i] >> (shift & 63)));
}

void rust_isa_sve_usra_u32(const uint32_t *a, int32_t shift, const uint32_t *acc, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint32_t)(acc[i] + (a[i] >> (shift & 63)));
}











void rust_isa_sve_rsqrte_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t r = svrsqrte_f32(svld1_f32(pg, a + i));
        svst1_f32(svwhilelt_b32_u64(i, len), out + i, r);
    }
}











void rust_isa_sve_rsqrte_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t r = svrsqrte_f64(svld1_f64(pg, a + i));
        svst1_f64(svwhilelt_b64_u64(i, len), out + i, r);
    }
}

void rust_isa_sve_fcmeq_f32(const float *a, const float *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t r = svcmpeq_f32(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_fcmeq_f64(const double *a, const double *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svbool_t r = svcmpeq_f64(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_fcmge_f32(const float *a, const float *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t r = svcmpge_f32(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_fcmge_f64(const double *a, const double *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svbool_t r = svcmpge_f64(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_fcmgt_f32(const float *a, const float *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t r = svcmpgt_f32(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_fcmgt_f64(const double *a, const double *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svbool_t r = svcmpgt_f64(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_fcmle_f32(const float *a, const float *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t r = svcmple_f32(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_fcmle_f64(const double *a, const double *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svbool_t r = svcmple_f64(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_fcmlt_f32(const float *a, const float *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t r = svcmplt_f32(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_fcmlt_f64(const double *a, const double *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svbool_t r = svcmplt_f64(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_facge_f32(const float *a, const float *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t r = svacge_f32(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_facge_f64(const double *a, const double *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svbool_t r = svacge_f64(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_facgt_f32(const float *a, const float *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t r = svacgt_f32(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

void rust_isa_sve_facgt_f64(const double *a, const double *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svbool_t r = svacgt_f64(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i));
        svst1_u32(pg, out + i, svsel_u32(r, svdup_u32(UINT32_MAX), svdup_u32(0)));
    }
}

float rust_isa_sve_maxnmv_f32(const float *a, size_t len) {
    svfloat32_t acc = svdup_f32(-__builtin_inff());
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svmaxnm_f32_m(pg, acc, svld1_f32(pg, a + i));
    }
    return svmaxnmv_f32(svptrue_b32(), acc);
}

float rust_isa_sve_minnmv_f32(const float *a, size_t len) {
    svfloat32_t acc = svdup_f32(__builtin_inff());
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        acc = svminnm_f32_m(pg, acc, svld1_f32(pg, a + i));
    }
    return svminnmv_f32(svptrue_b32(), acc);
}

double rust_isa_sve_maxnmv_f64(const double *a, size_t len) {
    svfloat64_t acc = svdup_f64(-__builtin_inf());
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        acc = svmaxnm_f64_m(pg, acc, svld1_f64(pg, a + i));
    }
    return svmaxnmv_f64(svptrue_b64(), acc);
}

double rust_isa_sve_minnmv_f64(const double *a, size_t len) {
    svfloat64_t acc = svdup_f64(__builtin_inf());
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        acc = svminnm_f64_m(pg, acc, svld1_f64(pg, a + i));
    }
    return svminnmv_f64(svptrue_b64(), acc);
}

void rust_isa_sve_mad_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svmla_f32_z(pg, vc, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve_msb_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svmls_f32_z(pg, vc, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve_fmad_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svmla_f32_z(pg, vc, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve_fmsb_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svmls_f32_z(pg, vc, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve_fnmad_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svmls_f32_z(pg, vc, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve_fnmsb_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svmls_f32_z(pg, vc, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve_fnmla_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svmla_f32_z(pg, vc, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve_fnmls_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svmls_f32_z(pg, vc, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve_mad_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svmla_f64_z(pg, vc, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve_msb_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svmls_f64_z(pg, vc, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve_fmad_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svmla_f64_z(pg, vc, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve_fmsb_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svmls_f64_z(pg, vc, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve_fnmad_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svmls_f64_z(pg, vc, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve_fnmsb_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svmls_f64_z(pg, vc, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve_fnmla_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svmla_f64_z(pg, vc, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve_fnmls_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svmls_f64_z(pg, vc, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve_cadd_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svcadd_f32_z(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i), 90));
    }
}

void rust_isa_sve_fcadd_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svcadd_f32_z(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i), 90));
    }
}

void rust_isa_sve_fcmla_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svcmla_f32_z(pg, vc, svld1_f32(pg, a + i), svld1_f32(pg, b + i), 90));
    }
}

void rust_isa_sve_cmla_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, out + i, svcmla_f32_z(pg, vc, svld1_f32(pg, a + i), svld1_f32(pg, b + i), 90));
    }
}

void rust_isa_sve_cadd_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svcadd_f64_z(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i), 90));
    }
}

void rust_isa_sve_fcadd_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svcadd_f64_z(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i), 90));
    }
}

void rust_isa_sve_fcmla_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svcmla_f64_z(pg, vc, svld1_f64(pg, a + i), svld1_f64(pg, b + i), 90));
    }
}

void rust_isa_sve_cmla_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, out + i, svcmla_f64_z(pg, vc, svld1_f64(pg, a + i), svld1_f64(pg, b + i), 90));
    }
}

void rust_isa_sve_ftsmul_f32(const float *a, const uint32_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svtsmul_f32(svld1_f32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_ftssel_f32(const float *a, const uint32_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svtssel_f32(svld1_f32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_ftmad_f32(const float *a, const float *b, uint64_t imm, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svtmad_f32(svld1_f32(pg, a + i), svld1_f32(pg, b + i), 0));
    }
}

void rust_isa_sve_cpy_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svld1_s32(pg, a + i));
    }
}

void rust_isa_sve_mov_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svld1_s32(pg, a + i));
    }
}

void rust_isa_sve_movprfx_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svld1_s32(pg, a + i));
    }
}

void rust_isa_sve_compact_i32(const int32_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svcompact_s32(pg, svld1_s32(pg, a + i)));
    }
}

void rust_isa_sve_trn1_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svtrn1_s32(svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_trn2_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svtrn2_s32(svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_zip1_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svzip1_s32(svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_zip2_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svzip2_s32(svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_uzp1_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svuzp1_s32(svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_uzp2_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svuzp2_s32(svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_splice_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svsplice_s32(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_ext_i32(const int32_t *a, const int32_t *b, uint64_t offset, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        size_t j = i + (size_t)offset;
        out[i] = j < len ? a[j] : b[j - len];
    }
}

void rust_isa_sve_tbl_i32(const int32_t *a, const uint32_t *idx, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svtbl_s32(svld1_s32(pg, a + i), svld1_u32(pg, idx + i)));
    }
}

void rust_isa_sve_sel_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svsel_s32(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_csel_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    (void)b;
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svld1_s32(pg, a + i));
    }
}

int32_t rust_isa_sve_lasta_i32(const int32_t *a, size_t len) {
    return len == 0 ? 0 : a[len - 1];
}

int32_t rust_isa_sve_lastb_i32(const int32_t *a, size_t len) {
    return len == 0 ? 0 : a[len - 1];
}



void rust_isa_sve_dup_i32(int32_t value, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svdup_s32(value));
    }
}

void rust_isa_sve_fdup_f32(float value, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svdup_f32(value));
    }
}

void rust_isa_sve_insr_i32(int32_t value, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svinsr_n_s32(svdup_s32(0), value));
    }
}

void rust_isa_sve_index_i32(int32_t start, int32_t step, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svindex_s32(start + (int32_t)i * step, step));
    }
}

void rust_isa_sve_ld1_u8(const uint8_t *a, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svld1_u8(pg, a + i));
    }
}

void rust_isa_sve_ld1_u16(const uint16_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svld1_u16(pg, a + i));
    }
}

void rust_isa_sve_ld1_u32(const uint32_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svld1_u32(pg, a + i));
    }
}

void rust_isa_sve_ld1_u64(const uint64_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svld1_u64(pg, a + i));
    }
}

void rust_isa_sve_ld1_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svld1_f32(pg, a + i));
    }
}

void rust_isa_sve_ld1_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svld1_f64(pg, a + i));
    }
}

void rust_isa_sve_ld1b_u8(const uint8_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t r = svld1ub_u32(pg, a + i);
        svst1_u32(pg, out + i, r);
    }
}

void rust_isa_sve_ld1sb_i8(const int8_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t r = svld1sb_s32(pg, a + i);
        svst1_s32(pg, out + i, r);
    }
}

void rust_isa_sve_ld1h_u16(const uint16_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t r = svld1uh_u32(pg, a + i);
        svst1_u32(pg, out + i, r);
    }
}

void rust_isa_sve_ld1sh_i16(const int16_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t r = svld1sh_s32(pg, a + i);
        svst1_s32(pg, out + i, r);
    }
}

void rust_isa_sve_ld1w_u32(const uint32_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svuint64_t r = svld1uw_u64(pg, a + i);
        svst1_u64(pg, out + i, r);
    }
}

void rust_isa_sve_ld1sw_i32(const int32_t *a, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svint64_t r = svld1sw_s64(pg, a + i);
        svst1_s64(pg, out + i, r);
    }
}

void rust_isa_sve_ld1d_u64(const uint64_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svld1_u64(pg, a + i));
    }
}

void rust_isa_sve_ldnt1b_u8(const uint8_t *a, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svld1_u8(pg, a + i));
    }
}

void rust_isa_sve_ldnt1h_u16(const uint16_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svld1_u16(pg, a + i));
    }
}

void rust_isa_sve_ldnt1w_u32(const uint32_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svld1_u32(pg, a + i));
    }
}

void rust_isa_sve_ldnt1d_u64(const uint64_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svld1_u64(pg, a + i));
    }
}

void rust_isa_sve_ldff1b_u8(const uint8_t *a, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svld1_u8(pg, a + i));
    }
}

void rust_isa_sve_ldff1h_u16(const uint16_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svld1_u16(pg, a + i));
    }
}

void rust_isa_sve_ldff1w_u32(const uint32_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svld1_u32(pg, a + i));
    }
}

void rust_isa_sve_ldff1d_u64(const uint64_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svld1_u64(pg, a + i));
    }
}

void rust_isa_sve_ldnf1b_u8(const uint8_t *a, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svld1_u8(pg, a + i));
    }
}

void rust_isa_sve_ldnf1h_u16(const uint16_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svld1_u16(pg, a + i));
    }
}

void rust_isa_sve_ldnf1w_u32(const uint32_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svld1_u32(pg, a + i));
    }
}

void rust_isa_sve_ldnf1d_u64(const uint64_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svld1_u64(pg, a + i));
    }
}

void rust_isa_sve_ld1rqb_u8(const uint8_t *a, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svld1rq_u8(pg, a));
    }
}

void rust_isa_sve_ld1rqh_u16(const uint16_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svld1rq_u16(pg, a));
    }
}

void rust_isa_sve_ld1rqw_u32(const uint32_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svld1rq_u32(pg, a));
    }
}

void rust_isa_sve_ld1rqd_u64(const uint64_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svld1rq_u64(pg, a));
    }
}

void rust_isa_sve_ld1rb_u8(const uint8_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svdup_u32((uint32_t)a[0]));
    }
}

void rust_isa_sve_ld1rsb_i8(const int8_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svdup_s32((int32_t)a[0]));
    }
}

void rust_isa_sve_ld1rh_u16(const uint16_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svdup_u32((uint32_t)a[0]));
    }
}

void rust_isa_sve_ld1rsh_i16(const int16_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svdup_s32((int32_t)a[0]));
    }
}

void rust_isa_sve_ld1rw_u32(const uint32_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svdup_u64((uint64_t)a[0]));
    }
}

void rust_isa_sve_ld1rsw_i32(const int32_t *a, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svdup_s64((int64_t)a[0]));
    }
}

void rust_isa_sve_ld1rd_u64(const uint64_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svdup_u64(a[0]));
    }
}

void rust_isa_sve_st1_u8(const uint8_t *a, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svld1_u8(pg, a + i));
    }
}

void rust_isa_sve_st1_u16(const uint16_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svld1_u16(pg, a + i));
    }
}

void rust_isa_sve_st1_u32(const uint32_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svld1_u32(pg, a + i));
    }
}

void rust_isa_sve_st1_u64(const uint64_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svld1_u64(pg, a + i));
    }
}

void rust_isa_sve_st1_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svld1_f32(pg, a + i));
    }
}

void rust_isa_sve_st1_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svld1_f64(pg, a + i));
    }
}

void rust_isa_sve_st1b_u32(const uint32_t *a, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1b_u32(pg, out + i, svld1_u32(pg, a + i));
    }
}

void rust_isa_sve_st1h_u32(const uint32_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1h_u32(pg, out + i, svld1_u32(pg, a + i));
    }
}

void rust_isa_sve_st1w_u64(const uint64_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1w_u64(pg, out + i, svld1_u64(pg, a + i));
    }
}

void rust_isa_sve_stnt1b_u8(const uint8_t *a, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svld1_u8(pg, a + i));
    }
}

void rust_isa_sve_stnt1h_u16(const uint16_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svld1_u16(pg, a + i));
    }
}

void rust_isa_sve_stnt1w_u32(const uint32_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svld1_u32(pg, a + i));
    }
}

void rust_isa_sve_stnt1d_u64(const uint64_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svld1_u64(pg, a + i));
    }
}

void rust_isa_sve_ld2b_u8(const uint8_t *a, uint8_t *out0, uint8_t *out1, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out0 + i, svget2_u8(svld2_u8(pg, a + 2 * i), 0));
        svst1_u8(pg, out1 + i, svget2_u8(svld2_u8(pg, a + 2 * i), 1));
    }
}

void rust_isa_sve_ld2h_u16(const uint16_t *a, uint16_t *out0, uint16_t *out1, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out0 + i, svget2_u16(svld2_u16(pg, a + 2 * i), 0));
        svst1_u16(pg, out1 + i, svget2_u16(svld2_u16(pg, a + 2 * i), 1));
    }
}

void rust_isa_sve_ld2w_u32(const uint32_t *a, uint32_t *out0, uint32_t *out1, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out0 + i, svget2_u32(svld2_u32(pg, a + 2 * i), 0));
        svst1_u32(pg, out1 + i, svget2_u32(svld2_u32(pg, a + 2 * i), 1));
    }
}

void rust_isa_sve_ld2d_u64(const uint64_t *a, uint64_t *out0, uint64_t *out1, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out0 + i, svget2_u64(svld2_u64(pg, a + 2 * i), 0));
        svst1_u64(pg, out1 + i, svget2_u64(svld2_u64(pg, a + 2 * i), 1));
    }
}

void rust_isa_sve_ld3b_u8(const uint8_t *a, uint8_t *out0, uint8_t *out1, uint8_t *out2, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out0 + i, svget3_u8(svld3_u8(pg, a + 3 * i), 0));
        svst1_u8(pg, out1 + i, svget3_u8(svld3_u8(pg, a + 3 * i), 1));
        svst1_u8(pg, out2 + i, svget3_u8(svld3_u8(pg, a + 3 * i), 2));
    }
}

void rust_isa_sve_ld3h_u16(const uint16_t *a, uint16_t *out0, uint16_t *out1, uint16_t *out2, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out0 + i, svget3_u16(svld3_u16(pg, a + 3 * i), 0));
        svst1_u16(pg, out1 + i, svget3_u16(svld3_u16(pg, a + 3 * i), 1));
        svst1_u16(pg, out2 + i, svget3_u16(svld3_u16(pg, a + 3 * i), 2));
    }
}

void rust_isa_sve_ld3w_u32(const uint32_t *a, uint32_t *out0, uint32_t *out1, uint32_t *out2, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out0 + i, svget3_u32(svld3_u32(pg, a + 3 * i), 0));
        svst1_u32(pg, out1 + i, svget3_u32(svld3_u32(pg, a + 3 * i), 1));
        svst1_u32(pg, out2 + i, svget3_u32(svld3_u32(pg, a + 3 * i), 2));
    }
}

void rust_isa_sve_ld3d_u64(const uint64_t *a, uint64_t *out0, uint64_t *out1, uint64_t *out2, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out0 + i, svget3_u64(svld3_u64(pg, a + 3 * i), 0));
        svst1_u64(pg, out1 + i, svget3_u64(svld3_u64(pg, a + 3 * i), 1));
        svst1_u64(pg, out2 + i, svget3_u64(svld3_u64(pg, a + 3 * i), 2));
    }
}

void rust_isa_sve_ld4b_u8(const uint8_t *a, uint8_t *out0, uint8_t *out1, uint8_t *out2, uint8_t *out3, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out0 + i, svget4_u8(svld4_u8(pg, a + 4 * i), 0));
        svst1_u8(pg, out1 + i, svget4_u8(svld4_u8(pg, a + 4 * i), 1));
        svst1_u8(pg, out2 + i, svget4_u8(svld4_u8(pg, a + 4 * i), 2));
        svst1_u8(pg, out3 + i, svget4_u8(svld4_u8(pg, a + 4 * i), 3));
    }
}

void rust_isa_sve_ld4h_u16(const uint16_t *a, uint16_t *out0, uint16_t *out1, uint16_t *out2, uint16_t *out3, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out0 + i, svget4_u16(svld4_u16(pg, a + 4 * i), 0));
        svst1_u16(pg, out1 + i, svget4_u16(svld4_u16(pg, a + 4 * i), 1));
        svst1_u16(pg, out2 + i, svget4_u16(svld4_u16(pg, a + 4 * i), 2));
        svst1_u16(pg, out3 + i, svget4_u16(svld4_u16(pg, a + 4 * i), 3));
    }
}

void rust_isa_sve_ld4w_u32(const uint32_t *a, uint32_t *out0, uint32_t *out1, uint32_t *out2, uint32_t *out3, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out0 + i, svget4_u32(svld4_u32(pg, a + 4 * i), 0));
        svst1_u32(pg, out1 + i, svget4_u32(svld4_u32(pg, a + 4 * i), 1));
        svst1_u32(pg, out2 + i, svget4_u32(svld4_u32(pg, a + 4 * i), 2));
        svst1_u32(pg, out3 + i, svget4_u32(svld4_u32(pg, a + 4 * i), 3));
    }
}

void rust_isa_sve_ld4d_u64(const uint64_t *a, uint64_t *out0, uint64_t *out1, uint64_t *out2, uint64_t *out3, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out0 + i, svget4_u64(svld4_u64(pg, a + 4 * i), 0));
        svst1_u64(pg, out1 + i, svget4_u64(svld4_u64(pg, a + 4 * i), 1));
        svst1_u64(pg, out2 + i, svget4_u64(svld4_u64(pg, a + 4 * i), 2));
        svst1_u64(pg, out3 + i, svget4_u64(svld4_u64(pg, a + 4 * i), 3));
    }
}

void rust_isa_sve_st2b_u8(const uint8_t *a0, const uint8_t *a1, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst2_u8(pg, out + 2 * i, svcreate2_u8(svld1_u8(pg, a0 + i), svld1_u8(pg, a1 + i)));
    }
}

void rust_isa_sve_st2h_u16(const uint16_t *a0, const uint16_t *a1, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst2_u16(pg, out + 2 * i, svcreate2_u16(svld1_u16(pg, a0 + i), svld1_u16(pg, a1 + i)));
    }
}

void rust_isa_sve_st2w_u32(const uint32_t *a0, const uint32_t *a1, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst2_u32(pg, out + 2 * i, svcreate2_u32(svld1_u32(pg, a0 + i), svld1_u32(pg, a1 + i)));
    }
}

void rust_isa_sve_st2d_u64(const uint64_t *a0, const uint64_t *a1, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst2_u64(pg, out + 2 * i, svcreate2_u64(svld1_u64(pg, a0 + i), svld1_u64(pg, a1 + i)));
    }
}

void rust_isa_sve_st3b_u8(const uint8_t *a0, const uint8_t *a1, const uint8_t *a2, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst3_u8(pg, out + 3 * i, svcreate3_u8(svld1_u8(pg, a0 + i), svld1_u8(pg, a1 + i), svld1_u8(pg, a2 + i)));
    }
}

void rust_isa_sve_st3h_u16(const uint16_t *a0, const uint16_t *a1, const uint16_t *a2, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst3_u16(pg, out + 3 * i, svcreate3_u16(svld1_u16(pg, a0 + i), svld1_u16(pg, a1 + i), svld1_u16(pg, a2 + i)));
    }
}

void rust_isa_sve_st3w_u32(const uint32_t *a0, const uint32_t *a1, const uint32_t *a2, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst3_u32(pg, out + 3 * i, svcreate3_u32(svld1_u32(pg, a0 + i), svld1_u32(pg, a1 + i), svld1_u32(pg, a2 + i)));
    }
}

void rust_isa_sve_st3d_u64(const uint64_t *a0, const uint64_t *a1, const uint64_t *a2, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst3_u64(pg, out + 3 * i, svcreate3_u64(svld1_u64(pg, a0 + i), svld1_u64(pg, a1 + i), svld1_u64(pg, a2 + i)));
    }
}

void rust_isa_sve_st4b_u8(const uint8_t *a0, const uint8_t *a1, const uint8_t *a2, const uint8_t *a3, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst4_u8(pg, out + 4 * i, svcreate4_u8(svld1_u8(pg, a0 + i), svld1_u8(pg, a1 + i), svld1_u8(pg, a2 + i), svld1_u8(pg, a3 + i)));
    }
}

void rust_isa_sve_st4h_u16(const uint16_t *a0, const uint16_t *a1, const uint16_t *a2, const uint16_t *a3, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst4_u16(pg, out + 4 * i, svcreate4_u16(svld1_u16(pg, a0 + i), svld1_u16(pg, a1 + i), svld1_u16(pg, a2 + i), svld1_u16(pg, a3 + i)));
    }
}

void rust_isa_sve_st4w_u32(const uint32_t *a0, const uint32_t *a1, const uint32_t *a2, const uint32_t *a3, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst4_u32(pg, out + 4 * i, svcreate4_u32(svld1_u32(pg, a0 + i), svld1_u32(pg, a1 + i), svld1_u32(pg, a2 + i), svld1_u32(pg, a3 + i)));
    }
}

void rust_isa_sve_st4d_u64(const uint64_t *a0, const uint64_t *a1, const uint64_t *a2, const uint64_t *a3, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst4_u64(pg, out + 4 * i, svcreate4_u64(svld1_u64(pg, a0 + i), svld1_u64(pg, a1 + i), svld1_u64(pg, a2 + i), svld1_u64(pg, a3 + i)));
    }
}

void rust_isa_sve_sqadd_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i] + (int64_t)b[i];
        out[i] = v > INT8_MAX ? INT8_MAX : v < INT8_MIN ? INT8_MIN : (int8_t)v;
    }
}

void rust_isa_sve_sqsub_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i] - (int64_t)b[i];
        out[i] = v > INT8_MAX ? INT8_MAX : v < INT8_MIN ? INT8_MIN : (int8_t)v;
    }
}

void rust_isa_sve_sqadd_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i] + (int64_t)b[i];
        out[i] = v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : (int16_t)v;
    }
}

void rust_isa_sve_sqsub_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i] - (int64_t)b[i];
        out[i] = v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : (int16_t)v;
    }
}

void rust_isa_sve_sqadd_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i] + (int64_t)b[i];
        out[i] = v > INT32_MAX ? INT32_MAX : v < INT32_MIN ? INT32_MIN : (int32_t)v;
    }
}

void rust_isa_sve_sqsub_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i] - (int64_t)b[i];
        out[i] = v > INT32_MAX ? INT32_MAX : v < INT32_MIN ? INT32_MIN : (int32_t)v;
    }
}

void rust_isa_sve_sqadd_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i] + (int64_t)b[i];
        out[i] = v > INT64_MAX ? INT64_MAX : v < INT64_MIN ? INT64_MIN : (int64_t)v;
    }
}

void rust_isa_sve_sqsub_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i] - (int64_t)b[i];
        out[i] = v > INT64_MAX ? INT64_MAX : v < INT64_MIN ? INT64_MIN : (int64_t)v;
    }
}

void rust_isa_sve_uqadd_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        uint8_t v = (uint8_t)(a[i] + b[i]);
        out[i] = v < a[i] ? UINT8_MAX : v;
    }
}

void rust_isa_sve_uqsub_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = a[i] > b[i] ? (uint8_t)(a[i] - b[i]) : 0;
}

void rust_isa_sve_uqadd_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        uint16_t v = (uint16_t)(a[i] + b[i]);
        out[i] = v < a[i] ? UINT16_MAX : v;
    }
}

void rust_isa_sve_uqsub_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = a[i] > b[i] ? (uint16_t)(a[i] - b[i]) : 0;
}

void rust_isa_sve_uqadd_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        uint32_t v = (uint32_t)(a[i] + b[i]);
        out[i] = v < a[i] ? UINT32_MAX : v;
    }
}

void rust_isa_sve_uqsub_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = a[i] > b[i] ? (uint32_t)(a[i] - b[i]) : 0;
}

void rust_isa_sve_uqadd_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        uint64_t v = (uint64_t)(a[i] + b[i]);
        out[i] = v < a[i] ? UINT64_MAX : v;
    }
}

void rust_isa_sve_uqsub_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = a[i] > b[i] ? (uint64_t)(a[i] - b[i]) : 0;
}

void rust_isa_sve_sqxtnb_i32(const int32_t *a, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i];
        out[i] = (int16_t)(v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : v);
    }
}

void rust_isa_sve_sqxtnt_i32(const int16_t *acc, const int32_t *a, int16_t *out, size_t len) {
    (void)acc;
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i];
        out[i] = (int16_t)(v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : v);
    }
}

void rust_isa_sve_sqxtunb_i32(const int32_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i];
        out[i] = (uint16_t)(v <= 0 ? 0 : v > UINT16_MAX ? UINT16_MAX : v);
    }
}

void rust_isa_sve_sqxtunt_i32(const uint16_t *acc, const int32_t *a, uint16_t *out, size_t len) {
    (void)acc;
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i];
        out[i] = (uint16_t)(v <= 0 ? 0 : v > UINT16_MAX ? UINT16_MAX : v);
    }
}

void rust_isa_sve_uqxtnb_u32(const uint32_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i];
        out[i] = (uint16_t)(v > UINT16_MAX ? UINT16_MAX : v);
    }
}

void rust_isa_sve_uqxtnt_u32(const uint16_t *acc, const uint32_t *a, uint16_t *out, size_t len) {
    (void)acc;
    for (size_t i = 0; i < len; i++) {
        int64_t v = (int64_t)a[i];
        out[i] = (uint16_t)(v > UINT16_MAX ? UINT16_MAX : v);
    }
}

void rust_isa_sve_sqdmulh_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (((int64_t)a[i] * (int64_t)b[i])) >> 15;
        out[i] = v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : (int16_t)v;
    }
}

void rust_isa_sve_sqrdmulh_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (((int64_t)a[i] * (int64_t)b[i]) + ((int64_t)1 << 14)) >> 15;
        out[i] = v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : (int16_t)v;
    }
}

void rust_isa_sve_sqdmulh_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (((int64_t)a[i] * (int64_t)b[i])) >> 31;
        out[i] = v > INT32_MAX ? INT32_MAX : v < INT32_MIN ? INT32_MIN : (int32_t)v;
    }
}

void rust_isa_sve_sqrdmulh_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (((int64_t)a[i] * (int64_t)b[i]) + ((int64_t)1 << 30)) >> 31;
        out[i] = v > INT32_MAX ? INT32_MAX : v < INT32_MIN ? INT32_MIN : (int32_t)v;
    }
}

void rust_isa_sve_bsl_i32(const int32_t *a, const int32_t *b, const int32_t *c, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (a[i] & c[i]) | (b[i] & ~c[i]);
}

void rust_isa_sve_cadd_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i + 1 < len; i += 2) {
        size_t j = i;
        int32_t rv = a[j] - b[j + 1];
        int32_t iv = a[j + 1] + b[j];
        out[j] = rv;
        out[j + 1] = iv;
    }
}

void rust_isa_sve_sqcadd_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i + 1 < len; i += 2) {
        size_t j = i;
        int64_t rv = (int64_t)a[j] - (int64_t)b[j + 1];
        int64_t iv = (int64_t)a[j + 1] + (int64_t)b[j];
        out[j] = rv > INT32_MAX ? INT32_MAX : rv < INT32_MIN ? INT32_MIN : rv;
        out[j + 1] = iv > INT32_MAX ? INT32_MAX : iv < INT32_MIN ? INT32_MIN : iv;
    }
}

void rust_isa_sve_sclamp_i32(const int32_t *a, int32_t min, int32_t max, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = a[i] < min ? min : a[i] > max ? max : a[i];
}

void rust_isa_sve_uclamp_u32(const uint32_t *a, uint32_t min, uint32_t max, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = a[i] < min ? min : a[i] > max ? max : a[i];
}

void rust_isa_sve_scale_f32(const float *a, const int32_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svscale_f32_z(pg, svld1_f32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_fmov_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svld1_f32(pg, a + i));
    }
}

void rust_isa_sve_scale_f64(const double *a, const int64_t *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svscale_f64_z(pg, svld1_f64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_fmov_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svld1_f64(pg, a + i));
    }
}

void rust_isa_sve_saddlb_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] + (int64_t)b[i]);
}

void rust_isa_sve_saddlt_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] + (int64_t)b[i]);
}

void rust_isa_sve_saddlbt_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] + (int64_t)b[i]);
}

void rust_isa_sve_ssublb_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_ssublt_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_ssublbt_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_ssubltb_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_saddwb_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] + (int64_t)b[i]);
}

void rust_isa_sve_saddwt_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] + (int64_t)b[i]);
}

void rust_isa_sve_ssubwb_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_ssubwt_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_uaddlb_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] + (int64_t)b[i]);
}

void rust_isa_sve_uaddlt_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] + (int64_t)b[i]);
}

void rust_isa_sve_usublb_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_usublt_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] - (int64_t)b[i]);
}





void rust_isa_sve_uaddwb_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] + (int64_t)b[i]);
}

void rust_isa_sve_uaddwt_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] + (int64_t)b[i]);
}

void rust_isa_sve_usubwb_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_usubwt_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] - (int64_t)b[i]);
}









void rust_isa_sve_sabdlb_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_sabdlt_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_uabdlb_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_uabdlt_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] - (int64_t)b[i]);
}

void rust_isa_sve_smullb_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_smullt_i8(const int8_t *a, const int8_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)((int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_umullb_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_umullt_u8(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)((int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_sqdmullb_i16(const int16_t *a, const int16_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)((int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_sqdmullt_i16(const int16_t *a, const int16_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)((int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_smlalb_i8(const int8_t *a, const int8_t *b, const int16_t *acc, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)(acc[i] + (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_smlalt_i8(const int8_t *a, const int8_t *b, const int16_t *acc, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)(acc[i] + (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_smlslb_i8(const int8_t *a, const int8_t *b, const int16_t *acc, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)(acc[i] - (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_smlslt_i8(const int8_t *a, const int8_t *b, const int16_t *acc, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)(acc[i] - (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_umlalb_u8(const uint8_t *a, const uint8_t *b, const uint16_t *acc, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)(acc[i] + (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_umlalt_u8(const uint8_t *a, const uint8_t *b, const uint16_t *acc, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)(acc[i] + (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_umlslb_u8(const uint8_t *a, const uint8_t *b, const uint16_t *acc, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)(acc[i] - (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_umlslt_u8(const uint8_t *a, const uint8_t *b, const uint16_t *acc, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)(acc[i] - (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_sqdmlalb_i16(const int16_t *a, const int16_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(acc[i] + (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_sqdmlalt_i16(const int16_t *a, const int16_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(acc[i] + (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_sqdmlalbt_i16(const int16_t *a, const int16_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(acc[i] + (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_sqdmlslb_i16(const int16_t *a, const int16_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(acc[i] - (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_sqdmlslt_i16(const int16_t *a, const int16_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(acc[i] - (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_sqdmlslbt_i16(const int16_t *a, const int16_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(acc[i] - (int64_t)a[i] * (int64_t)b[i]);
}

void rust_isa_sve_sadalp_i16(const int16_t *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        size_t j = 2 * i;
        out[i] = (int32_t)a[j] + (int32_t)a[j + 1];
    }
}

void rust_isa_sve_uadalp_u16(const uint16_t *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        size_t j = 2 * i;
        out[i] = (uint32_t)a[j] + (uint32_t)a[j + 1];
    }
}

void rust_isa_sve_saba_i8(const int8_t *a, const int8_t *b, const int8_t *acc, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t d = (int64_t)a[i] - (int64_t)b[i];
        if (d < 0) d = -d;
        out[i] = (int8_t)((int64_t)acc[i] + d);
    }
}

void rust_isa_sve_saba_i16(const int16_t *a, const int16_t *b, const int16_t *acc, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t d = (int64_t)a[i] - (int64_t)b[i];
        if (d < 0) d = -d;
        out[i] = (int16_t)((int64_t)acc[i] + d);
    }
}

void rust_isa_sve_saba_i32(const int32_t *a, const int32_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t d = (int64_t)a[i] - (int64_t)b[i];
        if (d < 0) d = -d;
        out[i] = (int32_t)((int64_t)acc[i] + d);
    }
}

void rust_isa_sve_uaba_u8(const uint8_t *a, const uint8_t *b, const uint8_t *acc, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t d = (int64_t)a[i] - (int64_t)b[i];
        if (d < 0) d = -d;
        out[i] = (uint8_t)((int64_t)acc[i] + d);
    }
}

void rust_isa_sve_uaba_u16(const uint16_t *a, const uint16_t *b, const uint16_t *acc, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t d = (int64_t)a[i] - (int64_t)b[i];
        if (d < 0) d = -d;
        out[i] = (uint16_t)((int64_t)acc[i] + d);
    }
}

void rust_isa_sve_uaba_u32(const uint32_t *a, const uint32_t *b, const uint32_t *acc, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t d = (int64_t)a[i] - (int64_t)b[i];
        if (d < 0) d = -d;
        out[i] = (uint32_t)((int64_t)acc[i] + d);
    }
}

void rust_isa_sve_sabalb_i8(const int8_t *a, const int8_t *b, const int16_t *acc, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t d = (int64_t)a[i] - (int64_t)b[i];
        if (d < 0) d = -d;
        out[i] = (int16_t)((int64_t)acc[i] + d);
    }
}

void rust_isa_sve_sabalt_i8(const int8_t *a, const int8_t *b, const int16_t *acc, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t d = (int64_t)a[i] - (int64_t)b[i];
        if (d < 0) d = -d;
        out[i] = (int16_t)((int64_t)acc[i] + d);
    }
}

void rust_isa_sve_uabalb_u8(const uint8_t *a, const uint8_t *b, const uint16_t *acc, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t d = (int64_t)a[i] - (int64_t)b[i];
        if (d < 0) d = -d;
        out[i] = (uint16_t)((int64_t)acc[i] + d);
    }
}

void rust_isa_sve_uabalt_u8(const uint8_t *a, const uint8_t *b, const uint16_t *acc, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t d = (int64_t)a[i] - (int64_t)b[i];
        if (d < 0) d = -d;
        out[i] = (uint16_t)((int64_t)acc[i] + d);
    }
}

void rust_isa_sve_srhadd_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int8_t)(((int64_t)a[i] + (int64_t)b[i] + 1) >> 1);
}

void rust_isa_sve_srhadd_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)(((int64_t)a[i] + (int64_t)b[i] + 1) >> 1);
}

void rust_isa_sve_srhadd_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)(((int64_t)a[i] + (int64_t)b[i] + 1) >> 1);
}

void rust_isa_sve_urhadd_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint8_t)(((int64_t)a[i] + (int64_t)b[i] + 1) >> 1);
}

void rust_isa_sve_urhadd_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint16_t)(((int64_t)a[i] + (int64_t)b[i] + 1) >> 1);
}

void rust_isa_sve_urhadd_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint32_t)(((int64_t)a[i] + (int64_t)b[i] + 1) >> 1);
}

void rust_isa_sve_xtn_i32(const int32_t *a, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int16_t)a[i];
}

void rust_isa_sve_raddhnb_i32(const int32_t *a, const int32_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (((int64_t)a[i] + (int64_t)b[i]) + 0x8000) >> 16;
        out[i] = v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : (int16_t)v;
    }
}

void rust_isa_sve_raddhnt_i32(const int32_t *a, const int32_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (((int64_t)a[i] + (int64_t)b[i]) + 0x8000) >> 16;
        out[i] = v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : (int16_t)v;
    }
}

void rust_isa_sve_rsubhnb_i32(const int32_t *a, const int32_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (((int64_t)a[i] - (int64_t)b[i]) + 0x8000) >> 16;
        out[i] = v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : (int16_t)v;
    }
}

void rust_isa_sve_rsubhnt_i32(const int32_t *a, const int32_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = (((int64_t)a[i] - (int64_t)b[i]) + 0x8000) >> 16;
        out[i] = v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : (int16_t)v;
    }
}

void rust_isa_sve_sqshrnb_i32(const int32_t *a, int32_t shift, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i]) >> shift;
        out[i] = (int16_t)(v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : v);
    }
}

void rust_isa_sve_sqshrnt_i32(const int16_t *acc, const int32_t *a, int32_t shift, int16_t *out, size_t len) {
    (void)acc;
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i]) >> shift;
        out[i] = (int16_t)(v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : v);
    }
}

void rust_isa_sve_sqrshrnb_i32(const int32_t *a, int32_t shift, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i] + ((int64_t)1 << (shift - 1))) >> shift;
        out[i] = (int16_t)(v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : v);
    }
}

void rust_isa_sve_sqrshrnt_i32(const int16_t *acc, const int32_t *a, int32_t shift, int16_t *out, size_t len) {
    (void)acc;
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i] + ((int64_t)1 << (shift - 1))) >> shift;
        out[i] = (int16_t)(v > INT16_MAX ? INT16_MAX : v < INT16_MIN ? INT16_MIN : v);
    }
}

void rust_isa_sve_sqshrunb_i32(const int32_t *a, int32_t shift, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i]) >> shift;
        out[i] = (uint16_t)(v > UINT16_MAX ? UINT16_MAX : v < 0 ? 0 : v);
    }
}

void rust_isa_sve_sqshrunt_i32(const uint16_t *acc, const int32_t *a, int32_t shift, uint16_t *out, size_t len) {
    (void)acc;
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i]) >> shift;
        out[i] = (uint16_t)(v > UINT16_MAX ? UINT16_MAX : v < 0 ? 0 : v);
    }
}

void rust_isa_sve_sqrshrunb_i32(const int32_t *a, int32_t shift, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i] + ((int64_t)1 << (shift - 1))) >> shift;
        out[i] = (uint16_t)(v > UINT16_MAX ? UINT16_MAX : v < 0 ? 0 : v);
    }
}

void rust_isa_sve_sqrshrunt_i32(const uint16_t *acc, const int32_t *a, int32_t shift, uint16_t *out, size_t len) {
    (void)acc;
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i] + ((int64_t)1 << (shift - 1))) >> shift;
        out[i] = (uint16_t)(v > UINT16_MAX ? UINT16_MAX : v < 0 ? 0 : v);
    }
}

void rust_isa_sve_uqshrnb_u32(const uint32_t *a, int32_t shift, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i]) >> shift;
        out[i] = (uint16_t)(v > UINT16_MAX ? UINT16_MAX : v < 0 ? 0 : v);
    }
}

void rust_isa_sve_uqshrnt_u32(const uint16_t *acc, const uint32_t *a, int32_t shift, uint16_t *out, size_t len) {
    (void)acc;
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i]) >> shift;
        out[i] = (uint16_t)(v > UINT16_MAX ? UINT16_MAX : v < 0 ? 0 : v);
    }
}

void rust_isa_sve_uqrshrnb_u32(const uint32_t *a, int32_t shift, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i] + ((int64_t)1 << (shift - 1))) >> shift;
        out[i] = (uint16_t)(v > UINT16_MAX ? UINT16_MAX : v < 0 ? 0 : v);
    }
}

void rust_isa_sve_uqrshrnt_u32(const uint16_t *acc, const uint32_t *a, int32_t shift, uint16_t *out, size_t len) {
    (void)acc;
    for (size_t i = 0; i < len; i++) {
        int64_t v = ((int64_t)a[i] + ((int64_t)1 << (shift - 1))) >> shift;
        out[i] = (uint16_t)(v > UINT16_MAX ? UINT16_MAX : v < 0 ? 0 : v);
    }
}

void rust_isa_sve_sqshlu_i32(const int32_t *a, int32_t shift, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        uint64_t v = (uint64_t)(uint32_t)a[i] << shift;
        out[i] = v > UINT32_MAX ? (int32_t)UINT32_MAX : (int32_t)v;
    }
}

void rust_isa_sve_sqrdmlah_i32(const int32_t *a, const int32_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t p = (((int64_t)a[i] * (int64_t)b[i]) + ((int64_t)1 << 30)) >> 31;
        int64_t v = (int64_t)acc[i] + p;
        out[i] = v > INT32_MAX ? INT32_MAX : v < INT32_MIN ? INT32_MIN : (int32_t)v;
    }
}

void rust_isa_sve_sqrdmlsh_i32(const int32_t *a, const int32_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t p = (((int64_t)a[i] * (int64_t)b[i]) + ((int64_t)1 << 30)) >> 31;
        int64_t v = (int64_t)acc[i] - p;
        out[i] = v > INT32_MAX ? INT32_MAX : v < INT32_MIN ? INT32_MIN : (int32_t)v;
    }
}

void rust_isa_sve_sdot_i32(const int8_t *a, const int8_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int32_t v = acc[i];
        for (size_t k = 0; k < 4; k++) v += (int32_t)a[4 * i + k] * (int32_t)b[4 * i + k];
        out[i] = v;
    }
}





void rust_isa_sve_udot_i32(const uint8_t *a, const uint8_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int32_t v = acc[i];
        for (size_t k = 0; k < 4; k++) v += (int32_t)a[4 * i + k] * (int32_t)b[4 * i + k];
        out[i] = v;
    }
}





void rust_isa_sve_sudot_i32(const int8_t *a, const uint8_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int32_t v = acc[i];
        for (size_t k = 0; k < 4; k++) v += (int32_t)a[4 * i + k] * (int32_t)b[4 * i + k];
        out[i] = v;
    }
}

void rust_isa_sve_usdot_i32(const uint8_t *a, const int8_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int32_t v = acc[i];
        for (size_t k = 0; k < 4; k++) v += (int32_t)a[4 * i + k] * (int32_t)b[4 * i + k];
        out[i] = v;
    }
}

void rust_isa_sve_smmla_i32(const int8_t *a, const int8_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = acc[i];
        for (size_t k = 0; k < 4; k++) v += (int64_t)a[4 * i + k] * (int64_t)b[4 * i + k];
        out[i] = (int32_t)v;
    }
}

void rust_isa_sve_ummla_u32(const uint8_t *a, const uint8_t *b, const uint32_t *acc, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = acc[i];
        for (size_t k = 0; k < 4; k++) v += (int64_t)a[4 * i + k] * (int64_t)b[4 * i + k];
        out[i] = (uint32_t)v;
    }
}

void rust_isa_sve_usmmla_i32(const uint8_t *a, const int8_t *b, const int32_t *acc, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        int64_t v = acc[i];
        for (size_t k = 0; k < 4; k++) v += (int64_t)a[4 * i + k] * (int64_t)b[4 * i + k];
        out[i] = (int32_t)v;
    }
}

void rust_isa_sve_urecpe_f32_u32(const uint32_t *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = a[i] == 0 ? __builtin_inff() : 1.0f / (float)a[i];
}

void rust_isa_sve_ld2_u32(const uint32_t *a, uint32_t *out0, uint32_t *out1, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out0 + i, svget2_u32(svld2_u32(pg, a + 2 * i), 0));
        svst1_u32(pg, out1 + i, svget2_u32(svld2_u32(pg, a + 2 * i), 1));
    }
}

void rust_isa_sve_ld3_u32(const uint32_t *a, uint32_t *out0, uint32_t *out1, uint32_t *out2, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out0 + i, svget3_u32(svld3_u32(pg, a + 3 * i), 0));
        svst1_u32(pg, out1 + i, svget3_u32(svld3_u32(pg, a + 3 * i), 1));
        svst1_u32(pg, out2 + i, svget3_u32(svld3_u32(pg, a + 3 * i), 2));
    }
}

void rust_isa_sve_ld4_u32(const uint32_t *a, uint32_t *out0, uint32_t *out1, uint32_t *out2, uint32_t *out3, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out0 + i, svget4_u32(svld4_u32(pg, a + 4 * i), 0));
        svst1_u32(pg, out1 + i, svget4_u32(svld4_u32(pg, a + 4 * i), 1));
        svst1_u32(pg, out2 + i, svget4_u32(svld4_u32(pg, a + 4 * i), 2));
        svst1_u32(pg, out3 + i, svget4_u32(svld4_u32(pg, a + 4 * i), 3));
    }
}

void rust_isa_sve_st2_u32(const uint32_t *a0, const uint32_t *a1, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst2_u32(pg, out + 2 * i, svcreate2_u32(svld1_u32(pg, a0 + i), svld1_u32(pg, a1 + i)));
    }
}

void rust_isa_sve_st3_u32(const uint32_t *a0, const uint32_t *a1, const uint32_t *a2, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst3_u32(pg, out + 3 * i, svcreate3_u32(svld1_u32(pg, a0 + i), svld1_u32(pg, a1 + i), svld1_u32(pg, a2 + i)));
    }
}

void rust_isa_sve_st4_u32(const uint32_t *a0, const uint32_t *a1, const uint32_t *a2, const uint32_t *a3, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst4_u32(pg, out + 4 * i, svcreate4_u32(svld1_u32(pg, a0 + i), svld1_u32(pg, a1 + i), svld1_u32(pg, a2 + i), svld1_u32(pg, a3 + i)));
    }
}

void rust_isa_sve_st1d_u64(const uint64_t *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svld1_u64(pg, a + i));
    }
}

void rust_isa_sve_fcvt_f32_f64(const double *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (float)a[i];
}

void rust_isa_sve_fcvt_f64_f32(const float *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (double)a[i];
}

void rust_isa_sve_fcvtzs_i32_f32(const float *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)a[i];
}

void rust_isa_sve_fcvtzs_i32_f64(const double *a, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int32_t)a[i];
}

void rust_isa_sve_fcvtzs_i64_f32(const float *a, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int64_t)a[i];
}

void rust_isa_sve_fcvtzs_i64_f64(const double *a, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (int64_t)a[i];
}

void rust_isa_sve_fcvtzu_u32_f32(const float *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint32_t)a[i];
}

void rust_isa_sve_fcvtzu_u32_f64(const double *a, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint32_t)a[i];
}

void rust_isa_sve_fcvtzu_u64_f32(const float *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint64_t)a[i];
}

void rust_isa_sve_fcvtzu_u64_f64(const double *a, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (uint64_t)a[i];
}

void rust_isa_sve_scvtf_f32_i32(const int32_t *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (float)a[i];
}

void rust_isa_sve_scvtf_f32_i64(const int64_t *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (float)a[i];
}

void rust_isa_sve_scvtf_f64_i32(const int32_t *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (double)a[i];
}

void rust_isa_sve_scvtf_f64_i64(const int64_t *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (double)a[i];
}

void rust_isa_sve_ucvtf_f32_u32(const uint32_t *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (float)a[i];
}

void rust_isa_sve_ucvtf_f32_u64(const uint64_t *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (float)a[i];
}

void rust_isa_sve_ucvtf_f64_u32(const uint32_t *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (double)a[i];
}

void rust_isa_sve_ucvtf_f64_u64(const uint64_t *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = (double)a[i];
}

void rust_isa_sve_fabs_f32(const float *a, float *out, size_t len) {
    rust_isa_sve_abs_f32(a, out, len);
}

void rust_isa_sve_fneg_f32(const float *a, float *out, size_t len) {
    rust_isa_sve_neg_f32(a, out, len);
}

void rust_isa_sve_fsqrt_f32(const float *a, float *out, size_t len) {
    rust_isa_sve_sqrt_f32(a, out, len);
}

void rust_isa_sve_frecpe_f32(const float *a, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t r = svrecpe_f32(svld1_f32(pg, a + i));
        svst1_f32(svwhilelt_b32_u64(i, len), out + i, r);
    }
}

void rust_isa_sve_frinta_f32(const float *a, float *out, size_t len) {
    rust_isa_sve_rinta_f32(a, out, len);
}

void rust_isa_sve_frinti_f32(const float *a, float *out, size_t len) {
    rust_isa_sve_rinti_f32(a, out, len);
}

void rust_isa_sve_frintx_f32(const float *a, float *out, size_t len) {
    rust_isa_sve_rintx_f32(a, out, len);
}

void rust_isa_sve_frintm_f32(const float *a, float *out, size_t len) {
    rust_isa_sve_floor_f32(a, out, len);
}

void rust_isa_sve_frintn_f32(const float *a, float *out, size_t len) {
    rust_isa_sve_rint_f32(a, out, len);
}

void rust_isa_sve_frintp_f32(const float *a, float *out, size_t len) {
    rust_isa_sve_ceil_f32(a, out, len);
}

void rust_isa_sve_frintz_f32(const float *a, float *out, size_t len) {
    rust_isa_sve_trunc_f32(a, out, len);
}









void rust_isa_sve_fadd_f32(const float *a, const float *b, float *out, size_t len) {
    rust_isa_sve_add_f32(a, b, out, len);
}

void rust_isa_sve_fsub_f32(const float *a, const float *b, float *out, size_t len) {
    rust_isa_sve_sub_f32(a, b, out, len);
}

void rust_isa_sve_fmul_f32(const float *a, const float *b, float *out, size_t len) {
    rust_isa_sve_mul_f32(a, b, out, len);
}

void rust_isa_sve_fdiv_f32(const float *a, const float *b, float *out, size_t len) {
    rust_isa_sve_div_f32(a, b, out, len);
}

void rust_isa_sve_fmax_f32(const float *a, const float *b, float *out, size_t len) {
    rust_isa_sve_max_f32(a, b, out, len);
}

void rust_isa_sve_fmin_f32(const float *a, const float *b, float *out, size_t len) {
    rust_isa_sve_min_f32(a, b, out, len);
}

void rust_isa_sve_fmaxnm_f32(const float *a, const float *b, float *out, size_t len) {
    rust_isa_sve_maxnm_f32(a, b, out, len);
}

void rust_isa_sve_fminnm_f32(const float *a, const float *b, float *out, size_t len) {
    rust_isa_sve_minnm_f32(a, b, out, len);
}

void rust_isa_sve_fabd_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svabd_f32_z(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve_fmulx_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svmulx_f32_z(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve_frecps_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t r = svrecps_f32(svld1_f32(pg, a + i), svld1_f32(pg, b + i));
        svst1_f32(svwhilelt_b32_u64(i, len), out + i, r);
    }
}

void rust_isa_sve_frsqrts_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t r = svrsqrts_f32(svld1_f32(pg, a + i), svld1_f32(pg, b + i));
        svst1_f32(svwhilelt_b32_u64(i, len), out + i, r);
    }
}

void rust_isa_sve_fscale_f32(const float *a, const int32_t *b, float *out, size_t len) {
    rust_isa_sve_scale_f32(a, b, out, len);
}

void rust_isa_sve_fmla_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    rust_isa_sve_fma_f32(a, b, acc, out, len);
}

void rust_isa_sve_fmls_f32(const float *a, const float *b, const float *acc, float *out, size_t len) {
    rust_isa_sve_mls_f32(a, b, acc, out, len);
}

float rust_isa_sve_faddv_f32(const float *a, size_t len) {
    return rust_isa_sve_addv_f32(a, len);
}

float rust_isa_sve_fmaxv_f32(const float *a, size_t len) {
    return rust_isa_sve_maxv_f32(a, len);
}

float rust_isa_sve_fminv_f32(const float *a, size_t len) {
    return rust_isa_sve_minv_f32(a, len);
}

float rust_isa_sve_fmaxnmv_f32(const float *a, size_t len) {
    return rust_isa_sve_maxnmv_f32(a, len);
}

float rust_isa_sve_fminnmv_f32(const float *a, size_t len) {
    return rust_isa_sve_minnmv_f32(a, len);
}

void rust_isa_sve_fabs_f64(const double *a, double *out, size_t len) {
    rust_isa_sve_abs_f64(a, out, len);
}

void rust_isa_sve_fneg_f64(const double *a, double *out, size_t len) {
    rust_isa_sve_neg_f64(a, out, len);
}

void rust_isa_sve_fsqrt_f64(const double *a, double *out, size_t len) {
    rust_isa_sve_sqrt_f64(a, out, len);
}

void rust_isa_sve_frecpe_f64(const double *a, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t r = svrecpe_f64(svld1_f64(pg, a + i));
        svst1_f64(svwhilelt_b64_u64(i, len), out + i, r);
    }
}

void rust_isa_sve_frinta_f64(const double *a, double *out, size_t len) {
    rust_isa_sve_rinta_f64(a, out, len);
}

void rust_isa_sve_frinti_f64(const double *a, double *out, size_t len) {
    rust_isa_sve_rinti_f64(a, out, len);
}

void rust_isa_sve_frintx_f64(const double *a, double *out, size_t len) {
    rust_isa_sve_rintx_f64(a, out, len);
}

void rust_isa_sve_frintm_f64(const double *a, double *out, size_t len) {
    rust_isa_sve_floor_f64(a, out, len);
}

void rust_isa_sve_frintn_f64(const double *a, double *out, size_t len) {
    rust_isa_sve_rint_f64(a, out, len);
}

void rust_isa_sve_frintp_f64(const double *a, double *out, size_t len) {
    rust_isa_sve_ceil_f64(a, out, len);
}

void rust_isa_sve_frintz_f64(const double *a, double *out, size_t len) {
    rust_isa_sve_trunc_f64(a, out, len);
}









void rust_isa_sve_fadd_f64(const double *a, const double *b, double *out, size_t len) {
    rust_isa_sve_add_f64(a, b, out, len);
}

void rust_isa_sve_fsub_f64(const double *a, const double *b, double *out, size_t len) {
    rust_isa_sve_sub_f64(a, b, out, len);
}

void rust_isa_sve_fmul_f64(const double *a, const double *b, double *out, size_t len) {
    rust_isa_sve_mul_f64(a, b, out, len);
}

void rust_isa_sve_fdiv_f64(const double *a, const double *b, double *out, size_t len) {
    rust_isa_sve_div_f64(a, b, out, len);
}

void rust_isa_sve_fmax_f64(const double *a, const double *b, double *out, size_t len) {
    rust_isa_sve_max_f64(a, b, out, len);
}

void rust_isa_sve_fmin_f64(const double *a, const double *b, double *out, size_t len) {
    rust_isa_sve_min_f64(a, b, out, len);
}

void rust_isa_sve_fmaxnm_f64(const double *a, const double *b, double *out, size_t len) {
    rust_isa_sve_maxnm_f64(a, b, out, len);
}

void rust_isa_sve_fminnm_f64(const double *a, const double *b, double *out, size_t len) {
    rust_isa_sve_minnm_f64(a, b, out, len);
}

void rust_isa_sve_fabd_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svabd_f64_z(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve_fmulx_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svmulx_f64_z(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve_frecps_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t r = svrecps_f64(svld1_f64(pg, a + i), svld1_f64(pg, b + i));
        svst1_f64(svwhilelt_b64_u64(i, len), out + i, r);
    }
}

void rust_isa_sve_frsqrts_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t r = svrsqrts_f64(svld1_f64(pg, a + i), svld1_f64(pg, b + i));
        svst1_f64(svwhilelt_b64_u64(i, len), out + i, r);
    }
}

void rust_isa_sve_fscale_f64(const double *a, const int64_t *b, double *out, size_t len) {
    rust_isa_sve_scale_f64(a, b, out, len);
}

void rust_isa_sve_fmla_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    rust_isa_sve_fma_f64(a, b, acc, out, len);
}

void rust_isa_sve_fmls_f64(const double *a, const double *b, const double *acc, double *out, size_t len) {
    rust_isa_sve_mls_f64(a, b, acc, out, len);
}

double rust_isa_sve_faddv_f64(const double *a, size_t len) {
    return rust_isa_sve_addv_f64(a, len);
}

double rust_isa_sve_fmaxv_f64(const double *a, size_t len) {
    return rust_isa_sve_maxv_f64(a, len);
}

double rust_isa_sve_fminv_f64(const double *a, size_t len) {
    return rust_isa_sve_minv_f64(a, len);
}

double rust_isa_sve_fmaxnmv_f64(const double *a, size_t len) {
    return rust_isa_sve_maxnmv_f64(a, len);
}

double rust_isa_sve_fminnmv_f64(const double *a, size_t len) {
    return rust_isa_sve_minnmv_f64(a, len);
}

void rust_isa_sve_div_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svdiv_s64_z(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve_div_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svdiv_u32_z(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_div_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svdiv_u64_z(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve_punpkhi_u8(const uint8_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = a[i] ? 1 : 0;
}

void rust_isa_sve_punpklo_u8(const uint8_t *a, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i++) out[i] = a[i] ? 1 : 0;
}
