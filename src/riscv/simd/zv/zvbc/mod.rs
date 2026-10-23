//! RISC-V vector cryptography: Zvbc (carryless multiply).

#[allow(unused_imports)]
use super::crypto_binary;
use crate::riscv::simd::rvv::V128;
unsafe extern "C" {
    fn rust_isa_rvv_vclmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vclmulh(a: *const u8, b: *const u8, out: *mut u8);
}

crypto_binary!(
    /// `vclmul.vv` on 64-bit lanes.
    vclmul,
    rust_isa_rvv_vclmul,
    "zvbc"
);
crypto_binary!(
    /// `vclmulh.vv` on 64-bit lanes.
    vclmulh,
    rust_isa_rvv_vclmulh,
    "zvbc"
);
