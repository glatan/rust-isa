//! Additional NEON compare instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_cmge_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmge_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmge_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmle_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmle_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmle_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmlt_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmlt_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmlt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmtst_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmtst_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmtst_i32(a: *const u8, b: *const u8, out: *mut u8);
}

/// `cmge_i8`
///
/// Compare 8-bit signed lanes for greater than or equal.
#[inline(always)]
pub fn cmge_i8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmge_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `cmge_i16`
///
/// Compare 16-bit signed lanes for greater than or equal.
#[inline(always)]
pub fn cmge_i16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmge_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `cmge_i32`
///
/// Compare 32-bit signed lanes for greater than or equal.
#[inline(always)]
pub fn cmge_i32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmge_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `cmle_i8`
///
/// Compare 8-bit signed lanes for less than or equal.
#[inline(always)]
pub fn cmle_i8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmle_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `cmle_i16`
///
/// Compare 16-bit signed lanes for less than or equal.
#[inline(always)]
pub fn cmle_i16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmle_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `cmle_i32`
///
/// Compare 32-bit signed lanes for less than or equal.
#[inline(always)]
pub fn cmle_i32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmle_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `cmlt_i8`
///
/// Compare 8-bit signed lanes for less than.
#[inline(always)]
pub fn cmlt_i8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmlt_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `cmlt_i16`
///
/// Compare 16-bit signed lanes for less than.
#[inline(always)]
pub fn cmlt_i16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmlt_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `cmlt_i32`
///
/// Compare 32-bit signed lanes for less than.
#[inline(always)]
pub fn cmlt_i32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmlt_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `cmtst_i8`
///
/// Test whether the bitwise AND of 8-bit lanes is nonzero.
#[inline(always)]
pub fn cmtst_i8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmtst_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `cmtst_i16`
///
/// Test whether the bitwise AND of 16-bit lanes is nonzero.
#[inline(always)]
pub fn cmtst_i16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmtst_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `cmtst_i32`
///
/// Test whether the bitwise AND of 32-bit lanes is nonzero.
#[inline(always)]
pub fn cmtst_i32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_cmtst_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
