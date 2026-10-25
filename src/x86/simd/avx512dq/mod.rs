//! AVX-512DQ.
//!
//! AVX-512DQ is not part of the x86-64 baseline. Every instruction wrapper is `unsafe`
//! and the required target features are documented on each function. The
//! values are [`V512`], [`V256`], and [`V128`], depending on the result width.
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
//! use rust_isa::x86::simd::avx512dq::vpmullq;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! assert_eq!(unsafe { vpmullq(v, v) }.to_bytes(), [0; 64]);
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512dq::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vandnpd(v512, v512) };
//! let _ = unsafe { vandnps(v512, v512) };
//! let _ = unsafe { vandpd(v512, v512) };
//! let _ = unsafe { vandps(v512, v512) };
//! let _ = unsafe { vcmppd::<0>(v512, v512) };
//! let _ = unsafe { vcmpps::<0>(v512, v512) };
//! let _ = unsafe { vcmpsd::<0>(v128, v128) };
//! let _ = unsafe { vcmpss::<0>(v128, v128) };
//! let _ = unsafe { vcvtpd2qq(v512) };
//! let _ = unsafe { vcvtpd2uqq(v512) };
//! let _ = unsafe { vcvtps2qq(v256) };
//! let _ = unsafe { vcvtps2uqq(v256) };
//! let _ = unsafe { vcvtqq2pd(v512) };
//! let _ = unsafe { vcvtqq2ps(v512) };
//! let _ = unsafe { vcvttpd2qq(v512) };
//! let _ = unsafe { vcvttpd2uqq(v512) };
//! let _ = unsafe { vcvttps2qq(v256) };
//! let _ = unsafe { vcvttps2uqq(v256) };
//! let _ = unsafe { vcvtuqq2pd(v512) };
//! let _ = unsafe { vcvtuqq2ps(v512) };
//! let _ = unsafe { vfixupimmpd::<0>(v512, v512, v512) };
//! let _ = unsafe { vfixupimmps::<0>(v512, v512, v512) };
//! let _ = unsafe { vfixupimmsd::<0>(v128, v128, v128) };
//! let _ = unsafe { vfixupimmss::<0>(v128, v128, v128) };
//! let _ = unsafe { vfpclasspd::<0>(v512) };
//! let _ = unsafe { vfpclassps::<0>(v512) };
//! let _ = unsafe { vfpclasssd::<0>(v128) };
//! let _ = unsafe { vfpclassss::<0>(v128) };
//! let _ = unsafe { vgetexppd(v512) };
//! let _ = unsafe { vgetexpps(v512) };
//! let _ = unsafe { vgetexpsd(v128, v128) };
//! let _ = unsafe { vgetexpss(v128, v128) };
//! let _ = unsafe { vgetmantpd::<0>(v512) };
//! let _ = unsafe { vgetmantps::<0>(v512) };
//! let _ = unsafe { vgetmantsd::<0>(v128, v128) };
//! let _ = unsafe { vgetmantss::<0>(v128, v128) };
//! let _ = unsafe { vorpd(v512, v512) };
//! let _ = unsafe { vorps(v512, v512) };
//! let _ = unsafe { vpmovd2m(v512) };
//! let _ = unsafe { vpmovm2d(0) };
//! let _ = unsafe { vpmovm2q(0) };
//! let _ = unsafe { vpmovq2m(v512) };
//! let _ = unsafe { vpmullq(v512, v512) };
//! let _ = unsafe { vrangepd::<0>(v512, v512) };
//! let _ = unsafe { vrangeps::<0>(v512, v512) };
//! let _ = unsafe { vrangesd::<0>(v128, v128) };
//! let _ = unsafe { vrangess::<0>(v128, v128) };
//! let _ = unsafe { vrcp14pd(v512) };
//! let _ = unsafe { vrcp14ps(v512) };
//! let _ = unsafe { vrcp14sd(v128, v128) };
//! let _ = unsafe { vrcp14ss(v128, v128) };
//! let _ = unsafe { vreducepd::<0>(v512) };
//! let _ = unsafe { vreduceps::<0>(v512) };
//! let _ = unsafe { vreducesd::<0>(v128, v128) };
//! let _ = unsafe { vreducess::<0>(v128, v128) };
//! let _ = unsafe { vrsqrt14pd(v512) };
//! let _ = unsafe { vrsqrt14ps(v512) };
//! let _ = unsafe { vrsqrt14sd(v128, v128) };
//! let _ = unsafe { vrsqrt14ss(v128, v128) };
//! let _ = unsafe { vscalefpd(v512, v512) };
//! let _ = unsafe { vscalefps(v512, v512) };
//! let _ = unsafe { vscalefsd(v128, v128) };
//! let _ = unsafe { vscalefss(v128, v128) };
//! let _ = unsafe { vxorpd(v512, v512) };
//! let _ = unsafe { vxorps(v512, v512) };
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512dq::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vandnpd(v512, v512) };
//! let _ = unsafe { vandnps(v512, v512) };
//! let _ = unsafe { vandpd(v512, v512) };
//! let _ = unsafe { vandps(v512, v512) };
//! let _ = unsafe { vcmppd::<0>(v512, v512) };
//! let _ = unsafe { vcmpps::<0>(v512, v512) };
//! let _ = unsafe { vcmpsd::<0>(v128, v128) };
//! let _ = unsafe { vcmpss::<0>(v128, v128) };
//! let _ = unsafe { vcvtpd2qq(v512) };
//! let _ = unsafe { vcvtpd2uqq(v512) };
//! let _ = unsafe { vcvtps2qq(v256) };
//! let _ = unsafe { vcvtps2uqq(v256) };
//! let _ = unsafe { vcvtqq2pd(v512) };
//! let _ = unsafe { vcvtqq2ps(v512) };
//! let _ = unsafe { vcvttpd2qq(v512) };
//! let _ = unsafe { vcvttpd2uqq(v512) };
//! let _ = unsafe { vcvttps2qq(v256) };
//! let _ = unsafe { vcvttps2uqq(v256) };
//! let _ = unsafe { vcvtuqq2pd(v512) };
//! let _ = unsafe { vcvtuqq2ps(v512) };
//! let _ = unsafe { vfixupimmpd::<0>(v512, v512, v512) };
//! let _ = unsafe { vfixupimmps::<0>(v512, v512, v512) };
//! let _ = unsafe { vfixupimmsd::<0>(v128, v128, v128) };
//! let _ = unsafe { vfixupimmss::<0>(v128, v128, v128) };
//! let _ = unsafe { vfpclasspd::<0>(v512) };
//! let _ = unsafe { vfpclassps::<0>(v512) };
//! let _ = unsafe { vfpclasssd::<0>(v128) };
//! let _ = unsafe { vfpclassss::<0>(v128) };
//! let _ = unsafe { vgetexppd(v512) };
//! let _ = unsafe { vgetexpps(v512) };
//! let _ = unsafe { vgetexpsd(v128, v128) };
//! let _ = unsafe { vgetexpss(v128, v128) };
//! let _ = unsafe { vgetmantpd::<0>(v512) };
//! let _ = unsafe { vgetmantps::<0>(v512) };
//! let _ = unsafe { vgetmantsd::<0>(v128, v128) };
//! let _ = unsafe { vgetmantss::<0>(v128, v128) };
//! let _ = unsafe { vorpd(v512, v512) };
//! let _ = unsafe { vorps(v512, v512) };
//! let _ = unsafe { vpmovd2m(v512) };
//! let _ = unsafe { vpmovm2d(0) };
//! let _ = unsafe { vpmovm2q(0) };
//! let _ = unsafe { vpmovq2m(v512) };
//! let _ = unsafe { vpmullq(v512, v512) };
//! let _ = unsafe { vrangepd::<0>(v512, v512) };
//! let _ = unsafe { vrangeps::<0>(v512, v512) };
//! let _ = unsafe { vrangesd::<0>(v128, v128) };
//! let _ = unsafe { vrangess::<0>(v128, v128) };
//! let _ = unsafe { vrcp14pd(v512) };
//! let _ = unsafe { vrcp14ps(v512) };
//! let _ = unsafe { vrcp14sd(v128, v128) };
//! let _ = unsafe { vrcp14ss(v128, v128) };
//! let _ = unsafe { vreducepd::<0>(v512) };
//! let _ = unsafe { vreduceps::<0>(v512) };
//! let _ = unsafe { vreducesd::<0>(v128, v128) };
//! let _ = unsafe { vreducess::<0>(v128, v128) };
//! let _ = unsafe { vrsqrt14pd(v512) };
//! let _ = unsafe { vrsqrt14ps(v512) };
//! let _ = unsafe { vrsqrt14sd(v128, v128) };
//! let _ = unsafe { vrsqrt14ss(v128, v128) };
//! let _ = unsafe { vscalefpd(v512, v512) };
//! let _ = unsafe { vscalefps(v512, v512) };
//! let _ = unsafe { vscalefsd(v128, v128) };
//! let _ = unsafe { vscalefss(v128, v128) };
//! let _ = unsafe { vxorpd(v512, v512) };
//! let _ = unsafe { vxorps(v512, v512) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avx512dq::vandnpd as _;
//! use rust_isa::x86::simd::avx512dq::vandnps as _;
//! use rust_isa::x86::simd::avx512dq::vandpd as _;
//! use rust_isa::x86::simd::avx512dq::vandps as _;
//! use rust_isa::x86::simd::avx512dq::vcmppd as _;
//! use rust_isa::x86::simd::avx512dq::vcmpps as _;
//! use rust_isa::x86::simd::avx512dq::vcmpsd as _;
//! use rust_isa::x86::simd::avx512dq::vcmpss as _;
//! use rust_isa::x86::simd::avx512dq::vcvtpd2qq as _;
//! use rust_isa::x86::simd::avx512dq::vcvtpd2uqq as _;
//! use rust_isa::x86::simd::avx512dq::vcvtps2qq as _;
//! use rust_isa::x86::simd::avx512dq::vcvtps2uqq as _;
//! use rust_isa::x86::simd::avx512dq::vcvtqq2pd as _;
//! use rust_isa::x86::simd::avx512dq::vcvtqq2ps as _;
//! use rust_isa::x86::simd::avx512dq::vcvttpd2qq as _;
//! use rust_isa::x86::simd::avx512dq::vcvttpd2uqq as _;
//! use rust_isa::x86::simd::avx512dq::vcvttps2qq as _;
//! use rust_isa::x86::simd::avx512dq::vcvttps2uqq as _;
//! use rust_isa::x86::simd::avx512dq::vcvtuqq2pd as _;
//! use rust_isa::x86::simd::avx512dq::vcvtuqq2ps as _;
//! use rust_isa::x86::simd::avx512dq::vfixupimmpd as _;
//! use rust_isa::x86::simd::avx512dq::vfixupimmps as _;
//! use rust_isa::x86::simd::avx512dq::vfixupimmsd as _;
//! use rust_isa::x86::simd::avx512dq::vfixupimmss as _;
//! use rust_isa::x86::simd::avx512dq::vfpclasspd as _;
//! use rust_isa::x86::simd::avx512dq::vfpclassps as _;
//! use rust_isa::x86::simd::avx512dq::vfpclasssd as _;
//! use rust_isa::x86::simd::avx512dq::vfpclassss as _;
//! use rust_isa::x86::simd::avx512dq::vgetexppd as _;
//! use rust_isa::x86::simd::avx512dq::vgetexpps as _;
//! use rust_isa::x86::simd::avx512dq::vgetexpsd as _;
//! use rust_isa::x86::simd::avx512dq::vgetexpss as _;
//! use rust_isa::x86::simd::avx512dq::vgetmantpd as _;
//! use rust_isa::x86::simd::avx512dq::vgetmantps as _;
//! use rust_isa::x86::simd::avx512dq::vgetmantsd as _;
//! use rust_isa::x86::simd::avx512dq::vgetmantss as _;
//! use rust_isa::x86::simd::avx512dq::vorpd as _;
//! use rust_isa::x86::simd::avx512dq::vorps as _;
//! use rust_isa::x86::simd::avx512dq::vpmovd2m as _;
//! use rust_isa::x86::simd::avx512dq::vpmovm2d as _;
//! use rust_isa::x86::simd::avx512dq::vpmovm2q as _;
//! use rust_isa::x86::simd::avx512dq::vpmovq2m as _;
//! use rust_isa::x86::simd::avx512dq::vpmullq as _;
//! use rust_isa::x86::simd::avx512dq::vrangepd as _;
//! use rust_isa::x86::simd::avx512dq::vrangeps as _;
//! use rust_isa::x86::simd::avx512dq::vrangesd as _;
//! use rust_isa::x86::simd::avx512dq::vrangess as _;
//! use rust_isa::x86::simd::avx512dq::vrcp14pd as _;
//! use rust_isa::x86::simd::avx512dq::vrcp14ps as _;
//! use rust_isa::x86::simd::avx512dq::vrcp14sd as _;
//! use rust_isa::x86::simd::avx512dq::vrcp14ss as _;
//! use rust_isa::x86::simd::avx512dq::vreducepd as _;
//! use rust_isa::x86::simd::avx512dq::vreduceps as _;
//! use rust_isa::x86::simd::avx512dq::vreducesd as _;
//! use rust_isa::x86::simd::avx512dq::vreducess as _;
//! use rust_isa::x86::simd::avx512dq::vrsqrt14pd as _;
//! use rust_isa::x86::simd::avx512dq::vrsqrt14ps as _;
//! use rust_isa::x86::simd::avx512dq::vrsqrt14sd as _;
//! use rust_isa::x86::simd::avx512dq::vrsqrt14ss as _;
//! use rust_isa::x86::simd::avx512dq::vscalefpd as _;
//! use rust_isa::x86::simd::avx512dq::vscalefps as _;
//! use rust_isa::x86::simd::avx512dq::vscalefsd as _;
//! use rust_isa::x86::simd::avx512dq::vscalefss as _;
//! use rust_isa::x86::simd::avx512dq::vxorpd as _;
//! use rust_isa::x86::simd::avx512dq::vxorps as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::x86::simd::avx2::V256;
use crate::x86::simd::avx512f::V512;
use crate::x86::simd::sse2::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx512dq_vpmullq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullq`
/// Multiply the eight 64-bit integer lanes, keeping the low 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vpmullq(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vpmullq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vpmovm2d(mask: u16, out: *mut u8);
}

/// `vpmovm2d`
/// Expand the 16 mask bits to 32-bit all-ones or all-zeros lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vpmovm2d(mask: u16) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vpmovm2d(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vpmovm2q(mask: u8, out: *mut u8);
}

/// `vpmovm2q`
/// Expand the eight mask bits to 64-bit all-ones or all-zeros lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vpmovm2q(mask: u8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vpmovm2q(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vpmovd2m(a: *const u8) -> u16;
}

/// `vpmovd2m`
/// Collect the sign bits of the 16 32-bit lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vpmovd2m(a: V512) -> u16 {
    unsafe { rust_isa_avx512dq_vpmovd2m(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vpmovq2m(a: *const u8) -> u8;
}

/// `vpmovq2m`
/// Collect the sign bits of the eight 64-bit lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vpmovq2m(a: V512) -> u8 {
    unsafe { rust_isa_avx512dq_vpmovq2m(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vandpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vandpd`
