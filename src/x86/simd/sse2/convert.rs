//! Conversion instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_cvtsi2ss(value: i32, out: *mut u8);
}

/// `cvtsi2ss`
///
/// Convert a signed 32-bit integer to the low 32-bit float lane.
#[inline(always)]
pub fn cvtsi2ss(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cvtsi2ss(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cvtss2si(a: *const u8) -> i32;
}

/// `cvtss2si`
///
/// Convert the low 32-bit float lane to a signed 32-bit integer, rounding
/// according to the current rounding mode.
#[inline(always)]
pub fn cvtss2si(a: V128) -> i32 {
    unsafe { rust_isa_sse2_cvtss2si(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_sse2_cvttss2si(a: *const u8) -> i32;
}

/// `cvttss2si`
///
/// Convert the low 32-bit float lane to a signed 32-bit integer, truncating
/// toward zero.
#[inline(always)]
pub fn cvttss2si(a: V128) -> i32 {
    unsafe { rust_isa_sse2_cvttss2si(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_sse2_cvtps2pd(a: *const u8, out: *mut u8);
}

/// `cvtps2pd`
///
/// Convert the low two 32-bit float lanes to 64-bit float lanes.
#[inline(always)]
pub fn cvtps2pd(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cvtps2pd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cvtpd2ps(a: *const u8, out: *mut u8);
}

/// `cvtpd2ps`
///
/// Convert the two 64-bit float lanes to 32-bit float lanes. The upper
/// 64 bits are zero.
#[inline(always)]
pub fn cvtpd2ps(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cvtpd2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cvtdq2ps(a: *const u8, out: *mut u8);
}

/// `cvtdq2ps`
///
/// Convert four signed 32-bit integer lanes to 32-bit float lanes.
#[inline(always)]
pub fn cvtdq2ps(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cvtdq2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cvtps2dq(a: *const u8, out: *mut u8);
}

/// `cvtps2dq`
///
/// Convert four 32-bit float lanes to signed 32-bit integers, rounding
/// according to the current rounding mode.
#[inline(always)]
pub fn cvtps2dq(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cvtps2dq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cvttps2dq(a: *const u8, out: *mut u8);
}

/// `cvttps2dq`
///
/// Convert four 32-bit float lanes to signed 32-bit integers, truncating
/// toward zero.
#[inline(always)]
pub fn cvttps2dq(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cvttps2dq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cvtsi2sd(value: i32, out: *mut u8);
}

/// `cvtsi2sd`
///
/// Convert a signed 32-bit integer to the low 64-bit float lane.
#[inline(always)]
pub fn cvtsi2sd(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cvtsi2sd(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cvtsd2si(a: *const u8) -> i32;
}

/// `cvtsd2si`
///
/// Convert the low 64-bit float lane to a signed 32-bit integer, rounding
/// according to the current rounding mode.
#[inline(always)]
pub fn cvtsd2si(a: V128) -> i32 {
    unsafe { rust_isa_sse2_cvtsd2si(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_sse2_cvttsd2si(a: *const u8) -> i32;
}

/// `cvttsd2si`
///
/// Convert the low 64-bit float lane to a signed 32-bit integer, truncating
/// toward zero.
#[inline(always)]
pub fn cvttsd2si(a: V128) -> i32 {
    unsafe { rust_isa_sse2_cvttsd2si(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_sse2_cvtdq2pd(a: *const u8, out: *mut u8);
}

/// `cvtdq2pd`
///
/// Convert two signed 32-bit integer lanes to two 64-bit float lanes.
#[inline(always)]
pub fn cvtdq2pd(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cvtdq2pd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cvtpd2dq(a: *const u8, out: *mut u8);
}

/// `cvtpd2dq`
///
/// Convert two 64-bit float lanes to signed 32-bit integer lanes.
#[inline(always)]
pub fn cvtpd2dq(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cvtpd2dq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_cvttpd2dq(a: *const u8, out: *mut u8);
}

/// `cvttpd2dq`
///
/// Convert two 64-bit float lanes to signed 32-bit integer lanes with
/// truncation.
#[inline(always)]
pub fn cvttpd2dq(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_cvttpd2dq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
