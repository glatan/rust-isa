//! Shuffle instructions.
//!
//! The immediate is a const generic. The instruction is used directly because
//! the C intrinsic requires a compile-time immediate.

use core::arch::asm;

use super::V128;

/// `pshufd`
///
/// Shuffle the four 32-bit integer lanes with `IMM`.
#[inline(always)]
pub fn pshufd<const IMM: u8>(a: V128) -> V128 {
    let input = u128::from_le_bytes(a.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "pshufd {output}, {input}, {imm}",
            output = lateout(xmm_reg) output,
            input = in(xmm_reg) input,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}

/// `pshuflw`
///
/// Shuffle the low four 16-bit integer lanes with `IMM`.
#[inline(always)]
pub fn pshuflw<const IMM: u8>(a: V128) -> V128 {
    let input = u128::from_le_bytes(a.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "pshuflw {output}, {input}, {imm}",
            output = lateout(xmm_reg) output,
            input = in(xmm_reg) input,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}

/// `pshufhw`
///
/// Shuffle the high four 16-bit integer lanes with `IMM`.
#[inline(always)]
pub fn pshufhw<const IMM: u8>(a: V128) -> V128 {
    let input = u128::from_le_bytes(a.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "pshufhw {output}, {input}, {imm}",
            output = lateout(xmm_reg) output,
            input = in(xmm_reg) input,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}

/// `shufps`
///
/// Shuffle the four 32-bit float lanes with `IMM`. The low two lanes come from
/// `a` and the high two lanes come from `b`.
#[inline(always)]
pub fn shufps<const IMM: u8>(a: V128, b: V128) -> V128 {
    let a = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "shufps {output}, {b}, {imm}",
            output = inout(xmm_reg) a => output,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}

/// `shufpd`
///
/// Shuffle the two 64-bit float lanes with `IMM`. The low lane comes from `a`
/// and the high lane comes from `b`.
#[inline(always)]
pub fn shufpd<const IMM: u8>(a: V128, b: V128) -> V128 {
    let a = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "shufpd {output}, {b}, {imm}",
            output = inout(xmm_reg) a => output,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}
