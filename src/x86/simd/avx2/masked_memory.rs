//! Masked load and store instructions.

use super::V256;

unsafe extern "C" {
    fn rust_isa_avx2_vpmaskmovd_load(p: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmaskmovd_store(p: *mut u8, mask: *const u8, value: *const u8);
    fn rust_isa_avx2_vpmaskmovq_load(p: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmaskmovq_store(p: *mut u8, mask: *const u8, value: *const u8);
}

/// `vpmaskmovd` load form
///
/// Load the selected 32-bit lanes. Unselected lanes are zeroed.
///
/// # Safety
///
/// The caller must enable AVX2 and `p` must be valid for reading 32 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaskmovd_load(p: *const u8, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmaskmovd_load(p, mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmaskmovd` store form
///
/// Store the selected 32-bit lanes. Unselected lanes are not written.
///
/// # Safety
///
/// The caller must enable AVX2 and `p` must be valid for writing 32 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaskmovd_store(p: *mut u8, mask: V256, value: V256) {
    unsafe {
        rust_isa_avx2_vpmaskmovd_store(p, mask.as_ptr(), value.as_ptr());
    }
}

/// `vpmaskmovq` load form
///
/// Load the selected 64-bit lanes. Unselected lanes are zeroed.
///
/// # Safety
///
/// The caller must enable AVX2 and `p` must be valid for reading 32 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaskmovq_load(p: *const u8, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmaskmovq_load(p, mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmaskmovq` store form
///
/// Store the selected 64-bit lanes. Unselected lanes are not written.
///
/// # Safety
///
/// The caller must enable AVX2 and `p` must be valid for writing 32 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaskmovq_store(p: *mut u8, mask: V256, value: V256) {
    unsafe {
        rust_isa_avx2_vpmaskmovq_store(p, mask.as_ptr(), value.as_ptr());
    }
}
