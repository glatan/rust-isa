//! 32-bit ARM FP16.
//!
//! FP16 is unstable on this target. Every operation is `unsafe` and the caller
//! must compile the whole crate with the FP16 feature enabled.
//!
//! The values are [`V128`] of IEEE 754 binary16 lanes.

use super::neon::V128;

unsafe extern "C" {
    fn rust_isa_arm_fp16_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vadd.f16`
///
/// Add binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16 on the CPU and in the target features.
#[inline(always)]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_fp16_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmul.f16`
///
/// Multiply binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16 on the CPU and in the target features.
#[inline(always)]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_fp16_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
