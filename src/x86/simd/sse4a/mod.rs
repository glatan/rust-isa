//! SSE4a.
//!
//! SSE4a is not part of the x86-64 baseline and is available on AMD processors.
//! Every operation is `unsafe` and the caller must enable SSE4a with
//! `#[target_feature(enable = "sse4a")]` or a global target feature.
//!
//! The values are [`V128`]. `extrq` and `insertq` use the register-controlled
//! form. The control vector packs the length in bits 5:0 and the index in bits
//! 13:8 of its low 64 bits.

use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_sse4a_extrq(a: *const u8, control: *const u8, out: *mut u8);
    fn rust_isa_sse4a_insertq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `extrq`
///
/// Extract a bit field from the low 64 bits.
///
/// # Safety
///
/// The caller must enable SSE4a.
#[target_feature(enable = "sse4a")]
#[inline]
pub unsafe fn extrq(a: V128, control: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse4a_extrq(a.as_ptr(), control.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `insertq`
///
/// Insert the low bits of `b` into `a`. The register form takes the length
/// from bits 69:64 of `b` and the index from bits 77:72 of `b`; these are the
/// low six bits of bytes 8 and 9. A length of zero means 64. The source bits
/// are the low 64 bits of `b`. The high 64 bits of the result are undefined by
/// the instruction.
///
/// # Safety
///
/// The caller must enable SSE4a.
#[target_feature(enable = "sse4a")]
#[inline]
pub unsafe fn insertq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse4a_insertq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
