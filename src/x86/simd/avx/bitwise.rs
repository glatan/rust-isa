//! Bitwise instructions.

use super::V256;
unsafe extern "C" {
    fn rust_isa_avx_vandpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vandnps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vandnpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vxorpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vandps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vorps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vxorps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vandpd`
///
/// Bitwise AND of the 256-bit values.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vandpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vandpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vandnps`
///
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vandnps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vandnps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vandnpd`
///
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vandnpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vandnpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vxorpd`
///
/// Bitwise XOR of the 256-bit values.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vxorpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vxorpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vandps`
///
/// Bitwise AND of the 256-bit values.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vandps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vandps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vorps`
///
/// Bitwise OR of the 256-bit values.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vorps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vorps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vxorps`
///
/// Bitwise XOR of the 256-bit values.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vxorps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vxorps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
