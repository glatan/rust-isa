//! AVX-VNNI.
//!
//! AVX-VNNI is not part of the x86-64 baseline. Every operation is `unsafe` and
//! the caller must enable AVX-VNNI with
//! `#[target_feature(enable = "avxvnni")]` or a global target feature.
//!
//! The values are [`V256`]. The dot-product operations treat the first operand
//! as unsigned bytes or signed words and accumulate into 32-bit lanes. The
//! `*s` variants saturate the 32-bit result.

use super::avx2::V256;

unsafe extern "C" {
    fn rust_isa_avxvnni_vpdpbusd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnni_vpdpbusds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnni_vpdpwssd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avxvnni_vpdpwssds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
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
