//! Additional NEON instructions.
//!
//! The operations in this module cover the widening, narrowing, saturating,
//! shift, conversion, and structure instructions that are not in the other
//! NEON modules. Lanes are little-endian, as in the rest of the crate.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_sabd_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sabd_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sabd_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabd_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabd_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabd_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sabdl_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sabdl_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sabdl_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabdl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabdl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabdl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_abs_i64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_shadd_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shsub_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srhadd_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shadd_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shsub_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srhadd_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shsub_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srhadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhadd_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhsub_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urhadd_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhadd_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhsub_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urhadd_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhadd_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhsub_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urhadd_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smull_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smull2_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal2_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl2_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smull_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smull2_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal2_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl2_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smull_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smull2_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal2_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl2_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umull_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umull2_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal2_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl2_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umull_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umull2_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal_u16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal2_u16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl_u16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl2_u16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umull_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umull2_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal_u32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal2_u32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl_u32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl2_u32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmull_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmull2_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlal_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlal2_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlsl_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlsl2_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmulh_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrdmulh_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmull_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmull2_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlal_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlal2_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlsl_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlsl2_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmulh_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrdmulh_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqadd_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqsub_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqadd_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqsub_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqsub_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqadd_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqsub_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqadd_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqsub_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqadd_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqsub_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqadd_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqsub_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqadd_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqsub_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqabs_s8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_sqneg_s8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_sqabs_s16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_sqneg_s16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_sqabs_s32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_sqneg_s32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn_s8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn_s16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn_s32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn_u8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn_u16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn_u32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn2_i16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn2_i32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn2_i64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn2_i16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn2_i32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn2_i64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn2_i16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn2_i32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn2_i64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn2_i16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn2_i32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn2_i64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_pmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_pmull_p8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_pmull2_p8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_frecpe_f32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_frsqrte_f32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_frecps_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_frsqrts_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_frecpe_f64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_frsqrte_f64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_frecps_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_frsqrts_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_scvtf_f32_s32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ucvtf_f32_u32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_scvtf_f64_s64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ucvtf_f64_u64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_fcvtzs_s32_f32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_fcvtzu_u32_f32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_fcvtzs_s64_f64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_fcvtzu_u64_f64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_shl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sshl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srshl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqshl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrshl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sshl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srshl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqshl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrshl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sshl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srshl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqshl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrshl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shl_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sshl_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srshl_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqshl_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrshl_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ushl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urshl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqshl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqrshl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ushl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urshl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqshl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqrshl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ushl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urshl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqshl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqrshl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ushl_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urshl_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqshl_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqrshl_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sshr_s8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_srshr_s8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_ssra_s8(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_srsra_s8(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sshr_s16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_srshr_s16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_ssra_s16(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_srsra_s16(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sshr_s32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_srshr_s32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_ssra_s32(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_srsra_s32(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sshr_s64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_srshr_s64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_ssra_s64(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_srsra_s64(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_shrn_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shrn2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shrn_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shrn2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shrn_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shrn2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shll_i8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shll_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shll_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sli_s8(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sri_s8(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sli_s16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sri_s16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sli_s32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sri_s32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sli_s64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sri_s64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshlu_s8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshlu_s16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshlu_s32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshlu_s64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_urecpe_u32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ursqrte_u32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_cnt_u64(a: u64) -> u64;
    fn rust_isa_neon_clz_u64(a: u64) -> u64;
    fn rust_isa_neon_cls_u64(a: u64) -> u64;
    fn rust_isa_neon_frecpx_f32(a: f32) -> f32;
    fn rust_isa_neon_frecpx_f64(a: f64) -> f64;
    fn rust_isa_neon_pmull_p64(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! acc {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(acc: V128, a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), acc.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! narrow2 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, low: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), low.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! addhn2 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128, low: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), low.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! shift {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, count: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), count as i32, out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! shift_narrow2 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, low: V128, count: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), low.as_ptr(), count as i32, out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! shift_acc {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128, count: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), count as i32, b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! binary_count {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128, count: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), count as i32, out.as_mut_ptr()); }
            out
        }
    };
}

binary!(
    /// `sabd` on 8-bit signed lanes: absolute difference.
    sabd_i8,
    rust_isa_neon_sabd_i8
);

binary!(
    /// `sabd` on 16-bit signed lanes: absolute difference.
    sabd_i16,
    rust_isa_neon_sabd_i16
);

binary!(
    /// `sabd` on 32-bit signed lanes: absolute difference.
    sabd_i32,
    rust_isa_neon_sabd_i32
);

binary!(
    /// `uabd` on 8-bit unsigned lanes: absolute difference.
    uabd_u8,
    rust_isa_neon_uabd_u8
);

binary!(
    /// `uabd` on 16-bit unsigned lanes: absolute difference.
    uabd_u16,
    rust_isa_neon_uabd_u16
);

binary!(
    /// `uabd` on 32-bit unsigned lanes: absolute difference.
    uabd_u32,
    rust_isa_neon_uabd_u32
);

binary!(
    /// `sabdl` on 8-bit signed lanes: absolute difference widened to the next size.
    sabdl_i8,
    rust_isa_neon_sabdl_i8
);

binary!(
    /// `sabdl` on 16-bit signed lanes: absolute difference widened to the next size.
    sabdl_i16,
    rust_isa_neon_sabdl_i16
);

binary!(
    /// `sabdl` on 32-bit signed lanes: absolute difference widened to the next size.
    sabdl_i32,
    rust_isa_neon_sabdl_i32
);

binary!(
    /// `uabdl` on 8-bit unsigned lanes: absolute difference widened to the next size.
    uabdl_u8,
    rust_isa_neon_uabdl_u8
);

binary!(
    /// `uabdl` on 16-bit unsigned lanes: absolute difference widened to the next size.
    uabdl_u16,
    rust_isa_neon_uabdl_u16
);

binary!(
    /// `uabdl` on 32-bit unsigned lanes: absolute difference widened to the next size.
    uabdl_u32,
    rust_isa_neon_uabdl_u32
);

unary!(
    /// `abs` on 64-bit signed lanes.
    abs_i64,
    rust_isa_neon_abs_i64
);

binary!(
    /// `shadd` on 8-bit signed lanes: halving add.
    shadd_s8,
    rust_isa_neon_shadd_s8
);

binary!(
    /// `shsub` on 8-bit signed lanes: halving subtract.
    shsub_s8,
    rust_isa_neon_shsub_s8
);

binary!(
    /// `srhadd` on 8-bit signed lanes: rounding halving add.
    srhadd_s8,
    rust_isa_neon_srhadd_s8
);

binary!(
    /// `shadd` on 16-bit signed lanes: halving add.
    shadd_s16,
    rust_isa_neon_shadd_s16
);

binary!(
    /// `shsub` on 16-bit signed lanes: halving subtract.
    shsub_s16,
    rust_isa_neon_shsub_s16
);

binary!(
    /// `srhadd` on 16-bit signed lanes: rounding halving add.
    srhadd_s16,
    rust_isa_neon_srhadd_s16
);

binary!(
    /// `shadd` on 32-bit signed lanes: halving add.
    shadd_s32,
    rust_isa_neon_shadd_s32
);

binary!(
    /// `shsub` on 32-bit signed lanes: halving subtract.
    shsub_s32,
    rust_isa_neon_shsub_s32
);

