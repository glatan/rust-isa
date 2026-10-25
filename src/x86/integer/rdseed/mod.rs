//! RDSEED.
//!
//! RDSEED is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable RDSEED with `#[target_feature(enable = "rdseed")]` or
//! a global target feature.
//!
//! RDSEED is intended to seed another random-number generator. The functions
//! operate on 32-bit and 64-bit scalar values. They return `None` when the
//! hardware reports that no seed is available; the caller may retry.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::integer::rdseed::rdseed_u32;
//!
//! let value = unsafe { rdseed_u32() };
//! // RDRAND/RDSEED is nondeterministic and may report failure.
//! assert!(value.is_some() || value.is_none());
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::integer::rdseed::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! #[cfg(target_arch = "x86_64")] let _ = unsafe { rdseed_u64() };
//!
//! let _ = unsafe { rdseed_u32() };
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
//! use rust_isa::x86::integer::rdseed::rdseed_u32 as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::integer::rdseed::rdseed_u64 as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

unsafe extern "C" {
    fn rust_isa_rdseed_u32(out: *mut u32) -> i32;
    #[cfg(target_arch = "x86_64")]
    fn rust_isa_rdseed_u64(out: *mut u64) -> i32;
}

/// `rdseed` with a 32-bit destination.
///
/// # Safety
///
/// The caller must enable RDSEED.
#[target_feature(enable = "rdseed")]
#[inline]
pub unsafe fn rdseed_u32() -> Option<u32> {
    let mut value = 0u32;
    if unsafe { rust_isa_rdseed_u32(&mut value) } != 0 {
        Some(value)
    } else {
        None
    }
}

/// `rdseed` with a 64-bit destination.
///
/// # Safety
///
/// The caller must enable RDSEED.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "rdseed")]
#[inline]
pub unsafe fn rdseed_u64() -> Option<u64> {
    let mut value = 0u64;
    if unsafe { rust_isa_rdseed_u64(&mut value) } != 0 {
        Some(value)
    } else {
        None
    }
}
