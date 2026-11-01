#include <arm_sve.h>

#include <stdint.h>
#include <stddef.h>

void rust_isa_sve_fmlalb_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svfloat16_t va = svld1_f16(ph, (const float16_t *)(const void *)(a + i * 2));
        svfloat16_t vb = svld1_f16(ph, (const float16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svmlalb_f32(vacc, va, vb));
    }
}

void rust_isa_sve_fmlalt_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svfloat16_t va = svld1_f16(ph, (const float16_t *)(const void *)(a + i * 2));
        svfloat16_t vb = svld1_f16(ph, (const float16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svmlalt_f32(vacc, va, vb));
    }
}

void rust_isa_sve_fmlslb_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svfloat16_t va = svld1_f16(ph, (const float16_t *)(const void *)(a + i * 2));
        svfloat16_t vb = svld1_f16(ph, (const float16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svmlslb_f32(vacc, va, vb));
    }
}

void rust_isa_sve_fmlslt_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svfloat16_t va = svld1_f16(ph, (const float16_t *)(const void *)(a + i * 2));
        svfloat16_t vb = svld1_f16(ph, (const float16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svmlslt_f32(vacc, va, vb));
    }
}

void rust_isa_sve_cdot_i32(const int32_t *acc, const int8_t *a, const int8_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t p8 = svwhilelt_b8_u64(i * 4, len * 4);
        svint32_t vacc = svld1_s32(pg, acc + i);
        svint8_t va = svld1_s8(p8, a + i * 4);
        svint8_t vb = svld1_s8(p8, b + i * 4);
        svst1_s32(pg, out + i, svcdot_s32(vacc, va, vb, 0));
    }
}

void rust_isa_sve_cdot_lane_i32(const int32_t *acc, const int8_t *a, const int8_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t p8 = svwhilelt_b8_u64(i * 4, len * 4);
        svint32_t vacc = svld1_s32(pg, acc + i);
        svint8_t va = svld1_s8(p8, a + i * 4);
        svint8_t vb = svld1_s8(p8, b + i * 4);
        svst1_s32(pg, out + i, svcdot_lane_s32(vacc, va, vb, 0, 0));
    }
}

void rust_isa_sve_cmla_i32(const int32_t *acc, const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t vacc = svld1_s32(pg, acc + i);
        svint32_t va = svld1_s32(pg, a + i);
        svint32_t vb = svld1_s32(pg, b + i);
        svst1_s32(pg, out + i, svcmla_s32(vacc, va, vb, 0));
    }
}

void rust_isa_sve_cmla_lane_i32(const int32_t *acc, const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svint32_t vacc = svld1_s32(pg, acc + i);
        svint32_t va = svld1_s32(pg, a + i);
        svint32_t vb = svld1_s32(pg, b + i);
        svst1_s32(pg, out + i, svcmla_lane_s32(vacc, va, vb, 0, 0));
    }
}


void rust_isa_sve_fcmla_lane_f32(const float *acc, const float *a, const float *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svfloat32_t va = svld1_f32(pg, a + i);
        svfloat32_t vb = svld1_f32(pg, b + i);
        svst1_f32(pg, out + i, svcmla_lane_f32(vacc, va, vb, 0, 0));
    }
}

void rust_isa_sve_addhnb_i16(const int16_t *a, const int16_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svbool_t pgh = svwhilelt_b16_u64(i * 2, len * 2);
        svint16_t va = svld1_s16(pgh, a + i * 2);
        svint16_t vb = svld1_s16(pgh, b + i * 2);
        svst1_s8(pg, out + i, svaddhnb_s16(va, vb));
    }
}

void rust_isa_sve_addhnt_i16(const int16_t *a, const int16_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svbool_t pgh = svwhilelt_b16_u64(i * 2, len * 2);
        svint16_t va = svld1_s16(pgh, a + i * 2);
        svint16_t vb = svld1_s16(pgh, b + i * 2);
        svint8_t vacc = svld1_s8(pg, out + i);
        svst1_s8(pg, out + i, svaddhnt_s16(vacc, va, vb));
    }
}

