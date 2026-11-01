//! RISC-V Zbb.
//!
//! Zbb is a stable extension. Every instruction wrapper is `unsafe` and the caller must
//! enable Zbb with `#[target_feature(enable = "zbb")]` or a global target
//! feature.
//!
//! The functions operate on 64-bit values. `clz` and `ctz` return 64 for a zero
//! input. `rol` and `ror` use the low six bits of the shift amount.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::riscv::integer::zbb::clz;
//!
//! assert_eq!(unsafe { clz(1) }, 63);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::riscv::integer::zbb::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { andn(0, 0) };
//! let _ = unsafe { clz(0) };
//! let _ = unsafe { cpop(0) };
//! let _ = unsafe { ctz(0) };
//! let _ = unsafe { max(0, 0) };
//! let _ = unsafe { maxu(0, 0) };
//! let _ = unsafe { min(0, 0) };
//! let _ = unsafe { minu(0, 0) };
//! let _ = unsafe { orc_b(0) };
//! let _ = unsafe { orn(0, 0) };
//! let _ = unsafe { rol(0, 0) };
//! let _ = unsafe { ror(0, 0) };
//! let _ = unsafe { sext_b(0) };
//! let _ = unsafe { sext_h(0) };
//! let _ = unsafe { xnor(0, 0) };
//! let _ = unsafe { zext_h(0) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::riscv::integer::zbb::andn as _;
//! use rust_isa::riscv::integer::zbb::clz as _;
//! use rust_isa::riscv::integer::zbb::cpop as _;
//! use rust_isa::riscv::integer::zbb::ctz as _;
//! use rust_isa::riscv::integer::zbb::max as _;
//! use rust_isa::riscv::integer::zbb::maxu as _;
//! use rust_isa::riscv::integer::zbb::min as _;
//! use rust_isa::riscv::integer::zbb::minu as _;
//! use rust_isa::riscv::integer::zbb::orc_b as _;
//! use rust_isa::riscv::integer::zbb::orn as _;
//! use rust_isa::riscv::integer::zbb::rol as _;
//! use rust_isa::riscv::integer::zbb::ror as _;
//! use rust_isa::riscv::integer::zbb::sext_b as _;
//! use rust_isa::riscv::integer::zbb::sext_h as _;
//! use rust_isa::riscv::integer::zbb::xnor as _;
//! use rust_isa::riscv::integer::zbb::zext_h as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

unsafe extern "C" {
    fn rust_isa_riscv_zbb_andn(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_orn(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_xnor(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_clz(value: u64) -> u64;
    fn rust_isa_riscv_zbb_ctz(value: u64) -> u64;
    fn rust_isa_riscv_zbb_cpop(value: u64) -> u64;
    fn rust_isa_riscv_zbb_min(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_max(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_min_signed(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_max_signed(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_orc_b(value: u64) -> u64;
    fn rust_isa_riscv_zbb_sext_b(value: u64) -> u64;
    fn rust_isa_riscv_zbb_sext_h(value: u64) -> u64;
    fn rust_isa_riscv_zbb_zext_h(value: u64) -> u64;
    fn rust_isa_riscv_zbb_rol(value: u64, shift: u64) -> u64;
    fn rust_isa_riscv_zbb_ror(value: u64, shift: u64) -> u64;
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zbb.
        #[target_feature(enable = "zbb")]
        #[inline]
        pub unsafe fn $name(a: u64, b: u64) -> u64 {
            unsafe { $c(a, b) }
        }
    };
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zbb.
        #[target_feature(enable = "zbb")]
        #[inline]
        pub unsafe fn $name(value: u64) -> u64 {
            unsafe { $c(value) }
        }
    };
}

binary!(
    /// Bitwise AND with the complement of `b`.
    andn,
    rust_isa_riscv_zbb_andn
);
binary!(
    /// Bitwise OR with the complement of `b`.
    orn,
    rust_isa_riscv_zbb_orn
);
binary!(
    /// Bitwise XNOR.
    xnor,
    rust_isa_riscv_zbb_xnor
);
unary!(
    /// Count leading zero bits.
    clz,
    rust_isa_riscv_zbb_clz
);
unary!(
    /// Count trailing zero bits.
    ctz,
    rust_isa_riscv_zbb_ctz
);
unary!(
    /// Count set bits.
    cpop,
    rust_isa_riscv_zbb_cpop
);
binary!(
    /// Signed minimum.
    min,
    rust_isa_riscv_zbb_min_signed
);
binary!(
    /// Unsigned minimum.
    minu,
    rust_isa_riscv_zbb_min
);
binary!(
    /// Signed maximum.
    max,
    rust_isa_riscv_zbb_max_signed
);
binary!(
    /// Unsigned maximum.
    maxu,
    rust_isa_riscv_zbb_max
);
unary!(
    /// Set each byte to all ones when the byte is nonzero, otherwise zero.
    orc_b,
    rust_isa_riscv_zbb_orc_b
);
unary!(
    /// Sign-extend the low byte.
    sext_b,
    rust_isa_riscv_zbb_sext_b
);
unary!(
    /// Sign-extend the low 16 bits.
    sext_h,
    rust_isa_riscv_zbb_sext_h
);
unary!(
    /// Zero-extend the low 16 bits.
    zext_h,
    rust_isa_riscv_zbb_zext_h
);
binary!(
    /// Rotate left by the low six bits of `b`.
    rol,
    rust_isa_riscv_zbb_rol
);
binary!(
    /// Rotate right by the low six bits of `b`.
    ror,
    rust_isa_riscv_zbb_ror
);
