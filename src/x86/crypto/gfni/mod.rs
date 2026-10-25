//! GFNI.
//!
//! GFNI is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! requires the `gfni` target feature. The values are [`V128`].
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The build compiles the bundled C with clang; the immediate form uses
//! `asm!`. Immediate operands are const generics.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::gfni::gf2p8mulb;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { gf2p8mulb(v, v) }.to_bytes(), [0; 16]);
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::gfni::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { gf2p8affineqb::<0>(v128, v128) };
//! let _ = unsafe { gf2p8mulb(v128, v128) };
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::gfni::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { gf2p8affineqb::<0>(v128, v128) };
//! let _ = unsafe { gf2p8mulb(v128, v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::crypto::gfni::gf2p8affineqb as _;
//! use rust_isa::x86::crypto::gfni::gf2p8mulb as _;
//! ```
//! <!-- doctest-inventory:end -->

use core::arch::asm;

use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_gfni_gf2p8mulb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `gf2p8mulb`
///
/// Multiply the bytes in GF(2^8) with the AES polynomial.
///
/// # Safety
///
/// The caller must enable GFNI.
#[target_feature(enable = "gfni")]
#[inline]
pub unsafe fn gf2p8mulb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_gfni_gf2p8mulb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `gf2p8affineqb`
///
/// Apply the affine transform selected by `IMM`.
///
/// # Safety
///
/// The caller must enable GFNI.
#[target_feature(enable = "gfni")]
#[inline]
pub unsafe fn gf2p8affineqb<const IMM: u8>(a: V128, b: V128) -> V128 {
    let a = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "gf2p8affineqb {output}, {b}, {imm}",
            output = inout(xmm_reg) a => output,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}
