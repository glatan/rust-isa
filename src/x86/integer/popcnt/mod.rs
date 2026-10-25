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
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::integer::popcnt::popcnt_u32 as _;
//! use rust_isa::x86::integer::popcnt::popcnt_u64 as _;
//! ```
//! <!-- doctest-inventory:end -->

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
