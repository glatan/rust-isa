//! AArch64 I8MM.
//!
//! I8MM is not part of the AArch64 baseline. Every operation is `unsafe` and
//! the caller must enable I8MM with `#[target_feature(enable = "i8mm")]` or a
//! global target feature.
//!
//! The values are [`V128`]. The matrix multiply-accumulate instructions
//! interpret the byte operands as an 8x2 by 2x8 matrix product and accumulate
//! into four 32-bit lanes.

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_i8mm_vmmlaq_s32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_i8mm_vusmmlaq_s32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `smmla`
///
/// Multiply signed bytes and accumulate into 32-bit lanes.
///
/// # Safety
///
/// The caller must enable I8MM.
#[target_feature(enable = "i8mm")]
#[inline]
pub unsafe fn smmla(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_i8mm_vmmlaq_s32(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usmmla`
///
/// Multiply unsigned bytes in `a` by signed bytes in `b` and accumulate into
/// 32-bit lanes.
///
/// # Safety
///
/// The caller must enable I8MM.
#[target_feature(enable = "i8mm")]
#[inline]
pub unsafe fn usmmla(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_i8mm_vusmmlaq_s32(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
