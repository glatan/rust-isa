//! Permute and insert/extract instructions.
//!
//! # Compile-time immediate check
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx::{vextractf128, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vextractf128::<2>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx::{vinsertf128, V256};
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V256::from_bytes([0; 32]);
//! let w = V128::from_bytes([0; 16]);
//! let _ = unsafe { vinsertf128::<2>(v, w) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx::{vperm2f128, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vperm2f128::<0x44>(v, v) };
//! ```

use super::V256;
use crate::x86::simd::sse2::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx_vbroadcastf128(p: *const u8, out: *mut u8);
}

/// `vbroadcastf128`
///
/// Broadcast the 128-bit value at `p` to both 128-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX and `p` must be valid for reading 16 bytes.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vbroadcastf128(p: *const u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vbroadcastf128(p, out.as_mut_ptr());
    }
    out
}

/// `vextractf128`
///
/// Extract the 128-bit lane selected by the immediate.
///
/// # Safety
///
/// The caller must enable AVX. `IMM` must be 0 or 1.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vextractf128<const IMM: u8>(a: V256) -> V128 {
    const { assert!(IMM <= 1, "128-bit lane must be 0 or 1") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vextractf128 xmm1, ymm0, {imm}",
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

/// `vinsertf128`
///
/// Insert `b` into the 128-bit lane selected by the immediate.
///
/// # Safety
///
/// The caller must enable AVX. `IMM` must be 0 or 1.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vinsertf128<const IMM: u8>(a: V256, b: V128) -> V256 {
    const { assert!(IMM <= 1, "128-bit lane must be 0 or 1") };
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vinsertf128 ymm0, ymm0, xmm1, {imm}",
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

/// `vperm2f128`
///
/// Permute the two 128-bit lanes of `a` and `b` with the immediate.
///
/// # Safety
///
/// The caller must enable AVX. Reserved immediate bits must be zero.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vperm2f128<const IMM: u8>(a: V256, b: V256) -> V256 {
    const {
        assert!(
            IMM & 0x44 == 0,
            "vperm2f128 reserved immediate bits must be zero"
        )
    };
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vperm2f128 ymm0, ymm0, [{b}], {imm}",
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

/// `vpermilps`
///
/// Shuffle the 32-bit float lanes within each 128-bit lane with the immediate.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vpermilps<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vpermilps ymm0, [{a}], {imm}",
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

/// `vpermilpd`
///
/// Shuffle the 64-bit float lanes within each 128-bit lane with the immediate.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vpermilpd<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vpermilpd ymm0, [{a}], {imm}",
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
