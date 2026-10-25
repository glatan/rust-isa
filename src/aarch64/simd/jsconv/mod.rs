//! AArch64 JavaScript conversion.
//!
//! JSCONV is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and
//! requires the `jsconv` target feature. The value is a double-precision float
//! or [`V128`].
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::aarch64::simd::jsconv::fjcvtzs as _;
//! use rust_isa::aarch64::simd::jsconv::fjcvtzs_v128 as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_jsconv_fjcvtzs(value: f64) -> i32;
}

/// `fjcvtzs`
///
/// Convert a double-precision value to a 32-bit integer with JavaScript
/// `ToInt32` semantics: NaN and infinities become zero and the result wraps
/// modulo 2^32.
///
/// # Safety
///
/// The caller must enable JSCONV.
#[target_feature(enable = "jsconv")]
#[inline]
pub unsafe fn fjcvtzs(value: f64) -> i32 {
    unsafe { rust_isa_jsconv_fjcvtzs(value) }
}

/// `fjcvtzs` applied to the low lane of a vector.
///
/// # Safety
///
/// The caller must enable JSCONV.
#[target_feature(enable = "jsconv")]
#[inline]
pub unsafe fn fjcvtzs_v128(value: V128) -> i32 {
    let bytes = value.to_bytes();
    unsafe { rust_isa_jsconv_fjcvtzs(f64::from_le_bytes(bytes[..8].try_into().unwrap())) }
}
