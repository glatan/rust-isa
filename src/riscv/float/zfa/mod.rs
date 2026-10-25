//! RISC-V Zfa scalar floating-point instructions.
//!
//! Zfa adds immediate loads, minimum and maximum with NaN suppression, quiet
//! comparisons, rounding, and the modular conversion. The scalar F, D, Zfh,
//! and Zfhmin operations live in [`super::scalar`].
//!
//! Zfa is not part of the baseline. Every instruction wrapper except the
//! immediate loads is `unsafe` and the caller must enable Zfa. The wrappers do
//! not carry `#[target_feature]` because rustc does not expose it on stable.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.

//! # Examples
//!
//! ```
//! use rust_isa::riscv::float::zfa::fli_h;
//!
//! assert_eq!(fli_h(0), 0xbc00);
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::riscv::float::zfa::fcvtmod_w_d as _;
//! use rust_isa::riscv::float::zfa::fcvtmod_w_h as _;
//! use rust_isa::riscv::float::zfa::fleq_d as _;
//! use rust_isa::riscv::float::zfa::fleq_h as _;
//! use rust_isa::riscv::float::zfa::fleq_s as _;
//! use rust_isa::riscv::float::zfa::fli_d as _;
//! use rust_isa::riscv::float::zfa::fli_h as _;
//! use rust_isa::riscv::float::zfa::fli_s as _;
//! use rust_isa::riscv::float::zfa::fltq_d as _;
//! use rust_isa::riscv::float::zfa::fltq_h as _;
//! use rust_isa::riscv::float::zfa::fltq_s as _;
//! use rust_isa::riscv::float::zfa::fmaxm_d as _;
//! use rust_isa::riscv::float::zfa::fmaxm_h as _;
//! use rust_isa::riscv::float::zfa::fmaxm_s as _;
//! use rust_isa::riscv::float::zfa::fminm_d as _;
//! use rust_isa::riscv::float::zfa::fminm_h as _;
//! use rust_isa::riscv::float::zfa::fminm_s as _;
//! use rust_isa::riscv::float::zfa::fmvh_x_d as _;
//! use rust_isa::riscv::float::zfa::fmvp_d_x as _;
//! use rust_isa::riscv::float::zfa::fround_d as _;
//! use rust_isa::riscv::float::zfa::fround_h as _;
//! use rust_isa::riscv::float::zfa::fround_s as _;
//! use rust_isa::riscv::float::zfa::froundnx_d as _;
//! use rust_isa::riscv::float::zfa::froundnx_h as _;
//! use rust_isa::riscv::float::zfa::froundnx_s as _;
//! ```
//! <!-- doctest-inventory:end -->

unsafe extern "C" {
    fn rust_isa_riscv_zfa_fminm(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfa_fmaxm(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfa_fround(a: u16) -> u16;
    fn rust_isa_riscv_zfa_froundnx(a: u16) -> u16;
    fn rust_isa_riscv_zfa_fleq(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_zfa_fltq(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_zfa_fcvtmod_w_h(a: u16) -> i32;

    fn rust_isa_riscv_zfa_fminm_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfa_fmaxm_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfa_fround_s(a: u32) -> u32;
    fn rust_isa_riscv_zfa_froundnx_s(a: u32) -> u32;
    fn rust_isa_riscv_zfa_fleq_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfa_fltq_s(a: u32, b: u32) -> u32;

    fn rust_isa_riscv_zfa_fminm_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zfa_fmaxm_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zfa_fround_d(a: u64) -> u64;
    fn rust_isa_riscv_zfa_froundnx_d(a: u64) -> u64;
    fn rust_isa_riscv_zfa_fleq_d(a: u64, b: u64) -> u32;
    fn rust_isa_riscv_zfa_fltq_d(a: u64, b: u64) -> u32;
    fn rust_isa_riscv_zfa_fcvtmod_w_d(value: u64) -> i32;

    fn rust_isa_riscv_zfa_fmvh_x_d(value: u64) -> u32;
    fn rust_isa_riscv_zfa_fmvp_d_x(low: u32, high: u32) -> u64;
}

/// `fminm.h`.
///
/// Minimum of two binary16 values. If either input is NaN, the result is the
/// canonical NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fminm_h(a: u16, b: u16) -> u16 {
    unsafe { rust_isa_riscv_zfa_fminm(a, b) }
}

/// `fmaxm.h`.
///
/// Maximum of two binary16 values. If either input is NaN, the result is the
/// canonical NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fmaxm_h(a: u16, b: u16) -> u16 {
    unsafe { rust_isa_riscv_zfa_fmaxm(a, b) }
}

/// `fminm.s`.
///
/// Minimum of two binary32 values. If either input is NaN, the result is the
/// canonical NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fminm_s(a: u32, b: u32) -> u32 {
    unsafe { rust_isa_riscv_zfa_fminm_s(a, b) }
}

/// `fmaxm.s`.
///
/// Maximum of two binary32 values. If either input is NaN, the result is the
/// canonical NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fmaxm_s(a: u32, b: u32) -> u32 {
    unsafe { rust_isa_riscv_zfa_fmaxm_s(a, b) }
}

