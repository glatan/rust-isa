//! AArch64 SM4 instructions.
//!
//! SM4 is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable SM4 with `#[target_feature(enable = "sm4")]` or a global
//! target feature.
//!
//! The values are [`V128`]. The key-schedule and round instructions follow the
//! AArch64 SM4 instruction definitions.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_sm4_vsm4eq_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sm4_vsm4ekeyq_u32(a: *const u8, b: *const u8, out: *mut u8);
}

/// `sm4e`
///
/// Perform four SM4 encryption rounds.
///
/// # Safety
///
/// The caller must enable SM4.
#[target_feature(enable = "sm4")]
#[inline]
pub unsafe fn sm4e(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sm4_vsm4eq_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `sm4ekey`
///
/// Perform four SM4 key-schedule rounds.
///
/// # Safety
///
/// The caller must enable SM4.
#[target_feature(enable = "sm4")]
#[inline]
pub unsafe fn sm4ekey(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sm4_vsm4ekeyq_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
