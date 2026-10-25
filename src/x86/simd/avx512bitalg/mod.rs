//! AVX-512 BITALG.
//!
//! AVX-512 BITALG is not part of the x86-64 baseline. Every instruction wrapper is
//! `unsafe` and the caller must enable AVX-512 BITALG with
//! `#[target_feature(enable = "avx512bitalg")]` (which implies AVX-512 F and BW)
//! or a global target feature.
//!
//! The values are [`V512`]. The population-count instructions return the count
//! in each lane. `vpshufbitqmb` selects one bit from each 64-bit group and
//! returns the selected bits as a mask.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512bitalg::vpopcntb;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vpopcntb(v) };
//! ```
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512bitalg::*;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! assert_eq!(unsafe { vpopcntb(v) }.to_bytes(), [0; 64]);
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512bitalg::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpopcntb(v512) };
//! let _ = unsafe { vpopcntw(v512) };
//! let _ = unsafe { vpshufbitqmb(v512, v512) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avx512bitalg::vpopcntb as _;
//! use rust_isa::x86::simd::avx512bitalg::vpopcntw as _;
//! use rust_isa::x86::simd::avx512bitalg::vpshufbitqmb as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512bitalg_vpopcntb(a: *const u8, out: *mut u8);
    fn rust_isa_avx512bitalg_vpopcntw(a: *const u8, out: *mut u8);
    fn rust_isa_avx512bitalg_vpshufbitqmb(a: *const u8, idx: *const u8) -> u64;
}

/// `vpopcntb`
///
/// Count the set bits in each byte lane.
///
/// # Safety
///
/// The caller must enable AVX-512 BITALG.
#[target_feature(enable = "avx512bitalg")]
#[inline]
pub unsafe fn vpopcntb(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bitalg_vpopcntb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpopcntw`
///
/// Count the set bits in each 16-bit lane.
///
/// # Safety
///
/// The caller must enable AVX-512 BITALG.
#[target_feature(enable = "avx512bitalg")]
#[inline]
pub unsafe fn vpopcntw(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bitalg_vpopcntw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpshufbitqmb`
///
/// Select one bit from each 64-bit group using `idx` and return the selected
/// bits as a 64-bit mask.
///
/// # Safety
///
/// The caller must enable AVX-512 BITALG.
#[target_feature(enable = "avx512bitalg")]
#[inline]
pub unsafe fn vpshufbitqmb(a: V512, idx: V512) -> u64 {
    unsafe { rust_isa_avx512bitalg_vpshufbitqmb(a.as_ptr(), idx.as_ptr()) }
}
