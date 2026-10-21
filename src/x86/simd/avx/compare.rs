//! Compare and test instructions.

use super::V128;
use super::V256;

unsafe extern "C" {
    fn rust_isa_avx_vcomiss(a: *const u8, b: *const u8) -> u32;
    fn rust_isa_avx_vcomisd(a: *const u8, b: *const u8) -> u32;
    fn rust_isa_avx_vucomiss(a: *const u8, b: *const u8) -> u32;
    fn rust_isa_avx_vucomisd(a: *const u8, b: *const u8) -> u32;
    fn rust_isa_avx_vptest(a: *const u8, b: *const u8) -> u32;
    fn rust_isa_avx_vtestps(a: *const u8, b: *const u8) -> u32;
    fn rust_isa_avx_vtestpd(a: *const u8, b: *const u8) -> u32;
}

/// `vcomiss`
///
/// Compare the low 32-bit float lanes and return CF, PF, and ZF.
///
/// Returns bit 0 for CF, bit 1 for PF, and bit 2 for ZF.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcomiss(a: V128, b: V128) -> u32 {
    unsafe { rust_isa_avx_vcomiss(a.as_ptr(), b.as_ptr()) }
}

/// `vcomisd`
///
/// Compare the low 64-bit float lanes and return CF, PF, and ZF.
///
/// Returns bit 0 for CF, bit 1 for PF, and bit 2 for ZF.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcomisd(a: V128, b: V128) -> u32 {
    unsafe { rust_isa_avx_vcomisd(a.as_ptr(), b.as_ptr()) }
}

/// `vucomiss`
///
/// Compare the low 32-bit float lanes without signaling on quiet NaN and return CF, PF, and ZF.
///
/// Returns bit 0 for CF, bit 1 for PF, and bit 2 for ZF.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vucomiss(a: V128, b: V128) -> u32 {
    unsafe { rust_isa_avx_vucomiss(a.as_ptr(), b.as_ptr()) }
}

/// `vucomisd`
///
/// Compare the low 64-bit float lanes without signaling on quiet NaN and return CF, PF, and ZF.
///
/// Returns bit 0 for CF, bit 1 for PF, and bit 2 for ZF.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vucomisd(a: V128, b: V128) -> u32 {
    unsafe { rust_isa_avx_vucomisd(a.as_ptr(), b.as_ptr()) }
}

/// `vptest`
///
/// Test the bitwise AND of the two values.
///
/// Returns bit 0 for CF, bit 1 for PF, and bit 2 for ZF.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vptest(a: V256, b: V256) -> u32 {
    unsafe { rust_isa_avx_vptest(a.as_ptr(), b.as_ptr()) }
}

/// `vtestps`
///
/// Test the sign bits of the 32-bit float lanes.
///
/// Returns bit 0 for CF, bit 1 for PF, and bit 2 for ZF.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vtestps(a: V256, b: V256) -> u32 {
    unsafe { rust_isa_avx_vtestps(a.as_ptr(), b.as_ptr()) }
}

/// `vtestpd`
///
/// Test the sign bits of the 64-bit float lanes.
///
/// Returns bit 0 for CF, bit 1 for PF, and bit 2 for ZF.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vtestpd(a: V256, b: V256) -> u32 {
    unsafe { rust_isa_avx_vtestpd(a.as_ptr(), b.as_ptr()) }
}
