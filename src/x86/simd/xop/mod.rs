//! XOP.
//!
//! XOP is an AMD extension and is not part of the x86-64 baseline. rustc does
//! not allow `#[target_feature(enable = "xop")]` on stable, so the caller must
//! compile the whole crate with the feature enabled and the CPU must support
//! XOP. The wrappers do not carry `#[target_feature]`.
//!
//! The module contains the byte permute instruction. It selects bytes from the
//! concatenation of the first two operands using the low five bits of each
//! control byte.

use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_xop_vpperm(a: *const u8, b: *const u8, control: *const u8, out: *mut u8);
}

/// `vpperm`
///
/// Permute bytes from `a` and `b` using `control`.
///
/// # Safety
///
/// The caller must compile with XOP enabled and the CPU must support it.
#[inline(always)]
pub unsafe fn vpperm(a: V128, b: V128, control: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_xop_vpperm(a.as_ptr(), b.as_ptr(), control.as_ptr(), out.as_mut_ptr());
    }
    out
}
