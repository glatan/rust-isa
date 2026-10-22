//! Scalar compare instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_comiss(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse2_comisd(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse2_ucomiss(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse2_ucomisd(a: *const u8, b: *const u8) -> i32;
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
