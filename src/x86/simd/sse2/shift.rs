//! Shift instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_psllw(a: *const u8, count: u32, out: *mut u8);
}

/// `psllw`
///
/// Shift the eight 16-bit integer lanes left by `count`.
#[inline(always)]
pub fn psllw(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psllw(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pslld(a: *const u8, count: u32, out: *mut u8);
}

/// `pslld`
///
/// Shift the four 32-bit integer lanes left by `count`.
#[inline(always)]
pub fn pslld(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pslld(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psllq(a: *const u8, count: u32, out: *mut u8);
}

/// `psllq`
///
/// Shift the two 64-bit integer lanes left by `count`.
#[inline(always)]
pub fn psllq(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psllq(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psrlw(a: *const u8, count: u32, out: *mut u8);
}

/// `psrlw`
///
/// Shift the eight 16-bit integer lanes right by `count`.
#[inline(always)]
pub fn psrlw(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psrlw(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psrld(a: *const u8, count: u32, out: *mut u8);
}

/// `psrld`
///
/// Shift the four 32-bit integer lanes right by `count`.
#[inline(always)]
pub fn psrld(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psrld(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psrlq(a: *const u8, count: u32, out: *mut u8);
}

/// `psrlq`
///
/// Shift the two 64-bit integer lanes right by `count`.
#[inline(always)]
pub fn psrlq(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psrlq(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psraw(a: *const u8, count: u32, out: *mut u8);
}

/// `psraw`
///
/// Shift the eight signed 16-bit integer lanes right by `count`.
#[inline(always)]
pub fn psraw(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psraw(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psrad(a: *const u8, count: u32, out: *mut u8);
}

/// `psrad`
///
/// Shift the four signed 32-bit integer lanes right by `count`.
#[inline(always)]
pub fn psrad(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psrad(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}
