//! `i16x8.*` instructions.
//!
//! Rules shared by every SIMD128 instruction are documented on the [parent module](super).

use super::V128;

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_splat(value: i32, out: *mut u8);
}
/// `i16x8.splat`
///
/// Replicate the low 16 bits of `value` into every lane.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i16x8_extract_lane_s, i16x8_splat};
///
/// assert_eq!(i16x8_extract_lane_s::<0>(i16x8_splat(0x1_0002)), 2);
/// ```
#[inline(always)]
pub fn i16x8_splat(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_splat(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_s_0(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_s_1(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_s_2(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_s_3(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_s_4(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_s_5(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_s_6(a: *const u8) -> i32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_s_7(a: *const u8) -> i32;
}
/// `i16x8.extract_lane_s`
///
/// Sign-extend the lane to `i32`.
///
/// `LANE` is an immediate in `0..8`. A constant outside that range does not compile.
#[inline(always)]
pub fn i16x8_extract_lane_s<const LANE: u32>(a: V128) -> i32 {
    const {
        assert!(LANE < 8);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_i16x8_extract_lane_s_0(a.as_ptr()),
            1 => rust_isa_simd128_i16x8_extract_lane_s_1(a.as_ptr()),
            2 => rust_isa_simd128_i16x8_extract_lane_s_2(a.as_ptr()),
            3 => rust_isa_simd128_i16x8_extract_lane_s_3(a.as_ptr()),
            4 => rust_isa_simd128_i16x8_extract_lane_s_4(a.as_ptr()),
            5 => rust_isa_simd128_i16x8_extract_lane_s_5(a.as_ptr()),
            6 => rust_isa_simd128_i16x8_extract_lane_s_6(a.as_ptr()),
            7 => rust_isa_simd128_i16x8_extract_lane_s_7(a.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_u_0(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_u_1(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_u_2(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_u_3(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_u_4(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_u_5(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_u_6(a: *const u8) -> u32;
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extract_lane_u_7(a: *const u8) -> u32;
}
/// `i16x8.extract_lane_u`
///
/// Zero-extend the lane to `u32`.
///
/// `LANE` is an immediate in `0..8`. A constant outside that range does not compile.
#[inline(always)]
pub fn i16x8_extract_lane_u<const LANE: u32>(a: V128) -> u32 {
    const {
        assert!(LANE < 8);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_i16x8_extract_lane_u_0(a.as_ptr()),
            1 => rust_isa_simd128_i16x8_extract_lane_u_1(a.as_ptr()),
            2 => rust_isa_simd128_i16x8_extract_lane_u_2(a.as_ptr()),
            3 => rust_isa_simd128_i16x8_extract_lane_u_3(a.as_ptr()),
            4 => rust_isa_simd128_i16x8_extract_lane_u_4(a.as_ptr()),
            5 => rust_isa_simd128_i16x8_extract_lane_u_5(a.as_ptr()),
            6 => rust_isa_simd128_i16x8_extract_lane_u_6(a.as_ptr()),
            7 => rust_isa_simd128_i16x8_extract_lane_u_7(a.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_replace_lane_0(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_replace_lane_1(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_replace_lane_2(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_replace_lane_3(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_replace_lane_4(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_replace_lane_5(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_replace_lane_6(a: *const u8, value: i32, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i16x8_replace_lane_7(a: *const u8, value: i32, out: *mut u8);
}
/// `i16x8.replace_lane`
///
/// Replace the lane with the low 16 bits of `value`.
///
/// `LANE` is an immediate in `0..8`. A constant outside that range does not compile.
#[inline(always)]
pub fn i16x8_replace_lane<const LANE: u32>(a: V128, value: i32) -> V128 {
    const {
        assert!(LANE < 8);
    }
    unsafe {
        match LANE {
            0 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i16x8_replace_lane_0(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            1 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i16x8_replace_lane_1(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            2 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i16x8_replace_lane_2(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            3 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i16x8_replace_lane_3(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            4 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i16x8_replace_lane_4(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            5 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i16x8_replace_lane_5(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            6 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i16x8_replace_lane_6(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            7 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i16x8_replace_lane_7(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_eq(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.eq`
///
/// Lane-wise `a == b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i16x8_eq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_eq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_ne(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.ne`
///
/// Lane-wise `a != b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i16x8_ne(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_ne(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_lt_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.lt_s`
///
/// Lane-wise signed `a < b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i16x8_lt_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_lt_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_lt_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.lt_u`
///
/// Lane-wise unsigned `a < b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i16x8_lt_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_lt_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_gt_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.gt_s`
///
/// Lane-wise signed `a > b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i16x8_gt_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_gt_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_gt_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.gt_u`
///
/// Lane-wise unsigned `a > b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i16x8_gt_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_gt_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_le_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.le_s`
///
/// Lane-wise signed `a <= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i16x8_le_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_le_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_le_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.le_u`
///
/// Lane-wise unsigned `a <= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i16x8_le_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_le_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_ge_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.ge_s`
///
/// Lane-wise signed `a >= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i16x8_ge_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_ge_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_ge_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.ge_u`
///
/// Lane-wise unsigned `a >= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i16x8_ge_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_ge_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_abs(a: *const u8, out: *mut u8);
}
/// `i16x8.abs`
///
/// Lane-wise absolute value. The most negative signed value stays unchanged.
#[inline(always)]
pub fn i16x8_abs(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_abs(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_neg(a: *const u8, out: *mut u8);
}
/// `i16x8.neg`
///
/// Lane-wise negation. The most negative signed value wraps and stays unchanged.
#[inline(always)]
pub fn i16x8_neg(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_neg(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_all_true(a: *const u8) -> i32;
}
/// `i16x8.all_true`
///
/// Return `1` when every lane is non-zero, otherwise `0`.
#[inline(always)]
pub fn i16x8_all_true(a: V128) -> i32 {
    unsafe { rust_isa_simd128_i16x8_all_true(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_bitmask(a: *const u8) -> i32;
}
/// `i16x8.bitmask`
///
/// Pack each lane's sign bit into an `i32`, with the low lane in the low bit.
#[inline(always)]
pub fn i16x8_bitmask(a: V128) -> i32 {
    unsafe { rust_isa_simd128_i16x8_bitmask(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_shl(a: *const u8, count: u32, out: *mut u8);
}
/// `i16x8.shl`
///
/// Shift each lane left. `count` is taken modulo the number of bits in the lane.
#[inline(always)]
pub fn i16x8_shl(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_shl(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_shr_s(a: *const u8, count: u32, out: *mut u8);
}
/// `i16x8.shr_s`
///
/// Arithmetic right shift of each lane. `count` is taken modulo the number of bits in the lane.
#[inline(always)]
pub fn i16x8_shr_s(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_shr_s(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_shr_u(a: *const u8, count: u32, out: *mut u8);
}
/// `i16x8.shr_u`
///
/// Logical right shift of each lane. `count` is taken modulo the number of bits in the lane.
#[inline(always)]
pub fn i16x8_shr_u(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_shr_u(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_add(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.add`
///
/// Lane-wise wrapping addition.
#[inline(always)]
pub fn i16x8_add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_add_sat_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.add_sat_s`
///
/// Lane-wise signed saturating addition.
#[inline(always)]
pub fn i16x8_add_sat_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_add_sat_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_add_sat_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.add_sat_u`
///
/// Lane-wise unsigned saturating addition.
#[inline(always)]
pub fn i16x8_add_sat_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_add_sat_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_sub(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.sub`
///
/// Lane-wise wrapping subtraction.
#[inline(always)]
pub fn i16x8_sub(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_sub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_sub_sat_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.sub_sat_s`
///
/// Lane-wise signed saturating subtraction.
#[inline(always)]
pub fn i16x8_sub_sat_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_sub_sat_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_sub_sat_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.sub_sat_u`
///
/// Lane-wise unsigned saturating subtraction.
#[inline(always)]
pub fn i16x8_sub_sat_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_sub_sat_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_mul(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.mul`
///
/// Lane-wise wrapping multiplication.
#[inline(always)]
pub fn i16x8_mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_min_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.min_s`
///
/// Lane-wise signed minimum.
#[inline(always)]
pub fn i16x8_min_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_min_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_min_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.min_u`
///
/// Lane-wise unsigned minimum.
#[inline(always)]
pub fn i16x8_min_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_min_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_max_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.max_s`
///
/// Lane-wise signed maximum.
#[inline(always)]
pub fn i16x8_max_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_max_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_max_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.max_u`
///
/// Lane-wise unsigned maximum.
#[inline(always)]
pub fn i16x8_max_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_max_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_avgr_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.avgr_u`
///
/// Lane-wise unsigned average `(a + b + 1) / 2`.
#[inline(always)]
pub fn i16x8_avgr_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_avgr_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_narrow_i32x4_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.narrow_i32x4_s`
///
/// Signed saturating narrow. `a` supplies the low lanes of the result and `b` the high lanes.
#[inline(always)]
pub fn i16x8_narrow_i32x4_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_narrow_i32x4_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_narrow_i32x4_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.narrow_i32x4_u`
///
/// Unsigned saturating narrow. Inputs below `0` become `0`, and inputs above the unsigned maximum saturate. `a` supplies the low lanes of the result and `b` the high lanes.
#[inline(always)]
pub fn i16x8_narrow_i32x4_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_narrow_i32x4_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extend_low_i8x16_s(a: *const u8, out: *mut u8);
}
/// `i16x8.extend_low_i8x16_s`
///
/// Sign-extend the low half of the lanes.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i16x8_extend_low_i8x16_s, i16x8_extract_lane_s, V128};
///
/// let source = V128::from_bytes([0xff, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let extended = i16x8_extend_low_i8x16_s(source);
/// assert_eq!(i16x8_extract_lane_s::<0>(extended), -1);
/// assert_eq!(i16x8_extract_lane_s::<1>(extended), 1);
/// ```
#[inline(always)]
pub fn i16x8_extend_low_i8x16_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_extend_low_i8x16_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extend_high_i8x16_s(a: *const u8, out: *mut u8);
}
/// `i16x8.extend_high_i8x16_s`
///
/// Sign-extend the high half of the lanes.
#[inline(always)]
pub fn i16x8_extend_high_i8x16_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_extend_high_i8x16_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extend_low_i8x16_u(a: *const u8, out: *mut u8);
}
/// `i16x8.extend_low_i8x16_u`
///
/// Zero-extend the low half of the lanes.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i16x8_extend_low_i8x16_u, i16x8_extract_lane_s, V128};
///
/// let source = V128::from_bytes([0xff, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// assert_eq!(i16x8_extract_lane_s::<0>(i16x8_extend_low_i8x16_u(source)), 255);
/// ```
#[inline(always)]
pub fn i16x8_extend_low_i8x16_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_extend_low_i8x16_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extend_high_i8x16_u(a: *const u8, out: *mut u8);
}
/// `i16x8.extend_high_i8x16_u`
///
/// Zero-extend the high half of the lanes.
#[inline(always)]
pub fn i16x8_extend_high_i8x16_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_extend_high_i8x16_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extadd_pairwise_i8x16_s(a: *const u8, out: *mut u8);
}
/// `i16x8.extadd_pairwise_i8x16_s`
///
/// Sign-extend each lane, then add each adjacent pair.
#[inline(always)]
pub fn i16x8_extadd_pairwise_i8x16_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_extadd_pairwise_i8x16_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extadd_pairwise_i8x16_u(a: *const u8, out: *mut u8);
}
/// `i16x8.extadd_pairwise_i8x16_u`
///
/// Zero-extend each lane, then add each adjacent pair.
#[inline(always)]
pub fn i16x8_extadd_pairwise_i8x16_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_extadd_pairwise_i8x16_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extmul_low_i8x16_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.extmul_low_i8x16_s`
///
/// Sign-extend the low half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i16x8_extmul_low_i8x16_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_extmul_low_i8x16_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extmul_high_i8x16_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.extmul_high_i8x16_s`
///
/// Sign-extend the high half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i16x8_extmul_high_i8x16_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_extmul_high_i8x16_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extmul_low_i8x16_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.extmul_low_i8x16_u`
///
/// Zero-extend the low half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i16x8_extmul_low_i8x16_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_extmul_low_i8x16_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_extmul_high_i8x16_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.extmul_high_i8x16_u`
///
/// Zero-extend the high half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i16x8_extmul_high_i8x16_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_extmul_high_i8x16_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i16x8_q15mulr_sat_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.q15mulr_sat_s`
///
/// Lane-wise saturating Q15 multiply-round. The `i16` lanes are multiplied in `i32`, then `(product + 0x4000) >> 15` is saturated to `i16`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i16x8_extract_lane_s, i16x8_q15mulr_sat_s, V128};
///
/// let value = V128::from_bytes([0x00, 0x40, 0x00, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let product = i16x8_q15mulr_sat_s(value, value);
/// assert_eq!(i16x8_extract_lane_s::<0>(product), 0x2000);
/// assert_eq!(i16x8_extract_lane_s::<1>(product), 32767);
/// ```
#[inline(always)]
pub fn i16x8_q15mulr_sat_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i16x8_q15mulr_sat_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