binary!(
    /// `srhadd` on 32-bit signed lanes: rounding halving add.
    srhadd_s32,
    rust_isa_neon_srhadd_s32
);

binary!(
    /// `uhadd` on 8-bit unsigned lanes: halving add.
    uhadd_u8,
    rust_isa_neon_uhadd_u8
);

binary!(
    /// `uhsub` on 8-bit unsigned lanes: halving subtract.
    uhsub_u8,
    rust_isa_neon_uhsub_u8
);

binary!(
    /// `urhadd` on 8-bit unsigned lanes: rounding halving add.
    urhadd_u8,
    rust_isa_neon_urhadd_u8
);

binary!(
    /// `uhadd` on 16-bit unsigned lanes: halving add.
    uhadd_u16,
    rust_isa_neon_uhadd_u16
);

binary!(
    /// `uhsub` on 16-bit unsigned lanes: halving subtract.
    uhsub_u16,
    rust_isa_neon_uhsub_u16
);

binary!(
    /// `urhadd` on 16-bit unsigned lanes: rounding halving add.
    urhadd_u16,
    rust_isa_neon_urhadd_u16
);

binary!(
    /// `uhadd` on 32-bit unsigned lanes: halving add.
    uhadd_u32,
    rust_isa_neon_uhadd_u32
);

binary!(
    /// `uhsub` on 32-bit unsigned lanes: halving subtract.
    uhsub_u32,
    rust_isa_neon_uhsub_u32
);

binary!(
    /// `urhadd` on 32-bit unsigned lanes: rounding halving add.
    urhadd_u32,
    rust_isa_neon_urhadd_u32
);

binary!(
    /// `smull` from 8-bit signed lanes to 16-bit signed lanes.
    smull_s8,
    rust_isa_neon_smull_s8
);

binary!(
    /// `smull2` from the high 8-bit signed lanes to 16-bit signed lanes.
    smull2_s8,
    rust_isa_neon_smull2_s8
);

acc!(
    /// `smlal` from 8-bit signed lanes into a 16-bit accumulator.
    smlal_s8,
    rust_isa_neon_smlal_s8
);

acc!(
    /// `smlal2` from the high 8-bit signed lanes into a 16-bit accumulator.
    smlal2_s8,
    rust_isa_neon_smlal2_s8
);

acc!(
    /// `smlsl` from 8-bit signed lanes into a 16-bit accumulator.
    smlsl_s8,
    rust_isa_neon_smlsl_s8
);

acc!(
    /// `smlsl2` from the high 8-bit signed lanes into a 16-bit accumulator.
    smlsl2_s8,
    rust_isa_neon_smlsl2_s8
);

binary!(
    /// `smull` from 16-bit signed lanes to 32-bit signed lanes.
    smull_s16,
    rust_isa_neon_smull_s16
);

binary!(
    /// `smull2` from the high 16-bit signed lanes to 32-bit signed lanes.
    smull2_s16,
    rust_isa_neon_smull2_s16
);

acc!(
    /// `smlal` from 16-bit signed lanes into a 32-bit accumulator.
    smlal_s16,
    rust_isa_neon_smlal_s16
);

acc!(
    /// `smlal2` from the high 16-bit signed lanes into a 32-bit accumulator.
    smlal2_s16,
    rust_isa_neon_smlal2_s16
);

acc!(
    /// `smlsl` from 16-bit signed lanes into a 32-bit accumulator.
    smlsl_s16,
    rust_isa_neon_smlsl_s16
);

acc!(
    /// `smlsl2` from the high 16-bit signed lanes into a 32-bit accumulator.
    smlsl2_s16,
    rust_isa_neon_smlsl2_s16
);

binary!(
    /// `smull` from 32-bit signed lanes to 64-bit signed lanes.
    smull_s32,
    rust_isa_neon_smull_s32
);

binary!(
    /// `smull2` from the high 32-bit signed lanes to 64-bit signed lanes.
    smull2_s32,
    rust_isa_neon_smull2_s32
);

acc!(
    /// `smlal` from 32-bit signed lanes into a 64-bit accumulator.
    smlal_s32,
    rust_isa_neon_smlal_s32
);

acc!(
    /// `smlal2` from the high 32-bit signed lanes into a 64-bit accumulator.
    smlal2_s32,
    rust_isa_neon_smlal2_s32
);

acc!(
    /// `smlsl` from 32-bit signed lanes into a 64-bit accumulator.
    smlsl_s32,
    rust_isa_neon_smlsl_s32
);

acc!(
    /// `smlsl2` from the high 32-bit signed lanes into a 64-bit accumulator.
    smlsl2_s32,
    rust_isa_neon_smlsl2_s32
);

binary!(
    /// `umull` from 8-bit unsigned lanes to 16-bit unsigned lanes.
    umull_u8,
    rust_isa_neon_umull_u8
);

binary!(
    /// `umull2` from the high 8-bit unsigned lanes to 16-bit unsigned lanes.
    umull2_u8,
    rust_isa_neon_umull2_u8
);

acc!(
    /// `umlal` from 8-bit unsigned lanes into a 16-bit accumulator.
    umlal_u8,
    rust_isa_neon_umlal_u8
);

acc!(
    /// `umlal2` from the high 8-bit unsigned lanes into a 16-bit accumulator.
    umlal2_u8,
    rust_isa_neon_umlal2_u8
);

acc!(
    /// `umlsl` from 8-bit unsigned lanes into a 16-bit accumulator.
    umlsl_u8,
    rust_isa_neon_umlsl_u8
);

acc!(
    /// `umlsl2` from the high 8-bit unsigned lanes into a 16-bit accumulator.
    umlsl2_u8,
    rust_isa_neon_umlsl2_u8
);

binary!(
    /// `umull` from 16-bit unsigned lanes to 32-bit unsigned lanes.
    umull_u16,
    rust_isa_neon_umull_u16
);

binary!(
    /// `umull2` from the high 16-bit unsigned lanes to 32-bit unsigned lanes.
    umull2_u16,
    rust_isa_neon_umull2_u16
);

acc!(
    /// `umlal` from 16-bit unsigned lanes into a 32-bit accumulator.
    umlal_u16,
    rust_isa_neon_umlal_u16
);

acc!(
    /// `umlal2` from the high 16-bit unsigned lanes into a 32-bit accumulator.
    umlal2_u16,
    rust_isa_neon_umlal2_u16
);

acc!(
    /// `umlsl` from 16-bit unsigned lanes into a 32-bit accumulator.
    umlsl_u16,
    rust_isa_neon_umlsl_u16
);

acc!(
    /// `umlsl2` from the high 16-bit unsigned lanes into a 32-bit accumulator.
    umlsl2_u16,
    rust_isa_neon_umlsl2_u16
);

binary!(
    /// `umull` from 32-bit unsigned lanes to 64-bit unsigned lanes.
    umull_u32,
    rust_isa_neon_umull_u32
);

binary!(
    /// `umull2` from the high 32-bit unsigned lanes to 64-bit unsigned lanes.
    umull2_u32,
    rust_isa_neon_umull2_u32
);

acc!(
    /// `umlal` from 32-bit unsigned lanes into a 64-bit accumulator.
    umlal_u32,
    rust_isa_neon_umlal_u32
);

acc!(
    /// `umlal2` from the high 32-bit unsigned lanes into a 64-bit accumulator.
    umlal2_u32,
    rust_isa_neon_umlal2_u32
);

