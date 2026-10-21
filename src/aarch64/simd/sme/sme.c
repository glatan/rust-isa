#include <arm_sve.h>
#include <stddef.h>

__attribute__((always_inline))
void rust_isa_sme_start(void) {
    __asm__ volatile("smstart");
}

__attribute__((always_inline))
void rust_isa_sme_stop(void) {
    __asm__ volatile("smstop");
}

void rust_isa_sme_add_f32(const float *a, const float *b, float *out, size_t len) __arm_streaming {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t va = svld1_f32(pg, a + i);
        svfloat32_t vb = svld1_f32(pg, b + i);
        svst1_f32(pg, out + i, svadd_f32_z(pg, va, vb));
    }
}

void rust_isa_sme_mul_f32(const float *a, const float *b, float *out, size_t len) __arm_streaming {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t va = svld1_f32(pg, a + i);
        svfloat32_t vb = svld1_f32(pg, b + i);
        svst1_f32(pg, out + i, svmul_f32_z(pg, va, vb));
    }
}
