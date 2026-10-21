//! SSE4.2.
//!
//! SSE4.2 is not part of the x86-64 baseline. Every operation is `unsafe` and
//! the caller must enable SSE4.2 with `#[target_feature(enable = "sse4.2")]` or
//! a global target feature.
//!
//! The values are [`V128`]. The CRC functions implement CRC-32C (Castagnoli).
//! The packed string comparison instructions are intentionally omitted because
//! their control byte and result registers do not fit the value API.

use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_sse42_pcmpgtq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse42_crc32_u8(crc: u32, value: u8) -> u32;
    fn rust_isa_sse42_crc32_u16(crc: u32, value: u16) -> u32;
    fn rust_isa_sse42_crc32_u32(crc: u32, value: u32) -> u32;
    #[cfg(target_arch = "x86_64")]
    fn rust_isa_sse42_crc32_u64(crc: u64, value: u64) -> u64;
}

/// `pcmpgtq`
///
/// Compare the two signed 64-bit lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable SSE4.2.
#[target_feature(enable = "sse4.2")]
#[inline]
pub unsafe fn pcmpgtq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse42_pcmpgtq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `crc32` with an 8-bit value.
///
/// Accumulate `value` into the CRC-32C state `crc`.
///
/// # Safety
///
/// The caller must enable SSE4.2.
#[target_feature(enable = "sse4.2")]
#[inline]
pub unsafe fn crc32_u8(crc: u32, value: u8) -> u32 {
    unsafe { rust_isa_sse42_crc32_u8(crc, value) }
}

/// `crc32` with a 16-bit value.
///
/// Accumulate `value` into the CRC-32C state `crc`.
///
/// # Safety
///
/// The caller must enable SSE4.2.
#[target_feature(enable = "sse4.2")]
#[inline]
pub unsafe fn crc32_u16(crc: u32, value: u16) -> u32 {
    unsafe { rust_isa_sse42_crc32_u16(crc, value) }
}

/// `crc32` with a 32-bit value.
///
/// Accumulate `value` into the CRC-32C state `crc`.
///
/// # Safety
///
/// The caller must enable SSE4.2.
#[target_feature(enable = "sse4.2")]
#[inline]
pub unsafe fn crc32_u32(crc: u32, value: u32) -> u32 {
    unsafe { rust_isa_sse42_crc32_u32(crc, value) }
}

/// `crc32` with a 64-bit value.
///
/// Accumulate `value` into the CRC-32C state `crc`. This instruction exists
/// only in 64-bit mode.
///
/// # Safety
///
/// The caller must enable SSE4.2.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.2")]
#[inline]
pub unsafe fn crc32_u64(crc: u64, value: u64) -> u64 {
    unsafe { rust_isa_sse42_crc32_u64(crc, value) }
}