acc!(
    /// `umlsl` from 32-bit unsigned lanes into a 64-bit accumulator.
    umlsl_u32,
    rust_isa_neon_umlsl_u32
);

acc!(
    /// `umlsl2` from the high 32-bit unsigned lanes into a 64-bit accumulator.
    umlsl2_u32,
    rust_isa_neon_umlsl2_u32
);

binary!(
    /// `sqdmull` from 16-bit signed lanes to 32-bit signed lanes.
    sqdmull_s16,
    rust_isa_neon_sqdmull_s16
);

binary!(
    /// `sqdmull2` from the high 16-bit signed lanes to 32-bit signed lanes.
    sqdmull2_s16,
    rust_isa_neon_sqdmull2_s16
);

acc!(
    /// `sqdmlal` from 16-bit signed lanes into a 32-bit accumulator.
    sqdmlal_s16,
    rust_isa_neon_sqdmlal_s16
);

acc!(
    /// `sqdmlal2` from the high 16-bit signed lanes into a 32-bit accumulator.
    sqdmlal2_s16,
    rust_isa_neon_sqdmlal2_s16
);

acc!(
    /// `sqdmlsl` from 16-bit signed lanes into a 32-bit accumulator.
    sqdmlsl_s16,
    rust_isa_neon_sqdmlsl_s16
);

acc!(
    /// `sqdmlsl2` from the high 16-bit signed lanes into a 32-bit accumulator.
    sqdmlsl2_s16,
    rust_isa_neon_sqdmlsl2_s16
);

binary!(
    /// `sqdmulh` on 16-bit signed lanes: saturating doubling multiply high.
    sqdmulh_s16,
    rust_isa_neon_sqdmulh_s16
);

binary!(
    /// `sqrdmulh` on 16-bit signed lanes: saturating rounding doubling multiply high.
    sqrdmulh_s16,
    rust_isa_neon_sqrdmulh_s16
);

binary!(
    /// `sqdmull` from 32-bit signed lanes to 64-bit signed lanes.
    sqdmull_s32,
    rust_isa_neon_sqdmull_s32
);

binary!(
    /// `sqdmull2` from the high 32-bit signed lanes to 64-bit signed lanes.
    sqdmull2_s32,
    rust_isa_neon_sqdmull2_s32
);

acc!(
    /// `sqdmlal` from 32-bit signed lanes into a 64-bit accumulator.
    sqdmlal_s32,
    rust_isa_neon_sqdmlal_s32
);

acc!(
    /// `sqdmlal2` from the high 32-bit signed lanes into a 64-bit accumulator.
    sqdmlal2_s32,
    rust_isa_neon_sqdmlal2_s32
);

acc!(
    /// `sqdmlsl` from 32-bit signed lanes into a 64-bit accumulator.
    sqdmlsl_s32,
    rust_isa_neon_sqdmlsl_s32
);

acc!(
    /// `sqdmlsl2` from the high 32-bit signed lanes into a 64-bit accumulator.
    sqdmlsl2_s32,
    rust_isa_neon_sqdmlsl2_s32
);

binary!(
    /// `sqdmulh` on 32-bit signed lanes: saturating doubling multiply high.
    sqdmulh_s32,
    rust_isa_neon_sqdmulh_s32
);

binary!(
    /// `sqrdmulh` on 32-bit signed lanes: saturating rounding doubling multiply high.
    sqrdmulh_s32,
    rust_isa_neon_sqrdmulh_s32
);

binary!(
    /// `sqadd` on 8-bit signed lanes.
    sqadd_s8,
    rust_isa_neon_sqadd_s8
);

binary!(
    /// `sqsub` on 8-bit signed lanes.
    sqsub_s8,
    rust_isa_neon_sqsub_s8
);

binary!(
    /// `sqadd` on 16-bit signed lanes.
    sqadd_s16,
    rust_isa_neon_sqadd_s16
);

binary!(
    /// `sqsub` on 16-bit signed lanes.
    sqsub_s16,
    rust_isa_neon_sqsub_s16
);

binary!(
    /// `sqadd` on 32-bit signed lanes.
    sqadd_s32,
    rust_isa_neon_sqadd_s32
);

binary!(
    /// `sqsub` on 32-bit signed lanes.
    sqsub_s32,
    rust_isa_neon_sqsub_s32
);

binary!(
    /// `sqadd` on 64-bit signed lanes.
    sqadd_s64,
    rust_isa_neon_sqadd_s64
);

binary!(
    /// `sqsub` on 64-bit signed lanes.
    sqsub_s64,
    rust_isa_neon_sqsub_s64
);

binary!(
    /// `uqadd` on 8-bit unsigned lanes.
    uqadd_u8,
    rust_isa_neon_uqadd_u8
);

binary!(
    /// `uqsub` on 8-bit unsigned lanes.
    uqsub_u8,
    rust_isa_neon_uqsub_u8
);

binary!(
    /// `uqadd` on 16-bit unsigned lanes.
    uqadd_u16,
    rust_isa_neon_uqadd_u16
);

binary!(
    /// `uqsub` on 16-bit unsigned lanes.
    uqsub_u16,
    rust_isa_neon_uqsub_u16
);

binary!(
    /// `uqadd` on 32-bit unsigned lanes.
    uqadd_u32,
    rust_isa_neon_uqadd_u32
);

binary!(
    /// `uqsub` on 32-bit unsigned lanes.
    uqsub_u32,
    rust_isa_neon_uqsub_u32
);

binary!(
    /// `uqadd` on 64-bit unsigned lanes.
    uqadd_u64,
    rust_isa_neon_uqadd_u64
);

binary!(
    /// `uqsub` on 64-bit unsigned lanes.
    uqsub_u64,
    rust_isa_neon_uqsub_u64
);

unary!(
    /// `sqabs` on 8-bit signed lanes.
    sqabs_s8,
    rust_isa_neon_sqabs_s8
);

unary!(
    /// `sqneg` on 8-bit signed lanes.
    sqneg_s8,
    rust_isa_neon_sqneg_s8
);

unary!(
    /// `sqabs` on 16-bit signed lanes.
    sqabs_s16,
    rust_isa_neon_sqabs_s16
);

unary!(
    /// `sqneg` on 16-bit signed lanes.
    sqneg_s16,
    rust_isa_neon_sqneg_s16
);

unary!(
    /// `sqabs` on 32-bit signed lanes.
    sqabs_s32,
    rust_isa_neon_sqabs_s32
);

unary!(
    /// `sqneg` on 32-bit signed lanes.
    sqneg_s32,
    rust_isa_neon_sqneg_s32
);

unary!(
    /// `xtn` from 16-bit signed lanes to 8-bit signed lanes.
    xtn_s8,
    rust_isa_neon_xtn_s8
);

narrow2!(
    /// `xtn2` from 16-bit signed lanes into the upper half of the result.
    xtn2_s8,
    rust_isa_neon_xtn2_s8
);

unary!(
    /// `xtn` from 32-bit signed lanes to 16-bit signed lanes.
    xtn_s16,
    rust_isa_neon_xtn_s16
);

narrow2!(
    /// `xtn2` from 32-bit signed lanes into the upper half of the result.
    xtn2_s16,
    rust_isa_neon_xtn2_s16
);

unary!(
    /// `xtn` from 64-bit signed lanes to 32-bit signed lanes.
    xtn_s32,
    rust_isa_neon_xtn_s32
);

