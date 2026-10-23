//! Bitwise instructions.

use super::V256;

unsafe extern "C" {
    fn rust_isa_avx2_vpand(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpxor(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpand`
///
/// Bitwise AND of the 32 bytes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpand(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpand(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpor`
///
/// Bitwise OR of the 32 bytes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpor(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpor(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpxor`
///
/// Bitwise XOR of the 32 bytes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpxor(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpxor(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
