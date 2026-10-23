//! Permute instructions.

use super::V256;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx2_vpermps(a: *const u8, idx: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpermd(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermps`
///
/// Permute the eight 32-bit float lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpermps(a: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpermps(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpermd`
///
/// Permute the eight 32-bit integer lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpermd(a: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpermd(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

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