narrow2!(
    /// `xtn2` from 64-bit signed lanes into the upper half of the result.
    xtn2_s32,
    rust_isa_neon_xtn2_s32
);

unary!(
    /// `xtn` from 16-bit unsigned lanes to 8-bit unsigned lanes.
    xtn_u8,
    rust_isa_neon_xtn_u8
);

narrow2!(
    /// `xtn2` from 16-bit unsigned lanes into the upper half of the result.
    xtn2_u8,
    rust_isa_neon_xtn2_u8
);

unary!(
    /// `xtn` from 32-bit unsigned lanes to 16-bit unsigned lanes.
    xtn_u16,
    rust_isa_neon_xtn_u16
);

narrow2!(
    /// `xtn2` from 32-bit unsigned lanes into the upper half of the result.
    xtn2_u16,
    rust_isa_neon_xtn2_u16
);

unary!(
    /// `xtn` from 64-bit unsigned lanes to 32-bit unsigned lanes.
    xtn_u32,
    rust_isa_neon_xtn_u32
);

narrow2!(
    /// `xtn2` from 64-bit unsigned lanes into the upper half of the result.
    xtn2_u32,
    rust_isa_neon_xtn2_u32
);

binary!(
    /// `addhn` from 16-bit lanes, keeping the high half of each result.
    addhn_i16,
    rust_isa_neon_addhn_i16
);

addhn2!(
    /// `addhn2` from 16-bit lanes into the upper half of the result.
    addhn2_i16,
    rust_isa_neon_addhn2_i16
);

binary!(
    /// `addhn` from 32-bit lanes, keeping the high half of each result.
    addhn_i32,
    rust_isa_neon_addhn_i32
);

addhn2!(
    /// `addhn2` from 32-bit lanes into the upper half of the result.
    addhn2_i32,
    rust_isa_neon_addhn2_i32
);

binary!(
    /// `addhn` from 64-bit lanes, keeping the high half of each result.
    addhn_i64,
    rust_isa_neon_addhn_i64
);

addhn2!(
    /// `addhn2` from 64-bit lanes into the upper half of the result.
    addhn2_i64,
    rust_isa_neon_addhn2_i64
);

binary!(
    /// `raddhn` from 16-bit lanes, keeping the high half of each result.
    raddhn_i16,
    rust_isa_neon_raddhn_i16
);

addhn2!(
    /// `raddhn2` from 16-bit lanes into the upper half of the result.
    raddhn2_i16,
    rust_isa_neon_raddhn2_i16
);

binary!(
    /// `raddhn` from 32-bit lanes, keeping the high half of each result.
    raddhn_i32,
    rust_isa_neon_raddhn_i32
);

addhn2!(
    /// `raddhn2` from 32-bit lanes into the upper half of the result.
    raddhn2_i32,
    rust_isa_neon_raddhn2_i32
);

binary!(
    /// `raddhn` from 64-bit lanes, keeping the high half of each result.
    raddhn_i64,
    rust_isa_neon_raddhn_i64
);

addhn2!(
    /// `raddhn2` from 64-bit lanes into the upper half of the result.
    raddhn2_i64,
    rust_isa_neon_raddhn2_i64
);

binary!(
    /// `subhn` from 16-bit lanes, keeping the high half of each result.
    subhn_i16,
    rust_isa_neon_subhn_i16
);

addhn2!(
    /// `subhn2` from 16-bit lanes into the upper half of the result.
    subhn2_i16,
    rust_isa_neon_subhn2_i16
);

binary!(
    /// `subhn` from 32-bit lanes, keeping the high half of each result.
    subhn_i32,
    rust_isa_neon_subhn_i32
);

addhn2!(
    /// `subhn2` from 32-bit lanes into the upper half of the result.
    subhn2_i32,
    rust_isa_neon_subhn2_i32
);

binary!(
    /// `subhn` from 64-bit lanes, keeping the high half of each result.
    subhn_i64,
    rust_isa_neon_subhn_i64
);

addhn2!(
    /// `subhn2` from 64-bit lanes into the upper half of the result.
    subhn2_i64,
    rust_isa_neon_subhn2_i64
);

binary!(
    /// `rsubhn` from 16-bit lanes, keeping the high half of each result.
    rsubhn_i16,
    rust_isa_neon_rsubhn_i16
);

addhn2!(
    /// `rsubhn2` from 16-bit lanes into the upper half of the result.
    rsubhn2_i16,
    rust_isa_neon_rsubhn2_i16
);

binary!(
    /// `rsubhn` from 32-bit lanes, keeping the high half of each result.
    rsubhn_i32,
    rust_isa_neon_rsubhn_i32
);

addhn2!(
    /// `rsubhn2` from 32-bit lanes into the upper half of the result.
    rsubhn2_i32,
    rust_isa_neon_rsubhn2_i32
);

binary!(
    /// `rsubhn` from 64-bit lanes, keeping the high half of each result.
    rsubhn_i64,
    rust_isa_neon_rsubhn_i64
);

addhn2!(
    /// `rsubhn2` from 64-bit lanes into the upper half of the result.
    rsubhn2_i64,
    rust_isa_neon_rsubhn2_i64
);

binary!(
    /// `pmul` on polynomial bytes.
    pmul,
    rust_isa_neon_pmul
);

binary!(
    /// `pmull` on polynomial bytes, producing 16-bit polynomial lanes.
    pmull_p8,
    rust_isa_neon_pmull_p8
);

binary!(
    /// `pmull2` on the high polynomial bytes, producing 16-bit polynomial lanes.
    pmull2_p8,
    rust_isa_neon_pmull2_p8
);

unary!(
    /// `frecpe` estimate on 32-bit float lanes.
    frecpe_f32,
    rust_isa_neon_frecpe_f32
);

unary!(
    /// `frsqrte` estimate on 32-bit float lanes.
    frsqrte_f32,
    rust_isa_neon_frsqrte_f32
);

binary!(
    /// `frecps` step on 32-bit float lanes.
    frecps_f32,
    rust_isa_neon_frecps_f32
);

binary!(
    /// `frsqrts` step on 32-bit float lanes.
    frsqrts_f32,
    rust_isa_neon_frsqrts_f32
);

unary!(
    /// `frecpe` estimate on 64-bit float lanes.
    frecpe_f64,
    rust_isa_neon_frecpe_f64
);

unary!(
    /// `frsqrte` estimate on 64-bit float lanes.
    frsqrte_f64,
    rust_isa_neon_frsqrte_f64
);

binary!(
    /// `frecps` step on 64-bit float lanes.
    frecps_f64,
    rust_isa_neon_frecps_f64
);

binary!(
    /// `frsqrts` step on 64-bit float lanes.
    frsqrts_f64,
    rust_isa_neon_frsqrts_f64
);

unary!(
    /// `scvtf` from signed 32-bit lanes to f32 lanes.
    scvtf_f32_s32,
    rust_isa_neon_scvtf_f32_s32
);

unary!(
    /// `ucvtf` from unsigned 32-bit lanes to f32 lanes.
    ucvtf_f32_u32,
    rust_isa_neon_ucvtf_f32_u32
);

unary!(
    /// `scvtf` from signed 64-bit lanes to f64 lanes.
    scvtf_f64_s64,
    rust_isa_neon_scvtf_f64_s64
);

unary!(
    /// `ucvtf` from unsigned 64-bit lanes to f64 lanes.
    ucvtf_f64_u64,
    rust_isa_neon_ucvtf_f64_u64
);

