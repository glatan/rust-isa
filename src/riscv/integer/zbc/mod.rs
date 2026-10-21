//! RISC-V Zbc.
//!
//! Zbc is a stable extension. Every operation is `unsafe` and the caller must
//! enable Zbc with `#[target_feature(enable = "zbc")]` or a global target
//! feature.
//!
//! The functions implement carry-less multiplication of 64-bit values.

unsafe extern "C" {
    fn rust_isa_riscv_zbc_clmul(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbc_clmulh(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbc_clmulr(a: u64, b: u64) -> u64;
}

/// `clmul`
///
/// Return the low 64 bits of the carry-less product.
///
/// # Safety
///
/// The caller must enable Zbc.
#[target_feature(enable = "zbc")]
#[inline]
pub unsafe fn clmul(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zbc_clmul(a, b) }
}

/// `clmulh`
///
/// Return bits 64..127 of the carry-less product.
///
/// # Safety
///
/// The caller must enable Zbc.
#[target_feature(enable = "zbc")]
#[inline]
pub unsafe fn clmulh(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zbc_clmulh(a, b) }
}

/// `clmulr`
///
/// Return bits 63..126 of the carry-less product.
///
/// # Safety
///
/// The caller must enable Zbc.
#[target_feature(enable = "zbc")]
#[inline]
pub unsafe fn clmulr(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zbc_clmulr(a, b) }
}
