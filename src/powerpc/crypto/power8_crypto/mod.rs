//! PowerPC Power8 crypto instructions.
//!
//! Power8 crypto instructions are not part of the PowerPC baseline. Every
//! operation is `unsafe` and the caller must enable the Power8 feature set. The
//! C file is compiled with `-mcpu=power8`.
//!
//! The values are [`V128`]. The AES round instructions operate on the PowerPC
//! big-endian byte order used by the instruction set.

use crate::powerpc::simd::altivec::V128;

unsafe extern "C" {
    fn rust_isa_power8_crypto_cipher(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_ncipher(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_cipherlast(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_ncipherlast(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vcipher`
///
/// Perform one AES encryption round.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn cipher(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_cipher(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vncipher`
///
/// Perform one AES decryption round.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn ncipher(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_ncipher(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcipherlast`
///
/// Perform the final AES encryption round.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn cipherlast(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_cipherlast(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vncipherlast`
///
/// Perform the final AES decryption round.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn ncipherlast(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_ncipherlast(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
