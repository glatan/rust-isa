//! SSE4a.
//!
//! SSE4a is not part of the x86-64 baseline and is available on AMD processors.
//! Every operation is `unsafe` and the caller must enable SSE4a with
//! `#[target_feature(enable = "sse4a")]` or a global target feature.
//!
//! The values are [`V128`]. `extrq` and `insertq` use the register-controlled
//! form. The control vector packs the length in bits 5:0 and the index in bits
//! 13:8 of its low 64 bits. Immediate operands are const generics unless
//! documented otherwise.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.
//!
//! Instructions not listed in this module are not exposed.

use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_sse4a_extrq(a: *const u8, control: *const u8, out: *mut u8);
    fn rust_isa_sse4a_insertq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `extrq`
///
/// Extract a bit field from the low 64 bits.
///
/// # Safety
///
/// The caller must enable SSE4a.
#[target_feature(enable = "sse4a")]
#[inline]
pub unsafe fn extrq(a: V128, control: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse4a_extrq(a.as_ptr(), control.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `insertq`
///
/// Insert the low bits of `b` into `a`. The register form takes the length
/// from bits 69:64 of `b` and the index from bits 77:72 of `b`; these are the
/// low six bits of bytes 8 and 9. A length of zero means 64. The source bits
/// are the low 64 bits of `b`. The high 64 bits of the result are undefined by
/// the instruction.
///
/// # Safety
///
/// The caller must enable SSE4a.
#[target_feature(enable = "sse4a")]
#[inline]
pub unsafe fn insertq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse4a_insertq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

use core::arch::asm;

/// `extrq` with immediate operands.
///
/// Extract `LEN` bits starting at bit `IDX` from the low 64 bits. `LEN` and
/// `IDX` must be in `0..=63`.
///
/// # Safety
///
/// The caller must enable SSE4a.
#[target_feature(enable = "sse4a")]
#[inline]
pub unsafe fn extrq_imm<const LEN: u8, const IDX: u8>(a: V128) -> V128 {
    const {
        assert!(LEN <= 63, "extrq length must be in 0..=63");
        assert!(IDX <= 63, "extrq index must be in 0..=63");
    }
    let input = u128::from_le_bytes(a.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "extrq {output}, {len}, {idx}",
            output = inout(xmm_reg) input => output,
            len = const LEN,
            idx = const IDX,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}

/// `insertq` with immediate operands.
///
/// Insert `LEN` bits from `b` at bit `IDX` of `a`. `LEN` and `IDX` must be in
/// `0..=63`.
///
/// # Safety
///
/// The caller must enable SSE4a.
#[target_feature(enable = "sse4a")]
#[inline]
pub unsafe fn insertq_imm<const LEN: u8, const IDX: u8>(a: V128, b: V128) -> V128 {
    const {
        assert!(LEN <= 63, "insertq length must be in 0..=63");
        assert!(IDX <= 63, "insertq index must be in 0..=63");
    }
    let a = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "insertq {output}, {b}, {len}, {idx}",
            output = inout(xmm_reg) a => output,
            b = in(xmm_reg) b,
            len = const LEN,
            idx = const IDX,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}

unsafe extern "C" {
    fn rust_isa_sse4a_movntsd(p: *mut f32, a: *const u8);
    fn rust_isa_sse4a_movntss(p: *mut f32, a: *const u8);
}

/// `movntsd`
///
/// Store the low 64-bit float lane with a non-temporal hint.
///
/// # Safety
///
/// The caller must enable SSE4a and `p` must be valid for writing 8 bytes.
#[target_feature(enable = "sse4a")]
#[inline]
pub unsafe fn movntsd(p: *mut f64, a: V128) {
    unsafe {
        rust_isa_sse4a_movntsd(p.cast(), a.as_ptr());
    }
}

/// `movntss`
///
/// Store the low 32-bit float lane with a non-temporal hint.
///
/// # Safety
///
/// The caller must enable SSE4a and `p` must be valid for writing 4 bytes.
#[target_feature(enable = "sse4a")]
#[inline]
pub unsafe fn movntss(p: *mut f32, a: V128) {
    unsafe {
        rust_isa_sse4a_movntss(p, a.as_ptr());
    }
}
