//! Shuffle instructions.

use super::V256;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx2_vpshufb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpshufb`
///
/// Shuffle the bytes of `a` with the indices in `b`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpshufb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpshufb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
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