void rust_isa_sve_aese_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svuint8_t va = svld1_u8(pg, a + i);
        svuint8_t vb = svld1_u8(pg, b + i);
        svst1_u8(pg, out + i, svaese_u8(va, vb));
    }
}

void rust_isa_sve_aesd_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svuint8_t va = svld1_u8(pg, a + i);
        svuint8_t vb = svld1_u8(pg, b + i);
        svst1_u8(pg, out + i, svaesd_u8(va, vb));
    }
}

void rust_isa_sve_aesimc_u8(const uint8_t *a, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svuint8_t va = svld1_u8(pg, a + i);
        svst1_u8(pg, out + i, svaesimc_u8(va));
    }
}

void rust_isa_sve_aesmc_u8(const uint8_t *a, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svuint8_t va = svld1_u8(pg, a + i);
        svst1_u8(pg, out + i, svaesmc_u8(va));
    }
}

void rust_isa_sve_sm4e_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t va = svld1_u32(pg, a + i);
        svuint32_t vb = svld1_u32(pg, b + i);
        svst1_u32(pg, out + i, svsm4e_u32(va, vb));
    }
}

void rust_isa_sve_sm4ekey_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t va = svld1_u32(pg, a + i);
        svuint32_t vb = svld1_u32(pg, b + i);
        svst1_u32(pg, out + i, svsm4ekey_u32(va, vb));
    }
}

void rust_isa_sve_bcax_u64(const uint64_t *a, const uint64_t *b, const uint64_t *c, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svuint64_t va = svld1_u64(pg, a + i);
        svuint64_t vb = svld1_u64(pg, b + i);
        svuint64_t vc = svld1_u64(pg, c + i);
        svst1_u64(pg, out + i, svbcax_u64(va, vb, vc));
    }
}

void rust_isa_sve_eor3_u64(const uint64_t *a, const uint64_t *b, const uint64_t *c, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svuint64_t va = svld1_u64(pg, a + i);
        svuint64_t vb = svld1_u64(pg, b + i);
        svuint64_t vc = svld1_u64(pg, c + i);
        svst1_u64(pg, out + i, sveor3_u64(va, vb, vc));
    }
}

void rust_isa_sve_rax1_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svuint64_t va = svld1_u64(pg, a + i);
        svuint64_t vb = svld1_u64(pg, b + i);
        svst1_u64(pg, out + i, svrax1_u64(va, vb));
    }
}

void rust_isa_sve_xar_u64(const uint64_t *a, const uint64_t *b, uint64_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntd()) {
        svbool_t pg = svwhilelt_b64_u64(i, len);
        svuint64_t va = svld1_u64(pg, a + i);
        svuint64_t vb = svld1_u64(pg, b + i);
        svst1_u64(pg, out + i, svxar_n_u64(va, vb, 1));
    }
}

void rust_isa_sve_adclb_u32(const uint32_t *a, const uint32_t *b, const uint32_t *c, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t va = svld1_u32(pg, a + i);
        svuint32_t vb = svld1_u32(pg, b + i);
        svuint32_t vc = svld1_u32(pg, c + i);
        svst1_u32(pg, out + i, svadclb_u32(va, vb, vc));
    }
}

void rust_isa_sve_adclt_u32(const uint32_t *a, const uint32_t *b, const uint32_t *c, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t va = svld1_u32(pg, a + i);
        svuint32_t vb = svld1_u32(pg, b + i);
        svuint32_t vc = svld1_u32(pg, c + i);
        svst1_u32(pg, out + i, svadclt_u32(va, vb, vc));
    }
}

void rust_isa_sve_sbclb_u32(const uint32_t *a, const uint32_t *b, const uint32_t *c, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t va = svld1_u32(pg, a + i);
        svuint32_t vb = svld1_u32(pg, b + i);
        svuint32_t vc = svld1_u32(pg, c + i);
        svst1_u32(pg, out + i, svsbclb_u32(va, vb, vc));
    }
}

void rust_isa_sve_sbclt_u32(const uint32_t *a, const uint32_t *b, const uint32_t *c, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t va = svld1_u32(pg, a + i);
        svuint32_t vb = svld1_u32(pg, b + i);
        svuint32_t vc = svld1_u32(pg, c + i);
        svst1_u32(pg, out + i, svsbclt_u32(va, vb, vc));
    }
}

