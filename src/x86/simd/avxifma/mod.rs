//! AVX-IFMA.
//!
//! AVX-IFMA is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable AVX-IFMA with
//! `#[target_feature(enable = "avxifma")]` or a global target feature.
//!
//! The values are [`V256`] or [`V128`]. The instructions multiply the low 52 bits of each
//! unsigned 64-bit lane and accumulate. [`vpmadd52luq`] adds the low 52 bits of
//! the product and [`vpmadd52huq`] adds bits 52..104.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

use super::avx2::V256;
use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_avxifma_vpmadd52luq(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxifma_vpmadd52huq(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxifma_vpmadd52luq_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxifma_vpmadd52huq_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmadd52luq`
///
/// Multiply the low 52 bits of each 64-bit lane and add the low product bits to
/// `acc`.
///
/// # Safety
///
/// The caller must enable AVX-IFMA.
#[target_feature(enable = "avxifma")]
#[inline]
pub unsafe fn vpmadd52luq(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxifma_vpmadd52luq(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmadd52huq`
///
/// Multiply the low 52 bits of each 64-bit lane and add bits 52..104 of the
/// product to `acc`.
///
/// # Safety
///
/// The caller must enable AVX-IFMA.
#[target_feature(enable = "avxifma")]
#[inline]
pub unsafe fn vpmadd52huq(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avxifma_vpmadd52huq(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmadd52luq` on 128-bit values.
///
/// Multiply the low 52 bits of each 64-bit lane and add the low product bits to
/// `acc`.
///
/// # Safety
///
/// The caller must enable AVX-IFMA.
#[target_feature(enable = "avxifma")]
#[inline]
pub unsafe fn vpmadd52luq_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxifma_vpmadd52luq_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmadd52huq` on 128-bit values.
///
/// Multiply the low 52 bits of each 64-bit lane and add bits 52..104 of the
/// product to `acc`.
///
/// # Safety
///
/// The caller must enable AVX-IFMA.
#[target_feature(enable = "avxifma")]
#[inline]
pub unsafe fn vpmadd52huq_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avxifma_vpmadd52huq_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
