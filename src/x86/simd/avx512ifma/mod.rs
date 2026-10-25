//! AVX-512IFMA.
//!
//! AVX-512IFMA is not part of the x86-64 baseline. Every instruction wrapper is `unsafe`
//! and requires AVX-512F and AVX-512IFMA. The values are [`V512`].
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
//! use rust_isa::x86::simd::avx512ifma::vpmadd52luq;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vpmadd52luq(v, v, v) };
//! }
//! ```
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512ifma::*;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! assert_eq!(unsafe { vpmadd52luq(v, v, v) }.to_bytes(), [0; 64]);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512ifma::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpmadd52huq(v512, v512, v512) };
//! let _ = unsafe { vpmadd52luq(v512, v512, v512) };
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512ifma::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpmadd52huq(v512, v512, v512) };
//! let _ = unsafe { vpmadd52luq(v512, v512, v512) };
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
//! use rust_isa::x86::simd::avx512ifma::vpmadd52huq as _;
//! use rust_isa::x86::simd::avx512ifma::vpmadd52luq as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use crate::x86::simd::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512ifma_vpmadd52luq(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx512ifma_vpmadd52huq(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmadd52luq`
///
/// Add the low 52 bits of the unsigned 64-bit products to `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F and AVX-512IFMA.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512ifma")]
#[inline]
pub unsafe fn vpmadd52luq(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512ifma_vpmadd52luq(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmadd52huq`
///
/// Add the high 52 bits of the unsigned 64-bit products to `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F and AVX-512IFMA.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512ifma")]
#[inline]
pub unsafe fn vpmadd52huq(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512ifma_vpmadd52huq(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
