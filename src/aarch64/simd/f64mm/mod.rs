//! AArch64 SVE floating-point matrix multiply-accumulate (F64MM).
//!
//! F64MM is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must compile with F64MM and SVE enabled. The slice length must be
//! a multiple of the SVE vector length in 64-bit elements.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::aarch64::simd::f64mm::fmmla_f64;
//!
//! let a = [1.0f64; 4];
//! let b = [2.0f64; 4];
//! let mut acc = [0.0f64; 4];
//! unsafe { fmmla_f64(&a, &b, &mut acc) };
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::aarch64::simd::f64mm::*;
//! use rust_isa::aarch64::simd::neon::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { fmmla_f64(&[0.0f64; 4], &[0.0f64; 4], &mut[0.0f64; 4]) };
//! ```
//!
//! </details>
//!
//! <details>
//! <summary>Instruction inventory</summary>
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::aarch64::simd::f64mm::fmmla_f64 as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

unsafe extern "C" {
    fn rust_isa_f64mm_fmmla(a: *const f64, b: *const f64, acc: *mut f64, len: usize);
}

/// `fmmla` on 64-bit float slices.
///
/// Accumulate the 2x2 matrix product of `a` and `b` into `acc`.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and F64MM and the CPU must support them.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmmla_f64(a: &[f64], b: &[f64], acc: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_f64mm_fmmla(a.as_ptr(), b.as_ptr(), acc.as_mut_ptr(), a.len());
    }
}
