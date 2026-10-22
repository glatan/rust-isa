//! AVX-NE-CONVERT.
//!
//! AVX-NE-CONVERT is not part of the x86-64 baseline. Every operation is
//! `unsafe` and the caller must enable it with
//! `#[target_feature(enable = "avxneconvert")]` or a global target feature.
//!
//! The module converts between 32-bit float lanes and bfloat16 or IEEE 754
//! binary16 lanes. The conversions use round-to-nearest-even. A NaN input is
//! converted to the canonical quiet NaN of the destination format.

use super::avx2::V256;
use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_avxneconvert_vcvtneps2bf16(a: *const u8, out: *mut u8);
    fn rust_isa_avxneconvert_vbcstnebf162ps(value: *const u16, out: *mut u8);
    fn rust_isa_avxneconvert_vbcstnesh2ps(value: *const u16, out: *mut u8);
    fn rust_isa_avxneconvert_vcvtneebf162ps(a: *const u8, out: *mut u8);
    fn rust_isa_avxneconvert_vcvtneeph2ps(a: *const u8, out: *mut u8);
    fn rust_isa_avxneconvert_vcvtneepbf162ps(a: *const u8, out: *mut u8);
    fn rust_isa_avxneconvert_vcvtneepsh2ps(a: *const u8, out: *mut u8);
}

/// `vcvtneps2bf16`
///
/// Convert eight 32-bit float lanes to eight bfloat16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-NE-CONVERT.
#[target_feature(enable = "avxneconvert")]
#[inline]
pub unsafe fn vcvtneps2bf16(a: V256) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxneconvert_vcvtneps2bf16(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbcstnebf162ps`
///
/// Broadcast a bfloat16 value and convert it to eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-NE-CONVERT.
#[target_feature(enable = "avxneconvert")]
#[inline]
pub unsafe fn vbcstnebf162ps(value: u16) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxneconvert_vbcstnebf162ps(&value, out.as_mut_ptr());
    }
    out
}

/// `vbcstnesh2ps`
///
/// Broadcast a binary16 value and convert it to eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-NE-CONVERT.
#[target_feature(enable = "avxneconvert")]
#[inline]
pub unsafe fn vbcstnesh2ps(value: u16) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxneconvert_vbcstnesh2ps(&value, out.as_mut_ptr());
    }
    out
}

/// `vcvtneebf162ps`
///
/// Convert the even-indexed bfloat16 lanes of `a` to 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-NE-CONVERT.
#[target_feature(enable = "avxneconvert")]
#[inline]
pub unsafe fn vcvtneebf162ps(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxneconvert_vcvtneebf162ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtneeph2ps`
///
/// Convert the even-indexed binary16 lanes of `a` to 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-NE-CONVERT.
#[target_feature(enable = "avxneconvert")]
#[inline]
pub unsafe fn vcvtneeph2ps(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxneconvert_vcvtneeph2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtneepbf162ps`
///
/// Convert the odd-indexed bfloat16 lanes of `a` to 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-NE-CONVERT.
#[target_feature(enable = "avxneconvert")]
#[inline]
pub unsafe fn vcvtneepbf162ps(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxneconvert_vcvtneepbf162ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtneepsh2ps`
///
/// Convert the odd-indexed binary16 lanes of `a` to 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-NE-CONVERT.
#[target_feature(enable = "avxneconvert")]
#[inline]
pub unsafe fn vcvtneepsh2ps(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxneconvert_vcvtneepsh2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
