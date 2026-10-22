//! Pack instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_packsswb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `packsswb`
///
/// Pack 16-bit lanes from `a` and `b` into signed bytes with saturation.
#[inline(always)]
pub fn packsswb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_packsswb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_packuswb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `packuswb`
///
/// Pack 16-bit lanes from `a` and `b` into unsigned bytes with saturation.
#[inline(always)]
pub fn packuswb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_packuswb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_packssdw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `packssdw`
///
/// Pack 32-bit lanes from `a` and `b` into signed words with saturation.
#[inline(always)]
pub fn packssdw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_packssdw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
