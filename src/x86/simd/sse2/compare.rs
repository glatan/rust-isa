//! Scalar compare instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_comiss(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse2_comisd(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse2_ucomiss(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse2_ucomisd(a: *const u8, b: *const u8) -> i32;
}

/// `comiss`
///
/// Compare the low 32-bit float lanes. Returns `-1` when `a < b`, `0` when
/// `a == b`, `1` when `a > b`, and `2` for unordered operands.
#[inline(always)]
pub fn comiss(a: V128, b: V128) -> i32 {
    unsafe { rust_isa_sse2_comiss(a.as_ptr(), b.as_ptr()) }
}

/// `comisd`
///
/// Compare the low 64-bit float lanes. Returns `-1` when `a < b`, `0` when
/// `a == b`, `1` when `a > b`, and `2` for unordered operands.
#[inline(always)]
pub fn comisd(a: V128, b: V128) -> i32 {
    unsafe { rust_isa_sse2_comisd(a.as_ptr(), b.as_ptr()) }
}

/// `ucomiss`
///
/// Compare the low 32-bit float lanes without raising an invalid exception for
/// quiet NaNs. Returns the same codes as [`comiss`].
#[inline(always)]
pub fn ucomiss(a: V128, b: V128) -> i32 {
    unsafe { rust_isa_sse2_ucomiss(a.as_ptr(), b.as_ptr()) }
}

/// `ucomisd`
///
/// Compare the low 64-bit float lanes without raising an invalid exception for
/// quiet NaNs. Returns the same codes as [`comisd`].
#[inline(always)]
pub fn ucomisd(a: V128, b: V128) -> i32 {
    unsafe { rust_isa_sse2_ucomisd(a.as_ptr(), b.as_ptr()) }
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
    fn rust_isa_sse2_pcmpeqb(a: *const u8, b: *const u8, out: *mut u8);
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

unsafe extern "C" {
    fn rust_isa_sse2_pcmpgtb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pcmpgtb`
///
/// Compare the 16 signed byte lanes. A true lane is all bits one.
#[inline(always)]
pub fn pcmpgtb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pcmpgtb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pcmpeqw(a: *const u8, b: *const u8, out: *mut u8);
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

unsafe extern "C" {
    fn rust_isa_sse2_pcmpgtw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pcmpgtw`
///
/// Compare the eight signed 16-bit integer lanes. A true lane is all bits one.
#[inline(always)]
pub fn pcmpgtw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pcmpgtw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pcmpeqd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pcmpeqd`
///
/// Compare the four 32-bit integer lanes. A true lane is all bits one.
#[inline(always)]
pub fn pcmpeqd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pcmpeqd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pcmpgtd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pcmpgtd`
///
/// Compare the four signed 32-bit integer lanes. A true lane is all bits one.
#[inline(always)]
pub fn pcmpgtd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pcmpgtd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
