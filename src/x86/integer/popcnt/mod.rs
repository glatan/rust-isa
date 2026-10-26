//! POPCNT.
//!
//! POPCNT is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! requires the `popcnt` target feature. The functions operate on 32-bit and
//! 64-bit scalar values.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::x86::integer::popcnt::popcnt_u32;
//!
//! assert_eq!(unsafe { popcnt_u32(0xf0) }, 4);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::x86::integer::popcnt::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { popcnt_u32(0) };
//! let _ = unsafe { popcnt_u64(0) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::integer::popcnt::popcnt_u32 as _;
//! use rust_isa::x86::integer::popcnt::popcnt_u64 as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

unsafe extern "C" {
    fn rust_isa_popcnt_popcnt_u32(a: u32) -> u32;
    fn rust_isa_popcnt_popcnt_u64(a: u64) -> u64;
}

/// `popcnt` on 32-bit values.
///
/// # Safety
///
/// The caller must enable POPCNT.
#[target_feature(enable = "popcnt")]
#[inline]
pub unsafe fn popcnt_u32(a: u32) -> u32 {
    unsafe { rust_isa_popcnt_popcnt_u32(a) }
}

/// `popcnt` on 64-bit values.
///
/// # Safety
///
/// The caller must enable POPCNT.
#[target_feature(enable = "popcnt")]
#[inline]
pub unsafe fn popcnt_u64(a: u64) -> u64 {
    unsafe { rust_isa_popcnt_popcnt_u64(a) }
}
