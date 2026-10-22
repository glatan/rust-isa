//! NEON.
//!
//! NEON is part of the AArch64 baseline. The arithmetic operations are safe.
//! Loads and stores take raw pointers and are `unsafe`. The values are
//! [`V128`].
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//! The build compiles the bundled C with clang. Const-generic lane and shift
//! immediates document their valid range on each function.
//!
//! Instructions not listed in this module are not exposed.

mod absdiff;
mod compare;
mod convert;
mod crypto;
mod extract;
mod float_arith;
mod float_compare;
mod halving;
mod integer;
mod lane;
mod minmax;
mod narrow;
mod reciprocal;
mod reduce;
mod saturating;
mod scalar;
mod shift;
mod shift_immediate;
mod shuffle;
mod structure;
mod v128;
mod widen;

pub use absdiff::*;
pub use compare::*;
pub use convert::*;
pub use crypto::*;
pub use extract::*;
pub use float_arith::*;
pub use float_compare::*;
pub use halving::*;
pub use integer::*;
pub use lane::*;
pub use minmax::*;
pub use narrow::*;
pub use reciprocal::*;
pub use reduce::*;
pub use saturating::*;
pub use scalar::*;
pub use shift::*;
pub use shift_immediate::*;
pub use shuffle::*;
pub use structure::*;
pub use v128::*;
pub use widen::*;

unsafe extern "C" {
    fn rust_isa_neon_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_mul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_tbl(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `add`
///
/// Add the four 32-bit integer lanes.
#[inline(always)]
pub fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mul`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
#[inline(always)]
pub fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fadd`
///
/// Add the four 32-bit float lanes.
#[inline(always)]
pub fn fadd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fadd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmul`
///
/// Multiply the four 32-bit float lanes.
#[inline(always)]
pub fn fmul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `tbl`
///
/// Look up bytes in `a` with the indices in `idx`.
#[inline(always)]
pub fn tbl(a: V128, idx: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_tbl(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}
