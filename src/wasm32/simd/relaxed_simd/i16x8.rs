//! `i16x8.relaxed_*` instructions.
//!
//! Rules shared by Relaxed SIMD are documented on the [parent module](super).

use super::V128;

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i16x8_relaxed_q15mulr_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i16x8.relaxed_q15mulr_s`
///
/// Lane-wise Q15 multiply-round. The `i16` lanes are multiplied in `i32`, then `(product + 0x4000) >> 15` is saturated to `i16`. A product of two lanes equal to `i16::MIN` may instead stay `i16::MIN`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::i16x8_relaxed_q15mulr_s;
/// use rust_isa::wasm32::simd::simd128::{i16x8_extract_lane_s, V128};
///
/// let value = V128::from_bytes([0x00, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let product = i16x8_relaxed_q15mulr_s(value, value);
/// assert_eq!(i16x8_extract_lane_s::<0>(product), 0x2000);
/// ```
#[inline(always)]
pub fn i16x8_relaxed_q15mulr_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i16x8_relaxed_q15mulr_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i16x8_relaxed_dot_i8x16_i7x16_s(
        a: *const u8,
        b: *const u8,
        out: *mut u8,
    );
}
/// `i16x8.relaxed_dot_i8x16_i7x16_s`
///
/// Dot product of adjacent `i8` pairs into `i16`. `a` is sign-extended. `b` is either sign-extended or zero-extended. Each pair is then added either with wrapping or with saturation. One engine choice fixes both the extension of `b` and whether that addition saturates.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::i16x8_relaxed_dot_i8x16_i7x16_s;
/// use rust_isa::wasm32::simd::simd128::{i16x8_extract_lane_s, V128};
///
/// let a = V128::from_bytes([1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let b = V128::from_bytes([5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// assert_eq!(i16x8_extract_lane_s::<0>(i16x8_relaxed_dot_i8x16_i7x16_s(a, b)), 17);
/// ```
#[inline(always)]
pub fn i16x8_relaxed_dot_i8x16_i7x16_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i16x8_relaxed_dot_i8x16_i7x16_s(
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i16x8_relaxed_laneselect(
        a: *const u8,
        b: *const u8,
        c: *const u8,
        out: *mut u8,
    );
}
/// `i16x8.relaxed_laneselect`
///
/// Select bits from `a` and `b`. The mask is either `c`, or each `i16` lane of `c` expanded from its sign bit to all bits one or all bits zero. The bits then follow `v128.bitselect`: a `1` selects `a` and a `0` selects `b`.
#[inline(always)]
pub fn i16x8_relaxed_laneselect(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i16x8_relaxed_laneselect(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}
