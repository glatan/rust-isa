//! AVX-512 VPOPCNTDQ.
//!
//! AVX-512 VPOPCNTDQ is not part of the x86-64 baseline. Every operation is
//! `unsafe` and the caller must enable it with
//! `#[target_feature(enable = "avx512vpopcntdq")]` (which implies AVX-512 F) or
//! a global target feature.
//!
//! The values are [`V512`]. The result lane contains the number of set bits in
//! the corresponding source lane.

use super::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512vpopcntdq_vpopcntd(a: *const u8, out: *mut u8);
    fn rust_isa_avx512vpopcntdq_vpopcntq(a: *const u8, out: *mut u8);
}

/// `vpopcntd`
///
/// Count the set bits in each 32-bit lane.
///
/// # Safety
///
/// The caller must enable AVX-512 VPOPCNTDQ.
#[target_feature(enable = "avx512vpopcntdq")]
#[inline]
pub unsafe fn vpopcntd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vpopcntdq_vpopcntd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpopcntq`
///
/// Count the set bits in each 64-bit lane.
///
/// # Safety
///
/// The caller must enable AVX-512 VPOPCNTDQ.
#[target_feature(enable = "avx512vpopcntdq")]
#[inline]
pub unsafe fn vpopcntq(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vpopcntdq_vpopcntq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