/// `fminm.d`.
///
/// Minimum of two binary64 values. If either input is NaN, the result is the
/// canonical NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fminm_d(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zfa_fminm_d(a, b) }
}

/// `fmaxm.d`.
///
/// Maximum of two binary64 values. If either input is NaN, the result is the
/// canonical NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fmaxm_d(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zfa_fmaxm_d(a, b) }
}

/// `fleq.h`.
///
/// Quiet comparison: true when `a <= b`, including when either input is NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fleq_h(a: u16, b: u16) -> bool {
    unsafe { rust_isa_riscv_zfa_fleq(a, b) != 0 }
}

/// `fltq.h`.
///
/// Quiet comparison: true when `a < b`, including when either input is NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fltq_h(a: u16, b: u16) -> bool {
    unsafe { rust_isa_riscv_zfa_fltq(a, b) != 0 }
}

/// `fleq.s`.
///
/// Quiet comparison: true when `a <= b`, including when either input is NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fleq_s(a: u32, b: u32) -> bool {
    unsafe { rust_isa_riscv_zfa_fleq_s(a, b) != 0 }
}

/// `fltq.s`.
///
/// Quiet comparison: true when `a < b`, including when either input is NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fltq_s(a: u32, b: u32) -> bool {
    unsafe { rust_isa_riscv_zfa_fltq_s(a, b) != 0 }
}

/// `fleq.d`.
///
/// Quiet comparison: true when `a <= b`, including when either input is NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fleq_d(a: u64, b: u64) -> bool {
    unsafe { rust_isa_riscv_zfa_fleq_d(a, b) != 0 }
}

/// `fltq.d`.
///
/// Quiet comparison: true when `a < b`, including when either input is NaN.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fltq_d(a: u64, b: u64) -> bool {
    unsafe { rust_isa_riscv_zfa_fltq_d(a, b) != 0 }
}

/// `fround.h`.
///
/// Round a binary16 value to an integral value using the current rounding mode.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fround_h(a: u16) -> u16 {
    unsafe { rust_isa_riscv_zfa_fround(a) }
}

/// `froundnx.h`.
///
/// Round a binary16 value to an integral value and report whether the result is
/// inexact.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn froundnx_h(a: u16) -> u16 {
    unsafe { rust_isa_riscv_zfa_froundnx(a) }
}

/// `fround.s`.
///
/// Round a binary32 value to an integral value using the current rounding mode.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fround_s(a: u32) -> u32 {
    unsafe { rust_isa_riscv_zfa_fround_s(a) }
}

/// `froundnx.s`.
///
/// Round a binary32 value to an integral value and report whether the result is
/// inexact.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn froundnx_s(a: u32) -> u32 {
    unsafe { rust_isa_riscv_zfa_froundnx_s(a) }
}

/// `fround.d`.
///
/// Round a binary64 value to an integral value using the current rounding mode.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fround_d(a: u64) -> u64 {
    unsafe { rust_isa_riscv_zfa_fround_d(a) }
}

/// `froundnx.d`.
///
/// Round a binary64 value to an integral value and report whether the result is
/// inexact.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn froundnx_d(a: u64) -> u64 {
    unsafe { rust_isa_riscv_zfa_froundnx_d(a) }
}

/// `fcvtmod.w.h`.
///
/// Convert a binary16 value to a signed 32-bit integer with modulo semantics.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fcvtmod_w_h(a: u16) -> i32 {
    unsafe { rust_isa_riscv_zfa_fcvtmod_w_h(a) }
}

