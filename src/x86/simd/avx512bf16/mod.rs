//! AVX-512 BF16.
//!
//! AVX-512 BF16 is not part of the x86-64 baseline. Every operation is
//! `unsafe` and the caller must enable it with
//! `#[target_feature(enable = "avx512bf16")]` (which implies AVX-512 F) or a
//! global target feature.
//!
//! The module converts 32-bit float lanes to bfloat16 and accumulates
//! bfloat16 dot products into 32-bit float lanes. The dot-product pairs are
//! formed within each 32-bit lane; the result is added to `acc`.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).

use super::avx2::V256;
use super::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512bf16_vcvtneps2bf16(a: *const u8, out: *mut u8);
    fn rust_isa_avx512bf16_vdpbf16ps(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vcvtneps2bf16`
///
/// Convert sixteen 32-bit float lanes to sixteen bfloat16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512 BF16.
#[target_feature(enable = "avx512bf16")]
#[inline]
pub unsafe fn vcvtneps2bf16(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512bf16_vcvtneps2bf16(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdpbf16ps`
///
/// Multiply pairs of bfloat16 lanes, add the products, and accumulate into the
/// 32-bit float lanes of `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512 BF16.
#[target_feature(enable = "avx512bf16")]
#[inline]
pub unsafe fn vdpbf16ps(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bf16_vdpbf16ps(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
