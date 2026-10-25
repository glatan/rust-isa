//! AVX-512 CD.
//!
//! AVX-512 CD is not part of the x86-64 baseline. Every instruction wrapper is `unsafe`
//! and the caller must enable AVX-512 CD with
//! `#[target_feature(enable = "avx512cd")]` (which implies AVX-512 F) or a
//! global target feature.
//!
//! The values are [`V512`]. The conflict instructions return, for each lane,
//! the bitmask of earlier lanes with the same value.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512cd::vpconflictd;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! assert_eq!(unsafe { vpconflictd(v) }.to_bytes(), [0; 64]);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512cd::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpconflictd(v512) };
//! let _ = unsafe { vpconflictq(v512) };
//! let _ = unsafe { vplzcntd(v512) };
//! let _ = unsafe { vplzcntq(v512) };
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512cd::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpconflictd(v512) };
//! let _ = unsafe { vpconflictq(v512) };
//! let _ = unsafe { vplzcntd(v512) };
//! let _ = unsafe { vplzcntq(v512) };
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
//! use rust_isa::x86::simd::avx512cd::vpconflictd as _;
//! use rust_isa::x86::simd::avx512cd::vpconflictq as _;
//! use rust_isa::x86::simd::avx512cd::vplzcntd as _;
//! use rust_isa::x86::simd::avx512cd::vplzcntq as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use super::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512cd_vplzcntd(a: *const u8, out: *mut u8);
    fn rust_isa_avx512cd_vplzcntq(a: *const u8, out: *mut u8);
    fn rust_isa_avx512cd_vpconflictd(a: *const u8, out: *mut u8);
    fn rust_isa_avx512cd_vpconflictq(a: *const u8, out: *mut u8);
}

/// `vplzcntd`
///
/// Count the leading zero bits of each 32-bit lane.
///
/// # Safety
///
/// The caller must enable AVX-512 CD.
#[target_feature(enable = "avx512cd")]
#[inline]
pub unsafe fn vplzcntd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512cd_vplzcntd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vplzcntq`
///
/// Count the leading zero bits of each 64-bit lane.
///
/// # Safety
///
/// The caller must enable AVX-512 CD.
#[target_feature(enable = "avx512cd")]
#[inline]
pub unsafe fn vplzcntq(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512cd_vplzcntq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpconflictd`
///
/// Return, for each 32-bit lane, the bitmask of earlier equal lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 CD.
#[target_feature(enable = "avx512cd")]
#[inline]
pub unsafe fn vpconflictd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512cd_vpconflictd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpconflictq`
///
/// Return, for each 64-bit lane, the bitmask of earlier equal lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 CD.
#[target_feature(enable = "avx512cd")]
#[inline]
pub unsafe fn vpconflictq(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512cd_vpconflictq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
