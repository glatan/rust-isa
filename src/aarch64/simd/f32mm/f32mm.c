#include <stdint.h>
#include <stddef.h>
#include <arm_sve.h>

void rust_isa_f32mm_fmmla(const float *a, const float *b, float *acc, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t va = svld1_f32(pg, a + i);
        svfloat32_t vb = svld1_f32(pg, b + i);
        svfloat32_t vc = svld1_f32(pg, acc + i);
        svst1_f32(pg, acc + i, svmmla_f32(vc, va, vb));
    }
}
