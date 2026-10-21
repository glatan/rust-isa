//! Instruction wrappers.

use crate::x86::simd::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512vbmi_vpermb(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermb`
/// Permute the 64 bytes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[inline]
pub unsafe fn vpermb(a: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi_vpermb(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vbmi_vpermi2b(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2b`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[inline]
pub unsafe fn vpermi2b(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi_vpermi2b(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vbmi_vpermt2b(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2b`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[inline]
pub unsafe fn vpermt2b(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi_vpermt2b(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}
