//! PowerPC Power10 vector instructions.
//!
//! Power10 vector instructions are not part of the PowerPC baseline. Every
//! operation is `unsafe` and the caller must enable the Power10 feature set. The
//! C file is compiled with `-mcpu=power10`.
//!
//! The values are [`V128`].
//!
//! Instruction behavior follows the [Power ISA](https://openpowerfoundation.org/specifications/isa/).

use super::altivec::V128;

unsafe extern "C" {
    fn rust_isa_power10_vector_popcnt_i32(a: *const u8, out: *mut u8);
    fn rust_isa_power10_vector_cntlz_i32(a: *const u8, out: *mut u8);
}

/// `vpopcntw`.
///
/// # Safety
///
/// The caller must enable the Power10 vector feature set.
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
/// The caller must enable the Power10 vector feature set.
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
/// The caller must enable the Power10 vector feature set.
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
/// The caller must enable the Power9 vector feature set.
#[inline(always)]
pub unsafe fn vctzd(a: V128) -> V128 {
    unsafe { super::power9_vector::vctzd(a) }
}

/// `vpopcntd`.
///
/// # Safety
///
/// The caller must enable the Power10 vector feature set.
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
/// The caller must enable the Power10 vector feature set.
#[inline(always)]
pub unsafe fn vpopcntw(a: V128) -> V128 {
    unsafe { popcnt_i32(a) }
}
