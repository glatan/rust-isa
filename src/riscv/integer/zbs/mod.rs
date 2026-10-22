//! RISC-V Zbs.
//!
//! Zbs is a stable extension. Every operation is `unsafe` and the caller must
//! enable Zbs with `#[target_feature(enable = "zbs")]` or a global target
//! feature.
//!
//! The bit index uses the low six bits of the second operand.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).

unsafe extern "C" {
    fn rust_isa_riscv_zbs_bset(value: u64, index: u64) -> u64;
    fn rust_isa_riscv_zbs_bclr(value: u64, index: u64) -> u64;
    fn rust_isa_riscv_zbs_binv(value: u64, index: u64) -> u64;
    fn rust_isa_riscv_zbs_bext(value: u64, index: u64) -> u64;
}

/// `bset`
///
/// Set the selected bit.
///
/// # Safety
///
/// The caller must enable Zbs.
#[target_feature(enable = "zbs")]
#[inline]
pub unsafe fn bset(value: u64, index: u64) -> u64 {
    unsafe { rust_isa_riscv_zbs_bset(value, index) }
}

/// `bclr`
///
/// Clear the selected bit.
///
/// # Safety
///
/// The caller must enable Zbs.
#[target_feature(enable = "zbs")]
#[inline]
pub unsafe fn bclr(value: u64, index: u64) -> u64 {
    unsafe { rust_isa_riscv_zbs_bclr(value, index) }
}

/// `binv`
///
/// Invert the selected bit.
///
/// # Safety
///
/// The caller must enable Zbs.
#[target_feature(enable = "zbs")]
#[inline]
pub unsafe fn binv(value: u64, index: u64) -> u64 {
    unsafe { rust_isa_riscv_zbs_binv(value, index) }
}

/// `bext`
///
/// Extract the selected bit.
///
/// # Safety
///
/// The caller must enable Zbs.
#[target_feature(enable = "zbs")]
#[inline]
pub unsafe fn bext(value: u64, index: u64) -> u64 {
    unsafe { rust_isa_riscv_zbs_bext(value, index) }
}
