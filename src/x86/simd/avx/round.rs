//! Round instructions.

use super::V256;
use crate::x86::simd::sse2::V128;
use core::arch::asm;

/// `vroundps`
///
/// Round the eight 32-bit float lanes. The low two bits of `IMM` select the
/// rounding mode.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vroundps<const IMM: u8>(a: V256) -> V256 {
    const { assert!(IMM <= 0x0f, "vroundps immediate must be in 0..=15") };
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vroundps ymm0, [{a}], {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            out = in(reg) out.as_mut_ptr(),
            imm = const IMM,
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vroundpd`
///
/// Round the four 64-bit float lanes. The low two bits of `IMM` select the
/// rounding mode.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vroundpd<const IMM: u8>(a: V256) -> V256 {
    const { assert!(IMM <= 0x03, "vroundpd immediate must be in 0..=3") };
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vroundpd ymm0, [{a}], {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            out = in(reg) out.as_mut_ptr(),
            imm = const IMM,
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vroundss`
///
/// Round the low 32-bit float lane of `b` and preserve the upper lanes of `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vroundss<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 0x0f, "vroundss immediate must be in 0..=15") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vroundss xmm0, xmm0, [{b}], {imm}",
            "vmovdqu [{out}], xmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            out = in(reg) out.as_mut_ptr(),
            imm = const IMM,
            out("xmm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vroundsd`
///
/// Round the low 64-bit float lane of `b` and preserve the upper lane of `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vroundsd<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 0x0f, "vroundsd immediate must be in 0..=15") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vroundsd xmm0, xmm0, [{b}], {imm}",
            "vmovdqu [{out}], xmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            out = in(reg) out.as_mut_ptr(),
            imm = const IMM,
            out("xmm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}
