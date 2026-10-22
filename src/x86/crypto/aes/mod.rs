//! AES-NI.
//!
//! AES-NI is not part of the x86-64 baseline. Every operation is `unsafe` and
//! requires the `aes` target feature. The values are [`V128`].
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The build compiles the bundled C with clang; the `aeskeygenassist` immediate
//! form uses `asm!`. Immediate operands are const generics.
//!
//! Instructions not listed in this module are not exposed.

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
    let input = u128::from_le_bytes(a.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "aeskeygenassist {output}, {input}, {imm}",
            output = lateout(xmm_reg) output,
            input = in(xmm_reg) input,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}
