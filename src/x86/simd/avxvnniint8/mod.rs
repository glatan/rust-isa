//! AVX-VNNI-INT8.
//!
//! AVX-VNNI-INT8 is not part of the x86-64 baseline. Every instruction wrapper is
//! `unsafe` and the caller must enable it with
//! `#[target_feature(enable = "avxvnniint8")]` or a global target feature.
//!
//! The values are [`V256`] or [`V128`]. Each instruction multiplies four byte
//! products and accumulates into one 32-bit lane. The suffix describes the
//! signedness of the two byte operands: `ss` is signed/signed, `su` is
//! signed/unsigned, and `uu` is unsigned/unsigned. The `s` variants saturate
//! the 32-bit result. The `uu` variants use unsigned saturation.
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
//! use rust_isa::x86::simd::avxvnniint8::vpdpbssd as _;
//! use rust_isa::x86::simd::avxvnniint8::vpdpbssd_v128 as _;
//! use rust_isa::x86::simd::avxvnniint8::vpdpbssds as _;
//! use rust_isa::x86::simd::avxvnniint8::vpdpbssds_v128 as _;
//! use rust_isa::x86::simd::avxvnniint8::vpdpbsud as _;
//! use rust_isa::x86::simd::avxvnniint8::vpdpbsud_v128 as _;
//! use rust_isa::x86::simd::avxvnniint8::vpdpbsuds as _;
//! use rust_isa::x86::simd::avxvnniint8::vpdpbsuds_v128 as _;
//! use rust_isa::x86::simd::avxvnniint8::vpdpbuud as _;
//! use rust_isa::x86::simd::avxvnniint8::vpdpbuud_v128 as _;
//! use rust_isa::x86::simd::avxvnniint8::vpdpbuuds as _;
//! use rust_isa::x86::simd::avxvnniint8::vpdpbuuds_v128 as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::avx2::V256;
use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_avxvnniint8_vpdpbssd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbssds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbsud(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbsuds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbuud(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbuuds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbssd_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbssds_v128(
        acc: *const u8,
        a: *const u8,
        b: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avxvnniint8_vpdpbsud_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbsuds_v128(
        acc: *const u8,
        a: *const u8,
        b: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avxvnniint8_vpdpbuud_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint8_vpdpbuuds_v128(
        acc: *const u8,
        a: *const u8,
        b: *const u8,
        out: *mut u8,
    );
}

/// `vpdpbssd`
///
/// Multiply signed bytes and accumulate into 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbssd(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbssd(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbssds`
///
/// Like [`vpdpbssd`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbssds(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbssds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbsud`
///
/// Multiply signed bytes in `a` by unsigned bytes in `b` and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbsud(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbsud(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbsuds`
///
/// Like [`vpdpbsud`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbsuds(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbsuds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbuud`
///
/// Multiply unsigned bytes and accumulate into 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbuud(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbuud(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbuuds`
///
/// Like [`vpdpbuud`] with unsigned saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbuuds(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbuuds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbssd` on 128-bit values.
///
/// Multiply signed bytes and accumulate into 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbssd_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbssd_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbssds` on 128-bit values.
///
/// Like [`vpdpbssd_v128`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbssds_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbssds_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbsud` on 128-bit values.
///
/// Multiply signed bytes in `a` by unsigned bytes in `b` and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbsud_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbsud_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbsuds` on 128-bit values.
///
/// Like [`vpdpbsud_v128`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbsuds_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbsuds_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbuud` on 128-bit values.
///
/// Multiply unsigned bytes and accumulate into 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbuud_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbuud_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbuuds` on 128-bit values.
///
/// Like [`vpdpbuud_v128`] with unsigned saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT8.
#[target_feature(enable = "avxvnniint8")]
#[inline]
pub unsafe fn vpdpbuuds_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint8_vpdpbuuds_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
