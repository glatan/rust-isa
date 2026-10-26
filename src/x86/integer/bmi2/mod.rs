//! BMI2.
//!
//! BMI2 is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! requires the `bmi2` target feature. The functions operate on 32-bit scalar
//! values.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The build compiles the bundled C with clang; the shift and rotate forms use
//! `asm!`. Immediate operands are const generics.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::x86::integer::bmi2::pext;
//!
//! assert_eq!(unsafe { pext(0b1010, 0b1100) }, 0b0010);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::x86::integer::bmi2::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { bzhi(0, 0) };
//! let _ = unsafe { pdep(0, 0) };
//! let _ = unsafe { pext(0, 0) };
//! let _ = unsafe { rorx::<0>(0) };
//! let _ = unsafe { sarx(0, 0) };
//! let _ = unsafe { shlx(0, 0) };
//! let _ = unsafe { shrx(0, 0) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::integer::bmi2::bzhi as _;
//! use rust_isa::x86::integer::bmi2::pdep as _;
//! use rust_isa::x86::integer::bmi2::pext as _;
//! use rust_isa::x86::integer::bmi2::rorx as _;
//! use rust_isa::x86::integer::bmi2::sarx as _;
//! use rust_isa::x86::integer::bmi2::shlx as _;
//! use rust_isa::x86::integer::bmi2::shrx as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_bmi2_bzhi(a: u32, index: u32) -> u32;
    fn rust_isa_bmi2_pdep(a: u32, mask: u32) -> u32;
    fn rust_isa_bmi2_pext(a: u32, mask: u32) -> u32;
}

/// `bzhi`
///
/// Zero the bits from `index` upward.
///
/// # Safety
///
/// The caller must enable BMI2.
#[target_feature(enable = "bmi2")]
#[inline]
pub unsafe fn bzhi(a: u32, index: u32) -> u32 {
    unsafe { rust_isa_bmi2_bzhi(a, index) }
}

/// `pdep`
///
/// Deposit the low bits of `a` into the positions selected by `mask`.
///
/// # Safety
///
/// The caller must enable BMI2.
#[target_feature(enable = "bmi2")]
#[inline]
pub unsafe fn pdep(a: u32, mask: u32) -> u32 {
    unsafe { rust_isa_bmi2_pdep(a, mask) }
}

/// `pext`
///
/// Extract the bits selected by `mask`.
///
/// # Safety
///
/// The caller must enable BMI2.
#[target_feature(enable = "bmi2")]
#[inline]
pub unsafe fn pext(a: u32, mask: u32) -> u32 {
    unsafe { rust_isa_bmi2_pext(a, mask) }
}

/// `sarx`
///
/// Arithmetic shift right by `count`.
///
/// # Safety
///
/// The caller must enable BMI2.
#[target_feature(enable = "bmi2")]
#[inline]
pub unsafe fn sarx(a: u32, count: u32) -> u32 {
    let output: u32;
    unsafe {
        asm!(
            "sarx {out:e}, {a:e}, {count:e}",
            out = lateout(reg) output,
            a = in(reg) a,
            count = in(reg) count,
            options(nostack, nomem, pure)
        );
    }
    output
}

/// `shlx`
///
/// Logical shift left by `count`.
///
/// # Safety
///
/// The caller must enable BMI2.
#[target_feature(enable = "bmi2")]
#[inline]
pub unsafe fn shlx(a: u32, count: u32) -> u32 {
    let output: u32;
    unsafe {
        asm!(
            "shlx {out:e}, {a:e}, {count:e}",
            out = lateout(reg) output,
            a = in(reg) a,
            count = in(reg) count,
            options(nostack, nomem, pure)
        );
    }
    output
}

/// `shrx`
///
/// Logical shift right by `count`.
///
/// # Safety
///
/// The caller must enable BMI2.
#[target_feature(enable = "bmi2")]
#[inline]
pub unsafe fn shrx(a: u32, count: u32) -> u32 {
    let output: u32;
    unsafe {
        asm!(
            "shrx {out:e}, {a:e}, {count:e}",
            out = lateout(reg) output,
            a = in(reg) a,
            count = in(reg) count,
            options(nostack, nomem, pure)
        );
    }
    output
}

/// `rorx`
///
/// Rotate right by `IMM`.
///
/// # Safety
///
/// The caller must enable BMI2.
#[target_feature(enable = "bmi2")]
#[inline]
pub unsafe fn rorx<const IMM: u8>(a: u32) -> u32 {
    let output: u32;
    unsafe {
        asm!(
            "rorx {output:e}, {a:e}, {imm}",
            output = lateout(reg) output,
            a = in(reg) a,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    output
}
