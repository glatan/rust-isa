//! Non-temporal stores.

use super::V256;

unsafe extern "C" {
    fn rust_isa_avx_vmovntps(p: *mut u8, value: *const u8);
    fn rust_isa_avx_vmovntpd(p: *mut u8, value: *const u8);
}

/// `vmovntps`
///
/// Store 32 bytes with a non-temporal hint.
///
/// # Safety
///
/// The caller must enable AVX and `p` must be aligned to 32 bytes and valid for
/// writing 32 bytes.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovntps(p: *mut u8, value: V256) {
    unsafe {
        rust_isa_avx_vmovntps(p, value.as_ptr());
    }
}

/// `vmovntpd`
///
/// Store 32 bytes with a non-temporal hint.
///
/// # Safety
///
/// The caller must enable AVX and `p` must be aligned to 32 bytes and valid for
/// writing 32 bytes.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovntpd(p: *mut u8, value: V256) {
    unsafe {
        rust_isa_avx_vmovntpd(p, value.as_ptr());
    }
}
