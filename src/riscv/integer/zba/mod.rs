//! RISC-V Zba.
//!
//! Zba is a stable extension. Every instruction wrapper is `unsafe` and the caller must
//! enable Zba with `#[target_feature(enable = "zba")]` or a global target
//! feature.
//!
//! The functions operate on 64-bit scalar values and return the
//! address-generation result in a 64-bit register.
//!
//! # Examples
//!
//! ```
//! use rust_isa::riscv::integer::zba::sh1add;
//!
//! assert_eq!(unsafe { sh1add(3, 4) }, 10);
//! ```
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Instruction examples
//!
//! ```
//! use rust_isa::riscv::integer::zba::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { sh1add(0, 0) };
//! let _ = unsafe { sh2add(0, 0) };
//! let _ = unsafe { sh3add(0, 0) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::riscv::integer::zba::sh1add as _;
//! use rust_isa::riscv::integer::zba::sh2add as _;
//! use rust_isa::riscv::integer::zba::sh3add as _;
//! ```
//! <!-- doctest-inventory:end -->

unsafe extern "C" {
    fn rust_isa_riscv_zba_sh1add(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zba_sh2add(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zba_sh3add(a: u64, b: u64) -> u64;
}

/// `sh1add`
///
/// Return `(a << 1) + b`.
///
/// # Safety
///
/// The caller must enable Zba.
#[target_feature(enable = "zba")]
#[inline]
pub unsafe fn sh1add(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zba_sh1add(a, b) }
}

/// `sh2add`
///
/// Return `(a << 2) + b`.
///
/// # Safety
///
/// The caller must enable Zba.
#[target_feature(enable = "zba")]
#[inline]
pub unsafe fn sh2add(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zba_sh2add(a, b) }
}

/// `sh3add`
///
/// Return `(a << 3) + b`.
///
/// # Safety
///
/// The caller must enable Zba.
#[target_feature(enable = "zba")]
#[inline]
pub unsafe fn sh3add(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zba_sh3add(a, b) }
}
