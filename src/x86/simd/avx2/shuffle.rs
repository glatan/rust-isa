//! Shuffle instructions.

use super::V256;

unsafe extern "C" {
    fn rust_isa_avx2_vpshufb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpshufb`
///
/// Shuffle the bytes of `a` with the indices in `b`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpshufb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpshufb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
