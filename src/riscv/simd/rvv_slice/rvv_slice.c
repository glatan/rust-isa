#include <riscv_vector.h>
#include <stddef.h>

void rust_isa_rvv_slice_add_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        __riscv_vse32_v_f32m1(out + i, __riscv_vfadd_vv_f32m1(va, vb, vl), vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_mul_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        __riscv_vse32_v_f32m1(out + i, __riscv_vfmul_vv_f32m1(va, vb, vl), vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_add_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        __riscv_vse32_v_i32m1(out + i, __riscv_vadd_vv_i32m1(va, vb, vl), vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_mul_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        __riscv_vse32_v_i32m1(out + i, __riscv_vmul_vv_i32m1(va, vb, vl), vl);
        i += vl;
    }
}
