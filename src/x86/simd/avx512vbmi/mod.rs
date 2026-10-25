//! AVX-512VBMI.
//!
//! AVX-512VBMI is not part of the x86-64 baseline. Every instruction wrapper is `unsafe`
//! and the required target features are documented on each function. The values
//! are [`V512`].
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! #[cfg(target_arch = "x86_64")]
//! if std::env::var_os("RUST_ISA_SDE_FLAGS").is_some() {
//! use rust_isa::x86::simd::avx512vbmi::vpermb;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vpermb(v, v) };
//! }
//! ```
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512vbmi::*;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! assert_eq!(unsafe { vpermb(v, v) }.to_bytes(), [0; 64]);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512vbmi::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpermb(v512, v512) };
//! let _ = unsafe { vpermi2b(v512, v512, v512) };
//! let _ = unsafe { vpermt2b(v512, v512, v512) };
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512vbmi::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpermb(v512, v512) };
//! let _ = unsafe { vpermi2b(v512, v512, v512) };
//! let _ = unsafe { vpermt2b(v512, v512, v512) };
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
//! use rust_isa::x86::simd::avx512vbmi::vpermb as _;
//! use rust_isa::x86::simd::avx512vbmi::vpermi2b as _;
//! use rust_isa::x86::simd::avx512vbmi::vpermt2b as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use crate::x86::simd::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512vbmi_vpermb(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermb`
/// Permute the 64 bytes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[inline]
pub unsafe fn vpermb(a: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi_vpermb(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vbmi_vpermi2b(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2b`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[inline]
pub unsafe fn vpermi2b(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi_vpermi2b(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vbmi_vpermt2b(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2b`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[inline]
pub unsafe fn vpermt2b(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi_vpermt2b(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}
