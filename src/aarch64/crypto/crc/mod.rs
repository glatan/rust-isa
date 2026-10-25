//! AArch64 CRC instructions.
//!
//! CRC is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable CRC with `#[target_feature(enable = "crc")]` or a global
//! target feature.
//!
//! The functions implement CRC-32C (Castagnoli). The accumulator is the raw
//! hardware state; the caller applies the final XOR required by the protocol.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::aarch64::crypto::crc::crc32b;
//!
//! assert_eq!(unsafe { crc32b(0, 0) }, 0);
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::aarch64::crypto::crc::*;
//! use rust_isa::aarch64::simd::neon::V128;
//! use rust_isa::aarch64::simd::sme::SmeGuard;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! unsafe {
//!     let _ = unsafe { crc32b(0, 0) };
//!     let _ = unsafe { crc32h(0, 0) };
//!     let _ = unsafe { crc32w(0, 0) };
//!     let _ = unsafe { crc32x(0, 0) };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::aarch64::crypto::crc::crc32b as _;
//! use rust_isa::aarch64::crypto::crc::crc32h as _;
//! use rust_isa::aarch64::crypto::crc::crc32w as _;
//! use rust_isa::aarch64::crypto::crc::crc32x as _;
//! ```
//! <!-- doctest-inventory:end -->

unsafe extern "C" {
    fn rust_isa_crc_crc32b(crc: u32, value: u8) -> u32;
    fn rust_isa_crc_crc32h(crc: u32, value: u16) -> u32;
    fn rust_isa_crc_crc32w(crc: u32, value: u32) -> u32;
    fn rust_isa_crc_crc32x(crc: u32, value: u64) -> u32;
}

/// `crc32b`
///
/// Accumulate one byte into the CRC-32C state.
///
/// # Safety
///
/// The caller must enable CRC.
#[target_feature(enable = "crc")]
#[inline]
pub unsafe fn crc32b(crc: u32, value: u8) -> u32 {
    unsafe { rust_isa_crc_crc32b(crc, value) }
}

/// `crc32h`
///
/// Accumulate a 16-bit value into the CRC-32C state.
///
/// # Safety
///
/// The caller must enable CRC.
#[target_feature(enable = "crc")]
#[inline]
pub unsafe fn crc32h(crc: u32, value: u16) -> u32 {
    unsafe { rust_isa_crc_crc32h(crc, value) }
}

/// `crc32w`
///
/// Accumulate a 32-bit value into the CRC-32C state.
///
/// # Safety
///
/// The caller must enable CRC.
#[target_feature(enable = "crc")]
#[inline]
pub unsafe fn crc32w(crc: u32, value: u32) -> u32 {
    unsafe { rust_isa_crc_crc32w(crc, value) }
}

/// `crc32x`
///
/// Accumulate a 64-bit value into the CRC-32C state.
///
/// # Safety
///
/// The caller must enable CRC.
#[target_feature(enable = "crc")]
#[inline]
pub unsafe fn crc32x(crc: u32, value: u64) -> u32 {
    unsafe { rust_isa_crc_crc32x(crc, value) }
}
