//! `f32x4.*` instructions.
//!
//! Rules shared by every SIMD128 instruction are documented on the [parent module](super).
//!
//! # Compile-time lane check
//!
//! ```compile_fail,E0080
//! use rust_isa::wasm32::simd::simd128::{f32x4_extract_lane, f32x4_splat};
//!
//! let _ = f32x4_extract_lane::<4>(f32x4_splat(0.0));
//! ```
//!
//! ```compile_fail,E0080
//! use rust_isa::wasm32::simd::simd128::{f32x4_replace_lane, f32x4_splat};
//!
//! let _ = f32x4_replace_lane::<4>(f32x4_splat(0.0), 1.0);
//! ```

use super::V128;

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_splat(value: f32, out: *mut u8);
}
/// `f32x4.splat`
///
/// Replicate `value` into every lane.
#[inline(always)]
pub fn f32x4_splat(value: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_splat(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_extract_lane_0(a: *const u8) -> f32;
}
unsafe extern "C" {
    fn rust_isa_simd128_f32x4_extract_lane_1(a: *const u8) -> f32;
}
unsafe extern "C" {
    fn rust_isa_simd128_f32x4_extract_lane_2(a: *const u8) -> f32;
}
unsafe extern "C" {
    fn rust_isa_simd128_f32x4_extract_lane_3(a: *const u8) -> f32;
}
/// `f32x4.extract_lane`
///
/// Return the lane as `f32`.
///
/// `LANE` is an immediate in `0..4`. A constant outside that range does not compile.
#[inline(always)]
pub fn f32x4_extract_lane<const LANE: u32>(a: V128) -> f32 {
    const {
        assert!(LANE < 4);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_f32x4_extract_lane_0(a.as_ptr()),
            1 => rust_isa_simd128_f32x4_extract_lane_1(a.as_ptr()),
            2 => rust_isa_simd128_f32x4_extract_lane_2(a.as_ptr()),
            3 => rust_isa_simd128_f32x4_extract_lane_3(a.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_replace_lane_0(a: *const u8, value: f32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_f32x4_replace_lane_1(a: *const u8, value: f32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_f32x4_replace_lane_2(a: *const u8, value: f32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_f32x4_replace_lane_3(a: *const u8, value: f32, out: *mut u8);
}
/// `f32x4.replace_lane`
///
/// Replace the lane with `value`.
///
/// `LANE` is an immediate in `0..4`. A constant outside that range does not compile.
#[inline(always)]
pub fn f32x4_replace_lane<const LANE: u32>(a: V128, value: f32) -> V128 {
    const {
        assert!(LANE < 4);
    }
    unsafe {
        match LANE {
            0 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_f32x4_replace_lane_0(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            1 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_f32x4_replace_lane_1(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            2 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_f32x4_replace_lane_2(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            3 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_f32x4_replace_lane_3(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_eq(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.eq`
///
/// Lane-wise `a == b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f32x4_eq, f32x4_splat};
///
/// assert_eq!(f32x4_eq(f32x4_splat(f32::NAN), f32x4_splat(f32::NAN)).to_bytes()[0], 0);
/// ```
#[inline(always)]
pub fn f32x4_eq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_eq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_ne(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.ne`
///
/// Lane-wise `a != b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f32x4_ne, f32x4_splat};
///
/// assert_eq!(
///     f32x4_ne(f32x4_splat(f32::NAN), f32x4_splat(1.0)).to_bytes()[0],
///     0xff,
/// );
/// ```
#[inline(always)]
pub fn f32x4_ne(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_ne(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_lt(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.lt`
///
/// Lane-wise `a < b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
#[inline(always)]
pub fn f32x4_lt(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_lt(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_gt(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.gt`
///
/// Lane-wise `a > b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
#[inline(always)]
pub fn f32x4_gt(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_gt(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_le(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.le`
///
/// Lane-wise `a <= b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
#[inline(always)]
pub fn f32x4_le(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_le(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_ge(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.ge`
///
/// Lane-wise `a >= b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
#[inline(always)]
pub fn f32x4_ge(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_ge(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_abs(a: *const u8, out: *mut u8);
}
/// `f32x4.abs`
///
/// Clear the sign bit of each lane.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f32x4_abs, f32x4_extract_lane, f32x4_splat};
///
/// assert_eq!(
///     f32x4_extract_lane::<0>(f32x4_abs(f32x4_splat(-0.0))).to_bits(),
///     0.0f32.to_bits(),
/// );
/// ```
#[inline(always)]
pub fn f32x4_abs(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_abs(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_neg(a: *const u8, out: *mut u8);
}
/// `f32x4.neg`
///
/// Invert the sign bit of each lane.
#[inline(always)]
pub fn f32x4_neg(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_neg(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_sqrt(a: *const u8, out: *mut u8);
}
/// `f32x4.sqrt`
///
/// Lane-wise IEEE 754 square root.
#[inline(always)]
pub fn f32x4_sqrt(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_sqrt(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_ceil(a: *const u8, out: *mut u8);
}
/// `f32x4.ceil`
///
/// Round each lane toward +infinity to an integer.
#[inline(always)]
pub fn f32x4_ceil(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_ceil(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_floor(a: *const u8, out: *mut u8);
}
/// `f32x4.floor`
///
/// Round each lane toward -infinity to an integer.
#[inline(always)]
pub fn f32x4_floor(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_floor(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_trunc(a: *const u8, out: *mut u8);
}
/// `f32x4.trunc`
///
/// Round each lane toward zero to an integer.
#[inline(always)]
pub fn f32x4_trunc(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_trunc(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_nearest(a: *const u8, out: *mut u8);
}
/// `f32x4.nearest`
///
/// Round each lane to the nearest integer. Ties round to even.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f32x4_extract_lane, f32x4_nearest, f32x4_splat};
///
/// assert_eq!(f32x4_extract_lane::<0>(f32x4_nearest(f32x4_splat(2.5))), 2.0);
/// assert_eq!(f32x4_extract_lane::<0>(f32x4_nearest(f32x4_splat(0.5))), 0.0);
/// ```
#[inline(always)]
pub fn f32x4_nearest(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_nearest(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_add(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.add`
///
/// Lane-wise IEEE 754 addition.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f32x4_add, f32x4_extract_lane, f32x4_splat};
///
/// let sum = f32x4_add(f32x4_splat(1.5), f32x4_splat(2.5));
/// assert_eq!(f32x4_extract_lane::<0>(sum), 4.0);
/// ```
#[inline(always)]
pub fn f32x4_add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_sub(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.sub`
///
/// Lane-wise IEEE 754 subtraction.
#[inline(always)]
pub fn f32x4_sub(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_sub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_mul(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.mul`
///
/// Lane-wise IEEE 754 multiplication.
#[inline(always)]
pub fn f32x4_mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_div(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.div`
///
/// Lane-wise IEEE 754 division.
#[inline(always)]
pub fn f32x4_div(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_div(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_min(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.min`
///
/// Lane-wise minimum. If either operand is NaN, the result is NaN. `-0.0` is less than `+0.0`. `f32::min` ignores a NaN operand; this instruction does not.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f32x4_extract_lane, f32x4_min, f32x4_splat};
///
/// let nan = f32x4_splat(f32::NAN);
/// let one = f32x4_splat(1.0);
/// assert!(f32x4_extract_lane::<0>(f32x4_min(nan, one)).is_nan());
/// assert!(f32x4_extract_lane::<0>(f32x4_min(one, nan)).is_nan());
/// assert_eq!(f32::min(f32::NAN, 1.0), 1.0);
/// assert_eq!(
///     f32x4_extract_lane::<0>(f32x4_min(f32x4_splat(-0.0), f32x4_splat(0.0))).to_bits(),
///     (-0.0f32).to_bits(),
/// );
/// ```
#[inline(always)]
pub fn f32x4_min(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_min(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_max(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.max`
///
/// Lane-wise maximum. If either operand is NaN, the result is NaN. `-0.0` is less than `+0.0`. `f32::max` ignores a NaN operand; this instruction does not.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f32x4_extract_lane, f32x4_max, f32x4_splat};
///
/// let nan = f32x4_splat(f32::NAN);
/// assert!(f32x4_extract_lane::<0>(f32x4_max(nan, f32x4_splat(1.0))).is_nan());
/// assert_eq!(
///     f32x4_extract_lane::<0>(f32x4_max(f32x4_splat(-0.0), f32x4_splat(0.0))).to_bits(),
///     0.0f32.to_bits(),
/// );
/// ```
#[inline(always)]
pub fn f32x4_max(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_max(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_pmin(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.pmin`
///
/// Select `b` when `b < a`, otherwise `a`. A comparison with NaN is false.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f32x4_extract_lane, f32x4_pmin, f32x4_splat};
///
/// let nan = f32x4_splat(f32::NAN);
/// let one = f32x4_splat(1.0);
/// assert!(f32x4_extract_lane::<0>(f32x4_pmin(nan, one)).is_nan());
/// assert_eq!(f32x4_extract_lane::<0>(f32x4_pmin(one, nan)), 1.0);
/// ```
#[inline(always)]
pub fn f32x4_pmin(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_pmin(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_pmax(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f32x4.pmax`
///
/// Select `b` when `a < b`, otherwise `a`. A comparison with NaN is false.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f32x4_extract_lane, f32x4_pmax, f32x4_splat};
///
/// let nan = f32x4_splat(f32::NAN);
/// let three = f32x4_splat(3.0);
/// assert!(f32x4_extract_lane::<0>(f32x4_pmax(nan, three)).is_nan());
/// assert_eq!(f32x4_extract_lane::<0>(f32x4_pmax(three, nan)), 3.0);
/// ```
#[inline(always)]
pub fn f32x4_pmax(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_pmax(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_convert_i32x4_s(a: *const u8, out: *mut u8);
}
/// `f32x4.convert_i32x4_s`
///
/// Convert each `i32` lane to `f32`.
#[inline(always)]
pub fn f32x4_convert_i32x4_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_convert_i32x4_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_convert_i32x4_u(a: *const u8, out: *mut u8);
}
/// `f32x4.convert_i32x4_u`
///
/// Interpret each lane as `u32` and convert it to `f32`.
#[inline(always)]
pub fn f32x4_convert_i32x4_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_convert_i32x4_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f32x4_demote_f64x2_zero(a: *const u8, out: *mut u8);
}
/// `f32x4.demote_f64x2_zero`
///
/// Demote the two `f64` lanes to `f32` and set the upper two lanes to `0`.
#[inline(always)]
pub fn f32x4_demote_f64x2_zero(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f32x4_demote_f64x2_zero(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
