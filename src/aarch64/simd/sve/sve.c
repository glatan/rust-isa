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
