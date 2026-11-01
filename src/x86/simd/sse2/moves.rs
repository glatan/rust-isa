//! Move instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_movd(a: *const u8) -> u32;
    fn rust_isa_sse2_movq(a: *const u8) -> u64;
}

/// `movss`
///
/// Copy the low 32-bit float lane from `b` into `a` and preserve the upper
/// lanes of `a`.
#[inline(always)]
pub fn movss(a: V128, b: V128) -> V128 {
    let mut bytes = a.to_bytes();
    bytes[..4].copy_from_slice(&b.to_bytes()[..4]);
    V128::from_bytes(bytes)
}

/// `movsd`
///
/// Copy the low 64-bit float lane from `b` into `a` and preserve the upper
/// lane of `a`.
#[inline(always)]
pub fn movsd(a: V128, b: V128) -> V128 {
    let mut bytes = a.to_bytes();
    bytes[..8].copy_from_slice(&b.to_bytes()[..8]);
    V128::from_bytes(bytes)
}

/// `movlps`
///
/// Copy the low 64 bits from `b` into the low half of `a` and preserve the
/// upper half of `a`.
#[inline(always)]
pub fn movlps(a: V128, b: V128) -> V128 {
    let mut bytes = a.to_bytes();
    bytes[..8].copy_from_slice(&b.to_bytes()[..8]);
    V128::from_bytes(bytes)
}

/// `movhps`
///
/// Copy the low 64 bits from `b` into the high half of `a` and preserve the
/// low half of `a`.
#[inline(always)]
pub fn movhps(a: V128, b: V128) -> V128 {
    let mut bytes = a.to_bytes();
    bytes[8..].copy_from_slice(&b.to_bytes()[..8]);
    V128::from_bytes(bytes)
}

/// `movlhps`
///
/// Copy the low 64 bits from `b` into the high half of `a` and preserve the
/// low half of `a`.
#[inline(always)]
pub fn movlhps(a: V128, b: V128) -> V128 {
    movhps(a, b)
}

/// `movhlps`
///
/// Copy the high 64 bits from `b` into the low half of `a` and preserve the
/// high half of `a`.
#[inline(always)]
pub fn movhlps(a: V128, b: V128) -> V128 {
    let mut bytes = a.to_bytes();
    bytes[..8].copy_from_slice(&b.to_bytes()[8..]);
    V128::from_bytes(bytes)
}

/// `movd`
///
/// Return the low 32 bits of the value.
#[inline(always)]
pub fn movd(a: V128) -> u32 {
    unsafe { rust_isa_sse2_movd(a.as_ptr()) }
}

/// `movq`
///
/// Return the low 64 bits of the value.
#[inline(always)]
pub fn movq(a: V128) -> u64 {
    unsafe { rust_isa_sse2_movq(a.as_ptr()) }
}

/// `pinsrw`
///
/// Insert `value` into 16-bit lane `IMM`. `IMM` must be in `0..=7`.
#[inline(always)]
pub fn pinsrw<const IMM: u8>(a: V128, value: u16) -> V128 {
    const { assert!(IMM <= 7, "pinsrw lane must be in 0..=7") };
    let mut bytes = a.to_bytes();
    let offset = usize::from(IMM) * 2;
    bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    V128::from_bytes(bytes)
}

/// `pextrw`
///
/// Extract 16-bit lane `IMM` and zero-extend it. `IMM` must be in `0..=7`.
#[inline(always)]
pub fn pextrw<const IMM: u8>(a: V128) -> u16 {
    const { assert!(IMM <= 7, "pextrw lane must be in 0..=7") };
    let bytes = a.to_bytes();
    let offset = usize::from(IMM) * 2;
    u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
}
