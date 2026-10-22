//! AVX-512 VP2INTERSECT.
//!
//! AVX-512 VP2INTERSECT is not part of the x86-64 baseline. Every instruction wrapper is
//! `unsafe` and the caller must enable it with
//! `#[target_feature(enable = "avx512vp2intersect")]` (which implies AVX-512 F)
//! or a global target feature.
//!
//! The values are [`V512`]. Each function returns two masks: the lanes of `a`
//! that occur in `b`, and the lanes of `b` that occur in `a`.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

use super::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512vp2intersect_vp2intersectd(
        a: *const u8,
        b: *const u8,
        a_mask: *mut u32,
        b_mask: *mut u32,
    );
    fn rust_isa_avx512vp2intersect_vp2intersectq(
        a: *const u8,
        b: *const u8,
        a_mask: *mut u32,
        b_mask: *mut u32,
    );
}

/// `vp2intersectd`
///
/// Return the 16-bit intersection masks for 32-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 VP2INTERSECT.
#[target_feature(enable = "avx512vp2intersect")]
#[inline]
pub unsafe fn vp2intersectd(a: V512, b: V512) -> (u16, u16) {
    let mut a_mask = 0u32;
    let mut b_mask = 0u32;
    unsafe {
        rust_isa_avx512vp2intersect_vp2intersectd(a.as_ptr(), b.as_ptr(), &mut a_mask, &mut b_mask);
    }
    (a_mask as u16, b_mask as u16)
}

/// `vp2intersectq`
///
/// Return the 8-bit intersection masks for 64-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 VP2INTERSECT.
#[target_feature(enable = "avx512vp2intersect")]
#[inline]
pub unsafe fn vp2intersectq(a: V512, b: V512) -> (u8, u8) {
    let mut a_mask = 0u32;
    let mut b_mask = 0u32;
    unsafe {
        rust_isa_avx512vp2intersect_vp2intersectq(a.as_ptr(), b.as_ptr(), &mut a_mask, &mut b_mask);
    }
    (a_mask as u8, b_mask as u8)
}
