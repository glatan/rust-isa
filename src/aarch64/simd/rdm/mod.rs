//! AArch64 RDM.
//!
//! RDM is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable RDM with `#[target_feature(enable = "rdm")]` or a global
//! target feature.
//!
//! The values are [`V128`]. The instructions perform a rounding doubling
//! multiply and accumulate or subtract on 32-bit lanes.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::aarch64::simd::neon::V128;
//! use rust_isa::aarch64::simd::rdm::sqrdmlah;
//!
//! let zero = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { sqrdmlah(zero, zero, zero) }.to_bytes(), [0; 16]);
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::aarch64::simd::rdm::sqrdmlah as _;
//! use rust_isa::aarch64::simd::rdm::sqrdmlsh as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_rdm_vqrdmlahq_s32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rdm_vqrdmlshq_s32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `sqrdmlah`
///
/// Multiply 32-bit lanes, double the product, round, and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable RDM.
#[target_feature(enable = "rdm")]
#[inline]
pub unsafe fn sqrdmlah(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rdm_vqrdmlahq_s32(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `sqrdmlsh`
///
/// Multiply 32-bit lanes, double the product, round, and subtract from `acc`.
///
/// # Safety
///
/// The caller must enable RDM.
#[target_feature(enable = "rdm")]
#[inline]
pub unsafe fn sqrdmlsh(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rdm_vqrdmlshq_s32(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
