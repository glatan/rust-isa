//! Masked memory and load instructions.

use super::V256;

unsafe extern "C" {
    fn rust_isa_avx_vmaskmovps_load(p: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_avx_vmaskmovps_store(p: *mut u8, mask: *const u8, value: *const u8);
    fn rust_isa_avx_vmaskmovpd_load(p: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_avx_vmaskmovpd_store(p: *mut u8, mask: *const u8, value: *const u8);
    fn rust_isa_avx_vlddqu(p: *const u8, out: *mut u8);
}

/// `vmaskmovps` load form
///
/// Load the selected 32-bit float lanes. Unselected lanes are zeroed.
///
/// # Safety
///
/// The caller must enable AVX and `p` must be valid for reading 32 bytes.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmaskmovps_load(p: *const u8, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vmaskmovps_load(p, mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaskmovps` store form
///
/// Store the selected 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX and `p` must be valid for writing 32 bytes.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmaskmovps_store(p: *mut u8, mask: V256, value: V256) {
    unsafe {
        rust_isa_avx_vmaskmovps_store(p, mask.as_ptr(), value.as_ptr());
    }
}

/// `vmaskmovpd` load form
///
/// Load the selected 64-bit float lanes. Unselected lanes are zeroed.
///
/// # Safety
///
/// The caller must enable AVX and `p` must be valid for reading 32 bytes.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmaskmovpd_load(p: *const u8, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vmaskmovpd_load(p, mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaskmovpd` store form
///
/// Store the selected 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX and `p` must be valid for writing 32 bytes.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmaskmovpd_store(p: *mut u8, mask: V256, value: V256) {
    unsafe {
        rust_isa_avx_vmaskmovpd_store(p, mask.as_ptr(), value.as_ptr());
    }
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
