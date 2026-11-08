//! Integer, mask, broadcast, and lane-shift instructions.

use crate::x86::simd::Mask8;
use crate::x86::simd::Mask16;
use crate::x86::simd::avx512f::V512;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx512f_vpaddd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddd`
/// Add the 16 32-bit integer lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpaddd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpaddd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpsubd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubd`
/// Subtract the 16 32-bit integer lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpsubd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpsubd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpmulld(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulld`
/// Multiply the 16 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpmulld(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpmulld(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpaddq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddq`
/// Add the eight 64-bit integer lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpaddq(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpaddq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpsubq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubq`
/// Subtract the eight 64-bit integer lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpsubq(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpsubq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpandd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandd`
/// Bitwise AND of the 64 bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpandd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpandd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpandq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandq`
/// Bitwise AND of the 64 bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpandq(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpandq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpandnd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandnd`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpandnd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpandnd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpandnq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandnq`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpandnq(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpandnq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpord(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpord`
/// Bitwise OR of the 64 bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpord(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpord(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vporq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vporq`
/// Bitwise OR of the 64 bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vporq(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vporq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpxord(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpxord`
/// Bitwise XOR of the 64 bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpxord(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpxord(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpxorq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpxorq`
/// Bitwise XOR of the 64 bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpxorq(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpxorq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpternlogd`
/// Combine the three 32-bit lane values with the truth table in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpternlogd<const IMM: u8>(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vmovdqu64 zmm2, [{c}]",
            "vpternlogd zmm0, zmm1, zmm2, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            c = in(reg) c.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            out("zmm2") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpternlogq`
/// Combine the three 64-bit lane values with the truth table in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpternlogq<const IMM: u8>(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vmovdqu64 zmm2, [{c}]",
            "vpternlogq zmm0, zmm1, zmm2, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            c = in(reg) c.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            out("zmm2") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpcmpd`
/// Compare the 16 32-bit signed lanes with the predicate in `IMM` and return the mask. `IMM` must be in `0..=7`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpcmpd<const IMM: u8>(a: V512, b: V512) -> Mask16 {
    const {
        assert!(IMM <= 7, "vpcmp predicate must be in 0..=7");
    }
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpd k1, zmm0, zmm1, {imm}",
            "kmovw {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask16::from_bits(out as u16)
}

/// `vpcmpq`
/// Compare the eight 64-bit signed lanes with the predicate in `IMM` and return the mask. `IMM` must be in `0..=7`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpcmpq<const IMM: u8>(a: V512, b: V512) -> Mask8 {
    const {
        assert!(IMM <= 7, "vpcmp predicate must be in 0..=7");
    }
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpq k1, zmm0, zmm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask8::from_bits(out as u8)
}

/// `vpcmpud`
/// Compare the 16 32-bit unsigned lanes with the predicate in `IMM` and return the mask. `IMM` must be in `0..=7`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpcmpud<const IMM: u8>(a: V512, b: V512) -> Mask16 {
    const {
        assert!(IMM <= 7, "vpcmp predicate must be in 0..=7");
    }
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpud k1, zmm0, zmm1, {imm}",
            "kmovw {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask16::from_bits(out as u16)
}

/// `vpcmpuq`
/// Compare the eight 64-bit unsigned lanes with the predicate in `IMM` and return the mask. `IMM` must be in `0..=7`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpcmpuq<const IMM: u8>(a: V512, b: V512) -> Mask8 {
    const {
        assert!(IMM <= 7, "vpcmp predicate must be in 0..=7");
    }
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpuq k1, zmm0, zmm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask8::from_bits(out as u8)
}

unsafe extern "C" {
    fn rust_isa_avx512f_vptestmd(a: *const u8, b: *const u8) -> u16;
}

/// `vptestmd`
/// Test the bitwise AND of the 32-bit lanes and return the nonzero-lane mask.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vptestmd(a: V512, b: V512) -> Mask16 {
    unsafe { Mask16::from_bits(rust_isa_avx512f_vptestmd(a.as_ptr(), b.as_ptr())) }
}

unsafe extern "C" {
    fn rust_isa_avx512f_vptestmq(a: *const u8, b: *const u8) -> u8;
}

/// `vptestmq`
/// Test the bitwise AND of the 64-bit lanes and return the nonzero-lane mask.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vptestmq(a: V512, b: V512) -> Mask8 {
    unsafe { Mask8::from_bits(rust_isa_avx512f_vptestmq(a.as_ptr(), b.as_ptr())) }
}

unsafe extern "C" {
    fn rust_isa_avx512f_vptestnmd(a: *const u8, b: *const u8) -> u16;
}

/// `vptestnmd`
/// Test the bitwise AND of the 32-bit lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vptestnmd(a: V512, b: V512) -> Mask16 {
    unsafe { Mask16::from_bits(rust_isa_avx512f_vptestnmd(a.as_ptr(), b.as_ptr())) }
}

unsafe extern "C" {
    fn rust_isa_avx512f_vptestnmq(a: *const u8, b: *const u8) -> u8;
}

/// `vptestnmq`
/// Test the bitwise AND of the 64-bit lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vptestnmq(a: V512, b: V512) -> Mask8 {
    unsafe { Mask8::from_bits(rust_isa_avx512f_vptestnmq(a.as_ptr(), b.as_ptr())) }
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpbroadcastd(value: u32, out: *mut u8);
}

/// `vpbroadcastd`
/// Broadcast a 32-bit integer to the 16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpbroadcastd(value: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpbroadcastd(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpbroadcastq(value: u64, out: *mut u8);
}

/// `vpbroadcastq`
/// Broadcast a 64-bit integer to the eight lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpbroadcastq(value: u64) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpbroadcastq(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpermd(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermd`
/// Permute the 16 32-bit integer lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpermd(a: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpermd(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpermq(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermq`
/// Permute the eight 64-bit integer lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpermq(a: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpermq(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpslld(a: *const u8, count: u32, out: *mut u8);
}

/// `vpslld`
/// Shift the 16 32-bit integer lanes left by `count`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpslld(a: V512, count: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpslld(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpsrld(a: *const u8, count: u32, out: *mut u8);
}

/// `vpsrld`
/// Shift the 16 32-bit integer lanes right by `count`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpsrld(a: V512, count: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpsrld(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}
