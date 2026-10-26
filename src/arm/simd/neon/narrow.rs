//! NEON narrowing instructions.

use super::V128;
use super::wide_macros::*;

unsafe extern "C" {
    fn rust_isa_arm_neon_vaddhn_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmovn_s16(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqmovn_s16(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqmovun_s16(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqshrn_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vqshrun_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vqrshrn_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vqrshrun_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vraddhn_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrshrn_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vrsubhn_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vshrn_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vsubhn_s16(a: *const u8, b: *const u8, out: *mut u8);
}

binary!(
    /// `vaddhn.s16`. Add adjacent 16-bit lanes and keep the high byte of each
    /// result.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vaddhn,
    rust_isa_arm_neon_vaddhn_s16
);

unary!(
    /// `vmovn.s16`. Keep the low byte of each 16-bit lane.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmovn,
    rust_isa_arm_neon_vmovn_s16
);

unary!(
    /// `vqmovn.s16`. Saturating narrow of each 16-bit lane to a signed byte.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqmovn,
    rust_isa_arm_neon_vqmovn_s16
);

unary!(
    /// `vqmovun.s16`. Saturating narrow of each 16-bit lane to an unsigned
    /// byte.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqmovun,
    rust_isa_arm_neon_vqmovun_s16
);

shift_unary!(
    /// `vqshrn.s16`. Saturating shift right by `N` and narrow each 16-bit lane
    /// to a signed byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqshrn,
    rust_isa_arm_neon_vqshrn_s16,
    1,
    8
);

shift_unary!(
    /// `vqshrun.s16`. Saturating shift right by `N` and narrow each 16-bit
    /// lane to an unsigned byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqshrun,
    rust_isa_arm_neon_vqshrun_s16,
    1,
    8
);

shift_unary!(
    /// `vqrshrn.s16`. Saturating rounding shift right by `N` and narrow each
    /// 16-bit lane to a signed byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqrshrn,
    rust_isa_arm_neon_vqrshrn_s16,
    1,
    8
);

shift_unary!(
    /// `vqrshrun.s16`. Saturating rounding shift right by `N` and narrow each
    /// 16-bit lane to an unsigned byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqrshrun,
    rust_isa_arm_neon_vqrshrun_s16,
    1,
    8
);

binary!(
    /// `vraddhn.s16`. Add adjacent 16-bit lanes with rounding and keep the high
    /// byte of each result.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vraddhn,
    rust_isa_arm_neon_vraddhn_s16
);

shift_unary!(
    /// `vrshrn.s16`. Rounding shift right by `N` and narrow each 16-bit lane to
    /// a signed byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrshrn,
    rust_isa_arm_neon_vrshrn_s16,
    1,
    8
);

binary!(
    /// `vrsubhn.s16`. Subtract adjacent 16-bit lanes with rounding and keep the
    /// high byte of each result.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrsubhn,
    rust_isa_arm_neon_vrsubhn_s16
);

shift_unary!(
    /// `vshrn.s16`. Shift right by `N` and narrow each 16-bit lane to a signed
    /// byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vshrn,
    rust_isa_arm_neon_vshrn_s16,
    1,
    8
);

binary!(
    /// `vsubhn.s16`. Subtract adjacent 16-bit lanes and keep the high byte of
    /// each result.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsubhn,
    rust_isa_arm_neon_vsubhn_s16
);
