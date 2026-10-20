//! AVX-512DQ integer instructions.

use crate::x86::simd::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512dq_vpmullq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullq`
///
/// Multiply the eight 64-bit integer lanes, keeping the low 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F and AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vpmullq(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vpmullq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
