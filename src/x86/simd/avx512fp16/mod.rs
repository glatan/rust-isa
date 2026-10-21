//! AVX-512 FP16.
//!
//! AVX-512 FP16 is not part of the x86-64 baseline. Every operation is
//! `unsafe` and the caller must enable it with
//! `#[target_feature(enable = "avx512fp16")]` (which implies AVX-512 F) or a
//! global target feature.
//!
//! The values are [`V512`] of IEEE 754 binary16 lanes, or [`V512`] of 32-bit
//! float lanes for the conversion results. `vcvtps2phx` rounds to nearest even
//! without raising an inexact exception.

use super::avx2::V256;
use super::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512fp16_vaddph(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx512fp16_vsubph(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx512fp16_vmulph(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx512fp16_vdivph(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx512fp16_vcvtph2psx(a: *const u8, out: *mut u8);
    fn rust_isa_avx512fp16_vcvtps2phx(a: *const u8, out: *mut u8);
}

/// `vaddph`
///
/// Add the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vaddph(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vaddph(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubph`
///
/// Subtract the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vsubph(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vsubph(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulph`
///
/// Multiply the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vmulph(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vmulph(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdivph`
///
/// Divide the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vdivph(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vdivph(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtph2psx`
///
/// Convert sixteen binary16 lanes to sixteen 32-bit float lanes with the
/// AVX-512 FP16 `VCVTPH2PSX` semantics. Unlike F16C `VCVTPH2PS`, denormal and
/// NaN inputs are handled by the AVX-512 FP16 conversion rules.
///
/// # Safety
///
/// The caller must enable AVX-512 FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcvtph2psx(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vcvtph2psx(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtps2phx`
///
/// Convert sixteen 32-bit float lanes to sixteen binary16 lanes with the
/// AVX-512 FP16 `VCVTPS2PHX` semantics. This wrapper rounds to nearest even
/// without raising the inexact exception.
///
/// # Safety
///
/// The caller must enable AVX-512 FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcvtps2phx(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512fp16_vcvtps2phx(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
