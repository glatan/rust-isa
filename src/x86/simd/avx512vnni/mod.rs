//! Instruction wrappers.

use crate::x86::simd::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512vnni_vpdpbusd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusd`
/// Multiply unsigned bytes by signed bytes and add adjacent products to `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI.
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

unsafe extern "C" {
    fn rust_isa_avx512vnni_vpdpbusds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusds`
/// Multiply unsigned bytes by signed bytes, add adjacent products, and accumulate into `acc` with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[inline]
pub unsafe fn vpdpbusds(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vnni_vpdpbusds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vnni_vpdpwssd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssd`
/// Multiply signed 16-bit lane pairs, add the products, and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[inline]
pub unsafe fn vpdpwssd(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vnni_vpdpwssd(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vnni_vpdpwssds(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssds`
/// Multiply signed 16-bit lane pairs, add the products, and accumulate into `acc` with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[inline]
pub unsafe fn vpdpwssds(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vnni_vpdpwssds(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
