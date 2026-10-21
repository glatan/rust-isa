//! AVX-VNNI-INT16.
//!
//! AVX-VNNI-INT16 is not part of the x86-64 baseline. Every operation is
//! `unsafe` and the caller must enable it with
//! `#[target_feature(enable = "avxvnniint16")]` or a global target feature.
//!
//! The values are [`V256`]. Each instruction multiplies two adjacent 16-bit
//! products and accumulates into one 32-bit lane. The two suffixes describe the
//! signedness of the operands: `su` is signed/unsigned, `uu` is unsigned/
//! unsigned. The `s` variants saturate the 32-bit result.

use super::avx2::V256;

unsafe extern "C" {
    fn rust_isa_avxvnniint16_vpdpwsud(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint16_vpdpwsuds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint16_vpdpwusd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint16_vpdpwuud(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwsud`
///
/// Multiply signed words in `a` by unsigned words in `b` and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwsud(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwsud(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwsuds`
///
/// Like [`vpdpwsud`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwsuds(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwsuds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwusd`
///
/// Multiply unsigned words in `a` by signed words in `b` and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwusd(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwusd(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwuud`
///
/// Multiply unsigned words and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwuud(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwuud(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
