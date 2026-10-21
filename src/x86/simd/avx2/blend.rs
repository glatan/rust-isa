//! Blend instructions.

use super::V256;

unsafe extern "C" {
    fn rust_isa_avx2_vpblendvb(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
}

/// `vpblendvb`
///
/// Select each byte from `a` or `b` using the high bit of the corresponding
/// byte in `mask`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpblendvb(a: V256, b: V256, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpblendvb(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr());
    }
    out
}
