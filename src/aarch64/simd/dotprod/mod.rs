//! AArch64 dot product.
//!
//! DOTPROD is not part of the AArch64 baseline. Every instruction wrapper is `unsafe`
//! and requires the `dotprod` target feature. The values are [`V128`].
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
//! use rust_isa::aarch64::simd::dotprod::sdot as _;
//! use rust_isa::aarch64::simd::dotprod::udot as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_dotprod_sdot(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_dotprod_udot(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `sdot`
///
/// Multiply signed bytes in `a` by signed bytes in `b`, add adjacent products,
/// and add the result to `acc`.
///
/// # Safety
///
/// The caller must enable DOTPROD.
#[target_feature(enable = "dotprod")]
#[inline]
pub unsafe fn sdot(a: V128, b: V128, acc: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_dotprod_sdot(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `udot`
///
/// Multiply unsigned bytes, add adjacent products, and add the result to
/// `acc`.
///
/// # Safety
///
/// The caller must enable DOTPROD.
#[target_feature(enable = "dotprod")]
#[inline]
pub unsafe fn udot(a: V128, b: V128, acc: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_dotprod_udot(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
