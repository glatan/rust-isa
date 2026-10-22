//! AArch64 CRC instructions.
//!
//! CRC is not part of the AArch64 baseline. Every operation is `unsafe` and the
//! caller must enable CRC with `#[target_feature(enable = "crc")]` or a global
//! target feature.
//!
//! The functions implement CRC-32C (Castagnoli). The accumulator is the raw
//! hardware state; the caller applies the final XOR required by the protocol.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).

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
