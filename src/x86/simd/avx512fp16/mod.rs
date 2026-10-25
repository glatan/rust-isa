//! AVX-512FP16.
//!
//! AVX-512FP16 is not part of the x86-64 baseline. Every instruction wrapper is `unsafe`
//! and the required target features are documented on each function. The
//! values are [`V512`], [`V256`], and [`V128`], depending on the result width.
//! Binary16 values are bit patterns in the vector lanes.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The build compiles the bundled C with clang; some wrappers use `asm!`.
//! Immediate operands are const generics, and the valid range is documented on
//! each function. Mask arguments use one bit per lane, with the low lane in the
//! low bit.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512fp16::vaddph;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vaddph(v, v) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avx512fp16::vaddph as _;
//! use rust_isa::x86::simd::avx512fp16::vcmpph as _;
//! use rust_isa::x86::simd::avx512fp16::vcomish as _;
//! use rust_isa::x86::simd::avx512fp16::vcvtpd2ph as _;
//! use rust_isa::x86::simd::avx512fp16::vcvtph2pd as _;
//! use rust_isa::x86::simd::avx512fp16::vcvtph2psx as _;
//! use rust_isa::x86::simd::avx512fp16::vcvtps2phx as _;
//! use rust_isa::x86::simd::avx512fp16::vcvtsd2sh as _;
//! use rust_isa::x86::simd::avx512fp16::vcvtsh2sd as _;
//! use rust_isa::x86::simd::avx512fp16::vcvtsh2ss as _;
//! use rust_isa::x86::simd::avx512fp16::vcvtss2sh as _;
//! use rust_isa::x86::simd::avx512fp16::vdivph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmadd132ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmadd213ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmadd231ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmaddsub132ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmaddsub213ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmaddsub231ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmsub132ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmsub213ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmsub231ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmsubadd132ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmsubadd213ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfmsubadd231ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfnmadd132ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfnmadd213ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfnmadd231ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfnmsub132ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfnmsub213ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfnmsub231ph as _;
//! use rust_isa::x86::simd::avx512fp16::vfpclassph as _;
//! use rust_isa::x86::simd::avx512fp16::vgetexpph as _;
//! use rust_isa::x86::simd::avx512fp16::vgetmantph as _;
//! use rust_isa::x86::simd::avx512fp16::vmaxph as _;
//! use rust_isa::x86::simd::avx512fp16::vminph as _;
//! use rust_isa::x86::simd::avx512fp16::vmulph as _;
//! use rust_isa::x86::simd::avx512fp16::vrcpph as _;
//! use rust_isa::x86::simd::avx512fp16::vreduceph as _;
//! use rust_isa::x86::simd::avx512fp16::vrndscaleph as _;
//! use rust_isa::x86::simd::avx512fp16::vrsqrtph as _;
//! use rust_isa::x86::simd::avx512fp16::vscalefph as _;
//! use rust_isa::x86::simd::avx512fp16::vsqrtph as _;
//! use rust_isa::x86::simd::avx512fp16::vsubph as _;
//! use rust_isa::x86::simd::avx512fp16::vucomish as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::x86::simd::avx2::V256;
use crate::x86::simd::avx512f::V512;
use crate::x86::simd::sse2::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx512fp16_vaddph(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaddph`
/// Add the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vaddph(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vaddph(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vsubph(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vsubph`
/// Subtract the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vsubph(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vsubph(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vmulph(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vmulph`
/// Multiply the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vmulph(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vmulph(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vdivph(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vdivph`
/// Divide the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vdivph(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vdivph(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vsqrtph(a: *const u8, out: *mut u8);
}

/// `vsqrtph`
/// Compute the square root of the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vsqrtph(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vsqrtph(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vmaxph(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vmaxph`
/// Take the maximum of the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vmaxph(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vmaxph(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vminph(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vminph`
/// Take the minimum of the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vminph(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vminph(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcmpph`
/// Compare the 32 binary16 lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcmpph<const IMM: u8>(a: V512, b: V512) -> u32 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vcmpph k1, zmm0, zmm1, {imm}",
            "kmovd {out:e}, k1",
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
    out as u32
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vcomish(a: *const u8, b: *const u8) -> u32;
}

/// `vcomish`
/// Compare the low binary16 lanes and return CF, PF, and ZF.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcomish(a: V128, b: V128) -> u32 {
    unsafe { rust_isa_avx512fp16_vcomish(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vucomish(a: *const u8, b: *const u8) -> u32;
}

/// `vucomish`
/// Compare the low binary16 lanes without signaling on quiet NaN and return CF, PF, and ZF.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vucomish(a: V128, b: V128) -> u32 {
    unsafe { rust_isa_avx512fp16_vucomish(a.as_ptr(), b.as_ptr()) }
}

