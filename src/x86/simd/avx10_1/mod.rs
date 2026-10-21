//! AVX10.1.
//!
//! AVX10.1 is not part of the x86-64 baseline and rustc does not allow
//! `#[target_feature(enable = "avx10.1")]` on stable. The caller must compile
//! the whole crate with `-C target-feature=+avx10.1` and the CPU must support
//! AVX10.1. The wrappers do not carry `#[target_feature]`.
//!
//! The values are [`V256`] and [`V512`]. This module contains representative
//! 256-bit and 512-bit floating-point operations. The AVX10.1 vector-length
//! selection and the embedded rounding control are intentionally omitted.

use super::avx2::V256;
use super::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx10_1_vaddps_v256(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx10_1_vmulps_v256(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx10_1_vaddps_v512(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx10_1_vmulps_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaddps` with 256-bit values.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline(always)]
pub unsafe fn vaddps_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vaddps_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulps` with 256-bit values.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline(always)]
pub unsafe fn vmulps_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vmulps_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddps` with 512-bit values.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline(always)]
pub unsafe fn vaddps_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vaddps_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulps` with 512-bit values.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline(always)]
pub unsafe fn vmulps_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vmulps_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