void rust_isa_sve_bfdot_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svbfloat16_t va = svld1_bf16(ph, (const bfloat16_t *)(const void *)(a + i * 2));
        svbfloat16_t vb = svld1_bf16(ph, (const bfloat16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svbfdot_f32(vacc, va, vb));
    }
}

void rust_isa_sve_bfmlalb_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svbfloat16_t va = svld1_bf16(ph, (const bfloat16_t *)(const void *)(a + i * 2));
        svbfloat16_t vb = svld1_bf16(ph, (const bfloat16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svbfmlalb_f32(vacc, va, vb));
    }
}

void rust_isa_sve_bfmlalt_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svbfloat16_t va = svld1_bf16(ph, (const bfloat16_t *)(const void *)(a + i * 2));
        svbfloat16_t vb = svld1_bf16(ph, (const bfloat16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svbfmlalt_f32(vacc, va, vb));
    }
}

void rust_isa_sve_bfmmla_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svbfloat16_t va = svld1_bf16(ph, (const bfloat16_t *)(const void *)(a + i * 2));
        svbfloat16_t vb = svld1_bf16(ph, (const bfloat16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svbfmmla_f32(vacc, va, vb));
    }
}

void rust_isa_sve_shrnb_i16(const int16_t *a, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svbool_t pgh = svwhilelt_b16_u64(i, len);
        svint16_t va = svld1_s16(pgh, a + i);
        svst1_s8(pg, out + i, svshrnb_n_s16(va, 1));
    }
}

void rust_isa_sve_shrnt_i16(const int16_t *a, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svbool_t pgh = svwhilelt_b16_u64(i, len);
        svint16_t va = svld1_s16(pgh, a + i);
        svint8_t vacc = svld1_s8(pg, out + i);
        svst1_s8(pg, out + i, svshrnt_n_s16(vacc, va, 1));
    }
}

void rust_isa_sve_rshrnb_i16(const int16_t *a, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svbool_t pgh = svwhilelt_b16_u64(i, len);
        svint16_t va = svld1_s16(pgh, a + i);
        svst1_s8(pg, out + i, svrshrnb_n_s16(va, 1));
    }
}

void rust_isa_sve_rshrnt_i16(const int16_t *a, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svbool_t pgh = svwhilelt_b16_u64(i, len);
        svint16_t va = svld1_s16(pgh, a + i);
        svint8_t vacc = svld1_s8(pg, out + i);
        svst1_s8(pg, out + i, svrshrnt_n_s16(vacc, va, 1));
    }
}

void rust_isa_sve_pmul_u8(const uint8_t *a, const uint8_t *b, uint8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svuint8_t va = svld1_u8(pg, a + i);
        svuint8_t vb = svld1_u8(pg, b + i);
        svst1_u8(pg, out + i, svpmul_u8(va, vb));
    }
}

void rust_isa_sve_pmullb_u16(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svbool_t pb = svwhilelt_b8_u64(i, len);
        svuint8_t va = svld1_u8(pb, a + i);
        svuint8_t vb = svld1_u8(pb, b + i);
        svst1_u16(pg, out + i, svpmullb_u16(va, vb));
    }
}

void rust_isa_sve_pmullt_u16(const uint8_t *a, const uint8_t *b, uint16_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcnth()) {
        svbool_t pg = svwhilelt_b16_u64(i, len);
        svbool_t pb = svwhilelt_b8_u64(i, len);
        svuint8_t va = svld1_u8(pb, a + i);
        svuint8_t vb = svld1_u8(pb, b + i);
        svst1_u16(pg, out + i, svpmullt_u16(va, vb));
    }
}

void rust_isa_sve_smaxp_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svmaxp_s32_x(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_sminp_i32(const int32_t *a, const int32_t *b, int32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_s32(pg, out + i, svminp_s32_x(pg, svld1_s32(pg, a + i), svld1_s32(pg, b + i)));
    }
}

void rust_isa_sve_umaxp_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svmaxp_u32_x(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_uminp_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svst1_u32(pg, out + i, svminp_u32_x(pg, svld1_u32(pg, a + i), svld1_u32(pg, b + i)));
    }
}

