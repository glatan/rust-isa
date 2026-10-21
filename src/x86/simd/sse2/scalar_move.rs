//! Scalar move instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_comiss(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse2_comisd(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse2_ucomiss(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse2_ucomisd(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse2_movntps(p: *mut f32, a: *const u8);
    fn rust_isa_sse2_movntpd(p: *mut f64, a: *const u8);
    fn rust_isa_sse2_movntdq(p: *mut u8, a: *const u8);
    fn rust_isa_sse2_movnti(p: *mut i32, value: i32);
    #[cfg(target_arch = "x86_64")]
    fn rust_isa_sse2_movntq(p: *mut u8, a: *const u8);
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

/// `comiss`
///
/// Compare the low 32-bit float lanes. Returns `-1` when `a < b`, `0` when
/// `a == b`, `1` when `a > b`, and `2` for unordered operands.
#[inline(always)]
pub fn comiss(a: V128, b: V128) -> i32 {
    unsafe { rust_isa_sse2_comiss(a.as_ptr(), b.as_ptr()) }
}

/// `comisd`
///
/// Compare the low 64-bit float lanes. Returns `-1` when `a < b`, `0` when
/// `a == b`, `1` when `a > b`, and `2` for unordered operands.
#[inline(always)]
pub fn comisd(a: V128, b: V128) -> i32 {
    unsafe { rust_isa_sse2_comisd(a.as_ptr(), b.as_ptr()) }
}

/// `ucomiss`
///
/// Compare the low 32-bit float lanes without raising an invalid exception for
/// quiet NaNs. Returns the same codes as [`comiss`].
#[inline(always)]
pub fn ucomiss(a: V128, b: V128) -> i32 {
    unsafe { rust_isa_sse2_ucomiss(a.as_ptr(), b.as_ptr()) }
}

/// `ucomisd`
///
/// Compare the low 64-bit float lanes without raising an invalid exception for
/// quiet NaNs. Returns the same codes as [`comisd`].
#[inline(always)]
pub fn ucomisd(a: V128, b: V128) -> i32 {
    unsafe { rust_isa_sse2_ucomisd(a.as_ptr(), b.as_ptr()) }
}

/// `movntps`
///
/// Store four 32-bit float lanes with a non-temporal hint.
///
/// # Safety
///
/// `p` must be valid for writing 16 bytes and aligned to 16 bytes.
#[inline(always)]
pub unsafe fn movntps(p: *mut f32, a: V128) {
    unsafe {
        rust_isa_sse2_movntps(p, a.as_ptr());
    }
}

/// `movntpd`
///
/// Store two 64-bit float lanes with a non-temporal hint.
///
/// # Safety
///
/// `p` must be valid for writing 16 bytes and aligned to 16 bytes.
#[inline(always)]
pub unsafe fn movntpd(p: *mut f64, a: V128) {
    unsafe {
        rust_isa_sse2_movntpd(p, a.as_ptr());
    }
}

/// `movntdq`
///
/// Store 16 bytes with a non-temporal hint.
///
/// # Safety
///
/// `p` must be valid for writing 16 bytes and aligned to 16 bytes.
#[inline(always)]
pub unsafe fn movntdq(p: *mut u8, a: V128) {
    unsafe {
        rust_isa_sse2_movntdq(p, a.as_ptr());
    }
}

/// `movnti`
///
/// Store a 32-bit integer with a non-temporal hint.
///
/// # Safety
///
/// `p` must be valid for writing 4 bytes.
#[inline(always)]
pub unsafe fn movnti(p: *mut i32, value: i32) {
    unsafe {
        rust_isa_sse2_movnti(p, value);
    }
}

/// `movntq`
///
/// Store the low 64 bits with a non-temporal hint.
///
/// # Safety
///
/// `p` must be valid for writing 8 bytes.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn movntq(p: *mut u8, a: V128) {
    unsafe {
        rust_isa_sse2_movntq(p, a.as_ptr());
    }
}
