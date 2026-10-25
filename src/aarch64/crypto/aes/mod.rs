//! AArch64 AES instructions.
//!
//! AES is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable AES with `#[target_feature(enable = "aes")]` or a global
//! target feature.
//!
//! The values are [`V128`]. The round instructions operate on the standard
//! AES state layout used by the AArch64 instruction set. The caller builds the
//! key schedule and applies the rounds.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::aarch64::crypto::aes::aese;
//! use rust_isa::aarch64::simd::neon::V128;
//!
//! let zero = V128::from_bytes([0; 16]);
//! assert_ne!(unsafe { aese(zero, zero) }.to_bytes(), [0; 16]);
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::aarch64::crypto::aes::*;
//! use rust_isa::aarch64::simd::neon::V128;
//! use rust_isa::aarch64::simd::sme::SmeGuard;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! unsafe {
//!     let _ = unsafe { aesd(v128, v128) };
//!     let _ = unsafe { aese(v128, v128) };
//!     let _ = unsafe { aesimc(v128) };
//!     let _ = unsafe { aesmc(v128) };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::aarch64::crypto::aes::aesd as _;
//! use rust_isa::aarch64::crypto::aes::aese as _;
//! use rust_isa::aarch64::crypto::aes::aesimc as _;
//! use rust_isa::aarch64::crypto::aes::aesmc as _;
//! ```
//! <!-- doctest-inventory:end -->

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
