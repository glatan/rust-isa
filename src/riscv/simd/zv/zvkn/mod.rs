//! RISC-V vector cryptography: Zvkn (AES and SHA-2).

#[allow(unused_imports)]
use super::{crypto_binary, crypto_key_schedule, crypto_ternary};
use crate::riscv::simd::rvv::V128;
unsafe extern "C" {
    fn rust_isa_zvkn_vaesdf(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_zvkn_vaesdm(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_zvkn_vaesef(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_zvkn_vaesem(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_zvkn_vaeskf1(key: *const u8, round: u32, out: *mut u8);
    fn rust_isa_zvkn_vaeskf2(key: *const u8, round: u32, out: *mut u8);
    fn rust_isa_zvkn_vaesz(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_zvkn_vsha2ch(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_zvkn_vsha2cl(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_zvkn_vsha2ms(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

crypto_binary!(
    /// `vaesdf.vv`.
    vaesdf,
    rust_isa_zvkn_vaesdf,
    "zvkn"
);
crypto_binary!(
    /// `vaesdm.vv`.
    vaesdm,
    rust_isa_zvkn_vaesdm,
    "zvkn"
);
crypto_binary!(
    /// `vaesef.vv`.
    vaesef,
    rust_isa_zvkn_vaesef,
    "zvkn"
);
crypto_binary!(
    /// `vaesem.vv`.
    vaesem,
    rust_isa_zvkn_vaesem,
    "zvkn"
);
crypto_key_schedule!(
    /// `vaeskf1.vi`, with `round` in `1..=10`.
    vaeskf1,
    rust_isa_zvkn_vaeskf1,
    "zvkn"
);
crypto_key_schedule!(
    /// `vaeskf2.vi`, with `round` in `2..=10`.
    vaeskf2,
    rust_isa_zvkn_vaeskf2,
    "zvkn"
);
crypto_binary!(
    /// `vaesz.vs`.
    vaesz,
    rust_isa_zvkn_vaesz,
    "zvkn"
);
crypto_ternary!(
    /// `vsha2ch.vv`.
    vsha2ch,
    rust_isa_zvkn_vsha2ch,
    "zvkn"
);
crypto_ternary!(
    /// `vsha2cl.vv`.
    vsha2cl,
    rust_isa_zvkn_vsha2cl,
    "zvkn"
);
crypto_ternary!(
    /// `vsha2ms.vv`.
    vsha2ms,
    rust_isa_zvkn_vsha2ms,
    "zvkn"
);
