//! PowerPC Power9 vector instructions.
//!
//! Power9 vector instructions are not part of the PowerPC baseline. Every
//! operation is `unsafe` and the caller must enable the Power9 feature set. The
//! C file is compiled with `-mcpu=power9`.
//!
//! The values are [`V128`].

use super::altivec::V128;

unsafe extern "C" {
    fn rust_isa_power9_vector_neg_i32(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_neg_i64(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_abs_i32(a: *const u8, out: *mut u8);
}

/// `vnegw`.
///
/// # Safety
///
/// The caller must enable the Power9 vector feature set.
#[inline(always)]
pub unsafe fn neg_i32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power9_vector_neg_i32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vnegd`.
///
/// # Safety
///
/// The caller must enable the Power9 vector feature set.
#[inline(always)]
pub unsafe fn neg_i64(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power9_vector_neg_i64(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vabsduw` / `vabsd` representative.
///
/// # Safety
///
/// The caller must enable the Power9 vector feature set.
#[inline(always)]
pub unsafe fn abs_i32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power9_vector_abs_i32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
