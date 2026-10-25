//! AArch64 FCMA.
//!
//! FCMA is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable FCMA with `#[target_feature(enable = "fcma")]` or a
//! global target feature.
//!
//! The values are [`V128`] of 32-bit float lanes. The complex multiply-
//! accumulate treats adjacent lane pairs as complex numbers.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::aarch64::simd::fcma::fcmla;
//! use rust_isa::aarch64::simd::neon::V128;
//!
//! let zero = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { fcmla(zero, zero, zero) }.to_bytes(), [0; 16]);
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::aarch64::simd::fcma::*;
//! use rust_isa::aarch64::simd::neon::V128;
//! use rust_isa::aarch64::simd::sme::SmeGuard;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! unsafe {
//!     let _ = unsafe { fcmla(v128, v128, v128) };
//!     let _ = unsafe { fcmla_rot180(v128, v128, v128) };
//!     let _ = unsafe { fcmla_rot270(v128, v128, v128) };
//!     let _ = unsafe { fcmla_rot90(v128, v128, v128) };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::aarch64::simd::fcma::fcmla as _;
//! use rust_isa::aarch64::simd::fcma::fcmla_rot180 as _;
//! use rust_isa::aarch64::simd::fcma::fcmla_rot270 as _;
//! use rust_isa::aarch64::simd::fcma::fcmla_rot90 as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_fcma_vcmlaq_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fcma_vcmlaq_rot90_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fcma_vcmlaq_rot180_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fcma_vcmlaq_rot270_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `fcmla`
///
/// Multiply adjacent complex lane pairs and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable FCMA.
#[target_feature(enable = "fcma")]
#[inline]
pub unsafe fn fcmla(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fcma_vcmlaq_f32(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmla` with a 90-degree rotation of the second operand.
///
/// # Safety
///
/// The caller must enable FCMA.
#[target_feature(enable = "fcma")]
#[inline]
pub unsafe fn fcmla_rot90(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fcma_vcmlaq_rot90_f32(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmla` with a 180-degree rotation of the second operand.
///
/// # Safety
///
/// The caller must enable FCMA.
#[target_feature(enable = "fcma")]
#[inline]
pub unsafe fn fcmla_rot180(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fcma_vcmlaq_rot180_f32(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmla` with a 270-degree rotation of the second operand.
///
/// # Safety
///
/// The caller must enable FCMA.
#[target_feature(enable = "fcma")]
#[inline]
pub unsafe fn fcmla_rot270(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fcma_vcmlaq_rot270_f32(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
