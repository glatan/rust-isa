//! `i64x2.*` instructions.
//!
//! Rules shared by every SIMD128 instruction are documented on the [parent module](super).

use super::V128;

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_splat(value: i64, out: *mut u8);
}
/// `i64x2.splat`
///
/// Replicate `value` into both lanes.
#[inline(always)]
pub fn i64x2_splat(value: i64) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_splat(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extract_lane_s_0(a: *const u8) -> i64;
}
unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extract_lane_s_1(a: *const u8) -> i64;
}
/// `i64x2.extract_lane_s`
///
/// Return the lane bits as `i64`.
///
/// `LANE` is an immediate in `0..2`. A constant outside that range does not compile.
#[inline(always)]
pub fn i64x2_extract_lane_s<const LANE: u32>(a: V128) -> i64 {
    const {
        assert!(LANE < 2);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_i64x2_extract_lane_s_0(a.as_ptr()),
            1 => rust_isa_simd128_i64x2_extract_lane_s_1(a.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extract_lane_u_0(a: *const u8) -> u64;
}
unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extract_lane_u_1(a: *const u8) -> u64;
}
/// `i64x2.extract_lane_u`
///
/// Return the lane bits as `u64`.
///
/// `LANE` is an immediate in `0..2`. A constant outside that range does not compile.
#[inline(always)]
pub fn i64x2_extract_lane_u<const LANE: u32>(a: V128) -> u64 {
    const {
        assert!(LANE < 2);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_i64x2_extract_lane_u_0(a.as_ptr()),
            1 => rust_isa_simd128_i64x2_extract_lane_u_1(a.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_replace_lane_0(a: *const u8, value: i64, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_i64x2_replace_lane_1(a: *const u8, value: i64, out: *mut u8);
}
/// `i64x2.replace_lane`
///
/// Replace the lane with `value`.
///
/// `LANE` is an immediate in `0..2`. A constant outside that range does not compile.
#[inline(always)]
pub fn i64x2_replace_lane<const LANE: u32>(a: V128, value: i64) -> V128 {
    const {
        assert!(LANE < 2);
    }
    unsafe {
        match LANE {
            0 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i64x2_replace_lane_0(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            1 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_i64x2_replace_lane_1(a.as_ptr(), value, out.as_mut_ptr());
                out
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_eq(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.eq`
///
/// Lane-wise `a == b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i64x2_eq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_eq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_ne(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.ne`
///
/// Lane-wise `a != b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i64x2_ne(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_ne(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_lt_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.lt_s`
///
/// Lane-wise signed `a < b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i64x2_lt_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_lt_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_gt_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.gt_s`
///
/// Lane-wise signed `a > b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i64x2_gt_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_gt_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_le_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.le_s`
///
/// Lane-wise signed `a <= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i64x2_le_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_le_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_ge_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.ge_s`
///
/// Lane-wise signed `a >= b`. A true lane is all bits one. A false lane is all bits zero.
#[inline(always)]
pub fn i64x2_ge_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_ge_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_abs(a: *const u8, out: *mut u8);
}
/// `i64x2.abs`
///
/// Lane-wise absolute value. The most negative signed value stays unchanged.
#[inline(always)]
pub fn i64x2_abs(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_abs(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_neg(a: *const u8, out: *mut u8);
}
/// `i64x2.neg`
///
/// Lane-wise negation. The most negative signed value wraps and stays unchanged.
#[inline(always)]
pub fn i64x2_neg(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_neg(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_all_true(a: *const u8) -> i32;
}
/// `i64x2.all_true`
///
/// Return `1` when every lane is non-zero, otherwise `0`.
#[inline(always)]
pub fn i64x2_all_true(a: V128) -> i32 {
    unsafe { rust_isa_simd128_i64x2_all_true(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_bitmask(a: *const u8) -> i32;
}
/// `i64x2.bitmask`
///
/// Pack each lane's sign bit into an `i32`, with the low lane in the low bit.
#[inline(always)]
pub fn i64x2_bitmask(a: V128) -> i32 {
    unsafe { rust_isa_simd128_i64x2_bitmask(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_shl(a: *const u8, count: u32, out: *mut u8);
}
/// `i64x2.shl`
///
/// Shift each lane left. `count` is taken modulo the number of bits in the lane.
#[inline(always)]
pub fn i64x2_shl(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_shl(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_shr_s(a: *const u8, count: u32, out: *mut u8);
}
/// `i64x2.shr_s`
///
/// Arithmetic right shift of each lane. `count` is taken modulo the number of bits in the lane.
#[inline(always)]
pub fn i64x2_shr_s(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_shr_s(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_shr_u(a: *const u8, count: u32, out: *mut u8);
}
/// `i64x2.shr_u`
///
/// Logical right shift of each lane. `count` is taken modulo the number of bits in the lane.
#[inline(always)]
pub fn i64x2_shr_u(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_shr_u(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_add(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.add`
///
/// Lane-wise wrapping addition.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i64x2_add, i64x2_extract_lane_s, i64x2_splat};
///
/// assert_eq!(
///     i64x2_extract_lane_s::<0>(i64x2_add(i64x2_splat(i64::MAX), i64x2_splat(1))),
///     i64::MIN,
/// );
/// ```
#[inline(always)]
pub fn i64x2_add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_sub(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.sub`
///
/// Lane-wise wrapping subtraction.
#[inline(always)]
pub fn i64x2_sub(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_sub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_mul(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.mul`
///
/// Lane-wise wrapping multiplication.
#[inline(always)]
pub fn i64x2_mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extend_low_i32x4_s(a: *const u8, out: *mut u8);
}
/// `i64x2.extend_low_i32x4_s`
///
/// Sign-extend the low half of the lanes.
#[inline(always)]
pub fn i64x2_extend_low_i32x4_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_extend_low_i32x4_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extend_high_i32x4_s(a: *const u8, out: *mut u8);
}
/// `i64x2.extend_high_i32x4_s`
///
/// Sign-extend the high half of the lanes.
#[inline(always)]
pub fn i64x2_extend_high_i32x4_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_extend_high_i32x4_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extend_low_i32x4_u(a: *const u8, out: *mut u8);
}
/// `i64x2.extend_low_i32x4_u`
///
/// Zero-extend the low half of the lanes.
#[inline(always)]
pub fn i64x2_extend_low_i32x4_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_extend_low_i32x4_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extend_high_i32x4_u(a: *const u8, out: *mut u8);
}
/// `i64x2.extend_high_i32x4_u`
///
/// Zero-extend the high half of the lanes.
#[inline(always)]
pub fn i64x2_extend_high_i32x4_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_extend_high_i32x4_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extmul_low_i32x4_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.extmul_low_i32x4_s`
///
/// Sign-extend the low half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i64x2_extmul_low_i32x4_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_extmul_low_i32x4_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extmul_high_i32x4_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.extmul_high_i32x4_s`
///
/// Sign-extend the high half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i64x2_extmul_high_i32x4_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_extmul_high_i32x4_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extmul_low_i32x4_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.extmul_low_i32x4_u`
///
/// Zero-extend the low half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i64x2_extmul_low_i32x4_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_extmul_low_i32x4_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_i64x2_extmul_high_i32x4_u(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i64x2.extmul_high_i32x4_u`
///
/// Zero-extend the high half of the lanes, then multiply each pair.
#[inline(always)]
pub fn i64x2_extmul_high_i32x4_u(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_i64x2_extmul_high_i32x4_u(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
