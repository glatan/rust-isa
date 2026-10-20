//! PCLMULQDQ.

use core::arch::asm;

use crate::x86::simd::sse2::V128;

/// `pclmulqdq`
///
/// Multiply 64-bit halves and keep the 128-bit carry-less product. `IMM`
/// selects the halves.
///
/// # Safety
///
/// The caller must enable PCLMULQDQ.
#[target_feature(enable = "pclmulqdq")]
#[inline]
pub unsafe fn pclmulqdq<const IMM: u8>(a: V128, b: V128) -> V128 {
    let a = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "pclmulqdq {output}, {b}, {imm}",
            output = inout(xmm_reg) a => output,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}
