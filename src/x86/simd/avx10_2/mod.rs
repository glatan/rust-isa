//! AVX10.2.
//!
//! AVX10.2 is not part of the x86-64 baseline and rustc does not allow
//! `#[target_feature(enable = "avx10.2")]` on stable. The caller must compile
//! the whole crate with `-C target-feature=+avx10.2` and the CPU must support
//! AVX10.2. The wrappers do not carry `#[target_feature]`. Every wrapper is
//! `unsafe`. Immediate operands are const generics unless documented otherwise.
//!
//! The values are [`V128`], [`V256`], and [`V512`]. The module covers the
//! AVX10.2 `vminmax`, `vcomi`, `vcmp`, `vfpclass`, `vgetexp`, `vgetmant`,
//! `vscalef`, `vrndscale`, `vreduce`, `vfixupimm`, `vrange`, `vrcp14`,
//! `vrsqrt14`, and `vsqrt` forms that were previously listed as missing.
//! Embedded rounding control is omitted.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx10_2::vminmaxps_v256;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vminmaxps_v256::<0>(v, v) };
//! ```
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::x86::simd::avx10_2::*;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! assert_eq!(unsafe { vaddps_v256(v, v) }.to_bytes(), [0; 32]);
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx10_2::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vaddps_v256(v256, v256) };
//! let _ = unsafe { vcmppd_v256::<0>(v256, v256) };
//! let _ = unsafe { vcmppd_v512::<0>(v512, v512) };
//! let _ = unsafe { vcmpph_v256::<0>(v256, v256) };
//! let _ = unsafe { vcmpph_v512::<0>(v512, v512) };
//! let _ = unsafe { vcmpps_v256::<0>(v256, v256) };
//! let _ = unsafe { vcmpps_v512::<0>(v512, v512) };
//! let _ = unsafe { vcomisbf16(v128, v128) };
//! let _ = unsafe { vcomisd_v128(v128, v128) };
//! let _ = unsafe { vcomiss_v128(v128, v128) };
//! let _ = unsafe { vfixupimmpd_v256::<0>(v256, v256, v256) };
//! let _ = unsafe { vfixupimmpd_v512::<0>(v512, v512, v512) };
//! let _ = unsafe { vfixupimmps_v256::<0>(v256, v256, v256) };
//! let _ = unsafe { vfixupimmps_v512::<0>(v512, v512, v512) };
//! let _ = unsafe { vfixupimmsd_v128::<0>(v128, v128, v128) };
//! let _ = unsafe { vfixupimmss_v128::<0>(v128, v128, v128) };
//! let _ = unsafe { vfpclasspd_v256::<0>(v256) };
//! let _ = unsafe { vfpclasspd_v512::<0>(v512) };
//! let _ = unsafe { vfpclassph_v256::<0>(v256) };
//! let _ = unsafe { vfpclassph_v512::<0>(v512) };
//! let _ = unsafe { vfpclassps_v256::<0>(v256) };
//! let _ = unsafe { vfpclassps_v512::<0>(v512) };
//! let _ = unsafe { vgetexppd_v256(v256) };
//! let _ = unsafe { vgetexppd_v512(v512) };
//! let _ = unsafe { vgetexpph_v256(v256) };
//! let _ = unsafe { vgetexpph_v512(v512) };
//! let _ = unsafe { vgetexpps_v256(v256) };
//! let _ = unsafe { vgetexpps_v512(v512) };
//! let _ = unsafe { vgetmantpd_v256::<0>(v256) };
//! let _ = unsafe { vgetmantpd_v512::<0>(v512) };
//! let _ = unsafe { vgetmantph_v256::<0>(v256) };
//! let _ = unsafe { vgetmantph_v512::<0>(v512) };
//! let _ = unsafe { vgetmantps_v256::<0>(v256) };
//! let _ = unsafe { vgetmantps_v512::<0>(v512) };
//! let _ = unsafe { vgetmantsd_v128::<0>(v128, v128) };
//! let _ = unsafe { vgetmantss_v128::<0>(v128, v128) };
//! let _ = unsafe { vminmaxpd_v256::<0>(v256, v256) };
//! let _ = unsafe { vminmaxpd_v512::<0>(v512, v512) };
//! let _ = unsafe { vminmaxph_v256::<0>(v256, v256) };
//! let _ = unsafe { vminmaxph_v512::<0>(v512, v512) };
//! let _ = unsafe { vminmaxps_v256::<0>(v256, v256) };
//! let _ = unsafe { vminmaxps_v512::<0>(v512, v512) };
//! let _ = unsafe { vminmaxsd_v128::<0>(v128, v128) };
//! let _ = unsafe { vminmaxsh_v128::<0>(v128, v128) };
//! let _ = unsafe { vminmaxss_v128::<0>(v128, v128) };
//! let _ = unsafe { vmulps_v512(v512, v512) };
//! let _ = unsafe { vrangepd_v256::<0>(v256, v256) };
//! let _ = unsafe { vrangepd_v512::<0>(v512, v512) };
//! let _ = unsafe { vrangeps_v256::<0>(v256, v256) };
//! let _ = unsafe { vrangeps_v512::<0>(v512, v512) };
//! let _ = unsafe { vrangesd_v128::<0>(v128, v128) };
//! let _ = unsafe { vrangess_v128::<0>(v128, v128) };
//! let _ = unsafe { vrcp14pd_v256(v256) };
//! let _ = unsafe { vrcp14pd_v512(v512) };
//! let _ = unsafe { vrcp14ps_v256(v256) };
//! let _ = unsafe { vrcp14ps_v512(v512) };
//! let _ = unsafe { vrcp14sd_v128(v128, v128) };
//! let _ = unsafe { vrcp14ss_v128(v128, v128) };
//! let _ = unsafe { vreducepd_v256::<0>(v256) };
//! let _ = unsafe { vreducepd_v512::<0>(v512) };
//! let _ = unsafe { vreduceph_v256::<0>(v256) };
//! let _ = unsafe { vreduceph_v512::<0>(v512) };
//! let _ = unsafe { vreduceps_v256::<0>(v256) };
//! let _ = unsafe { vreduceps_v512::<0>(v512) };
//! let _ = unsafe { vreducesd_v128::<0>(v128, v128) };
//! let _ = unsafe { vreducess_v128::<0>(v128, v128) };
//! let _ = unsafe { vrndscalepd_v256::<0>(v256) };
//! let _ = unsafe { vrndscalepd_v512::<0>(v512) };
//! let _ = unsafe { vrndscaleph_v256::<0>(v256) };
//! let _ = unsafe { vrndscaleph_v512::<0>(v512) };
//! let _ = unsafe { vrndscaleps_v256::<0>(v256) };
//! let _ = unsafe { vrndscaleps_v512::<0>(v512) };
//! let _ = unsafe { vrsqrt14pd_v256(v256) };
//! let _ = unsafe { vrsqrt14pd_v512(v512) };
//! let _ = unsafe { vrsqrt14ps_v256(v256) };
//! let _ = unsafe { vrsqrt14ps_v512(v512) };
//! let _ = unsafe { vrsqrt14sd_v128(v128, v128) };
//! let _ = unsafe { vrsqrt14ss_v128(v128, v128) };
//! let _ = unsafe { vscalefpd_v256(v256, v256) };
//! let _ = unsafe { vscalefpd_v512(v512, v512) };
//! let _ = unsafe { vscalefph_v256(v256, v256) };
//! let _ = unsafe { vscalefph_v512(v512, v512) };
//! let _ = unsafe { vscalefps_v256(v256, v256) };
//! let _ = unsafe { vscalefps_v512(v512, v512) };
//! let _ = unsafe { vscalefsd_v128(v128, v128) };
//! let _ = unsafe { vscalefss_v128(v128, v128) };
//! let _ = unsafe { vsqrtpd_v256(v256) };
//! let _ = unsafe { vsqrtpd_v512(v512) };
//! let _ = unsafe { vsqrtps_v256(v256) };
//! let _ = unsafe { vsqrtps_v512(v512) };
//! let _ = unsafe { vsqrtsd_v128(v128, v128) };
//! let _ = unsafe { vsqrtss_v128(v128) };
//! let _ = unsafe { vucomisd_v128(v128, v128) };
//! let _ = unsafe { vucomiss_v128(v128, v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avx10_2::vaddps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vcmppd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vcmppd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vcmpph_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vcmpph_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vcmpps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vcmpps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vcomisbf16 as _;
//! use rust_isa::x86::simd::avx10_2::vcomisd_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vcomiss_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vfixupimmpd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vfixupimmpd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vfixupimmps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vfixupimmps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vfixupimmsd_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vfixupimmss_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vfpclasspd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vfpclasspd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vfpclassph_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vfpclassph_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vfpclassps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vfpclassps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vgetexppd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vgetexppd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vgetexpph_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vgetexpph_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vgetexpps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vgetexpps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vgetmantpd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vgetmantpd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vgetmantph_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vgetmantph_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vgetmantps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vgetmantps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vgetmantsd_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vgetmantss_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vminmaxpd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vminmaxpd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vminmaxph_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vminmaxph_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vminmaxps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vminmaxps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vminmaxsd_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vminmaxsh_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vminmaxss_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vmulps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vrangepd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vrangepd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vrangeps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vrangeps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vrangesd_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vrangess_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vrcp14pd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vrcp14pd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vrcp14ps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vrcp14ps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vrcp14sd_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vrcp14ss_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vreducepd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vreducepd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vreduceph_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vreduceph_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vreduceps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vreduceps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vreducesd_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vreducess_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vrndscalepd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vrndscalepd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vrndscaleph_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vrndscaleph_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vrndscaleps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vrndscaleps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vrsqrt14pd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vrsqrt14pd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vrsqrt14ps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vrsqrt14ps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vrsqrt14sd_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vrsqrt14ss_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vscalefpd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vscalefpd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vscalefph_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vscalefph_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vscalefps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vscalefps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vscalefsd_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vscalefss_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vsqrtpd_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vsqrtpd_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vsqrtps_v256 as _;
//! use rust_isa::x86::simd::avx10_2::vsqrtps_v512 as _;
//! use rust_isa::x86::simd::avx10_2::vsqrtsd_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vsqrtss_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vucomisd_v128 as _;
//! use rust_isa::x86::simd::avx10_2::vucomiss_v128 as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::x86::simd::avx2::V256;
use crate::x86::simd::avx512f::V512;
use crate::x86::simd::sse2::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx10_2_vaddps_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaddps_v256`
/// Add the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vaddps_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vaddps_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vmulps_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vmulps_v512`
/// Multiply the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vmulps_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vmulps_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vminmaxps_v256`
/// Select the minimum or maximum according to `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vminmaxps_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vminmaxps ymm0, ymm0, ymm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vminmaxps_v512`
/// Select the minimum or maximum according to `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vminmaxps_v512<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vminmaxps zmm0, zmm0, zmm1, {imm}",
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

