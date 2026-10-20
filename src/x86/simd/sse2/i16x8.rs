//! 16-bit integer instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_paddw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_psubw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_pcmpeqw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_pmullw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `paddw`
///
/// Add the eight 16-bit integer lanes, wrapping at 16 bits.
#[inline(always)]
pub fn paddw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `psubw`
///
/// Subtract the eight 16-bit integer lanes, wrapping at 16 bits.
#[inline(always)]
pub fn psubw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psubw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pcmpeqw`
///
/// Compare the eight 16-bit integer lanes. A true lane is all bits one.
#[inline(always)]
pub fn pcmpeqw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pcmpeqw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmullw`
///
/// Multiply the eight 16-bit integer lanes, keeping the low 16 bits.
#[inline(always)]
pub fn pmullw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pmullw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
