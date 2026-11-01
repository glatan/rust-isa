//! AVX512BMM (bit matrix multiply).
//!
//! This extension is not part of the x86-64 baseline. The caller must compile
//! the whole crate with `-C target-feature=+avx512bmm`; 128-bit and 256-bit
//! forms also require `+avx512vl`. The CPU must support AVX512BMM.
//!
//! rustc does not accept `#[target_feature(enable = "avx512bmm")]` on this
//! target, so the wrappers do not carry a target-feature attribute.
//!
//! The values are [`V128`], [`V256`], or [`V512`]. The instructions operate on
//! bits, not on integer or floating-point lanes.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! Instructions not listed in this module are not exposed.

//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512bmm::*;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vbitrevb_v128(v128) };
//! let _ = unsafe { vbitrevb_v256(v256) };
//! let _ = unsafe { vbitrevb_v512(v512) };
//! let _ = unsafe { vbmacor16x16x16_v256(v256, v256, v256) };
//! let _ = unsafe { vbmacor16x16x16_v512(v512, v512, v512) };
//! let _ = unsafe { vbmacxor16x16x16_v256(v256, v256, v256) };
//! let _ = unsafe { vbmacxor16x16x16_v512(v512, v512, v512) };
//! ```
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avx512bmm::vbitrevb_v128 as _;
//! use rust_isa::x86::simd::avx512bmm::vbitrevb_v256 as _;
//! use rust_isa::x86::simd::avx512bmm::vbitrevb_v512 as _;
//! use rust_isa::x86::simd::avx512bmm::vbmacor16x16x16_v256 as _;
//! use rust_isa::x86::simd::avx512bmm::vbmacor16x16x16_v512 as _;
//! use rust_isa::x86::simd::avx512bmm::vbmacxor16x16x16_v256 as _;
//! use rust_isa::x86::simd::avx512bmm::vbmacxor16x16x16_v512 as _;
//! ```
//!
//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512bmm::vbitrevb_v128;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vbitrevb_v128(v) };
//! ```

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avx512bmm::V128 as _;
//! use rust_isa::x86::simd::avx512bmm::V256 as _;
//! use rust_isa::x86::simd::avx512bmm::V512 as _;
//! use rust_isa::x86::simd::avx512bmm::vbitrevb_v128 as _;
//! use rust_isa::x86::simd::avx512bmm::vbitrevb_v256 as _;
//! use rust_isa::x86::simd::avx512bmm::vbitrevb_v512 as _;
//! use rust_isa::x86::simd::avx512bmm::vbmacor16x16x16_v256 as _;
//! use rust_isa::x86::simd::avx512bmm::vbmacor16x16x16_v512 as _;
//! use rust_isa::x86::simd::avx512bmm::vbmacxor16x16x16_v256 as _;
//! use rust_isa::x86::simd::avx512bmm::vbmacxor16x16x16_v512 as _;
//! ```
//! <!-- doctest-inventory:end -->

pub use crate::x86::simd::avx::V256;
pub use crate::x86::simd::avx512f::V512;
pub use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_avx512bmm_vbitrevb_v128(a: *const u8, out: *mut u8);
    fn rust_isa_avx512bmm_vbitrevb_v256(a: *const u8, out: *mut u8);
    fn rust_isa_avx512bmm_vbitrevb_v512(a: *const u8, out: *mut u8);
    fn rust_isa_avx512bmm_vbmacor16x16x16_v256(
        a: *const u8,
        b: *const u8,
        c: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avx512bmm_vbmacor16x16x16_v512(
        a: *const u8,
        b: *const u8,
        c: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avx512bmm_vbmacxor16x16x16_v256(
        a: *const u8,
        b: *const u8,
        c: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avx512bmm_vbmacxor16x16x16_v512(
        a: *const u8,
        b: *const u8,
        c: *const u8,
        out: *mut u8,
    );
}

/// `vbitrevb` (128-bit).
///
/// Reverse the bits in each byte.
///
/// # Safety
///
/// The caller must compile the whole crate with
/// `-C target-feature=+avx512bmm,+avx512vl` and the CPU must support AVX512BMM.
#[inline]
pub unsafe fn vbitrevb_v128(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512bmm_vbitrevb_v128(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitrevb` (256-bit).
///
/// Reverse the bits in each byte.
///
/// # Safety
///
/// The caller must compile the whole crate with
/// `-C target-feature=+avx512bmm,+avx512vl` and the CPU must support AVX512BMM.
#[inline]
pub unsafe fn vbitrevb_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512bmm_vbitrevb_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitrevb` (512-bit).
///
/// Reverse the bits in each byte.
///
/// # Safety
///
/// The caller must compile the whole crate with
/// `-C target-feature=+avx512bmm` and the CPU must support AVX512BMM.
#[inline]
pub unsafe fn vbitrevb_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bmm_vbitrevb_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbmacor16x16x16` (256-bit).
///
/// Multiply two 16x16 bit matrices and OR the product into `c`.
///
/// # Safety
///
/// The caller must compile the whole crate with
/// `-C target-feature=+avx512bmm,+avx512vl` and the CPU must support AVX512BMM.
#[inline]
pub unsafe fn vbmacor16x16x16_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512bmm_vbmacor16x16x16_v256(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `vbmacor16x16x16` (512-bit).
///
/// Multiply two pairs of 16x16 bit matrices and OR each product into `c`.
///
/// # Safety
///
/// The caller must compile the whole crate with
/// `-C target-feature=+avx512bmm` and the CPU must support AVX512BMM.
#[inline]
pub unsafe fn vbmacor16x16x16_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bmm_vbmacor16x16x16_v512(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `vbmacxor16x16x16` (256-bit).
///
/// Multiply two 16x16 bit matrices and XOR the product into `c`.
///
/// # Safety
///
/// The caller must compile the whole crate with
/// `-C target-feature=+avx512bmm,+avx512vl` and the CPU must support AVX512BMM.
#[inline]
pub unsafe fn vbmacxor16x16x16_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512bmm_vbmacxor16x16x16_v256(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `vbmacxor16x16x16` (512-bit).
///
/// Multiply two pairs of 16x16 bit matrices and XOR each product into `c`.
///
/// # Safety
///
/// The caller must compile the whole crate with
/// `-C target-feature=+avx512bmm` and the CPU must support AVX512BMM.
#[inline]
pub unsafe fn vbmacxor16x16x16_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bmm_vbmacxor16x16x16_v512(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}
