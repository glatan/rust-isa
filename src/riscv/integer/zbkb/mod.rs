//! RISC-V Zbkb bit-manipulation instructions.
//!
//! These extensions are not part of the baseline. Every instruction wrapper is `unsafe`
//! and the caller must enable the extension with `#[target_feature]` or a
//! global target feature. The functions operate on 64-bit scalar values.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::riscv::integer::zbkb::brev8;
//!
//! assert_eq!(unsafe { brev8(1) }, 0x80);
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::riscv::integer::zbkb::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! unsafe {
//!     let _ = unsafe { brev8(0) };
//!     let _ = unsafe { pack(0, 0) };
//!     let _ = unsafe { packh(0, 0) };
//!     let _ = unsafe { unzip(0) };
//!     let _ = unsafe { zip(0) };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::riscv::integer::zbkb::brev8 as _;
//! use rust_isa::riscv::integer::zbkb::pack as _;
//! use rust_isa::riscv::integer::zbkb::packh as _;
//! use rust_isa::riscv::integer::zbkb::unzip as _;
//! use rust_isa::riscv::integer::zbkb::zip as _;
//! ```
//! <!-- doctest-inventory:end -->

unsafe extern "C" {
    fn rust_isa_riscv_zbkb_pack(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbkb_packh(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbkb_brev8(a: u64) -> u64;
    fn rust_isa_riscv_zbkb_zip(value: u64) -> u64;
    fn rust_isa_riscv_zbkb_unzip(value: u64) -> u64;
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zbkb.
        #[target_feature(enable = "zbkb")]
        #[inline]
        pub unsafe fn $name(a: u64, b: u64) -> u64 {
            unsafe { $c(a, b) }
        }
    };
}

binary!(
    /// `pack`.
    pack,
    rust_isa_riscv_zbkb_pack
);
binary!(
    /// `packh`.
    packh,
    rust_isa_riscv_zbkb_packh
);

/// `brev8`.
///
/// Reverse the bits in each byte.
///
/// # Safety
///
/// The caller must enable Zbkb.
#[target_feature(enable = "zbkb")]
#[inline]
pub unsafe fn brev8(a: u64) -> u64 {
    unsafe { rust_isa_riscv_zbkb_brev8(a) }
}

/// `zip`.
///
/// The instruction exists only on RV32. On RV64 this function implements the
/// same 32-bit bit-interleave operation in software. Bits of the low half
/// become the even bits and bits of the high half become the odd bits.
///
/// # Safety
///
/// The caller must enable `zbkb` with `#[target_feature]` or a global target
/// feature.
#[target_feature(enable = "zbkb")]
#[inline]
pub unsafe fn zip(value: u32) -> u32 {
    unsafe { rust_isa_riscv_zbkb_zip(value as u64) as u32 }
}

/// `unzip`.
///
/// The instruction exists only on RV32. On RV64 this function implements the
/// same 32-bit bit-deinterleave operation in software.
///
/// # Safety
///
/// The caller must enable `zbkb` with `#[target_feature]` or a global target
/// feature.
#[target_feature(enable = "zbkb")]
#[inline]
pub unsafe fn unzip(value: u32) -> u32 {
    unsafe { rust_isa_riscv_zbkb_unzip(value as u64) as u32 }
}
