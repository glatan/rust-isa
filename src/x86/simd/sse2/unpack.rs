//! Unpack instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_punpcklbw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_punpckhbw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_punpcklwd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_punpckhwd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_punpckldq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_punpckhdq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_punpcklqdq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_punpckhqdq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_unpcklps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_unpckhps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_unpcklpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_unpckhpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `punpcklbw`
///
/// Interleave the low 8 bytes of `a` and `b`.
#[inline(always)]
pub fn punpcklbw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_punpcklbw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `punpckhbw`
///
/// Interleave the high 8 bytes of `a` and `b`.
#[inline(always)]
pub fn punpckhbw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_punpckhbw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `punpcklwd`
///
/// Interleave the low four 16-bit lanes of `a` and `b`.
#[inline(always)]
pub fn punpcklwd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_punpcklwd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `punpckhwd`
///
/// Interleave the high four 16-bit lanes of `a` and `b`.
#[inline(always)]
pub fn punpckhwd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_punpckhwd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `punpckldq`
///
/// Interleave the low two 32-bit lanes of `a` and `b`.
#[inline(always)]
pub fn punpckldq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_punpckldq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `punpckhdq`
///
/// Interleave the high two 32-bit lanes of `a` and `b`.
#[inline(always)]
pub fn punpckhdq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_punpckhdq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `punpcklqdq`
///
/// Interleave the low 64-bit lane of `a` and `b`.
#[inline(always)]
pub fn punpcklqdq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_punpcklqdq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `punpckhqdq`
///
/// Interleave the high 64-bit lane of `a` and `b`.
#[inline(always)]
pub fn punpckhqdq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_punpckhqdq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `unpcklps`
///
/// Interleave the low two 32-bit float lanes of `a` and `b`.
#[inline(always)]
pub fn unpcklps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_unpcklps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `unpckhps`
///
/// Interleave the high two 32-bit float lanes of `a` and `b`.
#[inline(always)]
pub fn unpckhps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_unpckhps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `unpcklpd`
///
/// Interleave the low 64-bit float lane of `a` and `b`.
#[inline(always)]
pub fn unpcklpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_unpcklpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `unpckhpd`
///
/// Interleave the high 64-bit float lane of `a` and `b`.
#[inline(always)]
pub fn unpckhpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_unpckhpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
