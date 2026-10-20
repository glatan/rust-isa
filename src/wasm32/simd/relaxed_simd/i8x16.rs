//! `i8x16.relaxed_*` instructions.
//!
//! Rules shared by Relaxed SIMD are documented on the [parent module](super).

use super::V128;

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i8x16_relaxed_swizzle(a: *const u8, b: *const u8, out: *mut u8);
}
/// `i8x16.relaxed_swizzle`
///
/// Lane `i` is `a[s[i]]` when `s[i] < 16`. A negative `s[i]` produces `0`. Any other index produces either `0` or `a[s[i] modulo 16]`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::i8x16_relaxed_swizzle;
/// use rust_isa::wasm32::simd::simd128::V128;
///
/// let source = V128::from_bytes([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let indices = V128::from_bytes([15, 0, 2, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// let out = i8x16_relaxed_swizzle(source, indices).to_bytes();
/// assert_eq!(out[0], 15);
/// assert_eq!(out[1], 0);
/// assert_eq!(out[2], 2);
/// assert_eq!(out[3], 0);
/// ```
#[inline(always)]
pub fn i8x16_relaxed_swizzle(a: V128, s: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i8x16_relaxed_swizzle(a.as_ptr(), s.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i8x16_relaxed_laneselect(
        a: *const u8,
        b: *const u8,
        c: *const u8,
        out: *mut u8,
    );
}
/// `i8x16.relaxed_laneselect`
///
/// Select bits from `a` and `b`. The mask is either `c`, or each byte of `c` expanded from its sign bit to `0x00` or `0xff`. The bits then follow `v128.bitselect`: a `1` selects `a` and a `0` selects `b`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::relaxed_simd::i8x16_relaxed_laneselect;
/// use rust_isa::wasm32::simd::simd128::V128;
///
/// let a = V128::from_bytes([0xff; 16]);
/// let b = V128::from_bytes([0x00; 16]);
/// let mask = V128::from_bytes([0xff; 16]);
/// assert_eq!(i8x16_relaxed_laneselect(a, b, mask).to_bytes(), [0xff; 16]);
/// ```
#[inline(always)]
pub fn i8x16_relaxed_laneselect(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i8x16_relaxed_laneselect(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}
