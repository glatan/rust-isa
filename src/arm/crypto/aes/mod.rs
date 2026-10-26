//! 32-bit ARM AES.
//!
//! AES is unstable on this target. Every instruction wrapper is `unsafe` and the caller
//! must compile the whole crate with the AES feature enabled.
//!
//! The values are [`V128`]. The caller builds the key schedule and applies the
//! rounds.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::arm::crypto::aes::aese;
//! use rust_isa::arm::simd::neon::V128;
//!
//! let zero = V128::from_bytes([0; 16]);
//! let out = unsafe { aese(zero, zero) };
//! assert_ne!(out.to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::arm::crypto::aes::*;
//! use rust_isa::arm::simd::neon::V128;
//! use rust_isa::arm::simd::neon::V64;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { aesd(v128, v128) };
//! let _ = unsafe { aese(v128, v128) };
//! let _ = unsafe { aesimc(v128) };
//! let _ = unsafe { aesmc(v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::arm::crypto::aes::aesd as _;
//! use rust_isa::arm::crypto::aes::aese as _;
//! use rust_isa::arm::crypto::aes::aesimc as _;
//! use rust_isa::arm::crypto::aes::aesmc as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use crate::arm::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_arm_aes_aese(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_aes_aesd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_aes_aesmc(a: *const u8, out: *mut u8);
    fn rust_isa_arm_aes_aesimc(a: *const u8, out: *mut u8);
}

/// `aese`
///
/// Perform the AES encryption round.
///
/// # Safety
///
/// The caller must enable AES on the CPU and in the target features.
#[inline(always)]
pub unsafe fn aese(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_aes_aese(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `aesd`
///
/// Perform the AES decryption round.
///
/// # Safety
///
/// The caller must enable AES on the CPU and in the target features.
#[inline(always)]
pub unsafe fn aesd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_aes_aesd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `aesmc`
///
/// Perform the AES MixColumns step.
///
/// # Safety
///
/// The caller must enable AES on the CPU and in the target features.
#[inline(always)]
pub unsafe fn aesmc(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_aes_aesmc(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `aesimc`
///
/// Perform the inverse AES MixColumns step.
///
/// # Safety
///
/// The caller must enable AES on the CPU and in the target features.
#[inline(always)]
pub unsafe fn aesimc(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_aes_aesimc(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
