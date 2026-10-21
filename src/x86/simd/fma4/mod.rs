//! FMA4.
//!
//! FMA4 is an AMD extension and is not part of the x86-64 baseline. rustc does
//! not allow `#[target_feature(enable = "fma4")]` on stable, so the caller must
//! compile the whole crate with the feature enabled and the CPU must support
//! FMA4. The wrappers do not carry `#[target_feature]`.
//!
//! The values are [`V128`]. The four-operand form is exposed as three operands
//! because the destination is also the first source in this API.

use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_fma4_macc_ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_macc_pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_msub_ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddps`
///
/// Compute `a * b + c` with a single rounding.
///
/// # Safety
///
/// The caller must compile with FMA4 enabled and the CPU must support it.
#[inline(always)]
pub unsafe fn macc_ps(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fma4_macc_ps(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfmaddpd`
///
/// Compute `a * b + c` with a single rounding on 64-bit lanes.
///
/// # Safety
///
/// The caller must compile with FMA4 enabled and the CPU must support it.
#[inline(always)]
pub unsafe fn macc_pd(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fma4_macc_pd(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfmsubps`
///
/// Compute `a * b - c` with a single rounding.
///
/// # Safety
///
/// The caller must compile with FMA4 enabled and the CPU must support it.
#[inline(always)]
pub unsafe fn msub_ps(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fma4_msub_ps(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}
