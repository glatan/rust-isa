//! s390x vector enhancements.
//!
//! The vector-enhancement facilities are not part of the s390x baseline. Every
//! operation is `unsafe` and the caller must compile with the corresponding
//! enhancement enabled. The C file is compiled with `-march=z14`.
//!
//! The values are [`V128`]. `cntlz` and `popcnt` operate on the lane type in
//! the function name. The byte order is big-endian.

use super::vector::V128;

unsafe extern "C" {
    fn rust_isa_s390x_enh_cntlz_i32(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_popcnt_i8(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_popcnt_i32(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_fma(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vclzf` on 32-bit lanes.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[inline(always)]
pub unsafe fn cntlz_i32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_enh_cntlz_i32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpopct` on byte lanes.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[inline(always)]
pub unsafe fn popcnt_i8(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_enh_popcnt_i8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpopctf` on 32-bit lanes.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[inline(always)]
pub unsafe fn popcnt_i32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_enh_popcnt_i32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfma` on binary floating-point lanes.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[inline(always)]
pub unsafe fn fma(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_enh_fma(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
