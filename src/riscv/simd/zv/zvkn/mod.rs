//! RISC-V vector cryptography: zvkn.

#[allow(unused_imports)]
use super::{crypto_binary, crypto_key_schedule, crypto_ternary};
use crate::riscv::simd::rvv::V128;

unsafe extern "C" {
    fn rust_isa_rvv_vaesdf(state: *const u8, key: *const u8, out: *mut u8);
}

crypto_binary!(
    /// `vaesdf.vv`.
    vaesdf,
    rust_isa_rvv_vaesdf
);
unsafe extern "C" {
    fn rust_isa_rvv_vaesdm(state: *const u8, key: *const u8, out: *mut u8);
}

crypto_binary!(
    /// `vaesdm.vv`.
    vaesdm,
    rust_isa_rvv_vaesdm
);
unsafe extern "C" {
    fn rust_isa_rvv_vaesef(state: *const u8, key: *const u8, out: *mut u8);
}

crypto_binary!(
    /// `vaesef.vv`.
    vaesef,
    rust_isa_rvv_vaesef
);
unsafe extern "C" {
    fn rust_isa_rvv_vaesem(state: *const u8, key: *const u8, out: *mut u8);
}

crypto_binary!(
    /// `vaesem.vv`.
    vaesem,
    rust_isa_rvv_vaesem
);
unsafe extern "C" {
    fn rust_isa_rvv_vaeskf1(key: *const u8, round: u32, out: *mut u8);
}

crypto_key_schedule!(
    /// `vaeskf1.vi`, with `round` in `1..=10`.
    vaeskf1,
    rust_isa_rvv_vaeskf1
);
unsafe extern "C" {
    fn rust_isa_rvv_vaeskf2(key: *const u8, round: u32, out: *mut u8);
}

crypto_key_schedule!(
    /// `vaeskf2.vi`, with `round` in `2..=10`.
    vaeskf2,
    rust_isa_rvv_vaeskf2
);
unsafe extern "C" {
    fn rust_isa_rvv_vaesz(state: *const u8, key: *const u8, out: *mut u8);
}

crypto_binary!(
    /// `vaesz.vs`.
    vaesz,
    rust_isa_rvv_vaesz
);
unsafe extern "C" {
    fn rust_isa_rvv_vsha2ch(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

crypto_ternary!(
    /// `vsha2ch.vv`.
    vsha2ch,
    rust_isa_rvv_vsha2ch
);
unsafe extern "C" {
    fn rust_isa_rvv_vsha2cl(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

crypto_ternary!(
    /// `vsha2cl.vv`.
    vsha2cl,
    rust_isa_rvv_vsha2cl
);
unsafe extern "C" {
    fn rust_isa_rvv_vsha2ms(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

crypto_ternary!(
    /// `vsha2ms.vv`.
    vsha2ms,
    rust_isa_rvv_vsha2ms
);
