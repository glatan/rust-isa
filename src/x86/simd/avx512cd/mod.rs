//! AVX-512 CD.
//!
//! AVX-512 CD is not part of the x86-64 baseline. Every operation is `unsafe`
//! and the caller must enable AVX-512 CD with
//! `#[target_feature(enable = "avx512cd")]` (which implies AVX-512 F) or a
//! global target feature.
//!
//! The values are [`V512`]. The conflict instructions return, for each lane,
//! the bitmask of earlier lanes with the same value.

use super::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512cd_vplzcntd(a: *const u8, out: *mut u8);
    fn rust_isa_avx512cd_vplzcntq(a: *const u8, out: *mut u8);
    fn rust_isa_avx512cd_vpconflictd(a: *const u8, out: *mut u8);
    fn rust_isa_avx512cd_vpconflictq(a: *const u8, out: *mut u8);
}

/// `vplzcntd`
///
/// Count the leading zero bits of each 32-bit lane.
///
/// # Safety
///
/// The caller must enable AVX-512 CD.
#[target_feature(enable = "avx512cd")]
#[inline]
pub unsafe fn vplzcntd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512cd_vplzcntd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vplzcntq`
///
/// Count the leading zero bits of each 64-bit lane.
///
/// # Safety
///
/// The caller must enable AVX-512 CD.
#[target_feature(enable = "avx512cd")]
#[inline]
pub unsafe fn vplzcntq(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512cd_vplzcntq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpconflictd`
///
/// Return, for each 32-bit lane, the bitmask of earlier equal lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 CD.
#[target_feature(enable = "avx512cd")]
#[inline]
pub unsafe fn vpconflictd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512cd_vpconflictd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpconflictq`
///
/// Return, for each 64-bit lane, the bitmask of earlier equal lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 CD.
#[target_feature(enable = "avx512cd")]
#[inline]
pub unsafe fn vpconflictq(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512cd_vpconflictq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
