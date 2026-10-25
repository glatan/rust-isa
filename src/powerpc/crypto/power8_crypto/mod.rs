//! PowerPC Power8 crypto instructions.
//!
//! Power8 crypto instructions are not part of the PowerPC baseline. Every
//! operation is `unsafe` and the caller must enable the Power8 feature set. The
//! C file is compiled with clang and `-mcpu=power8`.
//!
//! The values are [`V128`]. The AES round, S-box, and SHA sigma instructions
//! operate on the PowerPC big-endian byte order used by the instruction set.
//! Immediate operands are const generics unless documented otherwise.
//!
//! Instruction behavior follows the [Power ISA](https://openpowerfoundation.org/specifications/isa/).
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::powerpc::crypto::power8_crypto::vcipher;
//! use rust_isa::powerpc::simd::altivec::V128;
//!
//! let zero = V128::from_bytes([0; 16]);
//! assert_ne!(unsafe { vcipher(zero, zero) }.to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::powerpc::crypto::power8_crypto::*;
//! use rust_isa::powerpc::simd::altivec::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { cipher(v128, v128) };
//! let _ = unsafe { cipherlast(v128, v128) };
//! let _ = unsafe { ncipher(v128, v128) };
//! let _ = unsafe { ncipherlast(v128, v128) };
//! let _ = unsafe { vcipher(v128, v128) };
//! let _ = unsafe { vcipherlast(v128, v128) };
//! let _ = unsafe { vncipher(v128, v128) };
//! let _ = unsafe { vncipherlast(v128, v128) };
//! let _ = unsafe { vpermxor(v128, v128, v128) };
//! let _ = unsafe { vpmsumb(v128, v128) };
//! let _ = unsafe { vpmsumd(v128, v128) };
//! let _ = unsafe { vpmsumh(v128, v128) };
//! let _ = unsafe { vpmsumw(v128, v128) };
//! let _ = unsafe { vsbox(v128) };
//! let _ = unsafe { vshasigmad::<0, 0>(v128) };
//! let _ = unsafe { vshasigmaw::<0, 0>(v128) };
//! ```
//!
//! </details>
//!
//! <details>
//! <summary>Instruction inventory</summary>
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::powerpc::crypto::power8_crypto::cipher as _;
//! use rust_isa::powerpc::crypto::power8_crypto::cipherlast as _;
//! use rust_isa::powerpc::crypto::power8_crypto::ncipher as _;
//! use rust_isa::powerpc::crypto::power8_crypto::ncipherlast as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vcipher as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vcipherlast as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vncipher as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vncipherlast as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vpermxor as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vpmsumb as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vpmsumd as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vpmsumh as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vpmsumw as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vsbox as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vshasigmad as _;
//! use rust_isa::powerpc::crypto::power8_crypto::vshasigmaw as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use crate::powerpc::simd::altivec::V128;

unsafe extern "C" {
    fn rust_isa_power8_crypto_cipher(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_ncipher(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_cipherlast(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_ncipherlast(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_vsbox(a: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_vpmsumb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_vpmsumh(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_vpmsumw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_vpmsumd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_vpermxor(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_power8_crypto_vshasigmaw(a: *const u8, st_six: u32, out: *mut u8);
    fn rust_isa_power8_crypto_vshasigmad(a: *const u8, st_six: u32, out: *mut u8);
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

/// `vcipher`
///
/// Perform one AES encryption round.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vcipher(a: V128, b: V128) -> V128 {
    unsafe { cipher(a, b) }
}

/// `vncipher`
///
/// Perform one AES decryption round.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vncipher(a: V128, b: V128) -> V128 {
    unsafe { ncipher(a, b) }
}

/// `vcipherlast`
///
/// Perform the final AES encryption round.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vcipherlast(a: V128, b: V128) -> V128 {
    unsafe { cipherlast(a, b) }
}

/// `vncipherlast`
///
/// Perform the final AES decryption round.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vncipherlast(a: V128, b: V128) -> V128 {
    unsafe { ncipherlast(a, b) }
}

/// `vsbox`
///
/// Apply the AES S-box to every byte.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vsbox(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_vsbox(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmsumb`
///
/// Perform carry-less multiplication on adjacent byte pairs and XOR each pair
/// of products.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vpmsumb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_vpmsumb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmsumh`
///
/// Perform carry-less multiplication on adjacent halfword pairs and XOR each
/// pair of products.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vpmsumh(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_vpmsumh(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmsumw`
///
/// Perform carry-less multiplication on adjacent word pairs and XOR each pair
/// of products.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vpmsumw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_vpmsumw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmsumd`
///
/// Perform carry-less multiplication on both doubleword pairs and XOR the two
/// products.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vpmsumd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_vpmsumd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpermxor`
///
/// Select bytes from `a` and `b` using the nibbles of `c`, and XOR the selected
/// bytes. The high nibble of each control byte selects a byte from `a`, and the
/// low nibble selects a byte from `b`. Both indices count from the end of the
/// vectors.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vpermxor(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_vpermxor(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vshasigmaw`
///
/// Apply the SHA-256 sigma function selected by `ST` and `SIX` to each word.
/// `ST` must be `0` or `1`, and `SIX` must be in `0..=15`.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vshasigmaw<const ST: u32, const SIX: u32>(a: V128) -> V128 {
    const { assert!(ST <= 1, "ST must be 0 or 1") };
    const { assert!(SIX <= 15, "SIX must be in 0..=15") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_vshasigmaw(a.as_ptr(), (ST << 4) | SIX, out.as_mut_ptr());
    }
    out
}

/// `vshasigmad`
///
/// Apply the SHA-512 sigma function selected by `ST` and `SIX` to each
/// doubleword. `ST` must be `0` or `1`, and `SIX` must be in `0..=15`.
///
/// # Safety
///
/// The caller must enable the Power8 crypto feature set.
#[inline(always)]
pub unsafe fn vshasigmad<const ST: u32, const SIX: u32>(a: V128) -> V128 {
    const { assert!(ST <= 1, "ST must be 0 or 1") };
    const { assert!(SIX <= 15, "SIX must be in 0..=15") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power8_crypto_vshasigmad(a.as_ptr(), (ST << 4) | SIX, out.as_mut_ptr());
    }
    out
}
