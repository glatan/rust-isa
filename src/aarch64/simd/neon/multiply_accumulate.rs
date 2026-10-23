//! NEON multiply-accumulate instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_mla_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_mla_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_mla_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_mls_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_mls_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_mls_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `mla_s8`
///
/// Multiply signed lanes and add the result to the accumulator.
#[inline(always)]
pub fn mla_s8(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mla_s8(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mla_s16`
///
/// Multiply signed lanes and add the result to the accumulator.
#[inline(always)]
pub fn mla_s16(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mla_s16(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mla_s32`
///
/// Multiply signed lanes and add the result to the accumulator.
#[inline(always)]
pub fn mla_s32(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mla_s32(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mls_s8`
///
/// Multiply signed lanes and subtract the result from the accumulator.
#[inline(always)]
pub fn mls_s8(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mls_s8(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mls_s16`
///
/// Multiply signed lanes and subtract the result from the accumulator.
#[inline(always)]
pub fn mls_s16(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mls_s16(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mls_s32`
///
/// Multiply signed lanes and subtract the result from the accumulator.
#[inline(always)]
pub fn mls_s32(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mls_s32(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}
