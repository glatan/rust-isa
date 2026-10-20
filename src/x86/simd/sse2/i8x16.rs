//! 8-bit integer instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_paddb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_psubb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_pcmpeqb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `paddb`
///
/// Add the 16 byte lanes, wrapping at 8 bits.
#[inline(always)]
pub fn paddb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `psubb`
///
/// Subtract the 16 byte lanes, wrapping at 8 bits.
#[inline(always)]
pub fn psubb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psubb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pcmpeqb`
///
/// Compare the 16 byte lanes. A true lane is all bits one.
#[inline(always)]
pub fn pcmpeqb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pcmpeqb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
