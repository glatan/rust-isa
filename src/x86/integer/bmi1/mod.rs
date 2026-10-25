//! BMI1.
//!
//! BMI1 is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! requires the `bmi1` target feature. The functions operate on 32-bit scalar
//! values.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::integer::bmi1::andn as _;
//! use rust_isa::x86::integer::bmi1::bextr as _;
//! use rust_isa::x86::integer::bmi1::blsi as _;
//! use rust_isa::x86::integer::bmi1::blsmsk as _;
//! use rust_isa::x86::integer::bmi1::blsr as _;
//! ```
//! <!-- doctest-inventory:end -->

unsafe extern "C" {
    fn rust_isa_bmi1_andn(a: u32, b: u32) -> u32;
    fn rust_isa_bmi1_bextr(a: u32, start: u32, len: u32) -> u32;
    fn rust_isa_bmi1_blsi(a: u32) -> u32;
    fn rust_isa_bmi1_blsmsk(a: u32) -> u32;
    fn rust_isa_bmi1_blsr(a: u32) -> u32;
}

/// `andn`
///
/// Compute `!a & b`.
///
/// # Safety
///
/// The caller must enable BMI1.
#[target_feature(enable = "bmi1")]
#[inline]
pub unsafe fn andn(a: u32, b: u32) -> u32 {
    unsafe { rust_isa_bmi1_andn(a, b) }
}

/// `bextr`
///
/// Extract `len` bits starting at `start`.
///
/// # Safety
///
/// The caller must enable BMI1.
#[target_feature(enable = "bmi1")]
#[inline]
pub unsafe fn bextr(a: u32, start: u32, len: u32) -> u32 {
    unsafe { rust_isa_bmi1_bextr(a, start, len) }
}

/// `blsi`
///
/// Extract the lowest set bit.
///
/// # Safety
///
/// The caller must enable BMI1.
#[target_feature(enable = "bmi1")]
#[inline]
pub unsafe fn blsi(a: u32) -> u32 {
    unsafe { rust_isa_bmi1_blsi(a) }
}

/// `blsmsk`
///
/// Mask from the lowest set bit to bit 0.
///
/// # Safety
///
/// The caller must enable BMI1.
#[target_feature(enable = "bmi1")]
#[inline]
pub unsafe fn blsmsk(a: u32) -> u32 {
    unsafe { rust_isa_bmi1_blsmsk(a) }
}

/// `blsr`
///
/// Clear the lowest set bit.
///
/// # Safety
///
/// The caller must enable BMI1.
#[target_feature(enable = "bmi1")]
#[inline]
pub unsafe fn blsr(a: u32) -> u32 {
    unsafe { rust_isa_bmi1_blsr(a) }
}
