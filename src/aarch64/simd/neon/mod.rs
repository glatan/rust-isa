//! NEON.
//!
//! NEON is part of the AArch64 baseline. The arithmetic operations are safe.
//! Loads and stores take raw pointers and are `unsafe`.

mod compare;
mod compare_ext;
mod float;
mod integer;
mod lane;
mod shift;
mod shuffle;
mod v128;

pub use compare::*;
pub use compare_ext::*;
pub use float::*;
pub use integer::*;
pub use lane::*;
pub use shift::*;
pub use shuffle::*;
pub use v128::*;

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
