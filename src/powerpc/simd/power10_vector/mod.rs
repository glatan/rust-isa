//! PowerPC Power10 vector instructions.
//!
//! Power10 vector instructions are not part of the PowerPC baseline. Every
//! operation is `unsafe` and the caller must enable the Power10 feature set. The
//! C file is compiled with `-mcpu=power10`.
//!
//! The values are [`V128`].

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
