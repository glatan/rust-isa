//! Mask extraction instructions.

use super::V256;

unsafe extern "C" {
    fn rust_isa_avx2_vpmovmskb(a: *const u8) -> u32;
}

/// `vpmovmskb`
///
/// Extract the high bit of each byte into a 32-bit mask.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmovmskb(a: V256) -> u32 {
    unsafe { rust_isa_avx2_vpmovmskb(a.as_ptr()) }
}
