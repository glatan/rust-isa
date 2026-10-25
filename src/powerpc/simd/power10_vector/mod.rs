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
//! use rust_isa::powerpc::simd::power10_vector::popcnt_i32;
//!
//! let value = V128::from_bytes([0; 16]);
//! let _ = unsafe { popcnt_i32(value) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::powerpc::simd::power10_vector::cntlz_i32 as _;
//! use rust_isa::powerpc::simd::power10_vector::popcnt_i32 as _;
//! use rust_isa::powerpc::simd::power10_vector::vclzd as _;
//! use rust_isa::powerpc::simd::power10_vector::vctzb as _;
//! use rust_isa::powerpc::simd::power10_vector::vctzd as _;
//! use rust_isa::powerpc::simd::power10_vector::vctzh as _;
//! use rust_isa::powerpc::simd::power10_vector::vctzlsbb as _;
//! use rust_isa::powerpc::simd::power10_vector::vctzw as _;
//! use rust_isa::powerpc::simd::power10_vector::vextsb2d as _;
//! use rust_isa::powerpc::simd::power10_vector::vextsb2w as _;
//! use rust_isa::powerpc::simd::power10_vector::vextsh2d as _;
//! use rust_isa::powerpc::simd::power10_vector::vextsh2w as _;
//! use rust_isa::powerpc::simd::power10_vector::vextsw2d as _;
//! use rust_isa::powerpc::simd::power10_vector::vnegd as _;
//! use rust_isa::powerpc::simd::power10_vector::vnegw as _;
//! use rust_isa::powerpc::simd::power10_vector::vpopcntd as _;
//! use rust_isa::powerpc::simd::power10_vector::vpopcntw as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::altivec::V128;

unsafe extern "C" {
    fn rust_isa_power10_vector_popcnt_i32(a: *const u8, out: *mut u8);
    fn rust_isa_power10_vector_cntlz_i32(a: *const u8, out: *mut u8);
}

/// `vpopcntw`.
///
/// # Safety
///
/// The caller must enable the `power10-vector` target feature.
#[inline(always)]
pub unsafe fn popcnt_i32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power10_vector_popcnt_i32(a.as_ptr(), out.as_mut_ptr());
    }
    out
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

pub use super::power9_vector::{
    vctzb, vctzh, vctzlsbb, vctzw, vextsb2d, vextsb2w, vextsh2d, vextsh2w, vextsw2d, vnegd, vnegw,
};

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

/// `vctzd`.
///
/// # Safety
///
/// The caller must enable the `power9-vector` target feature (included in `power10-vector`).
#[inline(always)]
pub unsafe fn vctzd(a: V128) -> V128 {
    unsafe { super::power9_vector::vctzd(a) }
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
    unsafe { popcnt_i32(a) }
}
