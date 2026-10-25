//! `wide-arithmetic`.
//!
//! Enabling the `wide-arithmetic` Cargo feature makes the module require the
//! WebAssembly `wide-arithmetic` proposal.

//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::wasm32::wide_arithmetic::*;
//!
//! let (lo, hi) = unsafe { i64_add128((u64::MAX, 0), (1, 0)) };
//! assert_eq!((lo, hi), (0, 1));
//! ```
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::wasm32::wide_arithmetic::*;
//! use rust_isa::wasm32::simd::simd128::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { i64_add128((0, 0), (0, 0)) };
//! let _ = unsafe { i64_mul_wide_s(0, 0) };
//! let _ = unsafe { i64_mul_wide_u(0, 0) };
//! let _ = unsafe { i64_sub128((0, 0), (0, 0)) };
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
//! use rust_isa::wasm32::wide_arithmetic::i64_add128 as _;
//! use rust_isa::wasm32::wide_arithmetic::i64_mul_wide_s as _;
//! use rust_isa::wasm32::wide_arithmetic::i64_mul_wide_u as _;
//! use rust_isa::wasm32::wide_arithmetic::i64_sub128 as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

unsafe extern "C" {
    fn rust_isa_wide_arithmetic_add128(alo: u64, ahi: u64, blo: u64, bhi: u64, out: *mut u64);
    fn rust_isa_wide_arithmetic_sub128(alo: u64, ahi: u64, blo: u64, bhi: u64, out: *mut u64);
    fn rust_isa_wide_arithmetic_mul_wide_u(a: u64, b: u64, out: *mut u64);
    fn rust_isa_wide_arithmetic_mul_wide_s(a: i64, b: i64, out: *mut i64);
}

/// `i64.add128`
///
/// Add two 128-bit values given as low and high halves.
///
/// # Safety
///
/// The module must be validated with the `wide-arithmetic` feature enabled.
pub unsafe fn i64_add128(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    let mut out = [0u64; 2];
    unsafe {
        rust_isa_wide_arithmetic_add128(a.0, a.1, b.0, b.1, out.as_mut_ptr());
    }
    (out[0], out[1])
}

/// `i64.sub128`
///
/// Subtract two 128-bit values given as low and high halves.
///
/// # Safety
///
/// The module must be validated with the `wide-arithmetic` feature enabled.
pub unsafe fn i64_sub128(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    let mut out = [0u64; 2];
    unsafe {
        rust_isa_wide_arithmetic_sub128(a.0, a.1, b.0, b.1, out.as_mut_ptr());
    }
    (out[0], out[1])
}

/// `i64.mul_wide_u`
///
/// Multiply two unsigned 64-bit values into a 128-bit result.
///
/// # Safety
///
/// The module must be validated with the `wide-arithmetic` feature enabled.
pub unsafe fn i64_mul_wide_u(a: u64, b: u64) -> (u64, u64) {
    let mut out = [0u64; 2];
    unsafe {
        rust_isa_wide_arithmetic_mul_wide_u(a, b, out.as_mut_ptr());
    }
    (out[0], out[1])
}

/// `i64.mul_wide_s`
///
/// Multiply two signed 64-bit values into a 128-bit result.
///
/// # Safety
///
/// The module must be validated with the `wide-arithmetic` feature enabled.
pub unsafe fn i64_mul_wide_s(a: i64, b: i64) -> (i64, i64) {
    let mut out = [0i64; 2];
    unsafe {
        rust_isa_wide_arithmetic_mul_wide_s(a, b, out.as_mut_ptr());
    }
    (out[0], out[1])
}
