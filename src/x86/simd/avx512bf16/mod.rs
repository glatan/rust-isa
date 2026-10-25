//! AVX-512 BF16.
//!
//! AVX-512 BF16 is not part of the x86-64 baseline. Every instruction wrapper is
//! `unsafe` and the caller must enable it with
//! `#[target_feature(enable = "avx512bf16")]` (which implies AVX-512 F) or a
//! global target feature.
//!
//! The values are [`V256`] and [`V512`]. The module converts 32-bit float lanes
//! to bfloat16 and accumulates bfloat16 dot products into 32-bit float lanes.
//! The dot-product pairs are formed within each 32-bit lane; the result is
//! added to `acc`.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! #[cfg(target_arch = "x86_64")]
//! if std::env::var_os("RUST_ISA_SDE_FLAGS").is_some() {
//! use rust_isa::x86::simd::avx512bf16::vdpbf16ps;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vdpbf16ps(v, v, v) };
//! }
//! ```
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512bf16::*;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! assert_eq!(unsafe { vdpbf16ps(v, v, v) }.to_bytes(), [0; 64]);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512bf16::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vcvtneps2bf16(v512) };
//! let _ = unsafe { vdpbf16ps(v512, v512, v512) };
//!
//! ```
//!
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
//! use rust_isa::x86::simd::avx512bf16::vcvtneps2bf16 as _;
//! use rust_isa::x86::simd::avx512bf16::vdpbf16ps as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use super::avx2::V256;
use super::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512bf16_vcvtneps2bf16(a: *const u8, out: *mut u8);
    fn rust_isa_avx512bf16_vdpbf16ps(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vcvtneps2bf16`
///
/// Convert sixteen 32-bit float lanes to sixteen bfloat16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 BF16.
#[target_feature(enable = "avx512bf16")]
#[inline]
pub unsafe fn vcvtneps2bf16(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512bf16_vcvtneps2bf16(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdpbf16ps`
///
/// Multiply pairs of bfloat16 lanes, add the products, and accumulate into the
/// 32-bit float lanes of `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512 BF16.
#[target_feature(enable = "avx512bf16")]
#[inline]
pub unsafe fn vdpbf16ps(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bf16_vdpbf16ps(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
