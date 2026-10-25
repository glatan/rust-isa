//! RISC-V Zbkx bit-manipulation instructions.
//!
//! Zbkx is not part of the baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable Zbkx with `#[target_feature]` or a global target
//! feature. The functions operate on 64-bit scalar values.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::riscv::integer::zbkx::xperm8;
//!
//! assert_ne!(unsafe { xperm8(0x0123_4567_89ab_cdef, 0) }, 0);
//! ```
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::riscv::integer::zbkx::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { xperm4(0, 0) };
//! let _ = unsafe { xperm8(0, 0) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::riscv::integer::zbkx::xperm4 as _;
//! use rust_isa::riscv::integer::zbkx::xperm8 as _;
//! ```
//! <!-- doctest-inventory:end -->

unsafe extern "C" {
    fn rust_isa_riscv_zbkx_xperm4(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbkx_xperm8(a: u64, b: u64) -> u64;
}

/// `xperm4`.
///
/// Permute nibbles with the indices in `b`.
///
/// # Safety
///
/// The caller must enable Zbkx.
#[target_feature(enable = "zbkx")]
#[inline]
pub unsafe fn xperm4(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zbkx_xperm4(a, b) }
}

/// `xperm8`.
///
/// Permute bytes with the indices in `b`.
///
/// # Safety
///
/// The caller must enable Zbkx.
#[target_feature(enable = "zbkx")]
#[inline]
pub unsafe fn xperm8(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zbkx_xperm8(a, b) }
}
