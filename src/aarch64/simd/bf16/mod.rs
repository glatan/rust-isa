//! AArch64 BF16.
//!
//! BF16 is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable BF16 with `#[target_feature(enable = "bf16")]` or a
//! global target feature.
//!
//! The values are [`V128`]. The dot-product instructions interpret pairs of
//! bfloat16 lanes and accumulate into 32-bit float lanes.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_bf16_vbfdotq_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_bf16_vbfmmlaq_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `bfdot`
///
/// Multiply pairs of bfloat16 lanes and accumulate into the 32-bit float lanes
/// of `acc`.
///
/// # Safety
///
/// The caller must enable BF16.
#[target_feature(enable = "bf16")]
#[inline]
pub unsafe fn bfdot(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_bf16_vbfdotq_f32(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `bfmmla`
///
/// Perform the bfloat16 matrix multiply-accumulate into the 32-bit float lanes
/// of `acc`.
///
/// # Safety
///
/// The caller must enable BF16.
#[target_feature(enable = "bf16")]
#[inline]
pub unsafe fn bfmmla(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_bf16_vbfmmlaq_f32(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
