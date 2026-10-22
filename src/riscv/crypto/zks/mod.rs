//! RISC-V Zks crypto.
//!
//! Zks is a stable scalar cryptography extension. Every instruction wrapper is `unsafe`
//! and the caller must enable the required sub-extension (`zksed`, `zksh`) with
//! `#[target_feature]` or a global target feature.
//!
//! The functions operate on scalar registers. The SM4 instructions take a
//! two-bit byte-select immediate; `sm4ed::<BS>` and `sm4ks::<BS>` accept
//! `0..=3` and a larger value is a compile-time error.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

unsafe extern "C" {
    fn rust_isa_riscv_zks_sm3p0(value: u32) -> u32;
    fn rust_isa_riscv_zks_sm3p1(value: u32) -> u32;
    fn rust_isa_riscv_zks_sm4ed_0(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zks_sm4ed_1(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zks_sm4ed_2(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zks_sm4ed_3(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zks_sm4ks_0(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zks_sm4ks_1(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zks_sm4ks_2(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zks_sm4ks_3(a: u32, b: u32) -> u32;
}

/// SM3 `P0`.
///
/// # Safety
///
/// The caller must enable Zksh.
#[target_feature(enable = "zks")]
#[inline]
pub unsafe fn sm3p0(value: u32) -> u32 {
    unsafe { rust_isa_riscv_zks_sm3p0(value) }
}

/// SM3 `P1`.
///
/// # Safety
///
/// The caller must enable Zksh.
#[target_feature(enable = "zks")]
#[inline]
pub unsafe fn sm3p1(value: u32) -> u32 {
    unsafe { rust_isa_riscv_zks_sm3p1(value) }
}

/// `sm4ed::<BS>`
///
/// Perform an SM4 encryption round with the byte select `BS`.
///
/// # Safety
///
/// The caller must enable Zksed.
#[target_feature(enable = "zks")]
#[inline]
pub unsafe fn sm4ed<const BS: u8>(a: u32, b: u32) -> u32 {
    const { assert!(BS <= 3, "SM4 byte select must be in 0..=3") };
    macro_rules! call {
        ($f:ident) => {
            unsafe { $f(a, b) }
        };
    }
    match BS {
        0 => call!(rust_isa_riscv_zks_sm4ed_0),
        1 => call!(rust_isa_riscv_zks_sm4ed_1),
        2 => call!(rust_isa_riscv_zks_sm4ed_2),
        3 => call!(rust_isa_riscv_zks_sm4ed_3),
        _ => unreachable!(),
    }
}

/// `sm4ks::<BS>`
///
/// Perform an SM4 key-schedule round with the byte select `BS`.
///
/// # Safety
///
/// The caller must enable Zksed.
#[target_feature(enable = "zks")]
#[inline]
pub unsafe fn sm4ks<const BS: u8>(a: u32, b: u32) -> u32 {
    const { assert!(BS <= 3, "SM4 byte select must be in 0..=3") };
    macro_rules! call {
        ($f:ident) => {
            unsafe { $f(a, b) }
        };
    }
    match BS {
        0 => call!(rust_isa_riscv_zks_sm4ks_0),
        1 => call!(rust_isa_riscv_zks_sm4ks_1),
        2 => call!(rust_isa_riscv_zks_sm4ks_2),
        3 => call!(rust_isa_riscv_zks_sm4ks_3),
        _ => unreachable!(),
    }
}
