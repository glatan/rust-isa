//! PowerPC Power10 vector instructions.
//!
//! Power10 vector instructions are not part of the PowerPC baseline. Every
//! operation is `unsafe` and the caller must enable the `power10-vector` target feature. The
//! C file is compiled with clang and `-mcpu=power10`.
//!
//! The values are [`V128`].
//!
//! Instruction behavior follows the [Power ISA](https://openpowerfoundation.org/specifications/isa/).
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::powerpc::simd::altivec::V128;
//!
//! let value = V128::from_bytes([0; 16]);
//! ```
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::powerpc::simd::power10_vector::*;
//! use rust_isa::powerpc::simd::altivec::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { vpopcntw(v) }.to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::powerpc::simd::power10_vector::*;
//! use rust_isa::powerpc::simd::altivec::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { cntlz_i32(v128) };
//! let _ = unsafe { vclzd(v128) };
//! let _ = unsafe { vpopcntd(v128) };
//! let _ = unsafe { vpopcntw(v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::powerpc::simd::power10_vector::cntlz_i32 as _;
//! use rust_isa::powerpc::simd::power10_vector::vclzd as _;
//! use rust_isa::powerpc::simd::power10_vector::vpopcntd as _;
//! use rust_isa::powerpc::simd::power10_vector::vpopcntw as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use super::altivec::V128;

unsafe extern "C" {
    fn rust_isa_power10_vector_popcnt_i32(a: *const u8, out: *mut u8);
    fn rust_isa_power10_vector_cntlz_i32(a: *const u8, out: *mut u8);
}

/// `vclzw`.
///
/// # Safety
///
/// The caller must enable the `power10-vector` target feature.
#[inline(always)]
pub unsafe fn cntlz_i32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power10_vector_cntlz_i32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_power10_vector_vclzd(a: *const u8, out: *mut u8);
    fn rust_isa_power10_vector_vpopcntd(a: *const u8, out: *mut u8);
}

/// `vclzd`.
///
/// # Safety
///
/// The caller must enable the `power10-vector` target feature.
#[inline(always)]
pub unsafe fn vclzd(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power10_vector_vclzd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpopcntd`.
///
/// # Safety
///
/// The caller must enable the `power10-vector` target feature.
#[inline(always)]
pub unsafe fn vpopcntd(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power10_vector_vpopcntd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpopcntw`.
///
/// # Safety
///
/// The caller must enable the `power10-vector` target feature.
#[inline(always)]
pub unsafe fn vpopcntw(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power10_vector_popcnt_i32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