unary!(
    /// `fcvtzs` from f32 lanes to signed 32-bit lanes, rounding toward zero.
    fcvtzs_s32_f32,
    rust_isa_neon_fcvtzs_s32_f32
);

unary!(
    /// `fcvtzu` from f32 lanes to unsigned 32-bit lanes, rounding toward zero.
    fcvtzu_u32_f32,
    rust_isa_neon_fcvtzu_u32_f32
);

unary!(
    /// `fcvtzs` from f64 lanes to signed 64-bit lanes, rounding toward zero.
    fcvtzs_s64_f64,
    rust_isa_neon_fcvtzs_s64_f64
);

unary!(
    /// `fcvtzu` from f64 lanes to unsigned 64-bit lanes, rounding toward zero.
    fcvtzu_u64_f64,
    rust_isa_neon_fcvtzu_u64_f64
);

binary!(
    /// `shl` on 8-bit signed lanes by the signed counts in `b`.
    shl_s8,
    rust_isa_neon_shl_s8
);

binary!(
    /// `sshl` on 8-bit signed lanes by the signed counts in `b`.
    sshl_s8,
    rust_isa_neon_sshl_s8
);

binary!(
    /// `srshl` on 8-bit signed lanes by the signed counts in `b`, rounding.
    srshl_s8,
    rust_isa_neon_srshl_s8
);

binary!(
    /// `sqshl` on 8-bit signed lanes by the signed counts in `b`.
    sqshl_s8,
    rust_isa_neon_sqshl_s8
);

binary!(
    /// `sqrshl` on 8-bit signed lanes by the signed counts in `b`, rounding.
    sqrshl_s8,
    rust_isa_neon_sqrshl_s8
);

binary!(
    /// `shl` on 16-bit signed lanes by the signed counts in `b`.
    shl_s16,
    rust_isa_neon_shl_s16
);

binary!(
    /// `sshl` on 16-bit signed lanes by the signed counts in `b`.
    sshl_s16,
    rust_isa_neon_sshl_s16
);

binary!(
    /// `srshl` on 16-bit signed lanes by the signed counts in `b`, rounding.
    srshl_s16,
    rust_isa_neon_srshl_s16
);

binary!(
    /// `sqshl` on 16-bit signed lanes by the signed counts in `b`.
    sqshl_s16,
    rust_isa_neon_sqshl_s16
);

binary!(
    /// `sqrshl` on 16-bit signed lanes by the signed counts in `b`, rounding.
    sqrshl_s16,
    rust_isa_neon_sqrshl_s16
);

binary!(
    /// `shl` on 32-bit signed lanes by the signed counts in `b`.
    shl_s32,
    rust_isa_neon_shl_s32
);

binary!(
    /// `sshl` on 32-bit signed lanes by the signed counts in `b`.
    sshl_s32,
    rust_isa_neon_sshl_s32
);

binary!(
    /// `srshl` on 32-bit signed lanes by the signed counts in `b`, rounding.
    srshl_s32,
    rust_isa_neon_srshl_s32
);

binary!(
    /// `sqshl` on 32-bit signed lanes by the signed counts in `b`.
    sqshl_s32,
    rust_isa_neon_sqshl_s32
);

binary!(
    /// `sqrshl` on 32-bit signed lanes by the signed counts in `b`, rounding.
    sqrshl_s32,
    rust_isa_neon_sqrshl_s32
);

binary!(
    /// `shl` on 64-bit signed lanes by the signed counts in `b`.
    shl_s64,
    rust_isa_neon_shl_s64
);

binary!(
    /// `sshl` on 64-bit signed lanes by the signed counts in `b`.
    sshl_s64,
    rust_isa_neon_sshl_s64
);

binary!(
    /// `srshl` on 64-bit signed lanes by the signed counts in `b`, rounding.
    srshl_s64,
    rust_isa_neon_srshl_s64
);

binary!(
    /// `sqshl` on 64-bit signed lanes by the signed counts in `b`.
    sqshl_s64,
    rust_isa_neon_sqshl_s64
);

binary!(
    /// `sqrshl` on 64-bit signed lanes by the signed counts in `b`, rounding.
    sqrshl_s64,
    rust_isa_neon_sqrshl_s64
);

binary!(
    /// `ushl` on 8-bit unsigned lanes by the signed counts in `b`.
    ushl_u8,
    rust_isa_neon_ushl_u8
);

binary!(
    /// `urshl` on 8-bit unsigned lanes by the signed counts in `b`, rounding.
    urshl_u8,
    rust_isa_neon_urshl_u8
);

binary!(
    /// `uqshl` on 8-bit unsigned lanes by the signed counts in `b`.
    uqshl_u8,
    rust_isa_neon_uqshl_u8
);

binary!(
    /// `uqrshl` on 8-bit unsigned lanes by the signed counts in `b`, rounding.
    uqrshl_u8,
    rust_isa_neon_uqrshl_u8
);

binary!(
    /// `ushl` on 16-bit unsigned lanes by the signed counts in `b`.
    ushl_u16,
    rust_isa_neon_ushl_u16
);

binary!(
    /// `urshl` on 16-bit unsigned lanes by the signed counts in `b`, rounding.
    urshl_u16,
    rust_isa_neon_urshl_u16
);

binary!(
    /// `uqshl` on 16-bit unsigned lanes by the signed counts in `b`.
    uqshl_u16,
    rust_isa_neon_uqshl_u16
);

binary!(
    /// `uqrshl` on 16-bit unsigned lanes by the signed counts in `b`, rounding.
    uqrshl_u16,
    rust_isa_neon_uqrshl_u16
);

binary!(
    /// `ushl` on 32-bit unsigned lanes by the signed counts in `b`.
    ushl_u32,
    rust_isa_neon_ushl_u32
);

binary!(
    /// `urshl` on 32-bit unsigned lanes by the signed counts in `b`, rounding.
    urshl_u32,
    rust_isa_neon_urshl_u32
);

binary!(
    /// `uqshl` on 32-bit unsigned lanes by the signed counts in `b`.
    uqshl_u32,
    rust_isa_neon_uqshl_u32
);

binary!(
    /// `uqrshl` on 32-bit unsigned lanes by the signed counts in `b`, rounding.
    uqrshl_u32,
    rust_isa_neon_uqrshl_u32
);

binary!(
    /// `ushl` on 64-bit unsigned lanes by the signed counts in `b`.
    ushl_u64,
    rust_isa_neon_ushl_u64
);

binary!(
    /// `urshl` on 64-bit unsigned lanes by the signed counts in `b`, rounding.
    urshl_u64,
    rust_isa_neon_urshl_u64
);

binary!(
    /// `uqshl` on 64-bit unsigned lanes by the signed counts in `b`.
    uqshl_u64,
    rust_isa_neon_uqshl_u64
);

binary!(
    /// `uqrshl` on 64-bit unsigned lanes by the signed counts in `b`, rounding.
    uqrshl_u64,
    rust_isa_neon_uqrshl_u64
);

shift!(
    /// `sshr` on 8-bit signed lanes by `count` bits.
    sshr_s8,
    rust_isa_neon_sshr_s8
);

shift!(
    /// `srshr` on 8-bit signed lanes by `count` bits, rounding.
    srshr_s8,
    rust_isa_neon_srshr_s8
);

