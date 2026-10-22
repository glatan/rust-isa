//! 128-bit lane insert and extract instructions.

use super::V256;
use crate::x86::simd::sse2::V128;
use core::arch::asm;

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
