//! AES-NI.
//!
//! AES-NI is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! requires the `aes` target feature. The values are [`V128`].
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The build compiles the bundled C with clang; the `aeskeygenassist` immediate
//! form uses `asm!`. Immediate operands are const generics.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::x86::crypto::aes::aesenc;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_ne!(unsafe { aesenc(v, v) }.to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::x86::crypto::aes::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { aesdec(v128, v128) };
//! let _ = unsafe { aesdeclast(v128, v128) };
//! let _ = unsafe { aesenc(v128, v128) };
//! let _ = unsafe { aesenclast(v128, v128) };
//! let _ = unsafe { aesimc(v128) };
//! #[cfg(target_arch = "x86_64")] let _ = unsafe { aeskeygenassist::<0>(v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::crypto::aes::aesdec as _;
//! use rust_isa::x86::crypto::aes::aesdeclast as _;
//! use rust_isa::x86::crypto::aes::aesenc as _;
//! use rust_isa::x86::crypto::aes::aesenclast as _;
//! use rust_isa::x86::crypto::aes::aesimc as _;
//! use rust_isa::x86::crypto::aes::aeskeygenassist as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use core::arch::asm;

use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_aes_aesenc(a: *const u8, b: *const u8, out: *mut u8);
}

/// `aesenc`
///
/// Perform one AES encryption round.
///
/// # Safety
///
/// The caller must enable aes.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn aesenc(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_aes_aesenc(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_aes_aesdec(a: *const u8, b: *const u8, out: *mut u8);
}

/// `aesdec`
///
/// Perform one AES decryption round.
///
/// # Safety
///
/// The caller must enable aes.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn aesdec(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_aes_aesdec(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_aes_aesenclast(a: *const u8, b: *const u8, out: *mut u8);
}

/// `aesenclast`
///
/// Perform the last AES encryption round.
///
/// # Safety
///
/// The caller must enable aes.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn aesenclast(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_aes_aesenclast(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_aes_aesdeclast(a: *const u8, b: *const u8, out: *mut u8);
}

/// `aesdeclast`
///
/// Perform the last AES decryption round.
///
/// # Safety
///
/// The caller must enable aes.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn aesdeclast(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_aes_aesdeclast(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_aes_aesimc(a: *const u8, out: *mut u8);
}

/// `aesimc`
///
/// Apply the inverse MixColumns transform.
///
/// # Safety
///
/// The caller must enable aes.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn aesimc(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_aes_aesimc(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `aeskeygenassist`
///
/// Assist AES key expansion with `IMM`.
///
/// # Safety
///
/// The caller must enable AES.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn aeskeygenassist<const IMM: u8>(a: V128) -> V128 {
    let mut output = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "movdqu xmm0, [{input}]",
            "aeskeygenassist xmm0, xmm0, {imm}",
            "movdqu [{output}], xmm0",
            input = in(reg) a.as_ptr(),
            output = in(reg) output.as_mut_ptr(),
            imm = const IMM,
            out("xmm0") _,
            options(nostack, preserves_flags)
        );
    }
    output
}