/// `vminmaxpd_v256`
/// Select the minimum or maximum according to `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vminmaxpd_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vminmaxpd ymm0, ymm0, ymm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vminmaxpd_v512`
/// Select the minimum or maximum according to `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vminmaxpd_v512<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vminmaxpd zmm0, zmm0, zmm1, {imm}",
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

/// `vminmaxph_v256`
/// Select the minimum or maximum according to `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vminmaxph_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vminmaxph ymm0, ymm0, ymm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vminmaxph_v512`
/// Select the minimum or maximum according to `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vminmaxph_v512<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vminmaxph zmm0, zmm0, zmm1, {imm}",
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

/// `vminmaxss_v128`
/// Select the minimum or maximum of the low lanes according to `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vminmaxss_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vminmaxss xmm0, xmm0, xmm1, {imm}",
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

/// `vminmaxsd_v128`
/// Select the minimum or maximum of the low lanes according to `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vminmaxsd_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vminmaxsd xmm0, xmm0, xmm1, {imm}",
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

/// `vminmaxsh_v128`
/// Select the minimum or maximum of the low lanes according to `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vminmaxsh_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vminmaxsh xmm0, xmm0, xmm1, {imm}",
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

/// `vcmpps_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcmpps_v256<const IMM: u8>(a: V256, b: V256) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vcmpps k1, ymm0, ymm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u8
}