/// Bitwise AND of the 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vandpd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vandpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vandnpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vandnpd`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vandnpd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vandnpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vorpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vorpd`
/// Bitwise OR of the 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vorpd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vorpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vxorpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vxorpd`
/// Bitwise XOR of the 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vxorpd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vxorpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vandps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vandps`
/// Bitwise AND of the 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vandps(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vandps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vandnps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vandnps`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vandnps(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vandnps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vorps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vorps`
/// Bitwise OR of the 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vorps(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vorps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vxorps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vxorps`
/// Bitwise XOR of the 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vxorps(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vxorps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcmpps`
/// Compare the 16 32-bit float lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcmpps<const IMM: u8>(a: V512, b: V512) -> u16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vcmpps k1, zmm0, zmm1, {imm}",
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
    out as u16
}

/// `vcmppd`
/// Compare the eight 64-bit float lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcmppd<const IMM: u8>(a: V512, b: V512) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vcmppd k1, zmm0, zmm1, {imm}",
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
    out as u8
}

/// `vcmpss`
/// Compare the low 32-bit float lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcmpss<const IMM: u8>(a: V128, b: V128) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vcmpss k1, xmm0, xmm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("xmm0") _,
            out("xmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u8
}

/// `vcmpsd`
/// Compare the low 64-bit float lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcmpsd<const IMM: u8>(a: V128, b: V128) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vcmpsd k1, xmm0, xmm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("xmm0") _,
            out("xmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u8
}

