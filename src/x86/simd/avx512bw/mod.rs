//! AVX-512BW integer instructions.

use crate::x86::simd::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512bw_vpaddb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddb`
///
/// Add the 64 byte lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F and AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpaddb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpaddb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpsubb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubb`
///
/// Subtract the 64 byte lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F and AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpsubb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpsubb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmullw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullw`
///
/// Multiply the 32 16-bit integer lanes, keeping the low 16 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F and AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmullw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpmullw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
