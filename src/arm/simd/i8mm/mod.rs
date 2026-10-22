//! 32-bit ARM I8MM.
//!
//! I8MM is unstable on this target. Every operation is `unsafe` and the caller
//! must compile the whole crate with the I8MM feature enabled.
//!
//! The values are [`V128`]. The matrix multiply-accumulate instructions
//! interpret the byte operands as an 8x2 by 2x8 matrix product.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

use super::neon::V128;

unsafe extern "C" {
    fn rust_isa_arm_i8mm_smmla(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_i8mm_usmmla(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_i8mm_vummla(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `smmla`
///
/// Multiply signed bytes and accumulate into 32-bit lanes.
///
/// # Safety
///
/// The caller must enable I8MM on the CPU and in the target features.
#[inline(always)]
pub unsafe fn smmla(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_i8mm_smmla(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vummla`
///
/// Multiply unsigned bytes and accumulate into unsigned 32-bit lanes.
///
/// # Safety
///
/// The caller must enable I8MM on the CPU and in the target features.
#[inline(always)]
pub unsafe fn vummla(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_i8mm_vummla(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usmmla`
///
/// Multiply unsigned bytes in `a` by signed bytes in `b` and accumulate.
///
/// # Safety
///
/// The caller must enable I8MM on the CPU and in the target features.
#[inline(always)]
pub unsafe fn usmmla(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_i8mm_usmmla(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
