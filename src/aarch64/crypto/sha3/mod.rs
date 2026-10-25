//! AArch64 SHA-3 instructions.
//!
//! SHA-3 is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable SHA-3 with `#[target_feature(enable = "sha3")]` or a
//! global target feature.
//!
//! The values are [`V128`]. The instructions implement the ternary logic used
//! by the Keccak round. [`xar`] takes the 64-bit lane rotation as a const
//! generic; the instruction encodes it as an immediate.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::aarch64::crypto::sha3::eor3;
//! use rust_isa::aarch64::simd::neon::V128;
//!
//! let zero = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { eor3(zero, zero, zero) }.to_bytes(), [0; 16]);
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::aarch64::crypto::sha3::*;
//! use rust_isa::aarch64::simd::neon::V128;
//! use rust_isa::aarch64::simd::sme::SmeGuard;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! unsafe {
//!     let _ = unsafe { bcax(v128, v128, v128) };
//!     let _ = unsafe { eor3(v128, v128, v128) };
//!     let _ = unsafe { rax1(v128, v128) };
//!     let _ = unsafe { xar::<0>(v128, v128) };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::aarch64::crypto::sha3::bcax as _;
//! use rust_isa::aarch64::crypto::sha3::eor3 as _;
//! use rust_isa::aarch64::crypto::sha3::rax1 as _;
//! use rust_isa::aarch64::crypto::sha3::xar as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_sha3_veor3q_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sha3_vbcaxq_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sha3_vrax1q_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sha3_vxarq_u64(a: *const u8, b: *const u8, imm: i32, out: *mut u8);
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

/// `xar`
///
/// Rotate each 64-bit lane of `a` right by `IMM` bits and XOR with `b`.
/// `IMM` must be in `0..64`.
///
/// # Safety
///
/// The caller must enable SHA-3.
#[target_feature(enable = "sha3")]
#[inline]
pub unsafe fn xar<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM < 64, "xar immediate must be in 0..64") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sha3_vxarq_u64(a.as_ptr(), b.as_ptr(), IMM as i32, out.as_mut_ptr());
    }
    out
}
