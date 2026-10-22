//! 32-bit ARM CRC.
//!
//! CRC is unstable on this target. Every operation is `unsafe` and the caller
//! must compile the whole crate with the CRC feature enabled.
//!
//! The functions implement CRC-32 (IEEE 802.3), not CRC-32C. The accumulator is
//! the raw hardware state.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).

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
