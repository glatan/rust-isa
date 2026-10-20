//! Altivec.
//!
//! The caller must compile with `-C target-feature=+altivec,+vsx`.

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_altivec_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vadduwm`
///
/// Add the four 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with Altivec and VSX enabled.
#[inline(always)]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmuluwm`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with Altivec and VSX enabled.
#[inline(always)]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
