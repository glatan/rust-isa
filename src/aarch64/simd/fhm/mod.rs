//! AArch64 FHM.
//!
//! FHM is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable FHM with `#[target_feature(enable = "fhm")]` or a global
//! target feature.
//!
//! The values are [`V128`]. The multiply-add-long instructions read two
//! binary16 lanes and accumulate into two 32-bit float lanes; the upper 64 bits
//! of the result are zero.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::aarch64::simd::fhm::fmlal_f16;
//! use rust_isa::aarch64::simd::neon::V128;
//!
//! let zero = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { fmlal_f16(zero, zero, zero) }.to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::aarch64::simd::fhm::*;
//! use rust_isa::aarch64::simd::neon::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { fmlal_f16(v128, v128, v128) };
//! let _ = unsafe { fmlal2_f16(v128, v128, v128) };
//! let _ = unsafe { fmlsl_f16(v128, v128, v128) };
//! let _ = unsafe { fmlsl2_f16(v128, v128, v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::aarch64::simd::fhm::fmlal2_f16 as _;
//! use rust_isa::aarch64::simd::fhm::fmlal_f16 as _;
//! use rust_isa::aarch64::simd::fhm::fmlsl2_f16 as _;
//! use rust_isa::aarch64::simd::fhm::fmlsl_f16 as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_fhm_fmlal_f16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fhm_fmlal2_f16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fhm_fmlsl_f16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fhm_fmlsl2_f16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `fmlal` multiplies the low two binary16 lanes and accumulates into two f32 lanes.
///
/// # Safety
///
/// The caller must enable FHM.
#[target_feature(enable = "fhm")]
#[inline]
pub unsafe fn fmlal_f16(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fhm_fmlal_f16(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmlal2` multiplies the high two binary16 lanes and accumulates into two f32 lanes.
///
/// # Safety
///
/// The caller must enable FHM.
#[target_feature(enable = "fhm")]
#[inline]
pub unsafe fn fmlal2_f16(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fhm_fmlal2_f16(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmlsl` multiplies the low two binary16 lanes and subtracts from two f32 lanes.
///
/// # Safety
///
/// The caller must enable FHM.
#[target_feature(enable = "fhm")]
#[inline]
pub unsafe fn fmlsl_f16(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fhm_fmlsl_f16(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmlsl2` multiplies the high two binary16 lanes and subtracts from two f32 lanes.
///
/// # Safety
///
/// The caller must enable FHM.
#[target_feature(enable = "fhm")]
#[inline]
pub unsafe fn fmlsl2_f16(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fhm_fmlsl2_f16(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
