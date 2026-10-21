//! AVX-VNNI-INT8.
//!
//! AVX-VNNI-INT8 is not part of the x86-64 baseline. Every operation is
//! `unsafe` and the caller must enable it with
//! `#[target_feature(enable = "avxvnniint8")]` or a global target feature.
//!
//! The values are [`V256`]. Each instruction multiplies four byte products and
//! accumulates into one 32-bit lane. The suffix describes the signedness of the
//! two byte operands: `ss` is signed/signed, `su` is signed/unsigned. The `s`
//! variants saturate the 32-bit result.

use super::avx2::V256;

unsafe extern "C" {
    fn rust_isa_avxvnniint8_vpdpbssd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbssds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbsud(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbsuds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbssd`
///
/// Multiply signed bytes and accumulate into 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbssd(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbssd(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbssds`
///
/// Like [`vpdpbssd`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbssds(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbssds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbsud`
///
/// Multiply signed bytes in `a` by unsigned bytes in `b` and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbsud(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbsud(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbsuds`
///
/// Like [`vpdpbsud`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbsuds(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbsuds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