shift_acc!(
    /// `ssra` adds `a >> count` to `b` on 8-bit signed lanes.
    ssra_s8,
    rust_isa_neon_ssra_s8
);

shift_acc!(
    /// `srsra` adds `a >> count` to `b` on 8-bit signed lanes, rounding.
    srsra_s8,
    rust_isa_neon_srsra_s8
);

shift!(
    /// `sshr` on 16-bit signed lanes by `count` bits.
    sshr_s16,
    rust_isa_neon_sshr_s16
);

shift!(
    /// `srshr` on 16-bit signed lanes by `count` bits, rounding.
    srshr_s16,
    rust_isa_neon_srshr_s16
);

shift_acc!(
    /// `ssra` adds `a >> count` to `b` on 16-bit signed lanes.
    ssra_s16,
    rust_isa_neon_ssra_s16
);

shift_acc!(
    /// `srsra` adds `a >> count` to `b` on 16-bit signed lanes, rounding.
    srsra_s16,
    rust_isa_neon_srsra_s16
);

shift!(
    /// `sshr` on 32-bit signed lanes by `count` bits.
    sshr_s32,
    rust_isa_neon_sshr_s32
);

shift!(
    /// `srshr` on 32-bit signed lanes by `count` bits, rounding.
    srshr_s32,
    rust_isa_neon_srshr_s32
);

shift_acc!(
    /// `ssra` adds `a >> count` to `b` on 32-bit signed lanes.
    ssra_s32,
    rust_isa_neon_ssra_s32
);

shift_acc!(
    /// `srsra` adds `a >> count` to `b` on 32-bit signed lanes, rounding.
    srsra_s32,
    rust_isa_neon_srsra_s32
);

shift!(
    /// `sshr` on 64-bit signed lanes by `count` bits.
    sshr_s64,
    rust_isa_neon_sshr_s64
);

shift!(
    /// `srshr` on 64-bit signed lanes by `count` bits, rounding.
    srshr_s64,
    rust_isa_neon_srshr_s64
);

shift_acc!(
    /// `ssra` adds `a >> count` to `b` on 64-bit signed lanes.
    ssra_s64,
    rust_isa_neon_ssra_s64
);

shift_acc!(
    /// `srsra` adds `a >> count` to `b` on 64-bit signed lanes, rounding.
    srsra_s64,
    rust_isa_neon_srsra_s64
);

shift!(
    /// `shrn` shifts 16-bit lanes right by `count` and narrows to 8-bit lanes.
    shrn_i16,
    rust_isa_neon_shrn_i16
);

shift_narrow2!(
    /// `shrn2` narrows the shifted high 16-bit lanes into the upper half of the result.
    shrn2_i16,
    rust_isa_neon_shrn2_i16
);

shift!(
    /// `shrn` shifts 32-bit lanes right by `count` and narrows to 16-bit lanes.
    shrn_i32,
    rust_isa_neon_shrn_i32
);

shift_narrow2!(
    /// `shrn2` narrows the shifted high 32-bit lanes into the upper half of the result.
    shrn2_i32,
    rust_isa_neon_shrn2_i32
);

shift!(
    /// `shrn` shifts 64-bit lanes right by `count` and narrows to 32-bit lanes.
    shrn_i64,
    rust_isa_neon_shrn_i64
);

shift_narrow2!(
    /// `shrn2` narrows the shifted high 64-bit lanes into the upper half of the result.
    shrn2_i64,
    rust_isa_neon_shrn2_i64
);

shift!(
    /// `sqshrn` shifts 16-bit lanes right by `count` and saturates to 8-bit lanes.
    sqshrn_i16,
    rust_isa_neon_sqshrn_i16
);

shift_narrow2!(
    /// `sqshrn2` saturating narrows into the upper half of the result.
    sqshrn2_i16,
    rust_isa_neon_sqshrn2_i16
);

shift!(
    /// `sqshrun` shifts signed 16-bit lanes right by `count` and saturates to unsigned 8-bit lanes.
    sqshrun_i16,
    rust_isa_neon_sqshrun_i16
);

shift_narrow2!(
    /// `sqshrun2` saturating narrows signed lanes to unsigned lanes into the upper half.
    sqshrun2_i16,
    rust_isa_neon_sqshrun2_i16
);

shift!(
    /// `sqrshrn` shifts 16-bit lanes right by `count` with rounding and saturating narrow.
    sqrshrn_i16,
    rust_isa_neon_sqrshrn_i16
);

shift_narrow2!(
    /// `sqrshrn2` rounding saturating narrows into the upper half of the result.
    sqrshrn2_i16,
    rust_isa_neon_sqrshrn2_i16
);

shift!(
    /// `sqrshrun` shifts signed 16-bit lanes right by `count` with rounding and saturating unsigned narrow.
    sqrshrun_i16,
    rust_isa_neon_sqrshrun_i16
);

shift_narrow2!(
    /// `sqrshrun2` rounding saturating unsigned narrows into the upper half.
    sqrshrun2_i16,
    rust_isa_neon_sqrshrun2_i16
);

shift!(
    /// `uqshrn` shifts unsigned 16-bit lanes right by `count` and saturating narrow.
    uqshrn_i16,
    rust_isa_neon_uqshrn_i16
);

shift_narrow2!(
    /// `uqshrn2` saturating narrows unsigned lanes into the upper half.
    uqshrn2_i16,
    rust_isa_neon_uqshrn2_i16
);

shift!(
    /// `uqrshrn` shifts unsigned 16-bit lanes right by `count` with rounding and saturating narrow.
    uqrshrn_i16,
    rust_isa_neon_uqrshrn_i16
);

shift_narrow2!(
    /// `uqrshrn2` rounding saturating unsigned narrows into the upper half.
    uqrshrn2_i16,
    rust_isa_neon_uqrshrn2_i16
);

shift!(
    /// `sqshrn` shifts 32-bit lanes right by `count` and saturates to 16-bit lanes.
    sqshrn_i32,
    rust_isa_neon_sqshrn_i32
);

shift_narrow2!(
    /// `sqshrn2` saturating narrows into the upper half of the result.
    sqshrn2_i32,
    rust_isa_neon_sqshrn2_i32
);

shift!(
    /// `sqshrun` shifts signed 32-bit lanes right by `count` and saturates to unsigned 16-bit lanes.
    sqshrun_i32,
    rust_isa_neon_sqshrun_i32
);

shift_narrow2!(
    /// `sqshrun2` saturating narrows signed lanes to unsigned lanes into the upper half.
    sqshrun2_i32,
    rust_isa_neon_sqshrun2_i32
);

shift!(
    /// `sqrshrn` shifts 32-bit lanes right by `count` with rounding and saturating narrow.
    sqrshrn_i32,
    rust_isa_neon_sqrshrn_i32
);

shift_narrow2!(
    /// `sqrshrn2` rounding saturating narrows into the upper half of the result.
    sqrshrn2_i32,
    rust_isa_neon_sqrshrn2_i32
);

shift!(
    /// `sqrshrun` shifts signed 32-bit lanes right by `count` with rounding and saturating unsigned narrow.
    sqrshrun_i32,
    rust_isa_neon_sqrshrun_i32
);

shift_narrow2!(
    /// `sqrshrun2` rounding saturating unsigned narrows into the upper half.
    sqrshrun2_i32,
    rust_isa_neon_sqrshrun2_i32
);

