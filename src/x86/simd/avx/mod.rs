//! AVX.
//!
//! AVX is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable AVX with `#[target_feature(enable = "avx")]` or a global
//! target feature.
//!
//! The 256-bit values are [`V256`]. The scalar `*ss` and `*sd` operations take
//! and return a [`V128`]; only the low lane is defined by the instruction and
//! the upper lanes are copied from the first operand.
//!
//! Immediate forms use const-generic inline assembly with memory operands.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::x86::simd::avx::vaddps;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! assert_eq!(unsafe { vaddps(v, v) }.to_bytes(), [0; 32]);
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! unsafe {
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vaddpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vaddps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vaddsd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vaddss(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vaddsubpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vaddsubps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vandnpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vandnps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vandpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vandps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vblendpd::<0>(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vblendps::<0>(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vblendvpd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vblendvps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vbroadcastf128(core::ptr::null()) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vbroadcastsd(0.0f64) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vbroadcastss(0.0f32) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcmppd::<0>(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcmpps::<0>(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcmpsd::<0>(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcmpss::<0>(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcomisd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcomiss(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcvtdq2ps(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcvtpd2ps(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcvtps2dq(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcvtps2pd(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcvtsd2ss(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcvtss2sd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vcvttps2dq(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vdivpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vdivps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vdppd::<0>(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vdpps::<0>(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vextractf128::<0>(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vinsertf128::<0>(v256, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vlddqu(core::ptr::null()) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmaskmovpd_load(core::ptr::null(), v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmaskmovpd_store(core::ptr::null_mut(), v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmaskmovps_load(core::ptr::null(), v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmaskmovps_store(core::ptr::null_mut(), v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmaxpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmaxps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vminpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vminps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmovd(0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmovd_to_u32(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmovddup(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmovmskpd(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmovmskps(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmovntpd(core::ptr::null_mut(), v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmovntps(core::ptr::null_mut(), v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmovq(0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmovq_to_u64(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmovshdup(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmovsldup(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmpsadbw::<0>(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmulpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vmulps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vorps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vperm2f128::<0>(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vpermilpd::<0>(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vpermilps::<0>(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vpsadbw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vptest(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vrcpps(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vroundpd::<0>(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vroundps::<0>(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vroundsd::<0>(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vroundss::<0>(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vshufpd::<0>(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vshufps::<0>(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vsqrtpd(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vsqrtps(v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vsubpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vsubps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vtestpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vtestps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vucomisd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vucomiss(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vunpckhpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vunpckhps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vunpcklpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vunpcklps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vxorpd(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vxorps(v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vzeroupper() };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avx::V128 as _;
//! use rust_isa::x86::simd::avx::vaddpd as _;
//! use rust_isa::x86::simd::avx::vaddps as _;
//! use rust_isa::x86::simd::avx::vaddsd as _;
//! use rust_isa::x86::simd::avx::vaddss as _;
//! use rust_isa::x86::simd::avx::vaddsubpd as _;
//! use rust_isa::x86::simd::avx::vaddsubps as _;
//! use rust_isa::x86::simd::avx::vandnpd as _;
//! use rust_isa::x86::simd::avx::vandnps as _;
//! use rust_isa::x86::simd::avx::vandpd as _;
//! use rust_isa::x86::simd::avx::vandps as _;
//! use rust_isa::x86::simd::avx::vblendpd as _;
//! use rust_isa::x86::simd::avx::vblendps as _;
//! use rust_isa::x86::simd::avx::vblendvpd as _;
//! use rust_isa::x86::simd::avx::vblendvps as _;
//! use rust_isa::x86::simd::avx::vbroadcastf128 as _;
//! use rust_isa::x86::simd::avx::vbroadcastsd as _;
//! use rust_isa::x86::simd::avx::vbroadcastss as _;
//! use rust_isa::x86::simd::avx::vcmppd as _;
//! use rust_isa::x86::simd::avx::vcmpps as _;
//! use rust_isa::x86::simd::avx::vcmpsd as _;
//! use rust_isa::x86::simd::avx::vcmpss as _;
//! use rust_isa::x86::simd::avx::vcomisd as _;
//! use rust_isa::x86::simd::avx::vcomiss as _;
//! use rust_isa::x86::simd::avx::vcvtdq2ps as _;
//! use rust_isa::x86::simd::avx::vcvtpd2ps as _;
//! use rust_isa::x86::simd::avx::vcvtps2dq as _;
//! use rust_isa::x86::simd::avx::vcvtps2pd as _;
//! use rust_isa::x86::simd::avx::vcvtsd2ss as _;
//! use rust_isa::x86::simd::avx::vcvtss2sd as _;
//! use rust_isa::x86::simd::avx::vcvttps2dq as _;
//! use rust_isa::x86::simd::avx::vdivpd as _;
//! use rust_isa::x86::simd::avx::vdivps as _;
//! use rust_isa::x86::simd::avx::vdppd as _;
//! use rust_isa::x86::simd::avx::vdpps as _;
//! use rust_isa::x86::simd::avx::vextractf128 as _;
//! use rust_isa::x86::simd::avx::vinsertf128 as _;
//! use rust_isa::x86::simd::avx::vlddqu as _;
//! use rust_isa::x86::simd::avx::vmaskmovpd_load as _;
//! use rust_isa::x86::simd::avx::vmaskmovpd_store as _;
//! use rust_isa::x86::simd::avx::vmaskmovps_load as _;
//! use rust_isa::x86::simd::avx::vmaskmovps_store as _;
//! use rust_isa::x86::simd::avx::vmaxpd as _;
//! use rust_isa::x86::simd::avx::vmaxps as _;
//! use rust_isa::x86::simd::avx::vminpd as _;
//! use rust_isa::x86::simd::avx::vminps as _;
//! use rust_isa::x86::simd::avx::vmovd as _;
//! use rust_isa::x86::simd::avx::vmovd_to_u32 as _;
//! use rust_isa::x86::simd::avx::vmovddup as _;
//! use rust_isa::x86::simd::avx::vmovmskpd as _;
//! use rust_isa::x86::simd::avx::vmovmskps as _;
//! use rust_isa::x86::simd::avx::vmovntpd as _;
//! use rust_isa::x86::simd::avx::vmovntps as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::simd::avx::vmovq as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::simd::avx::vmovq_to_u64 as _;
//! use rust_isa::x86::simd::avx::vmovshdup as _;
//! use rust_isa::x86::simd::avx::vmovsldup as _;
//! use rust_isa::x86::simd::avx::vmpsadbw as _;
//! use rust_isa::x86::simd::avx::vmulpd as _;
//! use rust_isa::x86::simd::avx::vmulps as _;
//! use rust_isa::x86::simd::avx::vorps as _;
//! use rust_isa::x86::simd::avx::vperm2f128 as _;
//! use rust_isa::x86::simd::avx::vpermilpd as _;
//! use rust_isa::x86::simd::avx::vpermilps as _;
//! use rust_isa::x86::simd::avx::vpsadbw as _;
//! use rust_isa::x86::simd::avx::vptest as _;
//! use rust_isa::x86::simd::avx::vrcpps as _;
//! use rust_isa::x86::simd::avx::vroundpd as _;
//! use rust_isa::x86::simd::avx::vroundps as _;
//! use rust_isa::x86::simd::avx::vroundsd as _;
//! use rust_isa::x86::simd::avx::vroundss as _;
//! use rust_isa::x86::simd::avx::vshufpd as _;
//! use rust_isa::x86::simd::avx::vshufps as _;
//! use rust_isa::x86::simd::avx::vsqrtpd as _;
//! use rust_isa::x86::simd::avx::vsqrtps as _;
//! use rust_isa::x86::simd::avx::vsubpd as _;
//! use rust_isa::x86::simd::avx::vsubps as _;
//! use rust_isa::x86::simd::avx::vtestpd as _;
//! use rust_isa::x86::simd::avx::vtestps as _;
//! use rust_isa::x86::simd::avx::vucomisd as _;
//! use rust_isa::x86::simd::avx::vucomiss as _;
//! use rust_isa::x86::simd::avx::vunpckhpd as _;
//! use rust_isa::x86::simd::avx::vunpckhps as _;
//! use rust_isa::x86::simd::avx::vunpcklpd as _;
//! use rust_isa::x86::simd::avx::vunpcklps as _;
//! use rust_isa::x86::simd::avx::vxorpd as _;
//! use rust_isa::x86::simd::avx::vxorps as _;
//! use rust_isa::x86::simd::avx::vzeroupper as _;
//! ```
//! <!-- doctest-inventory:end -->

pub use super::sse2::V128;

mod blend;
mod load;
mod masked_memory;
mod moves;
mod permute;
mod round;
mod shuffle;
mod v256;

pub use blend::*;
pub use load::*;
pub use masked_memory::*;
pub use moves::*;
pub use permute::*;
pub use round::*;
pub use shuffle::*;
pub use v256::*;

mod arithmetic;
mod bitwise;
mod compare;
mod compare_imm;
mod convert;
mod dot;
mod nontemporal;

pub use arithmetic::*;
pub use bitwise::*;
pub use compare::*;
pub use compare_imm::*;
pub use convert::*;
pub use dot::*;
pub use nontemporal::*;

unsafe extern "C" {
    fn rust_isa_avx_vaddps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vaddpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vsubps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vsubpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vmulps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vmulpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vdivps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vdivpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vaddss(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vaddsd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vbroadcastss(value: f32, out: *mut u8);
    fn rust_isa_avx_vbroadcastsd(value: f64, out: *mut u8);
    fn rust_isa_avx_vmovmskps(a: *const u8) -> u32;
    fn rust_isa_avx_vzeroupper();
}

/// `vaddps`
///
/// Add the eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vaddps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vaddps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddpd`
///
/// Add the four 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vaddpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vaddpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubps`
///
/// Subtract the eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vsubps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vsubps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubpd`
///
/// Subtract the four 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vsubpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vsubpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulps`
///
/// Multiply the eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmulps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vmulps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulpd`
///
/// Multiply the four 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmulpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vmulpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdivps`
///
/// Divide the eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vdivps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vdivps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdivpd`
///
/// Divide the four 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vdivpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vdivpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddss`
///
/// Add the low 32-bit float lane. The upper lanes are copied from `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vaddss(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx_vaddss(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddsd`
///
/// Add the low 64-bit float lane. The upper lane is copied from `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vaddsd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx_vaddsd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbroadcastss`
///
/// Broadcast a 32-bit float to all eight lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vbroadcastss(value: f32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vbroadcastss(value, out.as_mut_ptr());
    }
    out
}

/// `vbroadcastsd`
///
/// Broadcast a 64-bit float to all four lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vbroadcastsd(value: f64) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vbroadcastsd(value, out.as_mut_ptr());
    }
    out
}

/// `vmovmskps`
///
/// Return the sign bits of the eight 32-bit float lanes as a mask.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovmskps(a: V256) -> u32 {
    unsafe { rust_isa_avx_vmovmskps(a.as_ptr()) }
}

/// `vzeroupper`
///
/// Clear the upper halves of the vector registers.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vzeroupper() {
    unsafe {
        rust_isa_avx_vzeroupper();
    }
}
