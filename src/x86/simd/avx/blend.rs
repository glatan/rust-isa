//! Blend instructions.
//!
//! # Compile-time immediate check
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx::{vblendpd, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vblendpd::<16>(v, v) };
//! ```

use super::V256;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx_vblendvps(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_avx_vblendvpd(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
}

/// `vblendvps`
///
/// Select each 32-bit float lane from `a` or `b` using the sign bit of the
/// corresponding lane in `mask`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vblendvps(a: V256, b: V256, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vblendvps(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vblendvpd`
///
/// Select each 64-bit float lane from `a` or `b` using the sign bit of the
/// corresponding lane in `mask`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vblendvpd(a: V256, b: V256, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vblendvpd(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vblendps`
///
/// Select the eight 32-bit float lanes from `a` and `b` with the immediate
/// mask.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vblendps<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vblendps ymm0, ymm0, [{b}], {imm}",
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

/// `vblendpd`
///
/// Select the four 64-bit float lanes from `a` and `b` with the immediate
/// mask.
///
/// # Safety
///
/// The caller must enable AVX. `IMM` must be in `0..=15`.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vblendpd<const IMM: u8>(a: V256, b: V256) -> V256 {
    const { assert!(IMM <= 15, "vblendpd immediate must be in 0..=15") };
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vblendpd ymm0, ymm0, [{b}], {imm}",
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
