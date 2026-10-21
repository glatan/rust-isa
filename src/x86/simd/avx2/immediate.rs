//! Immediate-form AVX2 instructions.
//!
//! These functions use const-generic inline assembly and memory operands.
//! They are not zero-cost: each call loads its inputs and stores its result.

use super::V256;
use crate::x86::simd::sse2::V128;
use core::arch::asm;

/// `vpermq`
///
/// Permute the four 64-bit lanes with the 8-bit immediate.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpermq<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vpermq ymm0, [{a}], {imm}",
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

/// `vperm2i128`
///
/// Permute the two 128-bit lanes of `a` and `b` with the 8-bit immediate.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vperm2i128<const IMM: u8>(a: V256, b: V256) -> V256 {
    const {
        assert!(
            IMM & 0x44 == 0,
            "vperm2i128 reserved immediate bits must be zero"
        )
    };
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vperm2i128 ymm0, ymm0, [{b}], {imm}",
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

/// `vinserti128`
///
/// Insert `b` into the 128-bit lane selected by the immediate.
///
/// # Safety
///
/// The caller must enable AVX2. `IMM` must be 0 or 1.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vinserti128<const IMM: u8>(a: V256, b: V128) -> V256 {
    const { assert!(IMM <= 1, "128-bit lane must be 0 or 1") };
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vinserti128 ymm0, ymm0, xmm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            out = in(reg) out.as_mut_ptr(),
            imm = const IMM,
            out("ymm0") _,
            out("xmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vextracti128`
///
/// Extract the 128-bit lane selected by the immediate.
///
/// # Safety
///
/// The caller must enable AVX2. `IMM` must be 0 or 1.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vextracti128<const IMM: u8>(a: V256) -> V128 {
    const { assert!(IMM <= 1, "128-bit lane must be 0 or 1") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vextracti128 xmm1, ymm0, {imm}",
            "vmovdqu [{out}], xmm1",
            a = in(reg) a.as_ptr(),
            out = in(reg) out.as_mut_ptr(),
            imm = const IMM,
            out("ymm0") _,
            out("xmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpblendw`
///
/// Select 16-bit lanes from `a` and `b` with the immediate mask, within each
/// 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpblendw<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vpblendw ymm0, ymm0, [{b}], {imm}",
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

/// `vpblendd`
///
/// Select the eight 32-bit lanes from `a` and `b` with the immediate mask.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpblendd<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vpblendd ymm0, ymm0, [{b}], {imm}",
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

/// `vpshufd`
///
/// Shuffle the 32-bit lanes within each 128-bit lane with the immediate.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpshufd<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vpshufd ymm0, [{a}], {imm}",
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

/// `vpshufhw`
///
/// Shuffle the high four 16-bit lanes within each 128-bit lane with the immediate.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpshufhw<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vpshufhw ymm0, [{a}], {imm}",
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

/// `vpshuflw`
///
/// Shuffle the low four 16-bit lanes within each 128-bit lane with the immediate.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpshuflw<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vpshuflw ymm0, [{a}], {imm}",
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
