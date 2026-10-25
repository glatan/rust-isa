//! 32-bit ARM CRC.
//!
//! CRC is unstable on this target. Every instruction wrapper is `unsafe` and the caller
//! must compile the whole crate with the CRC feature enabled.
//!
//! The functions implement CRC-32 (IEEE 802.3), not CRC-32C. The accumulator is
//! the raw hardware state.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::arm::crypto::crc::crc32b;
//!
//! let mut crc = 0xffff_ffffu32;
//! for byte in b"123456789" {
//!     crc = unsafe { crc32b(crc, *byte) };
//! }
//! assert_eq!(crc, 0x340b_c6d9);
//! ```
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::arm::crypto::crc::*;
//! use rust_isa::arm::simd::neon::V128;
//! use rust_isa::arm::simd::neon::V64;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { crc32b(0, 0) };
//! let _ = unsafe { crc32h(0, 0) };
//! let _ = unsafe { crc32w(0, 0) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::arm::crypto::crc::crc32b as _;
//! use rust_isa::arm::crypto::crc::crc32h as _;
//! use rust_isa::arm::crypto::crc::crc32w as _;
//! ```
//! <!-- doctest-inventory:end -->

unsafe extern "C" {
    fn rust_isa_arm_crc_crc32b(crc: u32, value: u8) -> u32;
    fn rust_isa_arm_crc_crc32h(crc: u32, value: u16) -> u32;
    fn rust_isa_arm_crc_crc32w(crc: u32, value: u32) -> u32;
}

/// `crc32b`
///
/// Accumulate one byte into the CRC-32 state.
///
/// # Safety
///
/// The caller must enable CRC on the CPU and in the target features.
#[inline(always)]
pub unsafe fn crc32b(crc: u32, value: u8) -> u32 {
    unsafe { rust_isa_arm_crc_crc32b(crc, value) }
}

/// `crc32h`
///
/// Accumulate a 16-bit value into the CRC-32 state.
///
/// # Safety
///
/// The caller must enable CRC on the CPU and in the target features.
#[inline(always)]
pub unsafe fn crc32h(crc: u32, value: u16) -> u32 {
    unsafe { rust_isa_arm_crc_crc32h(crc, value) }
}

/// `crc32w`
///
/// Accumulate a 32-bit value into the CRC-32 state.
///
/// # Safety
///
/// The caller must enable CRC on the CPU and in the target features.
#[inline(always)]
pub unsafe fn crc32w(crc: u32, value: u32) -> u32 {
    unsafe { rust_isa_arm_crc_crc32w(crc, value) }
}
