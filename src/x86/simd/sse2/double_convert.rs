//! Packed integer and double-precision conversions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_cvtdq2pd(a: *const u8, out: *mut u8);
    fn rust_isa_sse2_cvtpd2dq(a: *const u8, out: *mut u8);
    fn rust_isa_sse2_cvttpd2dq(a: *const u8, out: *mut u8);
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
