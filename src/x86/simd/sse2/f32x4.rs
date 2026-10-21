//! Floating-point instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_addps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `addps`
///
/// Add the four 32-bit float lanes.
#[inline(always)]
pub fn addps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_addps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_subps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `subps`
///
/// Subtract the four 32-bit float lanes.
#[inline(always)]
pub fn subps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_subps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_mulps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `mulps`
///
/// Multiply the four 32-bit float lanes.
#[inline(always)]
pub fn mulps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_mulps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_divps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `divps`
///
/// Divide the four 32-bit float lanes.
#[inline(always)]
pub fn divps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_divps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_minps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `minps`
///
/// Take the minimum of the four 32-bit float lanes.
#[inline(always)]
pub fn minps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_minps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_maxps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `maxps`
///
/// Take the maximum of the four 32-bit float lanes.
#[inline(always)]
pub fn maxps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_maxps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cmpeqps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `cmpeqps`
///
/// Compare the four 32-bit float lanes for equality.
#[inline(always)]
pub fn cmpeqps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cmpeqps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cmpltps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `cmpltps`
///
/// Compare the four 32-bit float lanes with less-than.
#[inline(always)]
pub fn cmpltps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cmpltps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cmpleps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `cmpleps`
///
/// Compare the four 32-bit float lanes with less-than-or-equal.
#[inline(always)]
pub fn cmpleps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cmpleps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_sqrtps(a: *const u8, out: *mut u8);
}

/// `sqrtps`
///
/// Take the square root of the four 32-bit float lanes.
#[inline(always)]
pub fn sqrtps(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_sqrtps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_movmskps(a: *const u8) -> i32;
}

/// `movmskps`
///
/// Collect the sign bit of each 32-bit float lane into the low four bits.
#[inline(always)]
pub fn movmskps(a: V128) -> u32 {
    unsafe { rust_isa_sse2_movmskps(a.as_ptr()) as u32 }
}
