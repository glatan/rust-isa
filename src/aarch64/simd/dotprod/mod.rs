//! AArch64 dot product.

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_dotprod_sdot(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_dotprod_udot(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `sdot`
///
/// Multiply unsigned bytes by signed bytes, add adjacent products, and add
/// the result to `acc`.
///
/// # Safety
///
/// The caller must enable DOTPROD.
#[target_feature(enable = "dotprod")]
#[inline]
pub unsafe fn sdot(a: V128, b: V128, acc: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_dotprod_sdot(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `udot`
///
/// Multiply unsigned bytes, add adjacent products, and add the result to
/// `acc`.
///
/// # Safety
///
/// The caller must enable DOTPROD.
#[target_feature(enable = "dotprod")]
#[inline]
pub unsafe fn udot(a: V128, b: V128, acc: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_dotprod_udot(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
