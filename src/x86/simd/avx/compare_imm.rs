//! Immediate compare instructions.

use super::V128;
use super::V256;
use core::arch::asm;

/// `vcmpps`
///
/// Compare the 32-bit float lanes with the predicate in `IMM` and return an
/// all-ones or all-zero mask for each lane.
///
/// # Safety
///
/// The caller must enable AVX. `IMM` must be in `0..=31`.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcmpps<const IMM: u8>(a: V256, b: V256) -> V256 {
    const { assert!(IMM <= 31, "vcmpps predicate must be in 0..=31") };
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vcmpps ymm0, ymm0, [{b}], {imm}",
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

/// `vcmppd`
///
/// Compare the 64-bit float lanes with the predicate in `IMM` and return an
/// all-ones or all-zero mask for each lane.
///
/// # Safety
///
/// The caller must enable AVX. `IMM` must be in `0..=31`.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcmppd<const IMM: u8>(a: V256, b: V256) -> V256 {
    const { assert!(IMM <= 31, "vcmppd predicate must be in 0..=31") };
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vcmppd ymm0, ymm0, [{b}], {imm}",
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

/// `vcmpss`
///
/// Compare the low 32-bit float lanes with the predicate in `IMM`, return a
/// 32-bit mask in the low lane, and preserve the upper lanes of `a`.
///
/// # Safety
///
/// The caller must enable AVX. `IMM` must be in `0..=31`.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcmpss<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 31, "vcmpss predicate must be in 0..=31") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vcmpss xmm0, xmm0, [{b}], {imm}",
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

/// `vcmpsd`
///
/// Compare the low 64-bit float lanes with the predicate in `IMM`, return a
/// 64-bit mask in the low lane, and preserve the upper lane of `a`.
///
/// # Safety
///
/// The caller must enable AVX. `IMM` must be in `0..=31`.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcmpsd<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 31, "vcmpsd predicate must be in 0..=31") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vcmpsd xmm0, xmm0, [{b}], {imm}",
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