/// `vfpclassps`
/// Classify the 16 32-bit float lanes with the categories in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vfpclassps<const IMM: u8>(a: V512) -> u16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vfpclassps k1, zmm0, {imm}",
            "kmovw {out:e}, k1",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u16
}

/// `vfpclasspd`
/// Classify the eight 64-bit float lanes with the categories in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vfpclasspd<const IMM: u8>(a: V512) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vfpclasspd k1, zmm0, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u8
}

/// `vfpclassss`
/// Classify the low 32-bit float lane with the categories in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vfpclassss<const IMM: u8>(a: V128) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vfpclassss k1, xmm0, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("xmm0") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u8
}

/// `vfpclasssd`
/// Classify the low 64-bit float lane with the categories in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vfpclasssd<const IMM: u8>(a: V128) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vfpclasssd k1, xmm0, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("xmm0") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u8
}

/// `vrangeps`
/// Apply the range operation selected by `IMM` to the 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrangeps<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vrangeps zmm0, zmm0, zmm1, {imm}",
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

/// `vrangepd`
/// Apply the range operation selected by `IMM` to the 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrangepd<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vrangepd zmm0, zmm0, zmm1, {imm}",
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

/// `vrangess`
/// Apply the range operation selected by `IMM` to the low 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrangess<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vrangess xmm0, xmm0, xmm1, {imm}",
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