shift!(
    /// `uqshrn` shifts unsigned 32-bit lanes right by `count` and saturating narrow.
    uqshrn_i32,
    rust_isa_neon_uqshrn_i32
);

shift_narrow2!(
    /// `uqshrn2` saturating narrows unsigned lanes into the upper half.
    uqshrn2_i32,
    rust_isa_neon_uqshrn2_i32
);

shift!(
    /// `uqrshrn` shifts unsigned 32-bit lanes right by `count` with rounding and saturating narrow.
    uqrshrn_i32,
    rust_isa_neon_uqrshrn_i32
);

shift_narrow2!(
    /// `uqrshrn2` rounding saturating unsigned narrows into the upper half.
    uqrshrn2_i32,
    rust_isa_neon_uqrshrn2_i32
);

shift!(
    /// `sqshrn` shifts 64-bit lanes right by `count` and saturates to 32-bit lanes.
    sqshrn_i64,
    rust_isa_neon_sqshrn_i64
);

shift_narrow2!(
    /// `sqshrn2` saturating narrows into the upper half of the result.
    sqshrn2_i64,
    rust_isa_neon_sqshrn2_i64
);

shift!(
    /// `sqshrun` shifts signed 64-bit lanes right by `count` and saturates to unsigned 32-bit lanes.
    sqshrun_i64,
    rust_isa_neon_sqshrun_i64
);

shift_narrow2!(
    /// `sqshrun2` saturating narrows signed lanes to unsigned lanes into the upper half.
    sqshrun2_i64,
    rust_isa_neon_sqshrun2_i64
);

shift!(
    /// `sqrshrn` shifts 64-bit lanes right by `count` with rounding and saturating narrow.
    sqrshrn_i64,
    rust_isa_neon_sqrshrn_i64
);

shift_narrow2!(
    /// `sqrshrn2` rounding saturating narrows into the upper half of the result.
    sqrshrn2_i64,
    rust_isa_neon_sqrshrn2_i64
);

shift!(
    /// `sqrshrun` shifts signed 64-bit lanes right by `count` with rounding and saturating unsigned narrow.
    sqrshrun_i64,
    rust_isa_neon_sqrshrun_i64
);

shift_narrow2!(
    /// `sqrshrun2` rounding saturating unsigned narrows into the upper half.
    sqrshrun2_i64,
    rust_isa_neon_sqrshrun2_i64
);

shift!(
    /// `uqshrn` shifts unsigned 64-bit lanes right by `count` and saturating narrow.
    uqshrn_i64,
    rust_isa_neon_uqshrn_i64
);

shift_narrow2!(
    /// `uqshrn2` saturating narrows unsigned lanes into the upper half.
    uqshrn2_i64,
    rust_isa_neon_uqshrn2_i64
);

shift!(
    /// `uqrshrn` shifts unsigned 64-bit lanes right by `count` with rounding and saturating narrow.
    uqrshrn_i64,
    rust_isa_neon_uqrshrn_i64
);

shift_narrow2!(
    /// `uqrshrn2` rounding saturating unsigned narrows into the upper half.
    uqrshrn2_i64,
    rust_isa_neon_uqrshrn2_i64
);

shift!(
    /// `shll` shifts 8-bit lanes left by `count` and widens to 16-bit lanes.
    shll_i8,
    rust_isa_neon_shll_i8
);

shift!(
    /// `shll` shifts 16-bit lanes left by `count` and widens to 32-bit lanes.
    shll_i16,
    rust_isa_neon_shll_i16
);

shift!(
    /// `shll` shifts 32-bit lanes left by `count` and widens to 64-bit lanes.
    shll_i32,
    rust_isa_neon_shll_i32
);

binary_count!(
    /// `sli` shifts `a` left by `count` and inserts the low bits of `b`.
    sli_s8,
    rust_isa_neon_sli_s8
);

binary_count!(
    /// `sri` shifts `a` right by `count` and inserts into the upper bits of `b`.
    sri_s8,
    rust_isa_neon_sri_s8
);

binary_count!(
    /// `sli` shifts `a` left by `count` and inserts the low bits of `b`.
    sli_s16,
    rust_isa_neon_sli_s16
);

binary_count!(
    /// `sri` shifts `a` right by `count` and inserts into the upper bits of `b`.
    sri_s16,
    rust_isa_neon_sri_s16
);

binary_count!(
    /// `sli` shifts `a` left by `count` and inserts the low bits of `b`.
    sli_s32,
    rust_isa_neon_sli_s32
);

binary_count!(
    /// `sri` shifts `a` right by `count` and inserts into the upper bits of `b`.
    sri_s32,
    rust_isa_neon_sri_s32
);

binary_count!(
    /// `sli` shifts `a` left by `count` and inserts the low bits of `b`.
    sli_s64,
    rust_isa_neon_sli_s64
);

binary_count!(
    /// `sri` shifts `a` right by `count` and inserts into the upper bits of `b`.
    sri_s64,
    rust_isa_neon_sri_s64
);

shift!(
    /// `sqshlu` shifts signed 8-bit lanes left by `count` and saturates to unsigned.
    sqshlu_s8,
    rust_isa_neon_sqshlu_s8
);

shift!(
    /// `sqshlu` shifts signed 16-bit lanes left by `count` and saturates to unsigned.
    sqshlu_s16,
    rust_isa_neon_sqshlu_s16
);

shift!(
    /// `sqshlu` shifts signed 32-bit lanes left by `count` and saturates to unsigned.
    sqshlu_s32,
    rust_isa_neon_sqshlu_s32
);

shift!(
    /// `sqshlu` shifts signed 64-bit lanes left by `count` and saturates to unsigned.
    sqshlu_s64,
    rust_isa_neon_sqshlu_s64
);

unary!(
    /// `urecpe` estimate on unsigned 32-bit lanes.
    urecpe_u32,
    rust_isa_neon_urecpe_u32
);

unary!(
    /// `ursqrte` estimate on unsigned 32-bit lanes.
    ursqrte_u32,
    rust_isa_neon_ursqrte_u32
);

/// `cnt` on a 64-bit scalar: count of set bits.
#[inline(always)]
pub fn cnt_u64(a: u64) -> u64 {
    unsafe { rust_isa_neon_cnt_u64(a) }
}

/// `clz` on a 64-bit scalar: count of leading zero bits.
#[inline(always)]
pub fn clz_u64(a: u64) -> u64 {
    unsafe { rust_isa_neon_clz_u64(a) }
}

/// `cls` on a 64-bit scalar: count of leading sign bits minus one.
#[inline(always)]
pub fn cls_u64(a: u64) -> u64 {
    unsafe { rust_isa_neon_cls_u64(a) }
}

/// `frecpx` on a 32-bit float: reciprocal exponent.
#[inline(always)]
pub fn frecpx_f32(a: f32) -> f32 {
    unsafe { rust_isa_neon_frecpx_f32(a) }
}

/// `frecpx` on a 64-bit float: reciprocal exponent.
#[inline(always)]
pub fn frecpx_f64(a: f64) -> f64 {
    unsafe { rust_isa_neon_frecpx_f64(a) }
}

