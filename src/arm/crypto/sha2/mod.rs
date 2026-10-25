//! 32-bit ARM SHA-2.
//!
//! SHA-2 is unstable on this target. Every instruction wrapper is `unsafe` and the caller
//! must compile the whole crate with the SHA-2 feature enabled.
//!
//! The values are [`V128`]. The caller combines the round and schedule
//! instructions with the SHA-256 specification.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::arm::crypto::sha2::sha256h;
//! use rust_isa::arm::simd::neon::V128;
//!
//! let a = V128::from_bytes([1; 16]);
//! let b = V128::from_bytes([2; 16]);
//! let c = V128::from_bytes([3; 16]);
//! let out = unsafe { sha256h(a, b, c) };
//! assert_ne!(out.to_bytes(), [0; 16]);
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::arm::crypto::sha2::sha256h as _;
//! use rust_isa::arm::crypto::sha2::sha256h2 as _;
//! use rust_isa::arm::crypto::sha2::sha256su0 as _;
//! use rust_isa::arm::crypto::sha2::sha256su1 as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::arm::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_arm_sha2_sha256h(
        hash_abcd: *const u8,
        hash_efgh: *const u8,
        wk: *const u8,
        out: *mut u8,
    );
    fn rust_isa_arm_sha2_sha256h2(
        hash_abcd: *const u8,
        hash_efgh: *const u8,
        wk: *const u8,
        out: *mut u8,
    );
    fn rust_isa_arm_sha2_sha256su0(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_sha2_sha256su1(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `sha256h`
///
/// Perform one SHA-256 round on the first half of the state.
///
/// The operands are the instruction operands: `hash_abcd` is the first half
/// of the state, `hash_efgh` is the second half, and `wk` contains the four
/// message-schedule words for this group of rounds.
///
/// # Safety
///
/// The caller must enable SHA-2 on the CPU and in the target features.
#[inline(always)]
pub unsafe fn sha256h(hash_abcd: V128, hash_efgh: V128, wk: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_sha2_sha256h(
            hash_abcd.as_ptr(),
            hash_efgh.as_ptr(),
            wk.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `sha256h2`
///
/// Perform one SHA-256 round on the second half of the state.
///
/// The operands use the same layout as [`sha256h`].
///
/// # Safety
///
/// The caller must enable SHA-2 on the CPU and in the target features.
#[inline(always)]
pub unsafe fn sha256h2(hash_abcd: V128, hash_efgh: V128, wk: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_sha2_sha256h2(
            hash_abcd.as_ptr(),
            hash_efgh.as_ptr(),
            wk.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `sha256su0`
///
/// Perform the first part of the SHA-256 message schedule.
///
/// # Safety
///
/// The caller must enable SHA-2 on the CPU and in the target features.
#[inline(always)]
pub unsafe fn sha256su0(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_sha2_sha256su0(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `sha256su1`
///
/// Perform the second part of the SHA-256 message schedule.
///
/// # Safety
///
/// The caller must enable SHA-2 on the CPU and in the target features.
#[inline(always)]
pub unsafe fn sha256su1(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_sha2_sha256su1(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}
