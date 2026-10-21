//! AArch64 AES instructions.
//!
//! AES is not part of the AArch64 baseline. Every operation is `unsafe` and the
//! caller must enable AES with `#[target_feature(enable = "aes")]` or a global
//! target feature.
//!
//! The values are [`V128`]. The round instructions operate on the standard
//! AES state layout used by the AArch64 instruction set. The caller builds the
//! key schedule and applies the rounds.

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_aes_vaeseq_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_aes_vaesdq_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_aes_vaesmcq_u8(a: *const u8, out: *mut u8);
    fn rust_isa_aes_vaesimcq_u8(a: *const u8, out: *mut u8);
}

/// `aese`
///
/// Perform the AES encryption round: AddRoundKey, ShiftRows and SubBytes.
///
/// # Safety
///
/// The caller must enable AES.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn aese(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_aes_vaeseq_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `aesd`
///
/// Perform the AES decryption round: AddRoundKey, inverse ShiftRows and inverse
/// SubBytes.
///
/// # Safety
///
/// The caller must enable AES.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn aesd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_aes_vaesdq_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `aesmc`
///
/// Perform the AES MixColumns step.
///
/// # Safety
///
/// The caller must enable AES.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn aesmc(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_aes_vaesmcq_u8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `aesimc`
///
/// Perform the inverse AES MixColumns step.
///
/// # Safety
///
/// The caller must enable AES.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn aesimc(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_aes_vaesimcq_u8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