/// `vrangesd`
/// Apply the range operation selected by `IMM` to the low 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrangesd<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vrangesd xmm0, xmm0, xmm1, {imm}",
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

/// `vreduceps`
/// Reduce the 32-bit float lanes to the precision in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vreduceps<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vreduceps zmm0, zmm0, {imm}",
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

/// `vreducepd`
/// Reduce the 64-bit float lanes to the precision in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vreducepd<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vreducepd zmm0, zmm0, {imm}",
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

/// `vreducess`
/// Reduce the low 32-bit float lanes to the precision in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vreducess<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vreducess xmm0, xmm0, xmm1, {imm}",
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

/// `vreducesd`
/// Reduce the low 64-bit float lanes to the precision in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vreducesd<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vreducesd xmm0, xmm0, xmm1, {imm}",
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

unsafe extern "C" {
    fn rust_isa_avx512dq_vgetexpps(a: *const u8, out: *mut u8);
}

/// `vgetexpps`
/// Extract the exponent of the 16 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vgetexpps(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vgetexpps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vgetexppd(a: *const u8, out: *mut u8);
}

/// `vgetexppd`
/// Extract the exponent of the eight 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vgetexppd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vgetexppd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vgetexpss(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vgetexpss`
/// Extract the exponent of the low 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vgetexpss(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512dq_vgetexpss(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vgetexpsd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vgetexpsd`
/// Extract the exponent of the low 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vgetexpsd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512dq_vgetexpsd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vgetmantps`
/// Extract the mantissa of the 16 32-bit float lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
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
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
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