/// `vfpclassph`
/// Classify the 32 binary16 lanes with the categories in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfpclassph<const IMM: u8>(a: V512) -> u32 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vfpclassph k1, zmm0, {imm}",
            "kmovd {out:e}, k1",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u32
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vgetexpph(a: *const u8, out: *mut u8);
}

/// `vgetexpph`
/// Extract the exponent of the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vgetexpph(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vgetexpph(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vgetmantph`
/// Extract the mantissa of the 32 binary16 lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vgetmantph<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vgetmantph zmm0, zmm0, {imm}",
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
    fn rust_isa_avx512fp16_vscalefph(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefph`
/// Scale the 32 binary16 lanes by powers of two.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vscalefph(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vscalefph(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrndscaleph`
/// Round the 32 binary16 lanes to the precision and mode in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vrndscaleph<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vrndscaleph zmm0, zmm0, {imm}",
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

/// `vreduceph`
/// Reduce the 32 binary16 lanes to the precision in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vreduceph<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vreduceph zmm0, zmm0, {imm}",
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
    fn rust_isa_avx512fp16_vrcpph(a: *const u8, out: *mut u8);
}

/// `vrcpph`
/// Compute an approximate reciprocal of the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vrcpph(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vrcpph(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vrsqrtph(a: *const u8, out: *mut u8);
}

/// `vrsqrtph`
/// Compute an approximate reciprocal square root of the 32 binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vrsqrtph(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vrsqrtph(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vcvtph2pd(a: *const u8, out: *mut u8);
}

/// `vcvtph2pd`
/// Convert eight binary16 lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcvtph2pd(a: V128) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vcvtph2pd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vcvtpd2ph(a: *const u8, out: *mut u8);
}

/// `vcvtpd2ph`
/// Convert eight 64-bit float lanes to binary16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcvtpd2ph(a: V512) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512fp16_vcvtpd2ph(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vcvtsh2ss(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vcvtsh2ss`
/// Convert the low binary16 lane of `b` to 32 bits and copy it to `a`.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcvtsh2ss(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512fp16_vcvtsh2ss(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vcvtss2sh(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vcvtss2sh`
/// Convert the low 32-bit float lane of `b` to binary16 and copy it to `a`.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcvtss2sh(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512fp16_vcvtss2sh(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vcvtsh2sd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vcvtsh2sd`
/// Convert the low binary16 lane of `b` to 64 bits and copy it to `a`.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcvtsh2sd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512fp16_vcvtsh2sd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vcvtsd2sh(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vcvtsd2sh`
/// Convert the low 64-bit float lane of `b` to binary16 and copy it to `a`.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcvtsd2sh(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512fp16_vcvtsd2sh(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vcvtph2psx(a: *const u8, out: *mut u8);
}

/// `vcvtph2psx`
/// Convert sixteen binary16 lanes to 32-bit float lanes with the extended conversion rules.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcvtph2psx(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vcvtph2psx(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vcvtps2phx(a: *const u8, out: *mut u8);
}

/// `vcvtps2phx`
/// Convert sixteen 32-bit float lanes to binary16 lanes with the extended conversion rules.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vcvtps2phx(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512fp16_vcvtps2phx(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmadd132ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd132ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmadd132ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmadd132ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmadd213ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd213ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmadd213ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmadd213ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmadd231ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd231ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmadd231ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmadd231ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmsub132ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsub132ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmsub132ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmsub132ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmsub213ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsub213ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmsub213ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmsub213ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmsub231ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsub231ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmsub231ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmsub231ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfnmadd132ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd132ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfnmadd132ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfnmadd132ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfnmadd213ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd213ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfnmadd213ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfnmadd213ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfnmadd231ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd231ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfnmadd231ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfnmadd231ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfnmsub132ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub132ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfnmsub132ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfnmsub132ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfnmsub213ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub213ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfnmsub213ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfnmsub213ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfnmsub231ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub231ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfnmsub231ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfnmsub231ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmaddsub132ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub132ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmaddsub132ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmaddsub132ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmaddsub213ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub213ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmaddsub213ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmaddsub213ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmaddsub231ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub231ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmaddsub231ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmaddsub231ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmsubadd132ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd132ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmsubadd132ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmsubadd132ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmsubadd213ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd213ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmsubadd213ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmsubadd213ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512fp16_vfmsubadd231ph(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd231ph`
/// Multiply `a` and `b`, then add or subtract `c` or the product, according to the operation.
///
/// # Safety
///
/// The caller must enable AVX-512FP16.
#[target_feature(enable = "avx512fp16")]
#[inline]
pub unsafe fn vfmsubadd231ph(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512fp16_vfmsubadd231ph(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}
