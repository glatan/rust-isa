//! AVX-512VL.

use crate::x86::simd::avx2::V256;
use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmullq_v128(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx512vl_vpmullq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullq` on 128-bit values.
///
/// Multiply the two 64-bit integer lanes, keeping the low 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, and AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmullq_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmullq_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmullq` on 256-bit values.
///
/// Multiply the four 64-bit integer lanes, keeping the low 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, and AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmullq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmullq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
