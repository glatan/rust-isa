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

unsafe extern "C" {
    fn rust_isa_sse2_andps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_andnps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_orps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_xorps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_unpcklps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_unpckhps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_cvtsi2ss(value: i32, out: *mut u8);
    fn rust_isa_sse2_cvtss2si(a: *const u8) -> i32;
    fn rust_isa_sse2_cvttss2si(a: *const u8) -> i32;
    fn rust_isa_sse2_cvtps2pd(a: *const u8, out: *mut u8);
    fn rust_isa_sse2_cvtpd2ps(a: *const u8, out: *mut u8);
    fn rust_isa_sse2_cvtdq2ps(a: *const u8, out: *mut u8);
    fn rust_isa_sse2_cvtps2dq(a: *const u8, out: *mut u8);
    fn rust_isa_sse2_cvttps2dq(a: *const u8, out: *mut u8);
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
    /// `andps`
    ///
    /// Bitwise AND of the 16 bytes.
    andps,
    rust_isa_sse2_andps
);
binary!(
    /// `andnps`
    ///
    /// Bitwise AND of `b` with the complement of `a`.
    andnps,
    rust_isa_sse2_andnps
);
binary!(
    /// `orps`
    ///
    /// Bitwise OR of the 16 bytes.
    orps,
    rust_isa_sse2_orps
);
binary!(
    /// `xorps`
    ///
    /// Bitwise XOR of the 16 bytes.
    xorps,
    rust_isa_sse2_xorps
);
binary!(
    /// `unpcklps`
    ///
    /// Interleave the low two 32-bit float lanes of `a` and `b`.
    unpcklps,
    rust_isa_sse2_unpcklps
);
binary!(
    /// `unpckhps`
    ///
    /// Interleave the high two 32-bit float lanes of `a` and `b`.
    unpckhps,
    rust_isa_sse2_unpckhps
);

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

/// `cvtss2si`
///
/// Convert the low 32-bit float lane to a signed 32-bit integer, rounding
/// according to the current rounding mode.
#[inline(always)]
pub fn cvtss2si(a: V128) -> i32 {
    unsafe { rust_isa_sse2_cvtss2si(a.as_ptr()) }
}

/// `cvttss2si`
///
/// Convert the low 32-bit float lane to a signed 32-bit integer, truncating
/// toward zero.
#[inline(always)]
pub fn cvttss2si(a: V128) -> i32 {
    unsafe { rust_isa_sse2_cvttss2si(a.as_ptr()) }
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

unary!(
    /// `cvtps2pd`
    ///
    /// Convert the low two 32-bit float lanes to 64-bit float lanes.
    cvtps2pd,
    rust_isa_sse2_cvtps2pd
);
unary!(
    /// `cvtpd2ps`
    ///
    /// Convert the two 64-bit float lanes to 32-bit float lanes. The upper
    /// 64 bits are zero.
    cvtpd2ps,
    rust_isa_sse2_cvtpd2ps
);
unary!(
    /// `cvtdq2ps`
    ///
    /// Convert four signed 32-bit integer lanes to 32-bit float lanes.
    cvtdq2ps,
    rust_isa_sse2_cvtdq2ps
);
unary!(
    /// `cvtps2dq`
    ///
    /// Convert four 32-bit float lanes to signed 32-bit integers, rounding
    /// according to the current rounding mode.
    cvtps2dq,
    rust_isa_sse2_cvtps2dq
);
unary!(
    /// `cvttps2dq`
    ///
    /// Convert four 32-bit float lanes to signed 32-bit integers, truncating
    /// toward zero.
    cvttps2dq,
    rust_isa_sse2_cvttps2dq
);