/// `vcmpps_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcmpps_v512<const IMM: u8>(a: V512, b: V512) -> u16 {
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

/// `vcmppd_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcmppd_v256<const IMM: u8>(a: V256, b: V256) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vcmppd k1, ymm0, ymm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u8
}

/// `vcmppd_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcmppd_v512<const IMM: u8>(a: V512, b: V512) -> u8 {
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

/// `vcmpph_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcmpph_v256<const IMM: u8>(a: V256, b: V256) -> u16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vcmpph k1, ymm0, ymm1, {imm}",
            "kmovw {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u16
}

/// `vcmpph_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcmpph_v512<const IMM: u8>(a: V512, b: V512) -> u32 {
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

/// `vfpclassps_v256`
/// Classify the lanes with the categories in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfpclassps_v256<const IMM: u8>(a: V256) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vfpclassps k1, ymm0, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u8
}

/// `vfpclassps_v512`
/// Classify the lanes with the categories in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfpclassps_v512<const IMM: u8>(a: V512) -> u16 {
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

/// `vfpclasspd_v256`
/// Classify the lanes with the categories in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfpclasspd_v256<const IMM: u8>(a: V256) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vfpclasspd k1, ymm0, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u8
}

/// `vfpclasspd_v512`
/// Classify the lanes with the categories in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfpclasspd_v512<const IMM: u8>(a: V512) -> u8 {
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

