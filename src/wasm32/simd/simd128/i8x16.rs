//! `i8x16.*` instructions.
//!
//! Rules shared by every SIMD128 instruction are documented on the [parent module](super).

use super::V128;

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_splat(value: i32, out: *mut u8);
}
/// `i8x16.splat`
///
/// Replicate the low 8 bits of `value` into every lane.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_extract_lane_s, i8x16_splat};
///
/// assert_eq!(i8x16_extract_lane_s::<0>(i8x16_splat(0x105)), 5);
/// ```
#[inline(always)]
pub fn i8x16_splat(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_splat(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_0(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_1(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_2(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_3(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_4(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_5(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_6(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_7(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_8(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_9(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_10(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_11(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_12(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_13(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_14(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_s_15(a: *const u8) -> i32;
}
/// `i8x16.extract_lane_s`
///
/// Sign-extend the lane to `i32`.
///
/// `LANE` is an immediate in `0..16`. A constant outside that range does not compile.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_extract_lane_s, i8x16_splat};
///
/// assert_eq!(i8x16_extract_lane_s::<0>(i8x16_splat(-1)), -1);
/// ```
///
/// ```compile_fail,E0080
/// use rust_isa::wasm32::simd::simd128::{i8x16_extract_lane_s, i8x16_splat};
///
/// let _ = i8x16_extract_lane_s::<16>(i8x16_splat(0));
/// ```
#[inline(always)]
pub fn i8x16_extract_lane_s<const LANE: u32>(a: V128) -> i32 {
    const {
        assert!(LANE < 16);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_i8x16_extract_lane_s_0(a.as_ptr()),
            1 => rust_isa_simd128_i8x16_extract_lane_s_1(a.as_ptr()),
            2 => rust_isa_simd128_i8x16_extract_lane_s_2(a.as_ptr()),
            3 => rust_isa_simd128_i8x16_extract_lane_s_3(a.as_ptr()),
            4 => rust_isa_simd128_i8x16_extract_lane_s_4(a.as_ptr()),
            5 => rust_isa_simd128_i8x16_extract_lane_s_5(a.as_ptr()),
            6 => rust_isa_simd128_i8x16_extract_lane_s_6(a.as_ptr()),
            7 => rust_isa_simd128_i8x16_extract_lane_s_7(a.as_ptr()),
            8 => rust_isa_simd128_i8x16_extract_lane_s_8(a.as_ptr()),
            9 => rust_isa_simd128_i8x16_extract_lane_s_9(a.as_ptr()),
            10 => rust_isa_simd128_i8x16_extract_lane_s_10(a.as_ptr()),
            11 => rust_isa_simd128_i8x16_extract_lane_s_11(a.as_ptr()),
            12 => rust_isa_simd128_i8x16_extract_lane_s_12(a.as_ptr()),
            13 => rust_isa_simd128_i8x16_extract_lane_s_13(a.as_ptr()),
            14 => rust_isa_simd128_i8x16_extract_lane_s_14(a.as_ptr()),
            15 => rust_isa_simd128_i8x16_extract_lane_s_15(a.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_0(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_1(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_2(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_3(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_4(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_5(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_6(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_7(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_8(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_9(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_10(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_11(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_12(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_13(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_14(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_extract_lane_u_15(a: *const u8) -> u32;
}
/// `i8x16.extract_lane_u`
///
/// Zero-extend the lane to `u32`.
///
/// `LANE` is an immediate in `0..16`. A constant outside that range does not compile.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_extract_lane_u, i8x16_splat};
///
/// assert_eq!(i8x16_extract_lane_u::<0>(i8x16_splat(-1)), 0xff);
/// ```
#[inline(always)]
pub fn i8x16_extract_lane_u<const LANE: u32>(a: V128) -> u32 {
    const {
        assert!(LANE < 16);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_i8x16_extract_lane_u_0(a.as_ptr()),
            1 => rust_isa_simd128_i8x16_extract_lane_u_1(a.as_ptr()),
            2 => rust_isa_simd128_i8x16_extract_lane_u_2(a.as_ptr()),
            3 => rust_isa_simd128_i8x16_extract_lane_u_3(a.as_ptr()),
            4 => rust_isa_simd128_i8x16_extract_lane_u_4(a.as_ptr()),
            5 => rust_isa_simd128_i8x16_extract_lane_u_5(a.as_ptr()),
            6 => rust_isa_simd128_i8x16_extract_lane_u_6(a.as_ptr()),
            7 => rust_isa_simd128_i8x16_extract_lane_u_7(a.as_ptr()),
            8 => rust_isa_simd128_i8x16_extract_lane_u_8(a.as_ptr()),
            9 => rust_isa_simd128_i8x16_extract_lane_u_9(a.as_ptr()),
            10 => rust_isa_simd128_i8x16_extract_lane_u_10(a.as_ptr()),
            11 => rust_isa_simd128_i8x16_extract_lane_u_11(a.as_ptr()),
            12 => rust_isa_simd128_i8x16_extract_lane_u_12(a.as_ptr()),
            13 => rust_isa_simd128_i8x16_extract_lane_u_13(a.as_ptr()),
            14 => rust_isa_simd128_i8x16_extract_lane_u_14(a.as_ptr()),
            15 => rust_isa_simd128_i8x16_extract_lane_u_15(a.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_0(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_1(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_2(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_3(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_4(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_5(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_6(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_7(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_8(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_9(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_10(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_11(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_12(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_13(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_14(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i8x16_replace_lane_15(a: *const u8, value: i32, out: *mut u8);
}
/// `i8x16.replace_lane`
///
/// Replace the lane with the low 8 bits of `value`.
///
/// `LANE` is an immediate in `0..16`. A constant outside that range does not compile.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_extract_lane_s, i8x16_replace_lane, i8x16_splat};
///
/// let value = i8x16_replace_lane::<3>(i8x16_splat(-1), 0x109);
/// assert_eq!(i8x16_extract_lane_s::<3>(value), 9);
/// assert_eq!(i8x16_extract_lane_s::<2>(value), -1);
/// ```
#[inline(always)]
pub fn i8x16_replace_lane<const LANE: u32>(a: V128, value: i32) -> V128 {
    const {
        assert!(LANE < 16);
    }
    unsafe {
        match LANE {
            0 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_0(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            1 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_1(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            2 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_2(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            3 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_3(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            4 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_4(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            5 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_5(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            6 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_6(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            7 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_7(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            8 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_8(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            9 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_9(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            10 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_10(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            11 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_11(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            12 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_12(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            13 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_13(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            14 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_14(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            15 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i8x16_replace_lane_15(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_eq(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.eq`
///
/// Lane-wise `a == b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i8x16_eq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_eq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_ne(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.ne`
///
/// Lane-wise `a != b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i8x16_ne(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_ne(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_lt_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.lt_s`
///
/// Lane-wise signed `a < b`. A true lane is all bits one. A false lane is all bits zero.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_lt_s, i8x16_splat};
///
/// assert_eq!(i8x16_lt_s(i8x16_splat(-1), i8x16_splat(1)).to_bytes()[0], 0xff);
/// ```
#[inline(always)]
pub fn i8x16_lt_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_lt_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_lt_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.lt_u`
///
/// Lane-wise unsigned `a < b`. A true lane is all bits one. A false lane is all bits zero.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_lt_u, i8x16_splat};
///
/// assert_eq!(i8x16_lt_u(i8x16_splat(-1), i8x16_splat(1)).to_bytes()[0], 0);
/// ```
#[inline(always)]
pub fn i8x16_lt_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_lt_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_gt_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.gt_s`
///
/// Lane-wise signed `a > b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i8x16_gt_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_gt_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_gt_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.gt_u`
///
/// Lane-wise unsigned `a > b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i8x16_gt_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_gt_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_le_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.le_s`
///
/// Lane-wise signed `a <= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i8x16_le_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_le_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_le_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.le_u`
///
/// Lane-wise unsigned `a <= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i8x16_le_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_le_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_ge_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.ge_s`
///
/// Lane-wise signed `a >= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i8x16_ge_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_ge_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_ge_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.ge_u`
///
/// Lane-wise unsigned `a >= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i8x16_ge_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_ge_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_abs(a: *const u8, out: *mut u8);
}
/// `i8x16.abs`
///
/// Lane-wise absolute value. The most negative signed value stays unchanged.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_abs, i8x16_extract_lane_s, i8x16_splat};
///
/// assert_eq!(
///     i8x16_extract_lane_s::<0>(i8x16_abs(i8x16_splat(i32::from(i8::MIN)))),
///     i32::from(i8::MIN),
/// );
/// ```
#[inline(always)]
pub fn i8x16_abs(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_abs(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_neg(a: *const u8, out: *mut u8);
}
/// `i8x16.neg`
///
/// Lane-wise negation. The most negative signed value wraps and stays unchanged.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_extract_lane_s, i8x16_neg, i8x16_splat};
///
/// assert_eq!(
///     i8x16_extract_lane_s::<0>(i8x16_neg(i8x16_splat(i32::from(i8::MIN)))),
///     i32::from(i8::MIN),
/// );
/// ```
#[inline(always)]
pub fn i8x16_neg(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_neg(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_all_true(a: *const u8) -> i32;
}
/// `i8x16.all_true`
///
/// Return `1` when every lane is non-zero, otherwise `0`.
#[inline(always)]
pub fn i8x16_all_true(a: V128) -> i32 {
    unsafe { rust_isa_simd128_i8x16_all_true(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_bitmask(a: *const u8) -> i32;
}
/// `i8x16.bitmask`
///
/// Pack each lane's sign bit into an `i32`, with the low lane in the low bit.
#[inline(always)]
pub fn i8x16_bitmask(a: V128) -> i32 {
    unsafe { rust_isa_simd128_i8x16_bitmask(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_popcnt(a: *const u8, out: *mut u8);
}
/// `i8x16.popcnt`
///
/// Count the set bits in each lane.
#[inline(always)]
pub fn i8x16_popcnt(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_popcnt(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_shl(a: *const u8, count: u32, out: *mut u8);
}
/// `i8x16.shl`
///
/// Shift each lane left. `count` is taken modulo the number of bits in the lane.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_extract_lane_s, i8x16_shl, V128};
///
/// let value = V128::from_bytes([0xf8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// assert_eq!(i8x16_extract_lane_s::<0>(i8x16_shl(value, 1)), -16);
/// assert_eq!(
///     i8x16_extract_lane_s::<0>(i8x16_shl(value, 9)),
///     i8x16_extract_lane_s::<0>(i8x16_shl(value, 1)),
/// );
/// ```
#[inline(always)]
pub fn i8x16_shl(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_shl(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_shr_s(a: *const u8, count: u32, out: *mut u8);
}
/// `i8x16.shr_s`
///
/// Arithmetic right shift of each lane. `count` is taken modulo the number of bits in the lane.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_extract_lane_s, i8x16_shr_s, V128};
///
/// let value = V128::from_bytes([0xf8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// assert_eq!(i8x16_extract_lane_s::<0>(i8x16_shr_s(value, 1)), -4);
/// ```
#[inline(always)]
pub fn i8x16_shr_s(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_shr_s(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_shr_u(a: *const u8, count: u32, out: *mut u8);
}
/// `i8x16.shr_u`
///
/// Logical right shift of each lane. `count` is taken modulo the number of bits in the lane.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_shr_u, V128};
///
/// let value = V128::from_bytes([0xf8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// assert_eq!(i8x16_shr_u(value, 1).to_bytes()[0], 0x7c);
/// ```
#[inline(always)]
pub fn i8x16_shr_u(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_shr_u(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_add(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.add`
///
/// Lane-wise wrapping addition.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_add, i8x16_extract_lane_s, i8x16_splat};
///
/// assert_eq!(
///     i8x16_extract_lane_s::<0>(i8x16_add(i8x16_splat(127), i8x16_splat(1))),
///     -128,
/// );
/// ```
#[inline(always)]
pub fn i8x16_add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_add_sat_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.add_sat_s`
///
/// Lane-wise signed saturating addition.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_add_sat_s, i8x16_extract_lane_s, i8x16_splat};
///
/// assert_eq!(
///     i8x16_extract_lane_s::<0>(i8x16_add_sat_s(i8x16_splat(127), i8x16_splat(1))),
///     127,
/// );
/// ```
#[inline(always)]
pub fn i8x16_add_sat_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_add_sat_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_add_sat_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.add_sat_u`
///
/// Lane-wise unsigned saturating addition.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_add_sat_u, V128};
///
/// assert_eq!(
///     i8x16_add_sat_u(V128::from_bytes([200; 16]), V128::from_bytes([100; 16])).to_bytes(),
///     [255; 16],
/// );
/// ```
#[inline(always)]
pub fn i8x16_add_sat_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_add_sat_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_sub(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.sub`
///
/// Lane-wise wrapping subtraction.
#[inline(always)]
pub fn i8x16_sub(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_sub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_sub_sat_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.sub_sat_s`
///
/// Lane-wise signed saturating subtraction.
#[inline(always)]
pub fn i8x16_sub_sat_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_sub_sat_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_sub_sat_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.sub_sat_u`
///
/// Lane-wise unsigned saturating subtraction.
#[inline(always)]
pub fn i8x16_sub_sat_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_sub_sat_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_min_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.min_s`
///
/// Lane-wise signed minimum.
#[inline(always)]
pub fn i8x16_min_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_min_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_min_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.min_u`
///
/// Lane-wise unsigned minimum.
#[inline(always)]
pub fn i8x16_min_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_min_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_max_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.max_s`
///
/// Lane-wise signed maximum.
#[inline(always)]
pub fn i8x16_max_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_max_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_max_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.max_u`
///
/// Lane-wise unsigned maximum.
#[inline(always)]
pub fn i8x16_max_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_max_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_avgr_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.avgr_u`
///
/// Lane-wise unsigned average `(a + b + 1) / 2`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_avgr_u, V128};
///
/// assert_eq!(
///     i8x16_avgr_u(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes(),
///     [2; 16],
/// );
/// ```
#[inline(always)]
pub fn i8x16_avgr_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_avgr_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_swizzle(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.swizzle`
///
/// Lane `i` of the result is `a[b[i]]` when `b[i] < 16`, otherwise `0`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_swizzle, V128};
///
/// let source = V128::from_bytes([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let indices = V128::from_bytes([15, 0, 2, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let out = i8x16_swizzle(source, indices).to_bytes();
/// assert_eq!(out[0], 15);
/// assert_eq!(out[1], 0);
/// assert_eq!(out[2], 2);
/// assert_eq!(out[3], 0);
/// ```
#[inline(always)]
pub fn i8x16_swizzle(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_swizzle(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_narrow_i16x8_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.narrow_i16x8_s`
///
/// Signed saturating narrow. `a` supplies the low lanes of the result and `b` the high lanes.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_extract_lane_s, i8x16_narrow_i16x8_s, V128};
///
/// let wide = V128::from_bytes([200, 0, 0x38, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let zero = V128::from_bytes([0; 16]);
/// let narrowed = i8x16_narrow_i16x8_s(wide, zero);
/// assert_eq!(i8x16_extract_lane_s::<0>(narrowed), 127);
/// assert_eq!(i8x16_extract_lane_s::<1>(narrowed), -128);
/// ```
#[inline(always)]
pub fn i8x16_narrow_i16x8_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_narrow_i16x8_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i8x16_narrow_i16x8_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.narrow_i16x8_u`
///
/// Unsigned saturating narrow. Inputs below `0` become `0`, and inputs above the unsigned maximum saturate. `a` supplies the low lanes of the result and `b` the high lanes.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_narrow_i16x8_u, V128};
///
/// let wide = V128::from_bytes([200, 0, 0x38, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let zero = V128::from_bytes([0; 16]);
/// let narrowed = i8x16_narrow_i16x8_u(wide, zero).to_bytes();
/// assert_eq!(narrowed[0], 200);
/// assert_eq!(narrowed[1], 0);
/// ```
#[inline(always)]
pub fn i8x16_narrow_i16x8_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i8x16_narrow_i16x8_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