/// `fcvtmod.w.d`.
///
/// Convert a binary64 value to a signed 32-bit integer with modulo semantics.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fcvtmod_w_d(value: u64) -> i32 {
    unsafe { rust_isa_riscv_zfa_fcvtmod_w_d(value) }
}

/// `fmvh.x.d`.
///
/// Return bits 63:32. This instruction is only encodable on RV32; the value
/// API is also provided on RV64 for symmetry with [`fmvp_d_x`].
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fmvh_x_d(value: u64) -> u32 {
    unsafe { rust_isa_riscv_zfa_fmvh_x_d(value) }
}

/// `fmvp.d.x`.
///
/// Combine `low` (bits 31:0) and `high` (bits 63:32). This instruction is only
/// encodable on RV32; the value API is also provided on RV64.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fmvp_d_x(low: u32, high: u32) -> u64 {
    unsafe { rust_isa_riscv_zfa_fmvp_d_x(low, high) }
}

/// `fli.h`.
///
/// Load one of the 32 Zfa half-precision constants selected by `imm`.
#[inline]
pub fn fli_h(imm: u8) -> u16 {
    const VALUES: [u16; 32] = [
        0xbc00, 0x0400, 0x0100, 0x0200, 0x3800, 0x2000, 0x2c00, 0x3000, 0x3400, 0x3500, 0x3600,
        0x3700, 0x3800, 0x3900, 0x3a00, 0x3b00, 0x3c00, 0x3d00, 0x3e00, 0x3f00, 0x4000, 0x4100,
        0x4200, 0x4400, 0x4800, 0x4c00, 0x5800, 0x5c00, 0x7800, 0x7c00, 0x7c00, 0x7e00,
    ];
    VALUES[usize::from(imm & 31)]
}

/// `fli.s`.
///
/// Load one of the 32 Zfa single-precision constants selected by `imm`.
#[inline]
pub fn fli_s(imm: u8) -> u32 {
    const VALUES: [u32; 32] = [
        0xbf80_0000,
        0x0080_0000,
        0x3780_0000,
        0x3800_0000,
        0x3b80_0000,
        0x3c00_0000,
        0x3d80_0000,
        0x3e00_0000,
        0x3e80_0000,
        0x3ea0_0000,
        0x3ec0_0000,
        0x3ee0_0000,
        0x3f00_0000,
        0x3f20_0000,
        0x3f40_0000,
        0x3f60_0000,
        0x3f80_0000,
        0x3fa0_0000,
        0x3fc0_0000,
        0x3fe0_0000,
        0x4000_0000,
        0x4020_0000,
        0x4040_0000,
        0x4080_0000,
        0x4100_0000,
        0x4180_0000,
        0x4300_0000,
        0x4380_0000,
        0x4700_0000,
        0x4780_0000,
        0x7f80_0000,
        0x7fc0_0000,
    ];
    VALUES[usize::from(imm & 31)]
}

/// `fli.d`.
///
/// Load one of the 32 Zfa double-precision constants selected by `imm`.
#[inline]
pub fn fli_d(imm: u8) -> u64 {
    const VALUES: [u64; 32] = [
        0xbff0_0000_0000_0000,
        0x0010_0000_0000_0000,
        0x3ef0_0000_0000_0000,
        0x3f00_0000_0000_0000,
        0x3f70_0000_0000_0000,
        0x3f80_0000_0000_0000,
        0x3fb0_0000_0000_0000,
        0x3fc0_0000_0000_0000,
        0x3fd0_0000_0000_0000,
        0x3fd4_0000_0000_0000,
        0x3fd8_0000_0000_0000,
        0x3fdc_0000_0000_0000,
        0x3fe0_0000_0000_0000,
        0x3fe4_0000_0000_0000,
        0x3fe8_0000_0000_0000,
        0x3fec_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x3ff4_0000_0000_0000,
        0x3ff8_0000_0000_0000,
        0x3ffc_0000_0000_0000,
        0x4000_0000_0000_0000,
        0x4004_0000_0000_0000,
        0x4008_0000_0000_0000,
        0x4010_0000_0000_0000,
        0x4020_0000_0000_0000,
        0x4030_0000_0000_0000,
        0x4060_0000_0000_0000,
        0x4070_0000_0000_0000,
        0x40e0_0000_0000_0000,
        0x40f0_0000_0000_0000,
        0x7ff0_0000_0000_0000,
        0x7ff8_0000_0000_0000,
    ];
    VALUES[usize::from(imm & 31)]
}
