//! NEON widening instructions.
//!
//! The AArch32 widening forms read the low 8 bytes of each source operand.
//! Each function exposes one representative data width of the instruction.
//!
//! The caller must compile the whole crate with `-C target-feature=+neon`.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_vabal_s8(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vabdl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vaddl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vaddw_s8(acc: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmlal_s16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmlsl_s16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmull_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vpadal_s16(acc: *const u8, a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqdmlal_s16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqdmlsl_s16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqdmull_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vshll_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vsubl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vsubw_s8(acc: *const u8, b: *const u8, out: *mut u8);
}

use super::wide_macros::*;

multiply_accumulate!(
    /// `vabal.s8`. Absolute difference of the low 8 bytes, accumulated into
    /// 16-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vabal,
    rust_isa_arm_neon_vabal_s8
);
binary!(
    /// `vabdl.s8`. Absolute difference of the low 8 bytes, widened to 16 bits.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vabdl,
    rust_isa_arm_neon_vabdl_s8
);
binary!(
    /// `vaddl.s8`. Add the low 8 bytes, widened to 16 bits.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vaddl,
    rust_isa_arm_neon_vaddl_s8
);
accumulate!(
    /// `vaddw.s8`. Widen the low 8 bytes and add them to 16-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vaddw,
    rust_isa_arm_neon_vaddw_s8
);
multiply_accumulate!(
    /// `vmlal.s16`. Multiply the low four 16-bit lanes and accumulate into
    /// 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmlal,
    rust_isa_arm_neon_vmlal_s16
);
multiply_accumulate!(
    /// `vmlsl.s16`. Multiply the low four 16-bit lanes and subtract from
    /// 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmlsl,
    rust_isa_arm_neon_vmlsl_s16
);
binary!(
    /// `vmull.s16`. Multiply the low four 16-bit lanes, widened to 32 bits.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmull,
    rust_isa_arm_neon_vmull_s16
);
accumulate!(
    /// `vpadal.s16`. Add adjacent 16-bit lane pairs and accumulate into 32-bit
    /// lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vpadal,
    rust_isa_arm_neon_vpadal_s16
);
multiply_accumulate!(
    /// `vqdmlal.s16`. Saturating doubling multiply of the low four 16-bit
    /// lanes, accumulated into 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqdmlal,
    rust_isa_arm_neon_vqdmlal_s16
);
multiply_accumulate!(
    /// `vqdmlsl.s16`. Saturating doubling multiply of the low four 16-bit
    /// lanes, subtracted from 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqdmlsl,
    rust_isa_arm_neon_vqdmlsl_s16
);
binary!(
    /// `vqdmull.s16`. Saturating doubling multiply of the low four 16-bit
    /// lanes, widened to 32 bits.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqdmull,
    rust_isa_arm_neon_vqdmull_s16
);
shift_unary!(
    /// `vshll.s16`. Shift the low four 16-bit lanes left by `N` and widen to
    /// 32 bits. `N` must be in `0..=15`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vshll,
    rust_isa_arm_neon_vshll_s16,
    0,
    15
);
binary!(
    /// `vsubl.s8`. Subtract the low 8 bytes, widened to 16 bits.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsubl,
    rust_isa_arm_neon_vsubl_s8
);
accumulate!(
    /// `vsubw.s8`. Widen the low 8 bytes and subtract them from 16-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsubw,
    rust_isa_arm_neon_vsubw_s8
);
