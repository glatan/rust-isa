//! Sum of absolute differences instructions.
//!
//! # Compile-time immediate check
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx::{vmpsadbw, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vmpsadbw::<8>(v, v) };
//! ```

use super::V256;
use crate::x86::simd::sse2::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx_vpsadbw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsadbw`
///
/// Sum the absolute differences of each 8-byte block into a 64-bit lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vpsadbw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx_vpsadbw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmpsadbw`
///
/// Compute multiple packed sums of absolute differences with the immediate.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmpsadbw<const IMM: u8>(a: V256, b: V256) -> V256 {
    const { assert!(IMM <= 7, "vmpsadbw immediate must be in 0..=7") };
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmpsadbw ymm0, ymm0, [{b}], {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            out = in(reg) out.as_mut_ptr(),
            imm = const IMM,
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}
