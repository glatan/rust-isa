//! RISC-V Zbc.
//!
//! Zbc is a stable extension. Every instruction wrapper is `unsafe` and the caller must
//! enable Zbc with `#[target_feature(enable = "zbc")]` or a global target
//! feature.
//!
//! The functions implement carry-less multiplication of 64-bit values.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::riscv::integer::zbc::clmul;
//!
//! assert_eq!(unsafe { clmul(1, 1) }, 1);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::riscv::integer::zbc::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { clmul(0, 0) };
//! let _ = unsafe { clmulh(0, 0) };
//! let _ = unsafe { clmulr(0, 0) };
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
//! use rust_isa::riscv::integer::zbc::clmul as _;
//! use rust_isa::riscv::integer::zbc::clmulh as _;
//! use rust_isa::riscv::integer::zbc::clmulr as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

unsafe extern "C" {
    fn rust_isa_riscv_zbc_clmul(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbc_clmulh(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbc_clmulr(a: u64, b: u64) -> u64;
}

/// `clmul`
///
/// Return the low 64 bits of the carry-less product.
///
/// # Safety
///
/// The caller must enable Zbc.
#[target_feature(enable = "zbc")]
#[inline]
pub unsafe fn clmul(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zbc_clmul(a, b) }
}

/// `clmulh`
///
/// Return bits 64..127 of the carry-less product.
///
/// # Safety
///
/// The caller must enable Zbc.
#[target_feature(enable = "zbc")]
#[inline]
pub unsafe fn clmulh(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zbc_clmulh(a, b) }
}

/// `clmulr`
///
/// Return bits 63..126 of the carry-less product.
///
/// # Safety
///
/// The caller must enable Zbc.
#[target_feature(enable = "zbc")]
#[inline]
pub unsafe fn clmulr(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zbc_clmulr(a, b) }
}