/// `pmull` on the low 64-bit polynomial lanes.
///
/// # Safety
///
/// The caller must enable the `aes` feature.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn pmull_p64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_pmull_p64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_neon_dup_s8_0(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_0(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_1(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_1(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_2(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_2(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_3(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_3(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_4(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_4(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_5(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_5(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_6(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_6(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_7(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_7(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_8(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_9(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_9(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_10(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_10(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_11(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_11(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_12(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_12(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_13(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_13(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_14(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_14(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s8_15(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_15(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s16_0(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s16_0(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_dup_s16_1(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s16_1(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_dup_s16_2(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s16_2(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_dup_s16_3(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s16_3(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_dup_s16_4(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s16_4(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_dup_s16_5(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s16_5(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_dup_s16_6(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s16_6(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_dup_s16_7(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s16_7(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_dup_s32_0(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s32_0(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_dup_s32_1(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s32_1(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_dup_s32_2(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s32_2(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_dup_s32_3(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s32_3(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_dup_s64_0(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s64_0(a: *const u8, value: i64, out: *mut u8);
    fn rust_isa_neon_dup_s64_1(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s64_1(a: *const u8, value: i64, out: *mut u8);
    fn rust_isa_neon_ext(a: *const u8, b: *const u8, index: i32, out: *mut u8);
}

/// `dup` on 8-bit lanes: broadcast lane `LANE`.
///
/// `LANE` must be in `0..16`.
#[inline(always)]
pub fn dup_i8<const LANE: u8>(a: V128) -> V128 {
    const { assert!(LANE < 16, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_dup_s8_0),
        1 => call!(rust_isa_neon_dup_s8_1),
        2 => call!(rust_isa_neon_dup_s8_2),
        3 => call!(rust_isa_neon_dup_s8_3),
        4 => call!(rust_isa_neon_dup_s8_4),
        5 => call!(rust_isa_neon_dup_s8_5),
        6 => call!(rust_isa_neon_dup_s8_6),
        7 => call!(rust_isa_neon_dup_s8_7),
        8 => call!(rust_isa_neon_dup_s8_8),
        9 => call!(rust_isa_neon_dup_s8_9),
        10 => call!(rust_isa_neon_dup_s8_10),
        11 => call!(rust_isa_neon_dup_s8_11),
        12 => call!(rust_isa_neon_dup_s8_12),
        13 => call!(rust_isa_neon_dup_s8_13),
        14 => call!(rust_isa_neon_dup_s8_14),
        15 => call!(rust_isa_neon_dup_s8_15),
        _ => unreachable!(),
    }
}

/// `ins` on 8-bit lanes: replace lane `LANE` with `value`.
///
/// `LANE` must be in `0..16`.
#[inline(always)]
pub fn ins_i8<const LANE: u8>(a: V128, value: i8) -> V128 {
    const { assert!(LANE < 16, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), value, out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_ins_s8_0),
        1 => call!(rust_isa_neon_ins_s8_1),
        2 => call!(rust_isa_neon_ins_s8_2),
        3 => call!(rust_isa_neon_ins_s8_3),
        4 => call!(rust_isa_neon_ins_s8_4),
        5 => call!(rust_isa_neon_ins_s8_5),
        6 => call!(rust_isa_neon_ins_s8_6),
        7 => call!(rust_isa_neon_ins_s8_7),
        8 => call!(rust_isa_neon_ins_s8_8),
        9 => call!(rust_isa_neon_ins_s8_9),
        10 => call!(rust_isa_neon_ins_s8_10),
        11 => call!(rust_isa_neon_ins_s8_11),
        12 => call!(rust_isa_neon_ins_s8_12),
        13 => call!(rust_isa_neon_ins_s8_13),
        14 => call!(rust_isa_neon_ins_s8_14),
        15 => call!(rust_isa_neon_ins_s8_15),
        _ => unreachable!(),
    }
}

/// `dup` on 16-bit lanes: broadcast lane `LANE`.
///
/// `LANE` must be in `0..8`.
#[inline(always)]
pub fn dup_i16<const LANE: u8>(a: V128) -> V128 {
    const { assert!(LANE < 8, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_dup_s16_0),
        1 => call!(rust_isa_neon_dup_s16_1),
        2 => call!(rust_isa_neon_dup_s16_2),
        3 => call!(rust_isa_neon_dup_s16_3),
        4 => call!(rust_isa_neon_dup_s16_4),
        5 => call!(rust_isa_neon_dup_s16_5),
        6 => call!(rust_isa_neon_dup_s16_6),
        7 => call!(rust_isa_neon_dup_s16_7),
        _ => unreachable!(),
    }
}

/// `ins` on 16-bit lanes: replace lane `LANE` with `value`.
///
/// `LANE` must be in `0..8`.
#[inline(always)]
pub fn ins_i16<const LANE: u8>(a: V128, value: i16) -> V128 {
    const { assert!(LANE < 8, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), value, out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_ins_s16_0),
        1 => call!(rust_isa_neon_ins_s16_1),
        2 => call!(rust_isa_neon_ins_s16_2),
        3 => call!(rust_isa_neon_ins_s16_3),
        4 => call!(rust_isa_neon_ins_s16_4),
        5 => call!(rust_isa_neon_ins_s16_5),
        6 => call!(rust_isa_neon_ins_s16_6),
        7 => call!(rust_isa_neon_ins_s16_7),
        _ => unreachable!(),
    }
}

/// `dup` on 32-bit lanes: broadcast lane `LANE`.
///
/// `LANE` must be in `0..4`.
#[inline(always)]
pub fn dup_i32<const LANE: u8>(a: V128) -> V128 {
    const { assert!(LANE < 4, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_dup_s32_0),
        1 => call!(rust_isa_neon_dup_s32_1),
        2 => call!(rust_isa_neon_dup_s32_2),
        3 => call!(rust_isa_neon_dup_s32_3),
        _ => unreachable!(),
    }
}

/// `ins` on 32-bit lanes: replace lane `LANE` with `value`.
///
/// `LANE` must be in `0..4`.
#[inline(always)]
pub fn ins_i32<const LANE: u8>(a: V128, value: i32) -> V128 {
    const { assert!(LANE < 4, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), value, out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_ins_s32_0),
        1 => call!(rust_isa_neon_ins_s32_1),
        2 => call!(rust_isa_neon_ins_s32_2),
        3 => call!(rust_isa_neon_ins_s32_3),
        _ => unreachable!(),
    }
}

/// `dup` on 64-bit lanes: broadcast lane `LANE`.
///
/// `LANE` must be in `0..2`.
#[inline(always)]
pub fn dup_i64<const LANE: u8>(a: V128) -> V128 {
    const { assert!(LANE < 2, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_dup_s64_0),
        1 => call!(rust_isa_neon_dup_s64_1),
        _ => unreachable!(),
    }
}

/// `ins` on 64-bit lanes: replace lane `LANE` with `value`.
///
/// `LANE` must be in `0..2`.
#[inline(always)]
pub fn ins_i64<const LANE: u8>(a: V128, value: i64) -> V128 {
    const { assert!(LANE < 2, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), value, out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_ins_s64_0),
        1 => call!(rust_isa_neon_ins_s64_1),
        _ => unreachable!(),
    }
}

/// `ext`
///
/// Extract 16 bytes starting at byte `INDEX` from the concatenation of `a` and `b`.
/// `INDEX` must be less than 16.
#[inline(always)]
pub fn ext<const INDEX: u8>(a: V128, b: V128) -> V128 {
    const { assert!(INDEX < 16, "NEON ext index must be less than 16") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ext(a.as_ptr(), b.as_ptr(), INDEX as i32, out.as_mut_ptr());
    }
    out
}
