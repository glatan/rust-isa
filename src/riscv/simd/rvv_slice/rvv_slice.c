#include <riscv_vector.h>
#include <stddef.h>
#include <stdint.h>

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

void rust_isa_rvv_slice_sub_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        __riscv_vse32_v_f32m1(out + i, __riscv_vfsub_vv_f32m1(va, vb, vl), vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_div_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        __riscv_vse32_v_f32m1(out + i, __riscv_vfdiv_vv_f32m1(va, vb, vl), vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_min_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        __riscv_vse32_v_f32m1(out + i, __riscv_vfmin_vv_f32m1(va, vb, vl), vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_max_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        __riscv_vse32_v_f32m1(out + i, __riscv_vfmax_vv_f32m1(va, vb, vl), vl);
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

void rust_isa_rvv_slice_sub_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        __riscv_vse32_v_i32m1(out + i, __riscv_vsub_vv_i32m1(va, vb, vl), vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_div_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        __riscv_vse32_v_i32m1(out + i, __riscv_vdiv_vv_i32m1(va, vb, vl), vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_min_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        __riscv_vse32_v_i32m1(out + i, __riscv_vmin_vv_i32m1(va, vb, vl), vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_max_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        __riscv_vse32_v_i32m1(out + i, __riscv_vmax_vv_i32m1(va, vb, vl), vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmpeq_f32(const float *a, const float *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        vbool32_t mask = __riscv_vmfeq_vv_f32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmpne_f32(const float *a, const float *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        vbool32_t mask = __riscv_vmfne_vv_f32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmplt_f32(const float *a, const float *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        vbool32_t mask = __riscv_vmflt_vv_f32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmple_f32(const float *a, const float *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        vbool32_t mask = __riscv_vmfle_vv_f32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmpgt_f32(const float *a, const float *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        vbool32_t mask = __riscv_vmfgt_vv_f32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmpge_f32(const float *a, const float *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t va = __riscv_vle32_v_f32m1(a + i, vl);
        vfloat32m1_t vb = __riscv_vle32_v_f32m1(b + i, vl);
        vbool32_t mask = __riscv_vmfge_vv_f32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmpeq_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        vbool32_t mask = __riscv_vmseq_vv_i32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmpne_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        vbool32_t mask = __riscv_vmsne_vv_i32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmplt_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        vbool32_t mask = __riscv_vmslt_vv_i32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmple_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        vbool32_t mask = __riscv_vmsle_vv_i32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmpgt_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        vbool32_t mask = __riscv_vmsgt_vv_i32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_cmpge_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t va = __riscv_vle32_v_i32m1(a + i, vl);
        vint32m1_t vb = __riscv_vle32_v_i32m1(b + i, vl);
        vbool32_t mask = __riscv_vmsge_vv_i32m1_b32(va, vb, vl);
        vint32m1_t zero = __riscv_vmv_v_x_i32m1(0, vl);
        vint32m1_t result = __riscv_vmerge_vxm_i32m1(zero, -1, mask, vl);
        __riscv_vse32_v_i32m1(out + i, result, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_load_f32(const float *src, float *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t value = __riscv_vle32_v_f32m1(src + i, vl);
        __riscv_vse32_v_f32m1(out + i, value, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_load_i32(const int32_t *src, int32_t *out, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t value = __riscv_vle32_v_i32m1(src + i, vl);
        __riscv_vse32_v_i32m1(out + i, value, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_store_f32(float *dst, const float *src, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vfloat32m1_t value = __riscv_vle32_v_f32m1(src + i, vl);
        __riscv_vse32_v_f32m1(dst + i, value, vl);
        i += vl;
    }
}

void rust_isa_rvv_slice_store_i32(int32_t *dst, const int32_t *src, size_t len) {
    for (size_t i = 0; i < len;) {
        size_t vl = __riscv_vsetvl_e32m1(len - i);
        vint32m1_t value = __riscv_vle32_v_i32m1(src + i, vl);
        __riscv_vse32_v_i32m1(dst + i, value, vl);
        i += vl;
    }
}
