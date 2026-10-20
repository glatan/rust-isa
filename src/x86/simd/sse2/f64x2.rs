//! Floating-point instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_addpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `addpd`
///
/// Add the two 64-bit float lanes.
#[inline(always)]
pub fn addpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_addpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_subpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `subpd`
///
/// Subtract the two 64-bit float lanes.
#[inline(always)]
pub fn subpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_subpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_mulpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `mulpd`
///
/// Multiply the two 64-bit float lanes.
#[inline(always)]
pub fn mulpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_mulpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_divpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `divpd`
///
/// Divide the two 64-bit float lanes.
#[inline(always)]
pub fn divpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_divpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_minpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `minpd`
///
/// Take the minimum of the two 64-bit float lanes.
#[inline(always)]
pub fn minpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_minpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_maxpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `maxpd`
///
/// Take the maximum of the two 64-bit float lanes.
#[inline(always)]
pub fn maxpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_maxpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cmpeqpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `cmpeqpd`
///
/// Compare the two 64-bit float lanes for equality.
#[inline(always)]
pub fn cmpeqpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cmpeqpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cmpltpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `cmpltpd`
///
/// Compare the two 64-bit float lanes with less-than.
#[inline(always)]
pub fn cmpltpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cmpltpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cmplepd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `cmplepd`
///
/// Compare the two 64-bit float lanes with less-than-or-equal.
#[inline(always)]
pub fn cmplepd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cmplepd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_sqrtpd(a: *const u8, out: *mut u8);
}

/// `sqrtpd`
///
/// Take the square root of the two 64-bit float lanes.
#[inline(always)]
pub fn sqrtpd(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_sqrtpd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
