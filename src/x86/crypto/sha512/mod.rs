//! SHA-512 instructions.
//!
//! SHA-512 is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable SHA-512 with `#[target_feature(enable = "sha512")]`
//! or a global target feature.
//!
//! The state values are [`V256`]. The round constants are [`V128`].
//!
//! The module contains the message schedule and two-round instructions. The
//! SHA-512 instruction set does not define a full compression function; the
//! caller combines these operations with the SHA-512 specification.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

use crate::x86::simd::avx2::V256;
use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_sha512_vsha512msg1(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sha512_vsha512msg2(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sha512_vsha512rnds2(a: *const u8, b: *const u8, k: *const u8, out: *mut u8);
}

/// `vsha512msg1`
///
/// Perform the first part of the SHA-512 message-schedule update.
///
/// # Safety
///
/// The caller must enable SHA-512.
#[target_feature(enable = "sha512")]
#[inline]
pub unsafe fn vsha512msg1(a: V256, b: V128) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_sha512_vsha512msg1(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsha512msg2`
///
/// Perform the second part of the SHA-512 message-schedule update.
///
/// # Safety
///
/// The caller must enable SHA-512.
#[target_feature(enable = "sha512")]
#[inline]
pub unsafe fn vsha512msg2(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_sha512_vsha512msg2(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsha512rnds2`
///
/// Perform two SHA-512 rounds with the round constants in `k`.
///
/// # Safety
///
/// The caller must enable SHA-512.
#[target_feature(enable = "sha512")]
#[inline]
pub unsafe fn vsha512rnds2(a: V256, b: V256, k: V128) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_sha512_vsha512rnds2(a.as_ptr(), b.as_ptr(), k.as_ptr(), out.as_mut_ptr());
    }
    out
}
