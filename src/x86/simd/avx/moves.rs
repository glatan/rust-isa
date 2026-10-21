//! Move and duplicate instructions.

use super::V256;
use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_avx_vmovd(value: u32, out: *mut u8);
    fn rust_isa_avx_vmovd_to_u32(a: *const u8) -> u32;
    fn rust_isa_avx_vmovmskpd(a: *const u8) -> u32;
    fn rust_isa_avx_vmovshdup(a: *const u8, out: *mut u8);
    fn rust_isa_avx_vmovsldup(a: *const u8, out: *mut u8);
    fn rust_isa_avx_vmovddup(a: *const u8, out: *mut u8);
}

/// `vmovd` from a general-purpose register
///
/// Move a 32-bit integer into the low lane and zero the upper lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovd(value: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx_vmovd(value, out.as_mut_ptr());
    }
    out
}

/// `vmovd` to a general-purpose register
///
/// Extract the low 32-bit lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovd_to_u32(a: V128) -> u32 {
    unsafe { rust_isa_avx_vmovd_to_u32(a.as_ptr()) }
}

/// `vmovmskpd`
///
/// Return the sign bits of the four 64-bit float lanes as a mask.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovmskpd(a: V256) -> u32 {
    unsafe { rust_isa_avx_vmovmskpd(a.as_ptr()) }
}

/// `vmovshdup`
///
/// Duplicate the odd 32-bit float lanes within each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovshdup(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vmovshdup(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmovsldup`
///
/// Duplicate the even 32-bit float lanes within each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovsldup(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vmovsldup(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmovddup`
///
/// Duplicate the low 64-bit float lane within each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovddup(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vmovddup(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
