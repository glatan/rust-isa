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

unsafe extern "C" {
    fn rust_isa_sse2_movmskpd(a: *const u8) -> i32;
}

/// `movmskpd`
///
/// Collect the sign bit of each 64-bit float lane into the low two bits.
#[inline(always)]
pub fn movmskpd(a: V128) -> u32 {
    unsafe { rust_isa_sse2_movmskpd(a.as_ptr()) as u32 }
}

unsafe extern "C" {
    fn rust_isa_sse2_unpcklpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_unpckhpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_cvtsi2sd(value: i32, out: *mut u8);
    fn rust_isa_sse2_cvtsd2si(a: *const u8) -> i32;
    fn rust_isa_sse2_cvttsd2si(a: *const u8) -> i32;
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `unpcklpd`
    ///
    /// Interleave the low 64-bit float lane of `a` and `b`.
    unpcklpd,
    rust_isa_sse2_unpcklpd
);
binary!(
    /// `unpckhpd`
    ///
    /// Interleave the high 64-bit float lane of `a` and `b`.
    unpckhpd,
    rust_isa_sse2_unpckhpd
);

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

/// `cvtsd2si`
///
/// Convert the low 64-bit float lane to a signed 32-bit integer, rounding
/// according to the current rounding mode.
#[inline(always)]
pub fn cvtsd2si(a: V128) -> i32 {
    unsafe { rust_isa_sse2_cvtsd2si(a.as_ptr()) }
}

/// `cvttsd2si`
///
/// Convert the low 64-bit float lane to a signed 32-bit integer, truncating
/// toward zero.
#[inline(always)]
pub fn cvttsd2si(a: V128) -> i32 {
    unsafe { rust_isa_sse2_cvttsd2si(a.as_ptr()) }
}
