//! Floating-point, conversion, shuffle, and scalar conversion instructions.

use crate::x86::simd::avx2::V256;
use crate::x86::simd::avx512f::V512;
use crate::x86::simd::sse2::V128;
use core::arch::asm;

/// `vpermilps`
/// Permute the 16 32-bit float lanes within 128-bit blocks with `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpermilps<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vpermilps zmm0, zmm0, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpermilpd`
/// Permute the eight 64-bit float lanes within 128-bit blocks with `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpermilpd<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vpermilpd zmm0, zmm0, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vshufps`
/// Shuffle the 32-bit float lanes with `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vshufps<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vshufps zmm0, zmm0, zmm1, {imm}",
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

/// `vshufpd`
/// Shuffle the 64-bit float lanes with `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vshufpd<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vshufpd zmm0, zmm0, zmm1, {imm}",
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

/// `vinsertps`
/// Insert a 32-bit float lane selected by `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vinsertps<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vinsertps xmm0, xmm0, xmm1, {imm}",
            "vmovdqu [{out}], xmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("xmm0") _,
            out("xmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vinsertf32x4`
/// Insert a 128-bit block of 32-bit float lanes at the position in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vinsertf32x4<const IMM: u8>(a: V512, b: V128) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vinsertf32x4 zmm0, zmm0, xmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("xmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vinsertf64x4`
/// Insert a 256-bit block of 64-bit float lanes at the position in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vinsertf64x4<const IMM: u8>(a: V512, b: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vinsertf64x4 zmm0, zmm0, ymm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vextractf32x4`
/// Extract the 128-bit block of 32-bit float lanes selected by `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vextractf32x4<const IMM: u8>(a: V512) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vextractf32x4 xmm1, zmm0, {imm}",
            "vmovdqu [{out}], xmm1",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("xmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vextractf64x4`
/// Extract the 256-bit block of 64-bit float lanes selected by `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vextractf64x4<const IMM: u8>(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vextractf64x4 ymm1, zmm0, {imm}",
            "vmovdqu [{out}], ymm1",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vcompressps(a: *const u8, mask: u16, out: *mut u8);
}

/// `vcompressps`
/// Pack the selected 32-bit float lanes to the low end and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vcompressps(a: V512, mask: u16) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vcompressps(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vcompresspd(a: *const u8, mask: u8, out: *mut u8);
}

/// `vcompresspd`
/// Pack the selected 64-bit float lanes to the low end and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vcompresspd(a: V512, mask: u8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vcompresspd(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vexpandps(a: *const u8, mask: u16, out: *mut u8);
}

/// `vexpandps`
/// Write the low source lanes into the selected positions and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vexpandps(a: V512, mask: u16) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vexpandps(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vexpandpd(a: *const u8, mask: u8, out: *mut u8);
}

/// `vexpandpd`
/// Write the low source lanes into the selected positions and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vexpandpd(a: V512, mask: u8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vexpandpd(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vblendmps(a: *const u8, b: *const u8, mask: u16, out: *mut u8);
}

/// `vblendmps`
/// Blend the 32-bit float lanes selected by `mask` from `b` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vblendmps(a: V512, b: V512, mask: u16) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vblendmps(a.as_ptr(), b.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vblendmpd(a: *const u8, b: *const u8, mask: u8, out: *mut u8);
}

/// `vblendmpd`
/// Blend the 64-bit float lanes selected by `mask` from `b` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vblendmpd(a: V512, b: V512, mask: u8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vblendmpd(a.as_ptr(), b.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vmovaps(a: *const u8, src: *const u8, mask: u16, out: *mut u8);
}

/// `vmovaps`
/// Move the 32-bit float lanes selected by `mask` from `src` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vmovaps(a: V512, src: V512, mask: u16) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vmovaps(a.as_ptr(), src.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vmovapd(a: *const u8, src: *const u8, mask: u8, out: *mut u8);
}

