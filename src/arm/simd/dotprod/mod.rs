//! 32-bit ARM DOTPROD.
//!
//! DOTPROD is unstable on this target. Every instruction wrapper is `unsafe` and the
//! caller must compile the whole crate with the DOTPROD feature enabled.
//!
//! The values are [`V128`]. The dot-product instructions multiply bytes, add
//! adjacent products, and accumulate into 32-bit lanes.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::arm::simd::dotprod::sdot;
//! use rust_isa::arm::simd::neon::V128;
//!
//! let zero = V128::from_bytes([0; 16]);
//! let a = V128::from_bytes([1; 16]);
//! let b = V128::from_bytes([2; 16]);
//! let sum = unsafe { sdot(zero, a, b) };
//! assert_eq!(
//!     sum.to_bytes(),
//!     [8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0]
//! );
//! ```
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::arm::simd::dotprod::*;
//! use rust_isa::arm::simd::neon::V128;
//! use rust_isa::arm::simd::neon::V64;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { sdot(v128, v128, v128) };
//! let _ = unsafe { udot(v128, v128, v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::arm::simd::dotprod::sdot as _;
//! use rust_isa::arm::simd::dotprod::udot as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::neon::V128;

unsafe extern "C" {
    fn rust_isa_arm_dotprod_sdot(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_dotprod_udot(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vdot.s8`
///
/// Multiply signed bytes and accumulate into the 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable DOTPROD on the CPU and in the target features.
#[inline(always)]
pub unsafe fn sdot(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_dotprod_sdot(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdot.u8`
///
/// Multiply unsigned bytes and accumulate into the 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable DOTPROD on the CPU and in the target features.
#[inline(always)]
pub unsafe fn udot(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_dotprod_udot(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
