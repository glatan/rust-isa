//! Unaligned load instructions.

use super::V256;

unsafe extern "C" {
    fn rust_isa_avx_vlddqu(p: *const u8, out: *mut u8);
}

/// `vlddqu`
///
/// Load 32 unaligned bytes.
///
/// # Safety
///
/// The caller must enable AVX and `p` must be valid for reading 32 bytes.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vlddqu(p: *const u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vlddqu(p, out.as_mut_ptr());
    }
    out
}
