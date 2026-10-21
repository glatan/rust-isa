//! Instruction wrappers.

use crate::x86::simd::avx512f::V512;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx512vbmi2_vpcompressb(a: *const u8, mask: u64, out: *mut u8);
}

/// `vpcompressb`
/// Pack the selected bytes to the low end of the result and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpcompressb(a: V512, mask: u64) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi2_vpcompressb(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vbmi2_vpexpandb(a: *const u8, mask: u64, out: *mut u8);
}

/// `vpexpandb`
/// Write source bytes into the selected positions and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpexpandb(a: V512, mask: u64) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi2_vpexpandb(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vbmi2_vpcompressw(a: *const u8, mask: u32, out: *mut u8);
}

/// `vpcompressw`
/// Pack the selected 16-bit lanes to the low end of the result and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpcompressw(a: V512, mask: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi2_vpcompressw(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vbmi2_vpexpandw(a: *const u8, mask: u32, out: *mut u8);
}

/// `vpexpandw`
/// Write source 16-bit lanes into the selected positions and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpexpandw(a: V512, mask: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi2_vpexpandw(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

/// `vpshldw`
/// Shift and merge the 16-bit lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpshldw<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshldw zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshldd`
/// Shift and merge the 32-bit lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpshldd<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshldd zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshldq`
/// Shift and merge the 64-bit lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpshldq<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshldq zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshrdw`
/// Shift right and merge the 16-bit lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpshrdw<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshrdw zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshrdd`
/// Shift right and merge the 32-bit lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpshrdd<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshrdd zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshrdq`
/// Shift right and merge the 64-bit lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpshrdq<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshrdq zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}
