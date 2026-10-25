//! AVX-512VNNI.
//!
//! AVX-512VNNI is not part of the x86-64 baseline. Every instruction wrapper is `unsafe`
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
//! ```no_run
//! use rust_isa::x86::simd::avx512vnni::vpdpbusd;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vpdpbusd(v, v, v) };
//! ```
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512vnni::*;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! assert_eq!(unsafe { vpdpbusd(v, v, v) }.to_bytes(), [0; 64]);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512vnni::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpdpbusd(v512, v512, v512) };
//! let _ = unsafe { vpdpbusds(v512, v512, v512) };
//! let _ = unsafe { vpdpwssd(v512, v512, v512) };
//! let _ = unsafe { vpdpwssds(v512, v512, v512) };
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512vnni::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpdpbusd(v512, v512, v512) };
//! let _ = unsafe { vpdpbusds(v512, v512, v512) };
//! let _ = unsafe { vpdpwssd(v512, v512, v512) };
//! let _ = unsafe { vpdpwssds(v512, v512, v512) };
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
//! use rust_isa::x86::simd::avx512vnni::vpdpbusd as _;
//! use rust_isa::x86::simd::avx512vnni::vpdpbusds as _;
//! use rust_isa::x86::simd::avx512vnni::vpdpwssd as _;
//! use rust_isa::x86::simd::avx512vnni::vpdpwssds as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use crate::x86::simd::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512vnni_vpdpbusd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusd`
/// Multiply unsigned bytes by signed bytes and add adjacent products to `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[inline]
pub unsafe fn vpdpbusd(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vnni_vpdpbusd(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vnni_vpdpbusds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusds`
/// Multiply unsigned bytes by signed bytes, add adjacent products, and accumulate into `acc` with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[inline]
pub unsafe fn vpdpbusds(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vnni_vpdpbusds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vnni_vpdpwssd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssd`
/// Multiply signed 16-bit lane pairs, add the products, and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[inline]
pub unsafe fn vpdpwssd(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vnni_vpdpwssd(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vnni_vpdpwssds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssds`
/// Multiply signed 16-bit lane pairs, add the products, and accumulate into `acc` with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[inline]
pub unsafe fn vpdpwssds(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vnni_vpdpwssds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
