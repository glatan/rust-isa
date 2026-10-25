//! VAES.
//!
//! VAES is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and the
//! required target features are documented on each function. The values are
//! [`V256`], [`V512`], and [`V128`], depending on the width.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::vaes::vaesenc_v128;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vaesenc_v128(v, v) };
//! ```
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::x86::crypto::vaes::*;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { vaesenc_v128(v, v) }.to_bytes(), [0x63; 16]);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::vaes::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vaesdec(v256, v256) };
//! let _ = unsafe { vaesdec_v128(v128, v128) };
//! let _ = unsafe { vaesdec_v512(v512, v512) };
//! let _ = unsafe { vaesdeclast(v256, v256) };
//! let _ = unsafe { vaesdeclast_v128(v128, v128) };
//! let _ = unsafe { vaesdeclast_v512(v512, v512) };
//! let _ = unsafe { vaesenc(v256, v256) };
//! let _ = unsafe { vaesenc_v128(v128, v128) };
//! let _ = unsafe { vaesenc_v512(v512, v512) };
//! let _ = unsafe { vaesenclast(v256, v256) };
//! let _ = unsafe { vaesenclast_v128(v128, v128) };
//! let _ = unsafe { vaesenclast_v512(v512, v512) };
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::vaes::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vaesdec(v256, v256) };
//! let _ = unsafe { vaesdec_v128(v128, v128) };
//! let _ = unsafe { vaesdec_v512(v512, v512) };
//! let _ = unsafe { vaesdeclast(v256, v256) };
//! let _ = unsafe { vaesdeclast_v128(v128, v128) };
//! let _ = unsafe { vaesdeclast_v512(v512, v512) };
//! let _ = unsafe { vaesenc(v256, v256) };
//! let _ = unsafe { vaesenc_v128(v128, v128) };
//! let _ = unsafe { vaesenc_v512(v512, v512) };
//! let _ = unsafe { vaesenclast(v256, v256) };
//! let _ = unsafe { vaesenclast_v128(v128, v128) };
//! let _ = unsafe { vaesenclast_v512(v512, v512) };
//! ```
//!
//! </details>
//!
//! <details>
//! <summary>Instruction inventory</summary>
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::crypto::vaes::vaesdec as _;
//! use rust_isa::x86::crypto::vaes::vaesdec_v128 as _;
//! use rust_isa::x86::crypto::vaes::vaesdec_v512 as _;
//! use rust_isa::x86::crypto::vaes::vaesdeclast as _;
//! use rust_isa::x86::crypto::vaes::vaesdeclast_v128 as _;
//! use rust_isa::x86::crypto::vaes::vaesdeclast_v512 as _;
//! use rust_isa::x86::crypto::vaes::vaesenc as _;
//! use rust_isa::x86::crypto::vaes::vaesenc_v128 as _;
//! use rust_isa::x86::crypto::vaes::vaesenc_v512 as _;
//! use rust_isa::x86::crypto::vaes::vaesenclast as _;
//! use rust_isa::x86::crypto::vaes::vaesenclast_v128 as _;
//! use rust_isa::x86::crypto::vaes::vaesenclast_v512 as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use crate::x86::simd::avx2::V256;
use crate::x86::simd::avx512f::V512;
use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_vaes_vaesenc(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenc`
///
/// Perform one AES encryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenc(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_vaes_vaesenc(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesenc_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenc` on 128-bit values.
///
/// Perform one AES encryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenc_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vaes_vaesenc_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdec(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdec`
///
/// Perform one AES decryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdec(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_vaes_vaesdec(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdec_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdec` on 128-bit values.
///
/// Perform one AES decryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdec_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vaes_vaesdec_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesenclast(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenclast`
///
/// Perform the last AES encryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenclast(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_vaes_vaesenclast(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesenclast_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenclast` on 128-bit values.
///
/// Perform the last AES encryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenclast_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vaes_vaesenclast_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdeclast(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdeclast`
///
/// Perform the last AES decryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdeclast(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_vaes_vaesdeclast(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdeclast_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdeclast` on 128-bit values.
///
/// Perform the last AES decryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdeclast_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vaes_vaesdeclast_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesenc_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenc` on 512-bit values.
///
/// Perform one AES encryption round on the 512-bit value.
///
/// # Safety
///
/// The caller must enable AVX-512F and VAES.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenc_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_vaes_vaesenc_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdec_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdec` on 512-bit values.
///
/// Perform one AES decryption round on the 512-bit value.
///
/// # Safety
///
/// The caller must enable AVX-512F and VAES.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdec_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_vaes_vaesdec_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesenclast_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenclast` on 512-bit values.
///
/// Perform the last AES encryption round on the 512-bit value.
///
/// # Safety
///
/// The caller must enable AVX-512F and VAES.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenclast_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_vaes_vaesenclast_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdeclast_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdeclast` on 512-bit values.
///
/// Perform the last AES decryption round on the 512-bit value.
///
/// # Safety
///
/// The caller must enable AVX-512F and VAES.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdeclast_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_vaes_vaesdeclast_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
