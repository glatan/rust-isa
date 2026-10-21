//! Bitwise instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_pand(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_pandn(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_por(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_pxor(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pand`
///
/// Bitwise AND of the 16 bytes.
#[inline(always)]
pub fn pand(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pand(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pandn`
///
/// Bitwise AND of `b` with the complement of `a`.
#[inline(always)]
pub fn pandn(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pandn(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `por`
///
/// Bitwise OR of the 16 bytes.
#[inline(always)]
pub fn por(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_por(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pxor`
///
/// Bitwise XOR of the 16 bytes.
#[inline(always)]
pub fn pxor(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pxor(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
