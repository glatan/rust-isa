#include <stddef.h>
#include <stdint.h>
#include <arm_sve.h>

void rust_isa_sve2_addp_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svaddp_s8_x(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve2_addp_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svaddp_s16_x(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve2_addp_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svaddp_s32_x(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve2_addp_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svaddp_s64_x(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve2_addp_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svaddp_u8_x(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve2_addp_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svaddp_u16_x(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve2_addp_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svaddp_u32_x(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve2_addp_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svaddp_u64_x(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve2_addp_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svaddp_f32_x(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve2_addp_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svaddp_f64_x(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve2_shadd_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svhadd_s8_x(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve2_shadd_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svhadd_s16_x(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve2_shadd_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svhadd_s32_x(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve2_shadd_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svhadd_s64_x(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve2_shadd_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svhadd_u8_x(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve2_shadd_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svhadd_u16_x(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve2_shadd_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svhadd_u32_x(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve2_shadd_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svhadd_u64_x(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve2_shsub_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svhsub_s8_x(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve2_shsub_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svhsub_s16_x(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve2_shsub_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svhsub_s32_x(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve2_shsub_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svhsub_s64_x(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve2_shsub_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svhsub_u8_x(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve2_shsub_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svhsub_u16_x(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve2_shsub_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svhsub_u32_x(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve2_shsub_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svhsub_u64_x(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve2_uhadd_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svhadd_s8_x(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve2_uhadd_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svhadd_s16_x(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve2_uhadd_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svhadd_s32_x(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve2_uhadd_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svhadd_s64_x(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve2_uhadd_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svhadd_u8_x(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve2_uhadd_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svhadd_u16_x(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve2_uhadd_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svhadd_u32_x(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve2_uhadd_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svhadd_u64_x(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve2_uhsub_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svhsub_s8_x(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve2_uhsub_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svhsub_s16_x(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve2_uhsub_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svhsub_s32_x(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve2_uhsub_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svhsub_s64_x(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve2_uhsub_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svhsub_u8_x(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve2_uhsub_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svhsub_u16_x(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve2_uhsub_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svhsub_u32_x(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve2_uhsub_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svhsub_u64_x(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve2_smaxp_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svmaxp_s8_x(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve2_smaxp_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svmaxp_s16_x(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve2_smaxp_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svmaxp_s32_x(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve2_smaxp_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svmaxp_s64_x(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve2_sminp_i8(const int8_t *a, const int8_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_s8(pg, out + i, svminp_s8_x(pg, svld1_s8(pg, a + i), svld1_s8(pg, b + i)));
    }
}

void rust_isa_sve2_sminp_i16(const int16_t *a, const int16_t *b, int16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_s16(pg, out + i, svminp_s16_x(pg, svld1_s16(pg, a + i), svld1_s16(pg, b + i)));
    }
}

void rust_isa_sve2_sminp_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svminp_s32_x(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve2_sminp_i64(const int64_t *a, const int64_t *b, int64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_s64(pg, out + i, svminp_s64_x(pg, svld1_s64(pg, a + i), svld1_s64(pg, b + i)));
    }
}

void rust_isa_sve2_umaxp_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svmaxp_u8_x(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve2_umaxp_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svmaxp_u16_x(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve2_umaxp_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svmaxp_u32_x(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve2_umaxp_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svmaxp_u64_x(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve2_uminp_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svst1_u8(pg, out + i, svminp_u8_x(pg, svld1_u8(pg, a + i), svld1_u8(pg, b + i)));
    }
}

void rust_isa_sve2_uminp_u16(const uint16_t *a, const uint16_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svst1_u16(pg, out + i, svminp_u16_x(pg, svld1_u16(pg, a + i), svld1_u16(pg, b + i)));
    }
}

void rust_isa_sve2_uminp_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svminp_u32_x(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve2_uminp_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_u64(pg, out + i, svminp_u64_x(pg, svld1_u64(pg, a + i), svld1_u64(pg, b + i)));
    }
}

void rust_isa_sve2_faddp_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svaddp_f32_x(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve2_faddp_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svaddp_f64_x(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve2_fmaxp_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svmaxp_f32_x(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve2_fmaxp_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svmaxp_f64_x(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve2_fminp_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svminp_f32_x(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve2_fminp_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svminp_f64_x(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve2_fmaxnmp_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svmaxnmp_f32_x(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve2_fmaxnmp_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svmaxnmp_f64_x(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}

void rust_isa_sve2_fminnmp_f32(const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_f32(pg, out + i, svminnmp_f32_x(pg, svld1_f32(pg, a + i), svld1_f32(pg, b + i)));
    }
}

void rust_isa_sve2_fminnmp_f64(const double *a, const double *b, double *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svst1_f64(pg, out + i, svminnmp_f64_x(pg, svld1_f64(pg, a + i), svld1_f64(pg, b + i)));
    }
}
