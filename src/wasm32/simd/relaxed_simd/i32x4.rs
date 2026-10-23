//! `i32x4.relaxed_*` instructions.
//!
//! Rules shared by Relaxed SIMD are documented on the [parent module](super).

use super::V128;

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i32x4_relaxed_trunc_f32x4_s(a: *const u8, out: *mut u8);
}
/// `i32x4.relaxed_trunc_f32x4_s`
///
/// When truncation toward zero fits in `i32`, that integer is the result. Otherwise the lane is either the saturating truncation or another integer chosen by the engine.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::i32x4_relaxed_trunc_f32x4_s;
/// use rust_isa::wasm32::simd::simd128::{i32x4_extract_lane, V128};
///
/// let mut bytes = [0u8; 16];
/// bytes[0..4].copy_from_slice(&1.9f32.to_le_bytes());
/// bytes[4..8].copy_from_slice(&(-1.9f32).to_le_bytes());
/// let lanes = i32x4_relaxed_trunc_f32x4_s(V128::from_bytes(bytes));
/// assert_eq!(i32x4_extract_lane::<0>(lanes), 1);
/// assert_eq!(i32x4_extract_lane::<1>(lanes), -1);
/// ```
#[inline(always)]
pub fn i32x4_relaxed_trunc_f32x4_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i32x4_relaxed_trunc_f32x4_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i32x4_relaxed_trunc_f32x4_u(a: *const u8, out: *mut u8);
}
/// `i32x4.relaxed_trunc_f32x4_u`
///
/// When truncation toward zero fits in `u32`, that integer is the result. Otherwise the lane is either the saturating truncation or another integer chosen by the engine.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::i32x4_relaxed_trunc_f32x4_u;
/// use rust_isa::wasm32::simd::simd128::{i32x4_extract_lane, V128};
///
/// let mut bytes = [0u8; 16];
/// bytes[0..4].copy_from_slice(&1.9f32.to_le_bytes());
/// let lanes = i32x4_relaxed_trunc_f32x4_u(V128::from_bytes(bytes));
/// assert_eq!(i32x4_extract_lane::<0>(lanes), 1);
/// ```
#[inline(always)]
pub fn i32x4_relaxed_trunc_f32x4_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i32x4_relaxed_trunc_f32x4_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i32x4_relaxed_trunc_f64x2_s_zero(a: *const u8, out: *mut u8);
}
/// `i32x4.relaxed_trunc_f64x2_s_zero`
///
/// Truncate the two `f64` lanes toward zero into the low `i32` lanes when the values fit in `i32`, and set the upper two lanes to `0`. A lane that does not fit is either the saturating truncation or another integer chosen by the engine.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::i32x4_relaxed_trunc_f64x2_s_zero;
/// use rust_isa::wasm32::simd::simd128::{i32x4_extract_lane, V128};
///
/// let mut bytes = [0u8; 16];
/// bytes[..8].copy_from_slice(&1.9f64.to_le_bytes());
/// bytes[8..].copy_from_slice(&(-1.9f64).to_le_bytes());
/// let lanes = i32x4_relaxed_trunc_f64x2_s_zero(V128::from_bytes(bytes));
/// assert_eq!(i32x4_extract_lane::<0>(lanes), 1);
/// assert_eq!(i32x4_extract_lane::<1>(lanes), -1);
/// assert_eq!(i32x4_extract_lane::<2>(lanes), 0);
/// ```
#[inline(always)]
pub fn i32x4_relaxed_trunc_f64x2_s_zero(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i32x4_relaxed_trunc_f64x2_s_zero(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i32x4_relaxed_trunc_f64x2_u_zero(a: *const u8, out: *mut u8);
}
/// `i32x4.relaxed_trunc_f64x2_u_zero`
///
/// Truncate the two `f64` lanes toward zero into the low `i32` lanes when the values fit in `u32`, and set the upper two lanes to `0`. A lane that does not fit is either the saturating truncation or another integer chosen by the engine.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::i32x4_relaxed_trunc_f64x2_u_zero;
/// use rust_isa::wasm32::simd::simd128::{i32x4_extract_lane, V128};
///
/// let mut bytes = [0u8; 16];
/// bytes[8..].copy_from_slice(&4.2f64.to_le_bytes());
/// let lanes = i32x4_relaxed_trunc_f64x2_u_zero(V128::from_bytes(bytes));
/// assert_eq!(i32x4_extract_lane::<0>(lanes), 0);
/// assert_eq!(i32x4_extract_lane::<1>(lanes), 4);
/// assert_eq!(i32x4_extract_lane::<2>(lanes), 0);
/// ```
#[inline(always)]
pub fn i32x4_relaxed_trunc_f64x2_u_zero(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i32x4_relaxed_trunc_f64x2_u_zero(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i32x4_relaxed_dot_i8x16_i7x16_add_s(
        a: *const u8,
        b: *const u8,
        c: *const u8,
        out: *mut u8,
    );
}
/// `i32x4.relaxed_dot_i8x16_i7x16_add_s`
///
/// [`i16x8_relaxed_dot_i8x16_i7x16_s`](super::i16x8_relaxed_dot_i8x16_i7x16_s), then each adjacent pair of those `i16` lanes is sign-extended and added into an `i32`, then added to `c`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::i32x4_relaxed_dot_i8x16_i7x16_add_s;
/// use rust_isa::wasm32::simd::simd128::{i32x4_extract_lane, V128};
///
/// let a = V128::from_bytes([1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let b = V128::from_bytes([5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let c = V128::from_bytes([0; 16]);
/// let dot = i32x4_relaxed_dot_i8x16_i7x16_add_s(a, b, c);
/// assert_eq!(i32x4_extract_lane::<0>(dot), 70);
/// ```
#[inline(always)]
pub fn i32x4_relaxed_dot_i8x16_i7x16_add_s(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i32x4_relaxed_dot_i8x16_i7x16_add_s(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i32x4_relaxed_laneselect(
        a: *const u8,
        b: *const u8,
        c: *const u8,
        out: *mut u8,
    );
}
/// `i32x4.relaxed_laneselect`
///
/// Select bits from `a` and `b`. The mask is either `c`, or each `i32` lane of `c` expanded from its sign bit to all bits one or all bits zero. The bits then follow `v128.bitselect`: a `1` selects `a` and a `0` selects `b`.
#[inline(always)]
pub fn i32x4_relaxed_laneselect(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i32x4_relaxed_laneselect(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}
