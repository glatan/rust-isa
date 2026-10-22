//! NEON arithmetic, compare, and bitwise operations.
//!
//! Each function exposes one representative data width of the instruction.
//! The caller must compile the whole crate with `-C target-feature=+neon`.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_vaba_s8(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vabd_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vabs_s32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vceq_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcge_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcgt_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcle_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vclt_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcls_s8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vclz_s8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcnt_s8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcvt_s32_f32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vhadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vhsub_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmax_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmin_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmla_s32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmls_s32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vneg_s32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqabs_s32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqdmulh_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqneg_s32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqrdmulh_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqshl_s32(a: *const u8, shift: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqsub_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrshl_s32(a: *const u8, shift: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vshl_s32(a: *const u8, shift: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vbic_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vbsl_u8(mask: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmvn_u8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vorn_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vtst_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vfma_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vfms_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vfnma_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vfnms_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrecpe_f32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrecps_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrsqrte_f32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrsqrts_f32(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! ternary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(acc: V128, a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

ternary!(
    /// `vaba.s8`. Absolute difference of `a` and `b`, accumulated into `acc`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vaba,
    rust_isa_arm_neon_vaba_s8
);
binary!(
    /// `vabd.s8`. Absolute difference of the signed byte lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vabd,
    rust_isa_arm_neon_vabd_s8
);
unary!(
    /// `vabs.s32`. Absolute value of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vabs,
    rust_isa_arm_neon_vabs_s32
);
binary!(
    /// `vceq.i8`. A true byte lane is all bits one.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vceq,
    rust_isa_arm_neon_vceq_s8
);
binary!(
    /// `vcge.s8`. A true byte lane is all bits one.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcge,
    rust_isa_arm_neon_vcge_s8
);
binary!(
    /// `vcgt.s8`. A true byte lane is all bits one.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcgt,
    rust_isa_arm_neon_vcgt_s8
);
binary!(
    /// `vcle.s8`. A true byte lane is all bits one.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcle,
    rust_isa_arm_neon_vcle_s8
);
binary!(
    /// `vclt.s8`. A true byte lane is all bits one.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vclt,
    rust_isa_arm_neon_vclt_s8
);
unary!(
    /// `vcls.s8`. Count the leading sign bits of each byte lane.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcls,
    rust_isa_arm_neon_vcls_s8
);
unary!(
    /// `vclz.i8`. Count the leading zeros of each byte lane.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vclz,
    rust_isa_arm_neon_vclz_s8
);
unary!(
    /// `vcnt.8`. Count the set bits of each byte lane.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcnt,
    rust_isa_arm_neon_vcnt_s8
);
unary!(
    /// `vcvt.s32.f32`. Convert the 32-bit float lanes to signed integers,
    /// rounding toward zero.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcvt,
    rust_isa_arm_neon_vcvt_s32_f32
);
binary!(
    /// `vhadd.s32`. Halving add of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vhadd,
    rust_isa_arm_neon_vhadd_s32
);
binary!(
    /// `vhsub.s32`. Halving subtract of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vhsub,
    rust_isa_arm_neon_vhsub_s32
);
binary!(
    /// `vmax.s32`. Maximum of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmax,
    rust_isa_arm_neon_vmax_s32
);
binary!(
    /// `vmin.s32`. Minimum of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmin,
    rust_isa_arm_neon_vmin_s32
);
ternary!(
    /// `vmla.s32`. Multiply `a` and `b` and accumulate into `acc`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmla,
    rust_isa_arm_neon_vmla_s32
);
ternary!(
    /// `vmls.s32`. Multiply `a` and `b` and subtract from `acc`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmls,
    rust_isa_arm_neon_vmls_s32
);
unary!(
    /// `vneg.s32`. Negate the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vneg,
    rust_isa_arm_neon_vneg_s32
);
unary!(
    /// `vqabs.s32`. Saturating absolute value of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqabs,
    rust_isa_arm_neon_vqabs_s32
);
binary!(
    /// `vqadd.s32`. Saturating add of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqadd,
    rust_isa_arm_neon_vqadd_s32
);
binary!(
    /// `vqdmulh.s16`. Doubling multiply of the signed 16-bit lanes, keeping the
    /// high half and saturating.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqdmulh,
    rust_isa_arm_neon_vqdmulh_s16
);
unary!(
    /// `vqneg.s32`. Saturating negate of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqneg,
    rust_isa_arm_neon_vqneg_s32
);
binary!(
    /// `vqrdmulh.s16`. Rounding doubling multiply of the signed 16-bit lanes,
    /// keeping the high half and saturating.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqrdmulh,
    rust_isa_arm_neon_vqrdmulh_s16
);
binary!(
    /// `vqshl.s32`. Saturating shift left of `a` by the signed 32-bit lane
    /// counts in `shift`. A negative count shifts right.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqshl,
    rust_isa_arm_neon_vqshl_s32
);
binary!(
    /// `vqsub.s32`. Saturating subtract of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqsub,
    rust_isa_arm_neon_vqsub_s32
);
binary!(
    /// `vrshl.s32`. Rounding shift left of `a` by the signed 32-bit lane
    /// counts in `shift`. A negative count shifts right.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrshl,
    rust_isa_arm_neon_vrshl_s32
);
binary!(
    /// `vshl.s32`. Shift left of `a` by the signed 32-bit lane counts in
    /// `shift`. A negative count shifts right.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vshl,
    rust_isa_arm_neon_vshl_s32
);
binary!(
    /// `vbic.i8`. Clear the bits of `a` that are set in `b`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vbic,
    rust_isa_arm_neon_vbic_u8
);
ternary!(
    /// `vbsl.i8`. Select the bits of `a` where `mask` is set and the bits of
    /// `b` elsewhere.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vbsl,
    rust_isa_arm_neon_vbsl_u8
);
unary!(
    /// `vmvn.i8`. Invert every bit.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmvn,
    rust_isa_arm_neon_vmvn_u8
);
binary!(
    /// `vorn.i8`. Bitwise OR of `a` with the inverse of `b`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vorn,
    rust_isa_arm_neon_vorn_u8
);
binary!(
    /// `vtst.i8`. A byte lane is all bits one when the bitwise AND of the
    /// operands is nonzero.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vtst,
    rust_isa_arm_neon_vtst_u8
);
ternary!(
    /// `vfma.f32`. Fused multiply-add: `acc + a * b`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON and VFPv4 enabled.
    vfma,
    rust_isa_arm_neon_vfma_f32
);
ternary!(
    /// `vfms.f32`. Fused multiply-subtract: `acc - a * b`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON and VFPv4 enabled.
    vfms,
    rust_isa_arm_neon_vfms_f32
);
ternary!(
    /// `vfnma.f32`. Fused negated multiply-add: `-acc + a * b`.
    ///
    /// AArch32 has no vector form of `VFNMA`, so this negates `acc` and uses
    /// `VFMA`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON and VFPv4 enabled.
    vfnma,
    rust_isa_arm_neon_vfnma_f32
);
ternary!(
    /// `vfnms.f32`. Fused negated multiply-subtract: `-acc - a * b`.
    ///
    /// AArch32 has no vector form of `VFNMS`, so this negates `acc` and uses
    /// `VFMS`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON and VFPv4 enabled.
    vfnms,
    rust_isa_arm_neon_vfnms_f32
);
unary!(
    /// `vrecpe.f32`. Reciprocal estimate of the 32-bit float lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrecpe,
    rust_isa_arm_neon_vrecpe_f32
);
binary!(
    /// `vrecps.f32`. Reciprocal step: `2 - a * b`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrecps,
    rust_isa_arm_neon_vrecps_f32
);
unary!(
    /// `vrsqrte.f32`. Reciprocal square root estimate of the 32-bit float
    /// lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrsqrte,
    rust_isa_arm_neon_vrsqrte_f32
);
binary!(
    /// `vrsqrts.f32`. Reciprocal square root step: `(3 - a * b) / 2`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrsqrts,
    rust_isa_arm_neon_vrsqrts_f32
);
