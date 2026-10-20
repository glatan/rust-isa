//! GFNI.

use core::arch::asm;

use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_gfni_gf2p8mulb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `gf2p8mulb`
///
/// Multiply the bytes in GF(2^8) with the AES polynomial.
///
/// # Safety
///
/// The caller must enable GFNI.
#[target_feature(enable = "gfni")]
#[inline]
pub unsafe fn gf2p8mulb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_gfni_gf2p8mulb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `gf2p8affineqb`
///
/// Apply the affine transform selected by `IMM`.
///
/// # Safety
///
/// The caller must enable GFNI.
#[target_feature(enable = "gfni")]
#[inline]
pub unsafe fn gf2p8affineqb<const IMM: u8>(a: V128, b: V128) -> V128 {
    let a = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "gf2p8affineqb {output}, {b}, {imm}",
            output = inout(xmm_reg) a => output,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}
