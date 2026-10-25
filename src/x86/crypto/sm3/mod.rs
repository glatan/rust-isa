//! SM3 instructions.
//!
//! SM3 is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable SM3 with `#[target_feature(enable = "sm3")]` or a global
//! target feature.
//!
//! The values are [`V128`]. The round instruction takes a two-bit immediate;
//! `vsm3rnds2::<IMM>` accepts `0..=3` and a larger value is a compile-time
//! error. The caller combines the message schedule and round instructions with
//! the SM3 specification.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::sm3::vsm3msg1;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vsm3msg1(v, v, v) };
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::sm3::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! unsafe {
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vsm3msg1(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vsm3msg2(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vsm3rnds2::<0>(v128, v128, v128) };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::crypto::sm3::vsm3msg1 as _;
//! use rust_isa::x86::crypto::sm3::vsm3msg2 as _;
//! use rust_isa::x86::crypto::sm3::vsm3rnds2 as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_sm3_vsm3msg1(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sm3_vsm3msg2(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sm3_vsm3rnds2_0(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sm3_vsm3rnds2_1(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sm3_vsm3rnds2_2(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sm3_vsm3rnds2_3(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vsm3msg1`
///
/// Perform the first part of the SM3 message expansion.
///
/// # Safety
///
/// The caller must enable SM3.
#[target_feature(enable = "sm3")]
#[inline]
pub unsafe fn vsm3msg1(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sm3_vsm3msg1(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsm3msg2`
///
/// Perform the second part of the SM3 message expansion.
///
/// # Safety
///
/// The caller must enable SM3.
#[target_feature(enable = "sm3")]
#[inline]
pub unsafe fn vsm3msg2(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sm3_vsm3msg2(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsm3rnds2::<IMM>`
///
/// Perform two SM3 rounds. `IMM` selects the round group and must be in
/// `0..=3`.
///
/// # Safety
///
/// The caller must enable SM3.
#[target_feature(enable = "sm3")]
#[inline]
pub unsafe fn vsm3rnds2<const IMM: u8>(a: V128, b: V128, c: V128) -> V128 {
    const { assert!(IMM <= 3, "vsm3rnds2 immediate must be in 0..=3") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match IMM {
        0 => call!(rust_isa_sm3_vsm3rnds2_0),
        1 => call!(rust_isa_sm3_vsm3rnds2_1),
        2 => call!(rust_isa_sm3_vsm3rnds2_2),
        3 => call!(rust_isa_sm3_vsm3rnds2_3),
        _ => unreachable!(),
    }
}
