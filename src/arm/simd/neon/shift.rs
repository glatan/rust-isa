//! NEON shift instructions with saturation and rounding.

use super::V128;
use super::wide_macros::*;

unsafe extern "C" {
    fn rust_isa_arm_neon_vshr_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vqshl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrshl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vshl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqshlu_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vqrshl_s16(a: *const u8, shift: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrshr_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vrsra_s16(acc: *const u8, a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vsli_s16(a: *const u8, b: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vsra_s16(acc: *const u8, a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vsri_s16(a: *const u8, b: *const u8, n: i32, out: *mut u8);
}

shift_unary!(
    /// `vqshlu.s16`. Saturating shift left by `N` into unsigned 16-bit lanes.
    /// `N` must be in `0..=15`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqshlu,
    rust_isa_arm_neon_vqshlu_s16,
    0,
    15
);

binary!(
    /// `vqrshl.s16`. Saturating rounding shift left by the signed lane counts
    /// in `shift`. A negative count shifts right.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqrshl,
    rust_isa_arm_neon_vqrshl_s16
);

shift_unary!(
    /// `vrshr.s16`. Rounding shift right by `N`. `N` must be in `1..=16`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrshr,
    rust_isa_arm_neon_vrshr_s16,
    1,
    16
);

shift_accumulate!(
    /// `vrsra.s16`. Rounding shift right by `N` and accumulate. `N` must be in
    /// `1..=16`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrsra,
    rust_isa_arm_neon_vrsra_s16,
    1,
    16
);

shift_binary!(
    /// `vsli.s16`. Shift `b` left by `N` and insert it into `a`. `N` must be in
    /// `0..=15`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsli,
    rust_isa_arm_neon_vsli_s16,
    0,
    15
);

shift_accumulate!(
    /// `vsra.s16`. Shift `a` right by `N` and accumulate into `acc`. `N` must
    /// be in `1..=16`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsra,
    rust_isa_arm_neon_vsra_s16,
    1,
    16
);

shift_binary!(
    /// `vsri.s16`. Shift `b` right by `N` and insert it into `a`. `N` must be
    /// in `1..=16`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsri,
    rust_isa_arm_neon_vsri_s16,
    1,
    16
);

/// `vshr.s16`. Shift each signed 16-bit lane right by `N`. `N` must be in
/// `1..=16`.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vshr<const N: u32>(a: V128) -> V128 {
    const { assert!(N >= 1 && N <= 16, "shift immediate out of range") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vshr_s16(a.as_ptr(), N as i32, out.as_mut_ptr());
    }
    out
}

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