/// `vfpclassph_v256`
/// Classify the lanes with the categories in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfpclassph_v256<const IMM: u8>(a: V256) -> u16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vfpclassph k1, ymm0, {imm}",
            "kmovw {out:e}, k1",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    out as u16
}

/// `vfpclassph_v512`
/// Classify the lanes with the categories in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfpclassph_v512<const IMM: u8>(a: V512) -> u32 {
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
    fn rust_isa_avx10_2_vgetexpps_v256(a: *const u8, out: *mut u8);
}

/// `vgetexpps_v256`
/// Extract the exponent of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetexpps_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vgetexpps_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vgetexpps_v512(a: *const u8, out: *mut u8);
}

/// `vgetexpps_v512`
/// Extract the exponent of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetexpps_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vgetexpps_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vgetexppd_v256(a: *const u8, out: *mut u8);
}

/// `vgetexppd_v256`
/// Extract the exponent of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetexppd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vgetexppd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vgetexppd_v512(a: *const u8, out: *mut u8);
}

/// `vgetexppd_v512`
/// Extract the exponent of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetexppd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vgetexppd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vgetexpph_v256(a: *const u8, out: *mut u8);
}

/// `vgetexpph_v256`
/// Extract the exponent of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetexpph_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vgetexpph_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vgetexpph_v512(a: *const u8, out: *mut u8);
}

/// `vgetexpph_v512`
/// Extract the exponent of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetexpph_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vgetexpph_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vgetmantps_v256`
/// Extract the mantissa of the lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetmantps_v256<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vgetmantps ymm0, ymm0, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vgetmantps_v512`
/// Extract the mantissa of the lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetmantps_v512<const IMM: u8>(a: V512) -> V512 {
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

/// `vgetmantpd_v256`
/// Extract the mantissa of the lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetmantpd_v256<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vgetmantpd ymm0, ymm0, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vgetmantpd_v512`
/// Extract the mantissa of the lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetmantpd_v512<const IMM: u8>(a: V512) -> V512 {
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

/// `vgetmantph_v256`
/// Extract the mantissa of the lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetmantph_v256<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vgetmantph ymm0, ymm0, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vgetmantph_v512`
/// Extract the mantissa of the lanes. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetmantph_v512<const IMM: u8>(a: V512) -> V512 {
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
    fn rust_isa_avx10_2_vscalefps_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefps_v256`
/// Scale the lanes by powers of two.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vscalefps_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vscalefps_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vscalefps_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefps_v512`
/// Scale the lanes by powers of two.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vscalefps_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vscalefps_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vscalefpd_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefpd_v256`
/// Scale the lanes by powers of two.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vscalefpd_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vscalefpd_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vscalefpd_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefpd_v512`
/// Scale the lanes by powers of two.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vscalefpd_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vscalefpd_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vscalefph_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefph_v256`
/// Scale the lanes by powers of two.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vscalefph_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vscalefph_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vscalefph_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefph_v512`
/// Scale the lanes by powers of two.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vscalefph_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vscalefph_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrndscaleps_v256`
/// Round the lanes to the precision and mode in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrndscaleps_v256<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vrndscaleps ymm0, ymm0, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vrndscaleps_v512`
/// Round the lanes to the precision and mode in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrndscaleps_v512<const IMM: u8>(a: V512) -> V512 {
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

/// `vrndscalepd_v256`
/// Round the lanes to the precision and mode in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrndscalepd_v256<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vrndscalepd ymm0, ymm0, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vrndscalepd_v512`
/// Round the lanes to the precision and mode in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrndscalepd_v512<const IMM: u8>(a: V512) -> V512 {
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

/// `vrndscaleph_v256`
/// Round the lanes to the precision and mode in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrndscaleph_v256<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vrndscaleph ymm0, ymm0, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vrndscaleph_v512`
/// Round the lanes to the precision and mode in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrndscaleph_v512<const IMM: u8>(a: V512) -> V512 {
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

/// `vreduceps_v256`
/// Reduce the lanes to the precision in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vreduceps_v256<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vreduceps ymm0, ymm0, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vreduceps_v512`
/// Reduce the lanes to the precision in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vreduceps_v512<const IMM: u8>(a: V512) -> V512 {
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