/// `vmovapd`
/// Move the 64-bit float lanes selected by `mask` from `src` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vmovapd(a: V512, src: V512, mask: u8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vmovapd(a.as_ptr(), src.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vcvtps2pd(a: *const u8, out: *mut u8);
}

/// `vcvtps2pd`
/// Convert eight 32-bit float lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vcvtps2pd(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vcvtps2pd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vcvtpd2ps(a: *const u8, out: *mut u8);
}

/// `vcvtpd2ps`
/// Convert eight 64-bit float lanes to 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vcvtpd2ps(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512f_vcvtpd2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vcvtss2sd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vcvtss2sd`
/// Convert the low 32-bit float lane of `b` to 64 bits and copy it to `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vcvtss2sd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512f_vcvtss2sd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vcvtsd2ss(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vcvtsd2ss`
/// Convert the low 64-bit float lane of `b` to 32 bits and copy it to `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vcvtsd2ss(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512f_vcvtsd2ss(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vcvtdq2ps(a: *const u8, out: *mut u8);
}

/// `vcvtdq2ps`
/// Convert the 16 signed 32-bit integer lanes to 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vcvtdq2ps(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vcvtdq2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vcvtps2dq(a: *const u8, out: *mut u8);
}

/// `vcvtps2dq`
/// Convert the 16 32-bit float lanes to signed 32-bit integers.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vcvtps2dq(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vcvtps2dq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vcvttps2dq(a: *const u8, out: *mut u8);
}

/// `vcvttps2dq`
/// Convert the 16 32-bit float lanes to signed 32-bit integers with truncation.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vcvttps2dq(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vcvttps2dq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vcvtpd2dq(a: *const u8, out: *mut u8);
}

/// `vcvtpd2dq`
/// Convert the eight 64-bit float lanes to signed 32-bit integers.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vcvtpd2dq(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512f_vcvtpd2dq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vcvttpd2dq(a: *const u8, out: *mut u8);
}

/// `vcvttpd2dq`
/// Convert the eight 64-bit float lanes to signed 32-bit integers with truncation.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vcvttpd2dq(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512f_vcvttpd2dq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vsqrtps(a: *const u8, out: *mut u8);
}

/// `vsqrtps`
/// Compute the square root of the 16 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vsqrtps(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vsqrtps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vsqrtpd(a: *const u8, out: *mut u8);
}

/// `vsqrtpd`
/// Compute the square root of the eight 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vsqrtpd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vsqrtpd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vgetexpps(a: *const u8, out: *mut u8);
}

/// `vgetexpps`
/// Extract the exponent of the 16 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vgetexpps(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vgetexpps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vgetexppd(a: *const u8, out: *mut u8);
}

/// `vgetexppd`
/// Extract the exponent of the eight 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vgetexppd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vgetexppd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vgetmantps`
/// Extract the mantissa of the 16 32-bit float lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vgetmantps<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vgetmantps zmm0, zmm0, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vgetmantpd`
/// Extract the mantissa of the eight 64-bit float lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vgetmantpd<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vgetmantpd zmm0, zmm0, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vscalefps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefps`
/// Scale the 32-bit float lanes by powers of two.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vscalefps(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vscalefps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vscalefpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefpd`
/// Scale the 64-bit float lanes by powers of two.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vscalefpd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vscalefpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrndscaleps`
/// Round the 32-bit float lanes to the precision and mode in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vrndscaleps<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vrndscaleps zmm0, zmm0, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vrndscalepd`
/// Round the 64-bit float lanes to the precision and mode in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vrndscalepd<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vrndscalepd zmm0, zmm0, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vrcpps(a: *const u8, out: *mut u8);
}

/// `vrcpps`
/// Compute an approximate reciprocal of the 16 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vrcpps(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vrcpps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vrsqrtps(a: *const u8, out: *mut u8);
}

/// `vrsqrtps`
/// Compute an approximate reciprocal square root of the 16 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vrsqrtps(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vrsqrtps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