/// `vgetmantss`
/// Extract the mantissa of the low 32-bit float lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vgetmantss<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vgetmantss xmm0, xmm0, xmm1, {imm}",
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

/// `vgetmantsd`
/// Extract the mantissa of the low 64-bit float lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vgetmantsd<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vgetmantsd xmm0, xmm0, xmm1, {imm}",
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

unsafe extern "C" {
    fn rust_isa_avx512dq_vscalefps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefps`
/// Scale the 32-bit float lanes by powers of two.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vscalefps(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vscalefps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vscalefpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefpd`
/// Scale the 64-bit float lanes by powers of two.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vscalefpd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vscalefpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vscalefss(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefss`
/// Scale the low 32-bit float lane by a power of two.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vscalefss(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512dq_vscalefss(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vscalefsd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefsd`
/// Scale the low 64-bit float lane by a power of two.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vscalefsd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512dq_vscalefsd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vrcp14ps(a: *const u8, out: *mut u8);
}

/// `vrcp14ps`
/// Compute a 14-bit approximate reciprocal of the 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrcp14ps(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vrcp14ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vrcp14pd(a: *const u8, out: *mut u8);
}

/// `vrcp14pd`
/// Compute a 14-bit approximate reciprocal of the 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrcp14pd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vrcp14pd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vrcp14ss(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vrcp14ss`
/// Compute a 14-bit approximate reciprocal of the low 32-bit float lane.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrcp14ss(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512dq_vrcp14ss(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vrcp14sd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vrcp14sd`
/// Compute a 14-bit approximate reciprocal of the low 64-bit float lane.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrcp14sd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512dq_vrcp14sd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vrsqrt14ps(a: *const u8, out: *mut u8);
}

/// `vrsqrt14ps`
/// Compute a 14-bit approximate reciprocal square root of the 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrsqrt14ps(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vrsqrt14ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vrsqrt14pd(a: *const u8, out: *mut u8);
}

