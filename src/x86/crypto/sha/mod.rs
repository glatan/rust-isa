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
//! # Compile-time immediate check
//!
//! ```compile_fail
//! use rust_isa::x86::crypto::sha::sha1rnds4;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { sha1rnds4::<4>(v, v) };
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::x86::crypto::sha::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { sha1msg1(v128, v128) };
//! let _ = unsafe { sha1msg2(v128, v128) };
//! let _ = unsafe { sha1nexte(v128, v128) };
//! #[cfg(target_arch = "x86_64")] let _ = unsafe { sha1rnds4::<0>(v128, v128) };
//! let _ = unsafe { sha256msg1(v128, v128) };
//! let _ = unsafe { sha256msg2(v128, v128) };
//! let _ = unsafe { sha256rnds2(v128, v128, v128) };
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
//!
//! </details>

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
/// Perform four SHA-1 rounds with `IMM`. `IMM` selects the round function
/// and must be in `0..=3`; a larger value is a compile-time error.
///
/// # Safety
///
/// The caller must enable SHA.
#[target_feature(enable = "sha")]
#[inline]
pub unsafe fn sha1rnds4<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 3, "sha1rnds4 function must be in 0..=3") };
    #[cfg(target_arch = "x86_64")]
    {
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
    #[cfg(target_arch = "x86")]
    {
        let mut output = V128::from_bytes([0; 16]);
        unsafe {
            asm!(
                "movdqu xmm0, [{a}]",
                "movdqu xmm1, [{b}]",
                "sha1rnds4 xmm0, xmm1, {imm}",
                "movdqu [{output}], xmm0",
                a = in(reg) a.as_ptr(),
                b = in(reg) b.as_ptr(),
                output = in(reg) output.as_mut_ptr(),
                imm = const IMM,
                out("xmm0") _,
                out("xmm1") _,
                options(nostack, preserves_flags)
            );
        }
        output
    }
}