void rust_isa_sve_bdep_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t va = svld1_u32(pg, a + i);
        svuint32_t vb = svld1_u32(pg, b + i);
        svst1_u32(pg, out + i, svbdep_u32(va, vb));
    }
}

void rust_isa_sve_bext_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t va = svld1_u32(pg, a + i);
        svuint32_t vb = svld1_u32(pg, b + i);
        svst1_u32(pg, out + i, svbext_u32(va, vb));
    }
}

void rust_isa_sve_bgrp_u32(const uint32_t *a, const uint32_t *b, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t va = svld1_u32(pg, a + i);
        svuint32_t vb = svld1_u32(pg, b + i);
        svst1_u32(pg, out + i, svbgrp_u32(va, vb));
    }
}

void rust_isa_sve_eorbt_u32(const uint32_t *a, const uint32_t *b, const uint32_t *c, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t va = svld1_u32(pg, a + i);
        svuint32_t vb = svld1_u32(pg, b + i);
        svuint32_t vc = svld1_u32(pg, c + i);
        svst1_u32(pg, out + i, sveorbt_u32(va, vb, vc));
    }
}

void rust_isa_sve_eortb_u32(const uint32_t *a, const uint32_t *b, const uint32_t *c, uint32_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svuint32_t va = svld1_u32(pg, a + i);
        svuint32_t vb = svld1_u32(pg, b + i);
        svuint32_t vc = svld1_u32(pg, c + i);
        svst1_u32(pg, out + i, sveortb_u32(va, vb, vc));
    }
}

void rust_isa_sve_subhnb_i16(const int16_t *a, const int16_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svbool_t pgh = svwhilelt_b16_u64(i, len);
        svint16_t va = svld1_s16(pgh, a + i);
        svint16_t vb = svld1_s16(pgh, b + i);
        svst1_s8(pg, out + i, svsubhnb_s16(va, vb));
    }
}

void rust_isa_sve_subhnt_i16(const int16_t *a, const int16_t *b, int8_t *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntb()) {
        svbool_t pg = svwhilelt_b8_u64(i, len);
        svbool_t pgh = svwhilelt_b16_u64(i, len);
        svint16_t va = svld1_s16(pgh, a + i);
        svint16_t vb = svld1_s16(pgh, b + i);
        svint8_t vacc = svld1_s8(pg, out + i);
        svst1_s8(pg, out + i, svsubhnt_s16(vacc, va, vb));
    }
}

void rust_isa_sve_fmlalb_lane_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svfloat16_t va = svld1_f16(ph, (const float16_t *)(const void *)(a + i * 2));
        svfloat16_t vb = svld1_f16(ph, (const float16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svmlalb_lane_f32(vacc, va, vb, 0));
    }
}

void rust_isa_sve_fmlalt_lane_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svfloat16_t va = svld1_f16(ph, (const float16_t *)(const void *)(a + i * 2));
        svfloat16_t vb = svld1_f16(ph, (const float16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svmlalt_lane_f32(vacc, va, vb, 0));
    }
}

void rust_isa_sve_fmlslb_lane_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svfloat16_t va = svld1_f16(ph, (const float16_t *)(const void *)(a + i * 2));
        svfloat16_t vb = svld1_f16(ph, (const float16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svmlslb_lane_f32(vacc, va, vb, 0));
    }
}

void rust_isa_sve_fmlslt_lane_f32(const float *acc, const uint16_t *a, const uint16_t *b, float *out, size_t len) {
    for (size_t i = 0; i < len; i += svcntw()) {
        svbool_t pg = svwhilelt_b32_u64(i, len);
        svbool_t ph = svwhilelt_b16_u64(i * 2, len * 2);
        svfloat32_t vacc = svld1_f32(pg, acc + i);
        svfloat16_t va = svld1_f16(ph, (const float16_t *)(const void *)(a + i * 2));
        svfloat16_t vb = svld1_f16(ph, (const float16_t *)(const void *)(b + i * 2));
        svst1_f32(pg, out + i, svmlslt_lane_f32(vacc, va, vb, 0));
    }
}
