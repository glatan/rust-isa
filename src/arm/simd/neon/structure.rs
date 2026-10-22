//! NEON structure instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_zip1_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_uzp1_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_trn1_u8(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vzip.8`
///
/// Interleave the low halves of two byte vectors.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn zip1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_zip1_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vuzp.8`
///
/// Take the even bytes from two vectors and concatenate them.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn uzp1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_uzp1_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vtrn.8`
///
/// Transpose the even bytes of two vectors.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn trn1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_trn1_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
