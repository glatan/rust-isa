//! SSE4.1.
//!
//! SSE4.1 is not part of the x86-64 baseline. Every operation is `unsafe` and
//! the caller must enable SSE4.1 with `#[target_feature(enable = "sse4.1")]` or
//! a global target feature.
//!
//! The values are [`V128`]. Comparison and blend masks use the most significant
//! bit of each lane. The module contains representative integer, conversion and
//! blend operations. The dot-product and insert/extract instructions that take
//! an immediate are intentionally omitted.

use super::sse2::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_sse41_pmulld(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pminud(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmaxud(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pminuw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmaxuw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_ptest(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse41_blendvps(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_sse41_blendvpd(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_sse41_pblendvb(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_sse41_packusdw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_phminposuw(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovsxbd(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovsxbw(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovzxbw(a: *const u8, out: *mut u8);
}

/// `pmulld`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmulld(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmulld(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pminud`
///
/// Return the minimum of each unsigned 32-bit lane.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pminud(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pminud(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmaxud`
///
/// Return the maximum of each unsigned 32-bit lane.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmaxud(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmaxud(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pminuw`
///
/// Return the minimum of each unsigned 16-bit lane.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pminuw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pminuw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmaxuw`
///
/// Return the maximum of each unsigned 16-bit lane.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmaxuw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmaxuw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ptest`
///
/// Return `true` when the bitwise AND of the two values is zero.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn ptest(a: V128, b: V128) -> bool {
    unsafe { rust_isa_sse41_ptest(a.as_ptr(), b.as_ptr()) != 0 }
}

/// `blendvps`
///
/// Select 32-bit float lanes from `b` where the sign bit of the corresponding
/// lane in `mask` is set, otherwise from `a`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn blendvps(a: V128, b: V128, mask: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_blendvps(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `blendvpd`
///
/// Select 64-bit float lanes from `b` where the sign bit of the corresponding
/// lane in `mask` is set, otherwise from `a`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn blendvpd(a: V128, b: V128, mask: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_blendvpd(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pblendvb`
///
/// Select bytes from `b` where the high bit of the corresponding byte in `mask`
/// is set, otherwise from `a`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pblendvb(a: V128, b: V128, mask: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pblendvb(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `packusdw`
///
/// Convert signed 32-bit lanes to unsigned 16-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn packusdw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_packusdw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `phminposuw`
///
/// Return the minimum unsigned 16-bit lane in the low lane and its index in the
/// next lane. The remaining lanes are zero.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn phminposuw(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_phminposuw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmovsxbd`
///
/// Sign-extend the low four bytes to four 32-bit lanes.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmovsxbd(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmovsxbd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmovsxbw`
///
/// Sign-extend the low eight bytes to eight 16-bit lanes.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmovsxbw(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmovsxbw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmovzxbw`
///
/// Zero-extend the low eight bytes to eight 16-bit lanes.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmovzxbw(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmovzxbw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pinsrb`
///
/// Insert the low byte of `value` into lane `IMM`. `IMM` must be in `0..=15`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pinsrb<const IMM: u8>(a: V128, value: u8) -> V128 {
    const { assert!(IMM <= 15, "pinsrb lane must be in 0..=15") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    unsafe {
        asm!(
            "pinsrb {out}, {value:e}, {imm}",
            out = inout(xmm_reg) out,
            value = in(reg) value as u32,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `pinsrd`
///
/// Insert `value` into 32-bit lane `IMM`. `IMM` must be in `0..=3`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pinsrd<const IMM: u8>(a: V128, value: u32) -> V128 {
    const { assert!(IMM <= 3, "pinsrd lane must be in 0..=3") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    unsafe {
        asm!(
            "pinsrd {out}, {value:e}, {imm}",
            out = inout(xmm_reg) out,
            value = in(reg) value,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `pextrb`
///
/// Extract lane `IMM` as a byte. `IMM` must be in `0..=15`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pextrb<const IMM: u8>(a: V128) -> u8 {
    const { assert!(IMM <= 15, "pextrb lane must be in 0..=15") };
    let a = u128::from_le_bytes(a.to_bytes());
    let out: u32;
    unsafe {
        asm!(
            "pextrb {out:e}, {a}, {imm}",
            out = lateout(reg) out,
            a = in(xmm_reg) a,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    out as u8
}

/// `pextrd`
///
/// Extract 32-bit lane `IMM`. `IMM` must be in `0..=3`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pextrd<const IMM: u8>(a: V128) -> u32 {
    const { assert!(IMM <= 3, "pextrd lane must be in 0..=3") };
    let a = u128::from_le_bytes(a.to_bytes());
    let out: u32;
    unsafe {
        asm!(
            "pextrd {out:e}, {a}, {imm}",
            out = lateout(reg) out,
            a = in(xmm_reg) a,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    out
}

/// `pinsrq`
///
/// Insert `value` into 64-bit lane `IMM`. `IMM` must be in `0..=1`.
///
/// # Safety
///
/// The caller must enable SSE4.1 and target x86-64.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pinsrq<const IMM: u8>(a: V128, value: u64) -> V128 {
    const { assert!(IMM <= 1, "pinsrq lane must be in 0..=1") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    unsafe {
        asm!(
            "pinsrq {out}, {value}, {imm}",
            out = inout(xmm_reg) out,
            value = in(reg) value,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `pextrq`
///
/// Extract 64-bit lane `IMM`. `IMM` must be in `0..=1`.
///
/// # Safety
///
/// The caller must enable SSE4.1 and target x86-64.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pextrq<const IMM: u8>(a: V128) -> u64 {
    const { assert!(IMM <= 1, "pextrq lane must be in 0..=1") };
    let a = u128::from_le_bytes(a.to_bytes());
    let out: u64;
    unsafe {
        asm!(
            "pextrq {out}, {a}, {imm}",
            out = lateout(reg) out,
            a = in(xmm_reg) a,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    out
}
