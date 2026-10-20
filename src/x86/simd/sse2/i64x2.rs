//! 64-bit integer instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_paddq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_psubq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `paddq`
///
/// Add the two 64-bit integer lanes, wrapping at 64 bits.
#[inline(always)]
pub fn paddq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `psubq`
///
/// Subtract the two 64-bit integer lanes, wrapping at 64 bits.
#[inline(always)]
pub fn psubq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psubq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psadbw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `psadbw`
///
/// Sum the absolute differences of the byte lanes into two 64-bit lanes.
#[inline(always)]
pub fn psadbw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psadbw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
