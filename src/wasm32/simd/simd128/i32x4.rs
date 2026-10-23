//! `i32x4.*` instructions.
//!
//! Rules shared by every SIMD128 instruction are documented on the [parent module](super).

use super::V128;

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_splat(value: i32, out: *mut u8);
}
/// `i32x4.splat`
///
/// Replicate `value` into every lane.
#[inline(always)]
pub fn i32x4_splat(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_splat(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extract_lane_0(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extract_lane_1(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extract_lane_2(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extract_lane_3(a: *const u8) -> i32;
}
/// `i32x4.extract_lane`
///
/// Sign-extend the lane to `i32`.
///
/// `LANE` is an immediate in `0..4`. A constant outside that range does not compile.
#[inline(always)]
pub fn i32x4_extract_lane<const LANE: u32>(a: V128) -> i32 {
    const {
        assert!(LANE < 4);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_i32x4_extract_lane_0(a.as_ptr()),
            1 => rust_isa_simd128_i32x4_extract_lane_1(a.as_ptr()),
            2 => rust_isa_simd128_i32x4_extract_lane_2(a.as_ptr()),
            3 => rust_isa_simd128_i32x4_extract_lane_3(a.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_replace_lane_0(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i32x4_replace_lane_1(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i32x4_replace_lane_2(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i32x4_replace_lane_3(a: *const u8, value: i32, out: *mut u8);
}
/// `i32x4.replace_lane`
///
/// Replace the lane with `value`.
///
/// `LANE` is an immediate in `0..4`. A constant outside that range does not compile.
#[inline(always)]
pub fn i32x4_replace_lane<const LANE: u32>(a: V128, value: i32) -> V128 {
    const {
        assert!(LANE < 4);
    }
    unsafe {
        match LANE {
            0 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i32x4_replace_lane_0(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            1 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i32x4_replace_lane_1(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            2 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i32x4_replace_lane_2(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            3 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i32x4_replace_lane_3(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_eq(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.eq`
///
/// Lane-wise `a == b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i32x4_eq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_eq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_ne(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.ne`
///
/// Lane-wise `a != b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i32x4_ne(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_ne(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_lt_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.lt_s`
///
/// Lane-wise signed `a < b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i32x4_lt_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_lt_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_lt_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.lt_u`
///
/// Lane-wise unsigned `a < b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i32x4_lt_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_lt_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_gt_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.gt_s`
///
/// Lane-wise signed `a > b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i32x4_gt_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_gt_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_gt_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.gt_u`
///
/// Lane-wise unsigned `a > b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i32x4_gt_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_gt_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_le_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.le_s`
///
/// Lane-wise signed `a <= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i32x4_le_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_le_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_le_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.le_u`
///
/// Lane-wise unsigned `a <= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i32x4_le_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_le_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_ge_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.ge_s`
///
/// Lane-wise signed `a >= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i32x4_ge_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_ge_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_ge_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.ge_u`
///
/// Lane-wise unsigned `a >= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i32x4_ge_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_ge_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_abs(a: *const u8, out: *mut u8);
}
/// `i32x4.abs`
///
/// Lane-wise absolute value. The most negative signed value stays unchanged.
#[inline(always)]
pub fn i32x4_abs(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_abs(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_neg(a: *const u8, out: *mut u8);
}
/// `i32x4.neg`
///
/// Lane-wise negation. The most negative signed value wraps and stays unchanged.
#[inline(always)]
pub fn i32x4_neg(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_neg(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_all_true(a: *const u8) -> i32;
}
/// `i32x4.all_true`
///
/// Return `1` when every lane is non-zero, otherwise `0`.
#[inline(always)]
pub fn i32x4_all_true(a: V128) -> i32 {
    unsafe { rust_isa_simd128_i32x4_all_true(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_bitmask(a: *const u8) -> i32;
}
/// `i32x4.bitmask`
///
/// Pack each lane's sign bit into an `i32`, with the low lane in the low bit.
#[inline(always)]
pub fn i32x4_bitmask(a: V128) -> i32 {
    unsafe { rust_isa_simd128_i32x4_bitmask(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_shl(a: *const u8, count: u32, out: *mut u8);
}
/// `i32x4.shl`
///
/// Shift each lane left. `count` is taken modulo the number of bits in the lane.
#[inline(always)]
pub fn i32x4_shl(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_shl(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_shr_s(a: *const u8, count: u32, out: *mut u8);
}
/// `i32x4.shr_s`
///
/// Arithmetic right shift of each lane. `count` is taken modulo the number of bits in the lane.
#[inline(always)]
pub fn i32x4_shr_s(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_shr_s(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_shr_u(a: *const u8, count: u32, out: *mut u8);
}
/// `i32x4.shr_u`
///
/// Logical right shift of each lane. `count` is taken modulo the number of bits in the lane.
#[inline(always)]
pub fn i32x4_shr_u(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_shr_u(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_add(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.add`
///
/// Lane-wise wrapping addition.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i32x4_add, i32x4_extract_lane, i32x4_splat};
///
/// assert_eq!(
///     i32x4_extract_lane::<0>(i32x4_add(i32x4_splat(i32::MAX), i32x4_splat(1))),
///     i32::MIN,
/// );
/// ```
#[inline(always)]
pub fn i32x4_add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_sub(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.sub`
///
/// Lane-wise wrapping subtraction.
#[inline(always)]
pub fn i32x4_sub(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_sub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_mul(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.mul`
///
/// Lane-wise wrapping multiplication.
#[inline(always)]
pub fn i32x4_mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_min_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.min_s`
///
/// Lane-wise signed minimum.
#[inline(always)]
pub fn i32x4_min_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_min_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_min_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.min_u`
///
/// Lane-wise unsigned minimum.
#[inline(always)]
pub fn i32x4_min_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_min_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_max_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.max_s`
///
/// Lane-wise signed maximum.
#[inline(always)]
pub fn i32x4_max_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_max_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_max_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.max_u`
///
/// Lane-wise unsigned maximum.
#[inline(always)]
pub fn i32x4_max_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_max_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_dot_i16x8_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.dot_i16x8_s`
///
/// Lane `i` is the sum of the products of sign-extended `i16` lanes `2i` and `2i + 1` from `a` and `b`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i32x4_dot_i16x8_s, i32x4_extract_lane, V128};
///
/// let left = V128::from_bytes([2, 0, 3, 0, 4, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let right = V128::from_bytes([4, 0, 5, 0, 6, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let dot = i32x4_dot_i16x8_s(left, right);
/// assert_eq!(i32x4_extract_lane::<0>(dot), 23);
/// assert_eq!(i32x4_extract_lane::<1>(dot), 59);
/// ```
#[inline(always)]
pub fn i32x4_dot_i16x8_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_dot_i16x8_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_trunc_sat_f32x4_s(a: *const u8, out: *mut u8);
}
/// `i32x4.trunc_sat_f32x4_s`
///
/// Truncate each lane toward zero and saturate to `i32`. NaN becomes `0`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i32x4_extract_lane, i32x4_trunc_sat_f32x4_s, V128};
///
/// let mut bytes = [0u8; 16];
/// bytes[0..4].copy_from_slice(&1.9f32.to_le_bytes());
/// bytes[4..8].copy_from_slice(&(-1.9f32).to_le_bytes());
/// bytes[8..12].copy_from_slice(&f32::NAN.to_le_bytes());
/// let lanes = i32x4_trunc_sat_f32x4_s(V128::from_bytes(bytes));
/// assert_eq!(i32x4_extract_lane::<0>(lanes), 1);
/// assert_eq!(i32x4_extract_lane::<1>(lanes), -1);
/// assert_eq!(i32x4_extract_lane::<2>(lanes), 0);
/// ```
#[inline(always)]
pub fn i32x4_trunc_sat_f32x4_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_trunc_sat_f32x4_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_trunc_sat_f32x4_u(a: *const u8, out: *mut u8);
}
/// `i32x4.trunc_sat_f32x4_u`
///
/// Truncate each lane toward zero and saturate to `u32`. NaN and negative values become `0`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i32x4_extract_lane, i32x4_trunc_sat_f32x4_u, V128};
///
/// let mut bytes = [0u8; 16];
/// bytes[0..4].copy_from_slice(&(-1.9f32).to_le_bytes());
/// bytes[4..8].copy_from_slice(&1.9f32.to_le_bytes());
/// bytes[8..12].copy_from_slice(&f32::NAN.to_le_bytes());
/// let lanes = i32x4_trunc_sat_f32x4_u(V128::from_bytes(bytes));
/// assert_eq!(i32x4_extract_lane::<0>(lanes), 0);
/// assert_eq!(i32x4_extract_lane::<1>(lanes), 1);
/// assert_eq!(i32x4_extract_lane::<2>(lanes), 0);
/// ```
#[inline(always)]
pub fn i32x4_trunc_sat_f32x4_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_trunc_sat_f32x4_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_trunc_sat_f64x2_s_zero(a: *const u8, out: *mut u8);
}
/// `i32x4.trunc_sat_f64x2_s_zero`
///
/// Truncate the two `f64` lanes toward zero, saturate each to `i32`, and set the upper two lanes to `0`. NaN becomes `0`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i32x4_extract_lane, i32x4_trunc_sat_f64x2_s_zero, V128};
///
/// let mut bytes = [0u8; 16];
/// bytes[..8].copy_from_slice(&1.9f64.to_le_bytes());
/// bytes[8..].copy_from_slice(&(-1.9f64).to_le_bytes());
/// let lanes = i32x4_trunc_sat_f64x2_s_zero(V128::from_bytes(bytes));
/// assert_eq!(i32x4_extract_lane::<0>(lanes), 1);
/// assert_eq!(i32x4_extract_lane::<1>(lanes), -1);
/// assert_eq!(i32x4_extract_lane::<2>(lanes), 0);
/// ```
#[inline(always)]
pub fn i32x4_trunc_sat_f64x2_s_zero(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_trunc_sat_f64x2_s_zero(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_trunc_sat_f64x2_u_zero(a: *const u8, out: *mut u8);
}
/// `i32x4.trunc_sat_f64x2_u_zero`
///
/// Truncate the two `f64` lanes toward zero, saturate each to `u32`, and set the upper two lanes to `0`. NaN and negative values become `0`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i32x4_extract_lane, i32x4_trunc_sat_f64x2_u_zero, V128};
///
/// let mut bytes = [0u8; 16];
/// bytes[..8].copy_from_slice(&(-1.5f64).to_le_bytes());
/// bytes[8..].copy_from_slice(&4.2f64.to_le_bytes());
/// let lanes = i32x4_trunc_sat_f64x2_u_zero(V128::from_bytes(bytes));
/// assert_eq!(i32x4_extract_lane::<0>(lanes), 0);
/// assert_eq!(i32x4_extract_lane::<1>(lanes), 4);
/// assert_eq!(i32x4_extract_lane::<2>(lanes), 0);
/// ```
#[inline(always)]
pub fn i32x4_trunc_sat_f64x2_u_zero(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_trunc_sat_f64x2_u_zero(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extend_low_i16x8_s(a: *const u8, out: *mut u8);
}
/// `i32x4.extend_low_i16x8_s`
///
/// Sign-extend the low half of the lanes.
#[inline(always)]
pub fn i32x4_extend_low_i16x8_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_extend_low_i16x8_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extend_high_i16x8_s(a: *const u8, out: *mut u8);
}
/// `i32x4.extend_high_i16x8_s`
///
/// Sign-extend the high half of the lanes.
#[inline(always)]
pub fn i32x4_extend_high_i16x8_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_extend_high_i16x8_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extend_low_i16x8_u(a: *const u8, out: *mut u8);
}
/// `i32x4.extend_low_i16x8_u`
///
/// Zero-extend the low half of the lanes.
#[inline(always)]
pub fn i32x4_extend_low_i16x8_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_extend_low_i16x8_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extend_high_i16x8_u(a: *const u8, out: *mut u8);
}
/// `i32x4.extend_high_i16x8_u`
///
/// Zero-extend the high half of the lanes.
#[inline(always)]
pub fn i32x4_extend_high_i16x8_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_extend_high_i16x8_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extadd_pairwise_i16x8_s(a: *const u8, out: *mut u8);
}
/// `i32x4.extadd_pairwise_i16x8_s`
///
/// Sign-extend each lane, then add each adjacent pair.
#[inline(always)]
pub fn i32x4_extadd_pairwise_i16x8_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_extadd_pairwise_i16x8_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extadd_pairwise_i16x8_u(a: *const u8, out: *mut u8);
}
/// `i32x4.extadd_pairwise_i16x8_u`
///
/// Zero-extend each lane, then add each adjacent pair.
#[inline(always)]
pub fn i32x4_extadd_pairwise_i16x8_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_extadd_pairwise_i16x8_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extmul_low_i16x8_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.extmul_low_i16x8_s`
///
/// Sign-extend the low half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i32x4_extmul_low_i16x8_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_extmul_low_i16x8_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extmul_high_i16x8_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.extmul_high_i16x8_s`
///
/// Sign-extend the high half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i32x4_extmul_high_i16x8_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_extmul_high_i16x8_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extmul_low_i16x8_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.extmul_low_i16x8_u`
///
/// Zero-extend the low half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i32x4_extmul_low_i16x8_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_extmul_low_i16x8_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i32x4_extmul_high_i16x8_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i32x4.extmul_high_i16x8_u`
///
/// Zero-extend the high half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i32x4_extmul_high_i16x8_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i32x4_extmul_high_i16x8_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
