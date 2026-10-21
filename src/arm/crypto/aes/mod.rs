//! 32-bit ARM AES.
//!
//! AES is unstable on this target. Every operation is `unsafe` and the caller
//! must compile the whole crate with the AES feature enabled.
//!
//! The values are [`V128`]. The caller builds the key schedule and applies the
//! rounds.

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
