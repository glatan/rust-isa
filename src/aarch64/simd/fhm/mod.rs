//! AArch64 FHM.
//!
//! FHM is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable FHM with `#[target_feature(enable = "fhm")]` or a global
//! target feature.
//!
//! The values are [`V128`] of binary16 lanes. The fused multiply-add rounds only
//! once, at the end.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::aarch64::simd::fhm::fmla_f16;
//! use rust_isa::aarch64::simd::neon::V128;
//!
//! let zero = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { fmla_f16(zero, zero, zero) }.to_bytes(), [0; 16]);
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::aarch64::simd::fhm::fmla_f16 as _;
//! use rust_isa::aarch64::simd::fhm::fmls_f16 as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_fhm_vfmaq_f16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fhm_vfmsq_f16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `fmla` on binary16 lanes.
///
/// # Safety
///
/// The caller must enable FHM.
#[target_feature(enable = "fhm")]
#[inline]
pub unsafe fn fmla_f16(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fhm_vfmaq_f16(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmls` on binary16 lanes.
///
/// # Safety
///
/// The caller must enable FHM.
#[target_feature(enable = "fhm")]
#[inline]
pub unsafe fn fmls_f16(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fhm_vfmsq_f16(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
