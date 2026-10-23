//! RISC-V vector cryptography: Zvks (SM3 and SM4).

#[allow(unused_imports)]
use super::{crypto_binary, crypto_key_schedule};
use crate::riscv::simd::rvv::V128;
unsafe extern "C" {
    fn rust_isa_rvv_vsm3c(state: *const u8, round: u32, out: *mut u8);
    fn rust_isa_rvv_vsm3me(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsm4k(key: *const u8, round: u32, out: *mut u8);
    fn rust_isa_rvv_vsm4r(state: *const u8, key: *const u8, out: *mut u8);
}

/// `vsm3c.vi` on a 256-bit element group.
///
/// `round` must be at most 31.
///
/// # Safety
///
/// The caller must compile with the vector cryptography extensions enabled and
/// pass a valid round number.
#[inline(always)]
pub unsafe fn vsm3c(state: [V128; 2], round: u32) -> [V128; 2] {
    let mut out = [V128::from_bytes([0; 16]); 2];
    unsafe {
        rust_isa_rvv_vsm3c(state.as_ptr().cast(), round, out.as_mut_ptr().cast());
    }
    out
}
/// `vsm3me.vv` on a 256-bit element group.
///
/// # Safety
///
/// The caller must compile with the vector cryptography extensions enabled.
#[inline(always)]
pub unsafe fn vsm3me(a: [V128; 2], b: [V128; 2]) -> [V128; 2] {
    let mut out = [V128::from_bytes([0; 16]); 2];
    unsafe {
        rust_isa_rvv_vsm3me(
            a.as_ptr().cast(),
            b.as_ptr().cast(),
            out.as_mut_ptr().cast(),
        );
    }
    out
}
crypto_key_schedule!(
    /// `vsm4k.vi`, with `round` in `0..=7`.
    vsm4k,
    rust_isa_rvv_vsm4k
);
crypto_binary!(
    /// `vsm4r.vv`.
    vsm4r,
    rust_isa_rvv_vsm4r
);
