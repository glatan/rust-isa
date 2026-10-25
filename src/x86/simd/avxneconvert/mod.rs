//! AVX-NE-CONVERT.
//!
//! AVX-NE-CONVERT is not part of the x86-64 baseline. Every instruction wrapper is
//! `unsafe` and the caller must enable it with
//! `#[target_feature(enable = "avxneconvert")]` or a global target feature.
//!
//! The values are [`V128`] and [`V256`]. The module converts between 32-bit
//! float lanes and bfloat16 or IEEE 754 binary16 lanes. The conversions use
//! round-to-nearest-even. A NaN input is converted to the canonical quiet NaN
//! of the destination format.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avxneconvert::vcvtneps2bf16;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vcvtneps2bf16(v) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avxneconvert::vbcstnebf162ps as _;
//! use rust_isa::x86::simd::avxneconvert::vbcstnesh2ps as _;
//! use rust_isa::x86::simd::avxneconvert::vcvtneebf162ps as _;
//! use rust_isa::x86::simd::avxneconvert::vcvtneeph2ps as _;
//! use rust_isa::x86::simd::avxneconvert::vcvtneobf162ps as _;
//! use rust_isa::x86::simd::avxneconvert::vcvtneoph2ps as _;
//! use rust_isa::x86::simd::avxneconvert::vcvtneps2bf16 as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::avx2::V256;
use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_avxneconvert_vcvtneps2bf16(a: *const u8, out: *mut u8);
    fn rust_isa_avxneconvert_vbcstnebf162ps(value: *const u16, out: *mut u8);
    fn rust_isa_avxneconvert_vbcstnesh2ps(value: *const u16, out: *mut u8);
    fn rust_isa_avxneconvert_vcvtneebf162ps(a: *const u8, out: *mut u8);
    fn rust_isa_avxneconvert_vcvtneeph2ps(a: *const u8, out: *mut u8);
    fn rust_isa_avxneconvert_vcvtneobf162ps(a: *const u8, out: *mut u8);
    fn rust_isa_avxneconvert_vcvtneoph2ps(a: *const u8, out: *mut u8);
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

/// `vcvtneobf162ps`
///
/// Convert the odd-indexed bfloat16 lanes of `a` to 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-NE-CONVERT.
#[target_feature(enable = "avxneconvert")]
#[inline]
pub unsafe fn vcvtneobf162ps(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxneconvert_vcvtneobf162ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtneoph2ps`
///
/// Convert the odd-indexed binary16 lanes of `a` to 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-NE-CONVERT.
#[target_feature(enable = "avxneconvert")]
#[inline]
pub unsafe fn vcvtneoph2ps(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxneconvert_vcvtneoph2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
