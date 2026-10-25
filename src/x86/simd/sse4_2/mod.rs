//! SSE4.2.
//!
//! SSE4.2 is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable SSE4.2 with `#[target_feature(enable = "sse4.2")]` or
//! a global target feature.
//!
//! The values are [`V128`]. The CRC functions implement CRC-32C (Castagnoli).
//! The packed string comparison instructions use the low two bits of `imm` for
//! the element size, bits 3-2 for the aggregation mode, bits 5-4 for polarity,
//! and bit 6 for output selection.
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
//! use rust_isa::x86::simd::sse4_2::crc32_u16 as _;
//! use rust_isa::x86::simd::sse4_2::crc32_u32 as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::simd::sse4_2::crc32_u64 as _;
//! use rust_isa::x86::simd::sse4_2::crc32_u8 as _;
//! use rust_isa::x86::simd::sse4_2::pcmpestri as _;
//! use rust_isa::x86::simd::sse4_2::pcmpestrm as _;
//! use rust_isa::x86::simd::sse4_2::pcmpgtq as _;
//! use rust_isa::x86::simd::sse4_2::pcmpistri as _;
//! use rust_isa::x86::simd::sse4_2::pcmpistrm as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_sse42_pcmpgtq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse42_crc32_u8(crc: u32, value: u8) -> u32;
    fn rust_isa_sse42_crc32_u16(crc: u32, value: u16) -> u32;
    fn rust_isa_sse42_crc32_u32(crc: u32, value: u32) -> u32;
    #[cfg(target_arch = "x86_64")]
    fn rust_isa_sse42_crc32_u64(crc: u64, value: u64) -> u64;
    fn rust_isa_sse42_pcmpistri(a: *const u8, b: *const u8, imm: u32) -> i32;
    fn rust_isa_sse42_pcmpistrm(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
    fn rust_isa_sse42_pcmpestri(a: *const u8, la: u32, b: *const u8, lb: u32, imm: u32) -> i32;
    fn rust_isa_sse42_pcmpestrm(
        a: *const u8,
        la: u32,
        b: *const u8,
        lb: u32,
        imm: u32,
        out: *mut u8,
    );
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

/// `pcmpistri`
///
/// Compare two implicitly terminated byte strings and return the index selected
/// by `imm`.
///
/// # Safety
///
/// The caller must enable SSE4.2.
#[target_feature(enable = "sse4.2")]
#[inline]
pub unsafe fn pcmpistri(a: V128, b: V128, imm: u8) -> i32 {
    unsafe { rust_isa_sse42_pcmpistri(a.as_ptr(), b.as_ptr(), u32::from(imm)) }
}

/// `pcmpistrm`
///
/// Compare two implicitly terminated byte strings and return the mask selected
/// by `imm`.
///
/// # Safety
///
/// The caller must enable SSE4.2.
#[target_feature(enable = "sse4.2")]
#[inline]
pub unsafe fn pcmpistrm(a: V128, b: V128, imm: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse42_pcmpistrm(a.as_ptr(), b.as_ptr(), u32::from(imm), out.as_mut_ptr());
    }
    out
}

/// `pcmpestri`
///
/// Compare two explicit-length byte strings and return the index selected by
/// `imm`.
///
/// # Safety
///
/// The caller must enable SSE4.2.
#[target_feature(enable = "sse4.2")]
#[inline]
pub unsafe fn pcmpestri(a: V128, la: u32, b: V128, lb: u32, imm: u8) -> i32 {
    unsafe { rust_isa_sse42_pcmpestri(a.as_ptr(), la, b.as_ptr(), lb, u32::from(imm)) }
}

/// `pcmpestrm`
///
/// Compare two explicit-length byte strings and return the mask selected by
/// `imm`.
///
/// # Safety
///
/// The caller must enable SSE4.2.
#[target_feature(enable = "sse4.2")]
#[inline]
pub unsafe fn pcmpestrm(a: V128, la: u32, b: V128, lb: u32, imm: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse42_pcmpestrm(
            a.as_ptr(),
            la,
            b.as_ptr(),
            lb,
            u32::from(imm),
            out.as_mut_ptr(),
        );
    }
    out
}
