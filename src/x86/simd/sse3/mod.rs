//! SSE3.
//!
//! SSE3 is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable SSE3 with `#[target_feature(enable = "sse3")]` or a global
//! target feature.
//!
//! The values are [`V128`]. The instruction decides what the lanes mean.
//! The `*ps` functions use four 32-bit float lanes and the `*pd` functions use
//! two 64-bit float lanes. Memory operations do not require alignment.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::x86::simd::sse3::addsubps;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { addsubps(v, v) }.to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::x86::simd::sse3::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { addsubpd(v128, v128) };
//! let _ = unsafe { addsubps(v128, v128) };
//! let _ = unsafe { haddpd(v128, v128) };
//! let _ = unsafe { haddps(v128, v128) };
//! let _ = unsafe { hsubpd(v128, v128) };
//! let _ = unsafe { hsubps(v128, v128) };
//! let _ = unsafe { lddqu(storage.0.as_ptr().cast()) };
//! let _ = unsafe { movddup(v128) };
//! let _ = unsafe { movshdup(v128) };
//! let _ = unsafe { movsldup(v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::sse3::addsubpd as _;
//! use rust_isa::x86::simd::sse3::addsubps as _;
//! use rust_isa::x86::simd::sse3::haddpd as _;
//! use rust_isa::x86::simd::sse3::haddps as _;
//! use rust_isa::x86::simd::sse3::hsubpd as _;
//! use rust_isa::x86::simd::sse3::hsubps as _;
//! use rust_isa::x86::simd::sse3::lddqu as _;
//! use rust_isa::x86::simd::sse3::movddup as _;
//! use rust_isa::x86::simd::sse3::movshdup as _;
//! use rust_isa::x86::simd::sse3::movsldup as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_sse3_addsubps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse3_addsubpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse3_haddps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse3_haddpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse3_hsubps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse3_hsubpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse3_lddqu(p: *const u8, out: *mut u8);
    fn rust_isa_sse3_movsldup(a: *const u8, out: *mut u8);
    fn rust_isa_sse3_movshdup(a: *const u8, out: *mut u8);
    fn rust_isa_sse3_movddup(a: *const u8, out: *mut u8);
}

/// `addsubps`
///
/// Subtract the even 32-bit float lanes and add the odd lanes.
///
/// # Safety
///
/// The caller must enable SSE3.
#[target_feature(enable = "sse3")]
#[inline]
pub unsafe fn addsubps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse3_addsubps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `addsubpd`
///
/// Subtract the low 64-bit float lane and add the high lane.
///
/// # Safety
///
/// The caller must enable SSE3.
#[target_feature(enable = "sse3")]
#[inline]
pub unsafe fn addsubpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse3_addsubpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `haddps`
///
/// Add adjacent 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable SSE3.
#[target_feature(enable = "sse3")]
#[inline]
pub unsafe fn haddps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse3_haddps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `haddpd`
///
/// Add adjacent 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable SSE3.
#[target_feature(enable = "sse3")]
#[inline]
pub unsafe fn haddpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse3_haddpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `hsubps`
///
/// Subtract adjacent 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable SSE3.
#[target_feature(enable = "sse3")]
#[inline]
pub unsafe fn hsubps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse3_hsubps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `hsubpd`
///
/// Subtract adjacent 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable SSE3.
#[target_feature(enable = "sse3")]
#[inline]
pub unsafe fn hsubpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse3_hsubpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `lddqu`
///
/// Load 16 bytes from an unaligned address.
///
/// # Safety
///
/// The caller must enable SSE3. `p` must be valid for reading 16 bytes.
#[target_feature(enable = "sse3")]
#[inline]
pub unsafe fn lddqu(p: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse3_lddqu(p, out.as_mut_ptr());
    }
    out
}

/// `movsldup`
///
/// Duplicate the even 32-bit float lanes into both halves.
///
/// # Safety
///
/// The caller must enable SSE3.
#[target_feature(enable = "sse3")]
#[inline]
pub unsafe fn movsldup(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse3_movsldup(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `movshdup`
///
/// Duplicate the odd 32-bit float lanes into both halves.
///
/// # Safety
///
/// The caller must enable SSE3.
#[target_feature(enable = "sse3")]
#[inline]
pub unsafe fn movshdup(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse3_movshdup(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `movddup`
///
/// Duplicate the low 64-bit float lane.
///
/// # Safety
///
/// The caller must enable SSE3.
#[target_feature(enable = "sse3")]
#[inline]
pub unsafe fn movddup(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse3_movddup(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
