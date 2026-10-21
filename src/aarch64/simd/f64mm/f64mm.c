#include <stdint.h>
#include <stddef.h>
#include <arm_sve.h>

void rust_isa_f64mm_fmmla(const double *a, const double *b, double *acc, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svfloat64_t va = svld1_f64(pg, a + i);
        svfloat64_t vb = svld1_f64(pg, b + i);
        svfloat64_t vc = svld1_f64(pg, acc + i);
        svst1_f64(pg, acc + i, svmmla_f64(vc, va, vb));
    }
}
