//! AVX-512VNNI instructions.

use crate::x86::simd::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512vnni_vpdpbusd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusd`
///
/// Multiply unsigned bytes by signed bytes and add adjacent products to `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F and AVX-512VNNI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[inline]
pub unsafe fn vpdpbusd(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vnni_vpdpbusd(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
