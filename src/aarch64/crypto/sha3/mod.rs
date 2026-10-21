//! AArch64 SHA-3 instructions.
//!
//! SHA-3 is not part of the AArch64 baseline. Every operation is `unsafe` and
//! the caller must enable SHA-3 with `#[target_feature(enable = "sha3")]` or a
//! global target feature.
//!
//! The values are [`V128`]. The instructions implement the ternary logic used
//! by the Keccak round. The rotate-xor instruction that takes an immediate is
//! intentionally omitted; use [`vrax1`] and a software rotate if needed.

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_sha3_veor3q_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sha3_vbcaxq_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sha3_vrax1q_u64(a: *const u8, b: *const u8, out: *mut u8);
}

/// `eor3`
///
/// Compute `a ^ b ^ c` on bytes.
///
/// # Safety
///
/// The caller must enable SHA-3.
#[target_feature(enable = "sha3")]
#[inline]
pub unsafe fn eor3(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sha3_veor3q_u8(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `bcax`
///
/// Compute `a ^ (b & !c)` on bytes.
///
/// # Safety
///
/// The caller must enable SHA-3.
#[target_feature(enable = "sha3")]
#[inline]
pub unsafe fn bcax(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sha3_vbcaxq_u8(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `rax1`
///
/// Rotate each 64-bit lane of `a` left by one bit and XOR with `b`.
///
/// # Safety
///
/// The caller must enable SHA-3.
#[target_feature(enable = "sha3")]
#[inline]
pub unsafe fn rax1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sha3_vrax1q_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
