//! SHA-1 and SHA-256 instructions.
//!
//! These instructions are not part of the x86-64 baseline. Every instruction wrapper is
//! `unsafe` and requires the `sha` target feature. The values are [`V128`].
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The build compiles the bundled C with clang; the immediate form uses
//! `asm!`.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::x86::crypto::sha::sha1msg1;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { sha1msg1(v, v) }.to_bytes(), [0; 16]);
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::crypto::sha::sha1msg1 as _;
//! use rust_isa::x86::crypto::sha::sha1msg2 as _;
//! use rust_isa::x86::crypto::sha::sha1nexte as _;
//! use rust_isa::x86::crypto::sha::sha1rnds4 as _;
//! use rust_isa::x86::crypto::sha::sha256msg1 as _;
//! use rust_isa::x86::crypto::sha::sha256msg2 as _;
//! use rust_isa::x86::crypto::sha::sha256rnds2 as _;
//! ```
//! <!-- doctest-inventory:end -->

use core::arch::asm;

use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_sha_sha1nexte(a: *const u8, b: *const u8, out: *mut u8);
}

/// `sha1nexte`
///
/// Calculate the SHA-1 state for the next four rounds.
///
/// # Safety
///
/// The caller must enable sha.
#[target_feature(enable = "sha")]
#[inline]
pub unsafe fn sha1nexte(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sha_sha1nexte(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sha_sha1msg1(a: *const u8, b: *const u8, out: *mut u8);
}

/// `sha1msg1`
///
/// Perform the first SHA-1 message-schedule step.
///
/// # Safety
///
/// The caller must enable sha.
#[target_feature(enable = "sha")]
#[inline]
pub unsafe fn sha1msg1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sha_sha1msg1(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sha_sha1msg2(a: *const u8, b: *const u8, out: *mut u8);
}

/// `sha1msg2`
///
/// Perform the second SHA-1 message-schedule step.
///
/// # Safety
///
/// The caller must enable sha.
#[target_feature(enable = "sha")]
#[inline]
pub unsafe fn sha1msg2(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sha_sha1msg2(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sha_sha256msg1(a: *const u8, b: *const u8, out: *mut u8);
}

/// `sha256msg1`
///
/// Perform the first SHA-256 message-schedule step.
///
/// # Safety
///
/// The caller must enable sha.
#[target_feature(enable = "sha")]
#[inline]
pub unsafe fn sha256msg1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sha_sha256msg1(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sha_sha256msg2(a: *const u8, b: *const u8, out: *mut u8);
}

/// `sha256msg2`
///
/// Perform the second SHA-256 message-schedule step.
///
/// # Safety
///
/// The caller must enable sha.
#[target_feature(enable = "sha")]
#[inline]
pub unsafe fn sha256msg2(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sha_sha256msg2(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sha_sha256rnds2(a: *const u8, b: *const u8, k: *const u8, out: *mut u8);
}

/// `sha256rnds2`
///
/// Perform two SHA-256 rounds.
///
/// # Safety
///
/// The caller must enable SHA.
#[target_feature(enable = "sha")]
#[inline]
pub unsafe fn sha256rnds2(a: V128, b: V128, k: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sha_sha256rnds2(a.as_ptr(), b.as_ptr(), k.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `sha1rnds4`
///
/// Perform four SHA-1 rounds with `IMM`.
///
/// # Safety
///
/// The caller must enable SHA.
#[target_feature(enable = "sha")]
#[inline]
pub unsafe fn sha1rnds4<const IMM: u8>(a: V128, b: V128) -> V128 {
    let a = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "sha1rnds4 {output}, {b}, {imm}",
            output = inout(xmm_reg) a => output,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}
