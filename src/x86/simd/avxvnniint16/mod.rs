//! AVX-VNNI-INT16.
//!
//! AVX-VNNI-INT16 is not part of the x86-64 baseline. Every instruction wrapper is
//! `unsafe` and the caller must enable it with
//! `#[target_feature(enable = "avxvnniint16")]` or a global target feature.
//!
//! The values are [`V256`] or [`V128`]. Each instruction multiplies two
//! adjacent 16-bit products and accumulates into one 32-bit lane. The two
//! suffixes describe the signedness of the operands: `su` is signed/unsigned,
//! `us` is unsigned/signed, and `uu` is unsigned/unsigned. The `s` variants
//! saturate the 32-bit result. The `uu` variants use unsigned saturation.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avxvnniint16::vpdpwsud;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vpdpwsud(v, v, v) };
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avxvnniint16::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpdpwsud(v256, v256, v256) };
//! let _ = unsafe { vpdpwsud_v128(v128, v128, v128) };
//! let _ = unsafe { vpdpwsuds(v256, v256, v256) };
//! let _ = unsafe { vpdpwsuds_v128(v128, v128, v128) };
//! let _ = unsafe { vpdpwusd(v256, v256, v256) };
//! let _ = unsafe { vpdpwusd_v128(v128, v128, v128) };
//! let _ = unsafe { vpdpwusds(v256, v256, v256) };
//! let _ = unsafe { vpdpwusds_v128(v128, v128, v128) };
//! let _ = unsafe { vpdpwuud(v256, v256, v256) };
//! let _ = unsafe { vpdpwuud_v128(v128, v128, v128) };
//! let _ = unsafe { vpdpwuuds(v256, v256, v256) };
//! let _ = unsafe { vpdpwuuds_v128(v128, v128, v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avxvnniint16::vpdpwsud as _;
//! use rust_isa::x86::simd::avxvnniint16::vpdpwsud_v128 as _;
//! use rust_isa::x86::simd::avxvnniint16::vpdpwsuds as _;
//! use rust_isa::x86::simd::avxvnniint16::vpdpwsuds_v128 as _;
//! use rust_isa::x86::simd::avxvnniint16::vpdpwusd as _;
//! use rust_isa::x86::simd::avxvnniint16::vpdpwusd_v128 as _;
//! use rust_isa::x86::simd::avxvnniint16::vpdpwusds as _;
//! use rust_isa::x86::simd::avxvnniint16::vpdpwusds_v128 as _;
//! use rust_isa::x86::simd::avxvnniint16::vpdpwuud as _;
//! use rust_isa::x86::simd::avxvnniint16::vpdpwuud_v128 as _;
//! use rust_isa::x86::simd::avxvnniint16::vpdpwuuds as _;
//! use rust_isa::x86::simd::avxvnniint16::vpdpwuuds_v128 as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::avx2::V256;
use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_avxvnniint16_vpdpwsud(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint16_vpdpwsuds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint16_vpdpwusd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint16_vpdpwusds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint16_vpdpwuud(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint16_vpdpwuuds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnniint16_vpdpwsud_v128(
        acc: *const u8,
        a: *const u8,
        b: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avxvnniint16_vpdpwsuds_v128(
        acc: *const u8,
        a: *const u8,
        b: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avxvnniint16_vpdpwusd_v128(
        acc: *const u8,
        a: *const u8,
        b: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avxvnniint16_vpdpwusds_v128(
        acc: *const u8,
        a: *const u8,
        b: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avxvnniint16_vpdpwuud_v128(
        acc: *const u8,
        a: *const u8,
        b: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avxvnniint16_vpdpwuuds_v128(
        acc: *const u8,
        a: *const u8,
        b: *const u8,
        out: *mut u8,
    );
}

/// `vpdpwsud`
///
/// Multiply signed words in `a` by unsigned words in `b` and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwsud(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwsud(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwsuds`
///
/// Like [`vpdpwsud`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwsuds(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwsuds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwusd`
///
/// Multiply unsigned words in `a` by signed words in `b` and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwusd(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwusd(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwusds`
///
/// Like [`vpdpwusd`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwusds(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwusds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwuud`
///
/// Multiply unsigned words and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwuud(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwuud(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwuuds`
///
/// Like [`vpdpwuud`] with unsigned saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwuuds(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwuuds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwsud` on 128-bit values.
///
/// Multiply signed words in `a` by unsigned words in `b` and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwsud_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwsud_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwsuds` on 128-bit values.
///
/// Like [`vpdpwsud_v128`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwsuds_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwsuds_v128(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `vpdpwusd` on 128-bit values.
///
/// Multiply unsigned words in `a` by signed words in `b` and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwusd_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwusd_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwusds` on 128-bit values.
///
/// Like [`vpdpwusd_v128`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwusds_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwusds_v128(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `vpdpwuud` on 128-bit values.
///
/// Multiply unsigned words and accumulate.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwuud_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwuud_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwuuds` on 128-bit values.
///
/// Like [`vpdpwuud_v128`] with unsigned saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI-INT16.
#[target_feature(enable = "avxvnniint16")]
#[inline]
pub unsafe fn vpdpwuuds_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnniint16_vpdpwuuds_v128(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}
