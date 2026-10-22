//! Variable permute instructions.

use super::V256;
unsafe extern "C" {
    fn rust_isa_avx2_vpermps(a: *const u8, idx: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpermd(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermps`
///
/// Permute the eight 32-bit float lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpermps(a: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpermps(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpermd`
///
/// Permute the eight 32-bit integer lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpermd(a: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpermd(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}
