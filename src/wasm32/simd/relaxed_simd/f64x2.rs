//! `f64x2.relaxed_*` instructions.
//!
//! Rules shared by Relaxed SIMD are documented on the [parent module](super).

use super::V128;

unsafe extern "C" {
    fn rust_isa_relaxed_simd_f64x2_relaxed_min(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.relaxed_min`
///
/// Lane-wise minimum. When neither value is NaN and the values are not zeros of opposite sign, the result matches `f64x2.min`. Otherwise the lane is one of the results allowed by `frelaxed_min`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::f64x2_relaxed_min;
/// use rust_isa::wasm32::simd::simd128::{f64x2_extract_lane, f64x2_splat};
///
/// let min = f64x2_relaxed_min(f64x2_splat(1.0), f64x2_splat(3.0));
/// assert_eq!(f64x2_extract_lane::<0>(min), 1.0);
/// ```
#[inline(always)]
pub fn f64x2_relaxed_min(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_f64x2_relaxed_min(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_relaxed_simd_f64x2_relaxed_max(a: *const u8, b: *const u8, out: *mut u8);
}
/// `f64x2.relaxed_max`
///
/// Lane-wise maximum. When neither value is NaN and the values are not zeros of opposite sign, the result matches `f64x2.max`. Otherwise the lane is one of the results allowed by `frelaxed_max`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::f64x2_relaxed_max;
/// use rust_isa::wasm32::simd::simd128::{f64x2_extract_lane, f64x2_splat};
///
/// let max = f64x2_relaxed_max(f64x2_splat(1.0), f64x2_splat(3.0));
/// assert_eq!(f64x2_extract_lane::<0>(max), 3.0);
/// ```
#[inline(always)]
pub fn f64x2_relaxed_max(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_f64x2_relaxed_max(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_relaxed_simd_f64x2_relaxed_madd(
        a: *const u8,
        b: *const u8,
        c: *const u8,
        out: *mut u8,
    );
}
/// `f64x2.relaxed_madd`
///
/// `a * b + c`, either as a fused multiply-add or as a multiplication followed by an addition.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::f64x2_relaxed_madd;
/// use rust_isa::wasm32::simd::simd128::{f64x2_extract_lane, f64x2_splat};
///
/// let product = f64x2_relaxed_madd(f64x2_splat(2.0), f64x2_splat(3.0), f64x2_splat(4.0));
/// assert_eq!(f64x2_extract_lane::<0>(product), 10.0);
/// ```
#[inline(always)]
pub fn f64x2_relaxed_madd(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_f64x2_relaxed_madd(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_relaxed_simd_f64x2_relaxed_nmadd(
        a: *const u8,
        b: *const u8,
        c: *const u8,
        out: *mut u8,
    );
}
/// `f64x2.relaxed_nmadd`
///
/// [`f64x2_relaxed_madd`] of `-a`, `b`, and `c`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::f64x2_relaxed_nmadd;
/// use rust_isa::wasm32::simd::simd128::{f64x2_extract_lane, f64x2_splat};
///
/// let product = f64x2_relaxed_nmadd(f64x2_splat(2.0), f64x2_splat(3.0), f64x2_splat(4.0));
/// assert_eq!(f64x2_extract_lane::<0>(product), -2.0);
/// ```
#[inline(always)]
pub fn f64x2_relaxed_nmadd(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_f64x2_relaxed_nmadd(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}
