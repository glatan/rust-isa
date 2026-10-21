//! AArch64 JavaScript conversion.

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_jsconv_fjcvtzs(value: f64) -> i32;
}

/// `fjcvtzs`
///
/// Convert a double-precision value to a 32-bit integer with JavaScript
/// `ToInt32` semantics: NaN and infinities become zero and the result wraps
/// modulo 2^32.
///
/// # Safety
///
/// The caller must enable JSCONV.
#[target_feature(enable = "jsconv")]
#[inline]
pub unsafe fn fjcvtzs(value: f64) -> i32 {
    unsafe { rust_isa_jsconv_fjcvtzs(value) }
}

/// `fjcvtzs` applied to the low lane of a vector.
///
/// # Safety
///
/// The caller must enable JSCONV.
#[target_feature(enable = "jsconv")]
#[inline]
pub unsafe fn fjcvtzs_v128(value: V128) -> i32 {
    let bytes = value.to_bytes();
    unsafe { rust_isa_jsconv_fjcvtzs(f64::from_le_bytes(bytes[..8].try_into().unwrap())) }
}
