//! RISC-V vector cryptography: zvkg.

#[allow(unused_imports)]
use super::crypto_ternary;
use crate::riscv::simd::rvv::V128;

unsafe extern "C" {
    fn rust_isa_rvv_vghsh(acc: *const u8, value: *const u8, key: *const u8, out: *mut u8);
}

crypto_ternary!(
    /// `vghsh.vv`.
    vghsh,
    rust_isa_rvv_vghsh
);
unsafe extern "C" {
    fn rust_isa_rvv_vgmul(acc: *const u8, key: *const u8, out: *mut u8);
}

/// `vgmul.vv`.
///
/// # Safety
///
/// The caller must compile with the vector cryptography extensions enabled.
#[inline(always)]
pub unsafe fn vgmul(acc: V128, key: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vgmul(acc.as_ptr(), key.as_ptr(), out.as_mut_ptr());
    }
    out
}