/// `vreducepd_v256`
/// Reduce the lanes to the precision in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vreducepd_v256<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vreducepd ymm0, ymm0, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vreducepd_v512`
/// Reduce the lanes to the precision in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vreducepd_v512<const IMM: u8>(a: V512) -> V512 {
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

/// `vreduceph_v256`
/// Reduce the lanes to the precision in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vreduceph_v256<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vreduceph ymm0, ymm0, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vreduceph_v512`
/// Reduce the lanes to the precision in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vreduceph_v512<const IMM: u8>(a: V512) -> V512 {
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

/// `vfixupimmps_v256`
/// Fix up the lanes selected by the table in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfixupimmps_v256<const IMM: u8>(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vmovdqu ymm2, [{c}]",
            "vfixupimmps ymm0, ymm1, ymm2, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            c = in(reg) c.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            out("ymm2") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vfixupimmps_v512`
/// Fix up the lanes selected by the table in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfixupimmps_v512<const IMM: u8>(a: V512, b: V512, c: V512) -> V512 {
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

/// `vfixupimmpd_v256`
/// Fix up the lanes selected by the table in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfixupimmpd_v256<const IMM: u8>(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vmovdqu ymm2, [{c}]",
            "vfixupimmpd ymm0, ymm1, ymm2, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            c = in(reg) c.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            out("ymm2") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vfixupimmpd_v512`
/// Fix up the lanes selected by the table in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfixupimmpd_v512<const IMM: u8>(a: V512, b: V512, c: V512) -> V512 {
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

/// `vrangeps_v256`
/// Apply the range operation selected by `IMM` to the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrangeps_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vrangeps ymm0, ymm0, ymm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vrangeps_v512`
/// Apply the range operation selected by `IMM` to the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrangeps_v512<const IMM: u8>(a: V512, b: V512) -> V512 {
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

/// `vrangepd_v256`
/// Apply the range operation selected by `IMM` to the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrangepd_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vrangepd ymm0, ymm0, ymm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vrangepd_v512`
/// Apply the range operation selected by `IMM` to the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrangepd_v512<const IMM: u8>(a: V512, b: V512) -> V512 {
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

unsafe extern "C" {
    fn rust_isa_avx10_2_vrcp14ps_v256(a: *const u8, out: *mut u8);
}

/// `vrcp14ps_v256`
/// Compute a 14-bit approximate reciprocal of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrcp14ps_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vrcp14ps_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vrcp14ps_v512(a: *const u8, out: *mut u8);
}

/// `vrcp14ps_v512`
/// Compute a 14-bit approximate reciprocal of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrcp14ps_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vrcp14ps_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vrcp14pd_v256(a: *const u8, out: *mut u8);
}

/// `vrcp14pd_v256`
/// Compute a 14-bit approximate reciprocal of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrcp14pd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vrcp14pd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vrcp14pd_v512(a: *const u8, out: *mut u8);
}

/// `vrcp14pd_v512`
/// Compute a 14-bit approximate reciprocal of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrcp14pd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vrcp14pd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vrsqrt14ps_v256(a: *const u8, out: *mut u8);
}

/// `vrsqrt14ps_v256`
/// Compute a 14-bit approximate reciprocal square root of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrsqrt14ps_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vrsqrt14ps_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vrsqrt14ps_v512(a: *const u8, out: *mut u8);
}

/// `vrsqrt14ps_v512`
/// Compute a 14-bit approximate reciprocal square root of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrsqrt14ps_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vrsqrt14ps_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vrsqrt14pd_v256(a: *const u8, out: *mut u8);
}

/// `vrsqrt14pd_v256`
/// Compute a 14-bit approximate reciprocal square root of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrsqrt14pd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vrsqrt14pd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vrsqrt14pd_v512(a: *const u8, out: *mut u8);
}

/// `vrsqrt14pd_v512`
/// Compute a 14-bit approximate reciprocal square root of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrsqrt14pd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vrsqrt14pd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vsqrtps_v256(a: *const u8, out: *mut u8);
}

/// `vsqrtps_v256`
/// Compute the square root of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vsqrtps_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vsqrtps_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vsqrtps_v512(a: *const u8, out: *mut u8);
}

/// `vsqrtps_v512`
/// Compute the square root of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vsqrtps_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vsqrtps_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vsqrtpd_v256(a: *const u8, out: *mut u8);
}

