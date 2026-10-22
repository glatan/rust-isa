//! VAES.

use crate::x86::simd::avx2::V256;
use crate::x86::simd::avx512f::V512;
use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_vaes_vaesenc(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenc`
///
/// Perform one AES encryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenc(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_vaes_vaesenc(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesenc_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenc` on 128-bit values.
///
/// Perform one AES encryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenc_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vaes_vaesenc_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdec(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdec`
///
/// Perform one AES decryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdec(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_vaes_vaesdec(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdec_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdec` on 128-bit values.
///
/// Perform one AES decryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdec_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vaes_vaesdec_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesenclast(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenclast`
///
/// Perform the last AES encryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenclast(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_vaes_vaesenclast(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesenclast_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenclast` on 128-bit values.
///
/// Perform the last AES encryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenclast_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vaes_vaesenclast_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdeclast(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdeclast`
///
/// Perform the last AES decryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdeclast(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_vaes_vaesdeclast(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdeclast_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdeclast` on 128-bit values.
///
/// Perform the last AES decryption round on the 256-bit value.
///
/// # Safety
///
/// The caller must enable AVX2 and VAES.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdeclast_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vaes_vaesdeclast_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesenc_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenc` on 512-bit values.
///
/// Perform one AES encryption round on the 512-bit value.
///
/// # Safety
///
/// The caller must enable AVX-512F and VAES.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenc_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_vaes_vaesenc_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdec_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdec` on 512-bit values.
///
/// Perform one AES decryption round on the 512-bit value.
///
/// # Safety
///
/// The caller must enable AVX-512F and VAES.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdec_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_vaes_vaesdec_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesenclast_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesenclast` on 512-bit values.
///
/// Perform the last AES encryption round on the 512-bit value.
///
/// # Safety
///
/// The caller must enable AVX-512F and VAES.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesenclast_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_vaes_vaesenclast_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vaes_vaesdeclast_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaesdeclast` on 512-bit values.
///
/// Perform the last AES decryption round on the 512-bit value.
///
/// # Safety
///
/// The caller must enable AVX-512F and VAES.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "vaes")]
#[inline]
pub unsafe fn vaesdeclast_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_vaes_vaesdeclast_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