/// `vrsqrt14pd`
/// Compute a 14-bit approximate reciprocal square root of the 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrsqrt14pd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vrsqrt14pd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vrsqrt14ss(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vrsqrt14ss`
/// Compute a 14-bit approximate reciprocal square root of the low 32-bit float lane.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrsqrt14ss(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512dq_vrsqrt14ss(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vrsqrt14sd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vrsqrt14sd`
/// Compute a 14-bit approximate reciprocal square root of the low 64-bit float lane.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vrsqrt14sd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512dq_vrsqrt14sd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfixupimmps`
/// Fix up the 32-bit float lanes selected by the table in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vfixupimmps<const IMM: u8>(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vmovdqu64 zmm2, [{c}]",
            "vfixupimmps zmm0, zmm1, zmm2, {imm}",
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

/// `vfixupimmpd`
/// Fix up the 64-bit float lanes selected by the table in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vfixupimmpd<const IMM: u8>(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vmovdqu64 zmm2, [{c}]",
            "vfixupimmpd zmm0, zmm1, zmm2, {imm}",
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

/// `vfixupimmss`
/// Fix up the low 32-bit float lanes selected by the table in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vfixupimmss<const IMM: u8>(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vmovdqu xmm2, [{c}]",
            "vfixupimmss xmm0, xmm1, xmm2, {imm}",
            "vmovdqu [{out}], xmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            c = in(reg) c.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("xmm0") _,
            out("xmm1") _,
            out("xmm2") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vfixupimmsd`
/// Fix up the low 64-bit float lanes selected by the table in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vfixupimmsd<const IMM: u8>(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vmovdqu xmm2, [{c}]",
            "vfixupimmsd xmm0, xmm1, xmm2, {imm}",
            "vmovdqu [{out}], xmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            c = in(reg) c.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("xmm0") _,
            out("xmm1") _,
            out("xmm2") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvtpd2qq(a: *const u8, out: *mut u8);
}

/// `vcvtpd2qq`
/// Convert the eight 64-bit float lanes to signed 64-bit integers.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvtpd2qq(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vcvtpd2qq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvtpd2uqq(a: *const u8, out: *mut u8);
}

/// `vcvtpd2uqq`
/// Convert the eight 64-bit float lanes to unsigned 64-bit integers.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvtpd2uqq(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vcvtpd2uqq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvtps2qq(a: *const u8, out: *mut u8);
}

/// `vcvtps2qq`
/// Convert eight 32-bit float lanes to signed 64-bit integers.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvtps2qq(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vcvtps2qq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvtps2uqq(a: *const u8, out: *mut u8);
}

/// `vcvtps2uqq`
/// Convert eight 32-bit float lanes to unsigned 64-bit integers.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvtps2uqq(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vcvtps2uqq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvtqq2pd(a: *const u8, out: *mut u8);
}

/// `vcvtqq2pd`
/// Convert the eight signed 64-bit integer lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvtqq2pd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vcvtqq2pd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvtqq2ps(a: *const u8, out: *mut u8);
}

/// `vcvtqq2ps`
/// Convert the eight signed 64-bit integer lanes to 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvtqq2ps(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512dq_vcvtqq2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvtuqq2pd(a: *const u8, out: *mut u8);
}

/// `vcvtuqq2pd`
/// Convert the eight unsigned 64-bit integer lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvtuqq2pd(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vcvtuqq2pd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvtuqq2ps(a: *const u8, out: *mut u8);
}

/// `vcvtuqq2ps`
/// Convert the eight unsigned 64-bit integer lanes to 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvtuqq2ps(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512dq_vcvtuqq2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvttpd2qq(a: *const u8, out: *mut u8);
}

/// `vcvttpd2qq`
/// Convert the eight 64-bit float lanes to signed 64-bit integers with truncation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvttpd2qq(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vcvttpd2qq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvttpd2uqq(a: *const u8, out: *mut u8);
}

/// `vcvttpd2uqq`
/// Convert the eight 64-bit float lanes to unsigned 64-bit integers with truncation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvttpd2uqq(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vcvttpd2uqq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvttps2qq(a: *const u8, out: *mut u8);
}

/// `vcvttps2qq`
/// Convert eight 32-bit float lanes to signed 64-bit integers with truncation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvttps2qq(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vcvttps2qq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512dq_vcvttps2uqq(a: *const u8, out: *mut u8);
}

/// `vcvttps2uqq`
/// Convert eight 32-bit float lanes to unsigned 64-bit integers with truncation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[inline]
pub unsafe fn vcvttps2uqq(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512dq_vcvttps2uqq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
