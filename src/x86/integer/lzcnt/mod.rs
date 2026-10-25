//! LZCNT.
//!
//! LZCNT is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable LZCNT with `#[target_feature(enable = "lzcnt")]` or a
//! global target feature.
//!
//! The functions operate on 32-bit and 64-bit scalar values. The result is the
//! number of leading zero bits. A zero input returns the operand width.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::integer::lzcnt::lzcnt_u32 as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::integer::lzcnt::lzcnt_u64 as _;
//! ```
//! <!-- doctest-inventory:end -->

unsafe extern "C" {
    fn rust_isa_lzcnt_u32(value: u32) -> u32;
    #[cfg(target_arch = "x86_64")]
    fn rust_isa_lzcnt_u64(value: u64) -> u64;
}

/// `lzcnt` with a 32-bit value.
///
/// # Safety
///
/// The caller must enable LZCNT.
#[target_feature(enable = "lzcnt")]
#[inline]
pub unsafe fn lzcnt_u32(value: u32) -> u32 {
    unsafe { rust_isa_lzcnt_u32(value) }
}

/// `lzcnt` with a 64-bit value.
///
/// # Safety
///
/// The caller must enable LZCNT.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "lzcnt")]
#[inline]
pub unsafe fn lzcnt_u64(value: u64) -> u64 {
    unsafe { rust_isa_lzcnt_u64(value) }
}
