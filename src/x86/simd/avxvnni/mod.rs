//! AVX-VNNI.
//!
//! AVX-VNNI is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable AVX-VNNI with
//! `#[target_feature(enable = "avxvnni")]` or a global target feature.
//!
//! The values are [`V256`] or [`V128`]. The dot-product operations treat the first operand
//! as unsigned bytes or signed words and accumulate into 32-bit lanes. The
//! `*s` variants saturate the 32-bit result.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avxvnni::vpdpbusd;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vpdpbusd(v, v, v) };
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avxvnni::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpdpbusd(v256, v256, v256) };
//! let _ = unsafe { vpdpbusd_v128(v128, v128, v128) };
//! let _ = unsafe { vpdpbusds(v256, v256, v256) };
//! let _ = unsafe { vpdpbusds_v128(v128, v128, v128) };
//! let _ = unsafe { vpdpwssd(v256, v256, v256) };
//! let _ = unsafe { vpdpwssd_v128(v128, v128, v128) };
//! let _ = unsafe { vpdpwssds(v256, v256, v256) };
//! let _ = unsafe { vpdpwssds_v128(v128, v128, v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avxvnni::vpdpbusd as _;
//! use rust_isa::x86::simd::avxvnni::vpdpbusd_v128 as _;
//! use rust_isa::x86::simd::avxvnni::vpdpbusds as _;
//! use rust_isa::x86::simd::avxvnni::vpdpbusds_v128 as _;
//! use rust_isa::x86::simd::avxvnni::vpdpwssd as _;
//! use rust_isa::x86::simd::avxvnni::vpdpwssd_v128 as _;
//! use rust_isa::x86::simd::avxvnni::vpdpwssds as _;
//! use rust_isa::x86::simd::avxvnni::vpdpwssds_v128 as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::avx2::V256;
use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_avxvnni_vpdpbusd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnni_vpdpbusds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnni_vpdpwssd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnni_vpdpwssds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnni_vpdpbusd_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnni_vpdpbusds_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnni_vpdpwssd_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnni_vpdpwssds_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusd`
///
/// Multiply unsigned bytes in `a` by signed bytes in `b`, add adjacent
/// products, and accumulate into 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable AVX-VNNI.
#[target_feature(enable = "avxvnni")]
#[inline]
pub unsafe fn vpdpbusd(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnni_vpdpbusd(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbusds`
///
/// Like [`vpdpbusd`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI.
#[target_feature(enable = "avxvnni")]
#[inline]
pub unsafe fn vpdpbusds(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnni_vpdpbusds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwssd`
///
/// Multiply signed 16-bit lanes, add adjacent products, and accumulate into
/// 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable AVX-VNNI.
#[target_feature(enable = "avxvnni")]
#[inline]
pub unsafe fn vpdpwssd(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnni_vpdpwssd(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwssds`
///
/// Like [`vpdpwssd`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI.
#[target_feature(enable = "avxvnni")]
#[inline]
pub unsafe fn vpdpwssds(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxvnni_vpdpwssds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbusd` on 128-bit values.
///
/// Multiply unsigned bytes in `a` by signed bytes in `b`, add adjacent
/// products, and accumulate into 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable AVX-VNNI.
#[target_feature(enable = "avxvnni")]
#[inline]
pub unsafe fn vpdpbusd_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnni_vpdpbusd_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpbusds` on 128-bit values.
///
/// Like [`vpdpbusd_v128`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI.
#[target_feature(enable = "avxvnni")]
#[inline]
pub unsafe fn vpdpbusds_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnni_vpdpbusds_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwssd` on 128-bit values.
///
/// Multiply signed 16-bit lanes, add adjacent products, and accumulate into
/// 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable AVX-VNNI.
#[target_feature(enable = "avxvnni")]
#[inline]
pub unsafe fn vpdpwssd_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnni_vpdpwssd_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpdpwssds` on 128-bit values.
///
/// Like [`vpdpwssd_v128`] with signed saturation of the 32-bit result.
///
/// # Safety
///
/// The caller must enable AVX-VNNI.
#[target_feature(enable = "avxvnni")]
#[inline]
pub unsafe fn vpdpwssds_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxvnni_vpdpwssds_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
