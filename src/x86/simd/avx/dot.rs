//! Dot-product instructions.

use super::V128;
use super::V256;
use core::arch::asm;

/// `vdpps`
///
/// Compute the dot product of 32-bit float lanes. The immediate selects the
/// products and result lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vdpps<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vdpps ymm0, ymm0, [{b}], {imm}",
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

/// `vdppd`
///
/// Compute the dot product of 64-bit float lanes. The immediate selects the
/// products and result lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vdppd<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vdppd xmm0, xmm0, [{b}], {imm}",
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
