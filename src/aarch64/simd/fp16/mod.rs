//! AArch64 FP16.
//!
//! FP16 is not part of the AArch64 baseline. Every operation is `unsafe` and
//! the caller must enable FP16 with `#[target_feature(enable = "fp16")]` or a
//! global target feature.
//!
//! The values are [`V128`] of IEEE 754 binary16 lanes. The result of arithmetic
//! is rounded according to the FPCR rounding mode.

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_fp16_vaddq_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_vmulq_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_vsubq_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_vdivq_f16(a: *const u8, b: *const u8, out: *mut u8);
}

/// `fadd` on binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fadd_f16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_vaddq_f16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmul` on binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fmul_f16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_vmulq_f16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fsub` on binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fsub_f16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_vsubq_f16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fdiv` on binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fdiv_f16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_vdivq_f16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
