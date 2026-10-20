//! FMA.

use crate::x86::simd::avx2::V256;

unsafe extern "C" {
    fn rust_isa_fma_vfmadd132ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd132ps`
///
/// Compute `a * b + c` for the eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX2 and FMA.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "fma")]
#[inline]
pub unsafe fn vfmadd132ps(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_fma_vfmadd132ps(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}