/// `vsqrtpd_v256`
/// Compute the square root of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vsqrtpd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_2_vsqrtpd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vsqrtpd_v512(a: *const u8, out: *mut u8);
}

/// `vsqrtpd_v512`
/// Compute the square root of the lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vsqrtpd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_2_vsqrtpd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vcomisbf16(a: *const u8, b: *const u8) -> u32;
}

/// `vcomisbf16`
/// Compare the low bfloat16 lanes and return CF, PF, and ZF.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcomisbf16(a: V128, b: V128) -> u32 {
    unsafe { rust_isa_avx10_2_vcomisbf16(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vcomiss_v128(a: *const u8, b: *const u8) -> u32;
}

/// `vcomiss_v128`
/// Compare the low 32-bit float lanes and return CF, PF, and ZF.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcomiss_v128(a: V128, b: V128) -> u32 {
    unsafe { rust_isa_avx10_2_vcomiss_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vcomisd_v128(a: *const u8, b: *const u8) -> u32;
}

/// `vcomisd_v128`
/// Compare the low 64-bit float lanes and return CF, PF, and ZF.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcomisd_v128(a: V128, b: V128) -> u32 {
    unsafe { rust_isa_avx10_2_vcomisd_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vucomiss_v128(a: *const u8, b: *const u8) -> u32;
}

/// `vucomiss_v128`
/// Compare the low 32-bit float lanes without signaling on quiet NaN and return CF, PF, and ZF.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vucomiss_v128(a: V128, b: V128) -> u32 {
    unsafe { rust_isa_avx10_2_vucomiss_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vucomisd_v128(a: *const u8, b: *const u8) -> u32;
}

/// `vucomisd_v128`
/// Compare the low 64-bit float lanes without signaling on quiet NaN and return CF, PF, and ZF.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vucomisd_v128(a: V128, b: V128) -> u32 {
    unsafe { rust_isa_avx10_2_vucomisd_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vrcp14ss_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vrcp14ss_v128`
/// Compute a 14-bit approximate reciprocal of the low 32-bit float lane.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrcp14ss_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx10_2_vrcp14ss_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vrcp14sd_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vrcp14sd_v128`
/// Compute a 14-bit approximate reciprocal of the low 64-bit float lane.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrcp14sd_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx10_2_vrcp14sd_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vrsqrt14ss_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vrsqrt14ss_v128`
/// Compute a 14-bit approximate reciprocal square root of the low 32-bit float lane.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrsqrt14ss_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx10_2_vrsqrt14ss_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vrsqrt14sd_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vrsqrt14sd_v128`
/// Compute a 14-bit approximate reciprocal square root of the low 64-bit float lane.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrsqrt14sd_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx10_2_vrsqrt14sd_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vscalefss_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefss_v128`
/// Scale the low 32-bit float lane by a power of two.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vscalefss_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx10_2_vscalefss_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vscalefsd_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vscalefsd_v128`
/// Scale the low 64-bit float lane by a power of two.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vscalefsd_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx10_2_vscalefsd_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vsqrtss_v128(a: *const u8, out: *mut u8);
}

/// `vsqrtss_v128`
/// Compute the square root of the low 32-bit float lane.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vsqrtss_v128(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx10_2_vsqrtss_v128(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_2_vsqrtsd_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vsqrtsd_v128`
/// Compute the square root of the low 64-bit float lane.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vsqrtsd_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx10_2_vsqrtsd_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vgetmantss_v128`
/// Extract the mantissa of the low 32-bit float lane. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetmantss_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
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

/// `vgetmantsd_v128`
/// Extract the mantissa of the low 64-bit float lane. `IMM` selects the interval and sign control.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vgetmantsd_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
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

/// `vreducess_v128`
/// Reduce the low 32-bit float lane to the precision in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vreducess_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
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

/// `vreducesd_v128`
/// Reduce the low 64-bit float lane to the precision in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vreducesd_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
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

/// `vrangess_v128`
/// Apply the range operation selected by `IMM` to the low 32-bit float lane.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrangess_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
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

/// `vrangesd_v128`
/// Apply the range operation selected by `IMM` to the low 64-bit float lane.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrangesd_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
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

/// `vfixupimmss_v128`
/// Fix up the low 32-bit float lane selected by the table in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfixupimmss_v128<const IMM: u8>(a: V128, b: V128, c: V128) -> V128 {
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

/// `vfixupimmsd_v128`
/// Fix up the low 64-bit float lane selected by the table in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.2 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfixupimmsd_v128<const IMM: u8>(a: V128, b: V128, c: V128) -> V128 {
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
