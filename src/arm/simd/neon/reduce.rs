//! NEON pairwise reduction instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_vpadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vpmax_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vpmin_s32(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpadd.s32`. Add adjacent lane pairs of each operand.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vpadd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vpadd_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmax.s32`. Take the maximum of adjacent lane pairs of each operand.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vpmax(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vpmax_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmin.s32`. Take the minimum of adjacent lane pairs of each operand.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vpmin(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vpmin_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
