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
