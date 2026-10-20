//! `f64x2.*` instructions.
//!
//! Rules shared by every SIMD128 instruction are documented on the [parent module](super).

use super::V128;

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_splat(value: f64, out: *mut u8);
}
/// `f64x2.splat`
///
/// Replicate `value` into both lanes.
#[inline(always)]
pub fn f64x2_splat(value: f64) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_splat(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_extract_lane_0(a: *const u8) -> f64;
}
unsafe extern "C" {
    fn rust_isa_simd128_f64x2_extract_lane_1(a: *const u8) -> f64;
}
/// `f64x2.extract_lane`
///
/// Return the lane as `f64`.
///
/// `LANE` is an immediate in `0..2`. A constant outside that range does not compile.
#[inline(always)]
pub fn f64x2_extract_lane<const LANE: u32>(a: V128) -> f64 {
    const {
        assert!(LANE < 2);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_f64x2_extract_lane_0(a.as_ptr()),
            1 => rust_isa_simd128_f64x2_extract_lane_1(a.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_replace_lane_0(a: *const u8, value: f64, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_f64x2_replace_lane_1(a: *const u8, value: f64, out: *mut u8);
}
/// `f64x2.replace_lane`
///
/// Replace the lane with `value`.
///
/// `LANE` is an immediate in `0..2`. A constant outside that range does not compile.
#[inline(always)]
pub fn f64x2_replace_lane<const LANE: u32>(a: V128, value: f64) -> V128 {
    const {
        assert!(LANE < 2);
    }
    unsafe {
        match LANE {
            0 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_f64x2_replace_lane_0(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            1 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_f64x2_replace_lane_1(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_eq(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.eq`
///
/// Lane-wise `a == b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
#[inline(always)]
pub fn f64x2_eq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_eq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_ne(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.ne`
///
/// Lane-wise `a != b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
#[inline(always)]
pub fn f64x2_ne(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_ne(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_lt(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.lt`
///
/// Lane-wise `a < b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
#[inline(always)]
pub fn f64x2_lt(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_lt(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_gt(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.gt`
///
/// Lane-wise `a > b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
#[inline(always)]
pub fn f64x2_gt(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_gt(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_le(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.le`
///
/// Lane-wise `a <= b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
#[inline(always)]
pub fn f64x2_le(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_le(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_ge(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.ge`
///
/// Lane-wise `a >= b`. A true lane is all bits one. A false lane is all bits zero. A comparison with NaN is false, except `ne`, which is true.
#[inline(always)]
pub fn f64x2_ge(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_ge(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_abs(a: *const u8, out: *mut u8);
}
/// `f64x2.abs`
///
/// Clear the sign bit of each lane.
#[inline(always)]
pub fn f64x2_abs(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_abs(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_neg(a: *const u8, out: *mut u8);
}
/// `f64x2.neg`
///
/// Invert the sign bit of each lane.
#[inline(always)]
pub fn f64x2_neg(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_neg(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_sqrt(a: *const u8, out: *mut u8);
}
/// `f64x2.sqrt`
///
/// Lane-wise IEEE 754 square root.
#[inline(always)]
pub fn f64x2_sqrt(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_sqrt(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_ceil(a: *const u8, out: *mut u8);
}
/// `f64x2.ceil`
///
/// Round each lane toward +infinity to an integer.
#[inline(always)]
pub fn f64x2_ceil(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_ceil(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_floor(a: *const u8, out: *mut u8);
}
/// `f64x2.floor`
///
/// Round each lane toward -infinity to an integer.
#[inline(always)]
pub fn f64x2_floor(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_floor(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_trunc(a: *const u8, out: *mut u8);
}
/// `f64x2.trunc`
///
/// Round each lane toward zero to an integer.
#[inline(always)]
pub fn f64x2_trunc(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_trunc(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_nearest(a: *const u8, out: *mut u8);
}
/// `f64x2.nearest`
///
/// Round each lane to the nearest integer. Ties round to even.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f64x2_extract_lane, f64x2_nearest, f64x2_splat};
///
/// assert_eq!(f64x2_extract_lane::<0>(f64x2_nearest(f64x2_splat(0.5))), 0.0);
/// assert_eq!(f64x2_extract_lane::<0>(f64x2_nearest(f64x2_splat(2.5))), 2.0);
/// ```
#[inline(always)]
pub fn f64x2_nearest(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_nearest(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_add(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.add`
///
/// Lane-wise IEEE 754 addition.
#[inline(always)]
pub fn f64x2_add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_sub(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.sub`
///
/// Lane-wise IEEE 754 subtraction.
#[inline(always)]
pub fn f64x2_sub(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_sub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_mul(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.mul`
///
/// Lane-wise IEEE 754 multiplication.
#[inline(always)]
pub fn f64x2_mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_div(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.div`
///
/// Lane-wise IEEE 754 division.
#[inline(always)]
pub fn f64x2_div(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_div(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_min(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.min`
///
/// Lane-wise minimum. If either operand is NaN, the result is NaN. `-0.0` is less than `+0.0`. `f64::min` ignores a NaN operand; this instruction does not.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f64x2_extract_lane, f64x2_min, f64x2_splat};
///
/// let nan = f64x2_splat(f64::NAN);
/// assert!(f64x2_extract_lane::<0>(f64x2_min(nan, f64x2_splat(1.0))).is_nan());
/// assert_eq!(f64::min(f64::NAN, 1.0), 1.0);
/// assert_eq!(
///     f64x2_extract_lane::<0>(f64x2_min(f64x2_splat(-0.0), f64x2_splat(0.0))).to_bits(),
///     (-0.0f64).to_bits(),
/// );
/// ```
#[inline(always)]
pub fn f64x2_min(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_min(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_max(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.max`
///
/// Lane-wise maximum. If either operand is NaN, the result is NaN. `-0.0` is less than `+0.0`. `f64::max` ignores a NaN operand; this instruction does not.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f64x2_extract_lane, f64x2_max, f64x2_splat};
///
/// assert!(f64x2_extract_lane::<0>(f64x2_max(f64x2_splat(1.0), f64x2_splat(f64::NAN))).is_nan());
/// assert_eq!(
///     f64x2_extract_lane::<0>(f64x2_max(f64x2_splat(-0.0), f64x2_splat(0.0))).to_bits(),
///     0.0f64.to_bits(),
/// );
/// ```
#[inline(always)]
pub fn f64x2_max(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_max(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_pmin(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.pmin`
///
/// Select `b` when `b < a`, otherwise `a`. A comparison with NaN is false.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f64x2_extract_lane, f64x2_pmin, f64x2_splat};
///
/// let nan = f64x2_splat(f64::NAN);
/// let one = f64x2_splat(1.0);
/// assert!(f64x2_extract_lane::<0>(f64x2_pmin(nan, one)).is_nan());
/// assert_eq!(f64x2_extract_lane::<0>(f64x2_pmin(one, nan)), 1.0);
/// ```
#[inline(always)]
pub fn f64x2_pmin(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_pmin(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_pmax(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.pmax`
///
/// Select `b` when `a < b`, otherwise `a`. A comparison with NaN is false.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f64x2_extract_lane, f64x2_pmax, f64x2_splat};
///
/// let nan = f64x2_splat(f64::NAN);
/// let three = f64x2_splat(3.0);
/// assert!(f64x2_extract_lane::<0>(f64x2_pmax(nan, three)).is_nan());
/// assert_eq!(f64x2_extract_lane::<0>(f64x2_pmax(three, nan)), 3.0);
/// ```
#[inline(always)]
pub fn f64x2_pmax(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_pmax(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_convert_low_i32x4_s(a: *const u8, out: *mut u8);
}
/// `f64x2.convert_low_i32x4_s`
///
/// Convert the low two `i32` lanes to `f64`.
#[inline(always)]
pub fn f64x2_convert_low_i32x4_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_convert_low_i32x4_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_convert_low_i32x4_u(a: *const u8, out: *mut u8);
}
/// `f64x2.convert_low_i32x4_u`
///
/// Interpret the low two lanes as `u32` and convert them to `f64`.
#[inline(always)]
pub fn f64x2_convert_low_i32x4_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_convert_low_i32x4_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_f64x2_promote_low_f32x4(a: *const u8, out: *mut u8);
}
/// `f64x2.promote_low_f32x4`
///
/// Promote the low two `f32` lanes to `f64`.
#[inline(always)]
pub fn f64x2_promote_low_f32x4(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_f64x2_promote_low_f32x4(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
