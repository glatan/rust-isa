//! AVX-512VL.
//!
//! AVX-512VL is not part of the x86-64 baseline. Every instruction wrapper is `unsafe`
//! and the required target features are documented on each function. The values
//! are [`V256`] and [`V128`]. The functions also require the AVX-512 extension
//! that defines the instruction, such as AVX-512BW or AVX-512DQ.
//! `vp2intersectd` and `vp2intersectq` additionally require
//! AVX-512VP2INTERSECT.
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
//! use rust_isa::x86::simd::avx512vl::vpaddd_v128;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { vpaddd_v128(v, v) }.to_bytes(), [0; 16]);
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avx512vl::vblendmpd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vblendmpd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vblendmps_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vblendmps_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vcompresspd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vcompresspd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vcompressps_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vcompressps_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vexpandpd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vexpandpd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vexpandps_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vexpandps_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vgatherdpd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vgatherdpd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vgatherdps_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vgatherdps_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vgatherqpd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vgatherqpd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vgatherqps_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vgatherqps_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vmovapd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vmovapd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vmovaps_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vmovaps_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vp2intersectd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vp2intersectd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vp2intersectq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vp2intersectq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddsb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddsb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddsw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddsw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddusb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddusb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddusw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddusw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpaddw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpandd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpandd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpandnd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpandnd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpandnq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpandnq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpandq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpandq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpavgb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpavgb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpavgw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpavgw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpub_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpub_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpud_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpud_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpuq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpuq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpuw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpuw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpcmpw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpcompressb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpcompressb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpcompressw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpcompressw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpdpbusd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpdpbusd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpdpbusds_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpdpbusds_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpdpwssd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpdpwssd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpdpwssds_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpdpwssds_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpermb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpermb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpermd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpermd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpermi2b_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpermi2b_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpermilpd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpermilpd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpermilps_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpermilps_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpermq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpermq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpermt2b_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpermt2b_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpexpandb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpexpandb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpexpandw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpexpandw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmadd52huq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmadd52huq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmadd52luq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmadd52luq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmaddubsw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmaddubsw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmaddwd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmaddwd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmaxsb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmaxsb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmaxsw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmaxsw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmaxub_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmaxub_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpminsb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpminsb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpminsw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpminsw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpminub_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpminub_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovb2m_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovb2m_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovd2m_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovd2m_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovm2b_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovm2b_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovm2d_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovm2d_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovm2q_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovm2q_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovm2w_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovm2w_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovq2m_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovq2m_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovw2m_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmovw2m_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmulhuw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmulhuw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmulhw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmulhw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmullq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmullq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpmullw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpmullw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpopcntb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpopcntb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpopcntd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpopcntd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpopcntq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpopcntq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpopcntw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpopcntw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpord_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpord_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vporq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vporq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpshldd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpshldd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpshldq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpshldq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpshldw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpshldw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpshrdd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpshrdd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpshrdq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpshrdq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpshrdw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpshrdw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpshufbitqmb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpshufbitqmb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubsb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubsb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubsw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubsw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubusb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubusb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubusw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubusw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpsubw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpternlogd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpternlogd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpternlogq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpternlogq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vptestmb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vptestmb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vptestmd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vptestmd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vptestmq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vptestmq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vptestmw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vptestmw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vptestnmb_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vptestnmb_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vptestnmd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vptestnmd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vptestnmq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vptestnmq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vptestnmw_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vptestnmw_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpxord_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpxord_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vpxorq_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vpxorq_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vscatterdpd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vscatterdpd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vscatterdps_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vscatterdps_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vscatterqpd_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vscatterqpd_v256 as _;
//! use rust_isa::x86::simd::avx512vl::vscatterqps_v128 as _;
//! use rust_isa::x86::simd::avx512vl::vscatterqps_v256 as _;
//! ```
//! <!-- doctest-inventory:end -->

mod vp2intersect;

pub use vp2intersect::*;

use crate::x86::simd::avx2::V256;
use crate::x86::simd::sse2::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddd_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddd_v128`
/// Add the 32-bit lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddd_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpaddd_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddd_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddd_v256`
/// Add the 32-bit lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddd_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpaddd_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddq_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddq_v128`
/// Add the 64-bit lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddq_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpaddq_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddq_v256`
/// Add the 64-bit lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpaddq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubd_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubd_v128`
/// Subtract the 32-bit lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubd_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpsubd_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubd_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubd_v256`
/// Subtract the 32-bit lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubd_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpsubd_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubq_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubq_v128`
/// Subtract the 64-bit lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubq_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpsubq_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubq_v256`
/// Subtract the 64-bit lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpsubq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpandd_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandd_v128`
/// Bitwise AND.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpandd_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpandd_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpandd_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandd_v256`
/// Bitwise AND.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpandd_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpandd_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpandq_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandq_v128`
/// Bitwise AND.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpandq_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpandq_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpandq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandq_v256`
/// Bitwise AND.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpandq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpandq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpandnd_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandnd_v128`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpandnd_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpandnd_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpandnd_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandnd_v256`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpandnd_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpandnd_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpandnq_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandnq_v128`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpandnq_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpandnq_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpandnq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandnq_v256`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpandnq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpandnq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpord_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpord_v128`
/// Bitwise OR.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpord_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpord_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpord_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpord_v256`
/// Bitwise OR.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpord_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpord_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vporq_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vporq_v128`
/// Bitwise OR.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vporq_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vporq_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vporq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vporq_v256`
/// Bitwise OR.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vporq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vporq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpxord_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpxord_v128`
/// Bitwise XOR.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpxord_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpxord_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpxord_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpxord_v256`
/// Bitwise XOR.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpxord_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpxord_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpxorq_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpxorq_v128`
/// Bitwise XOR.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpxorq_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpxorq_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpxorq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpxorq_v256`
/// Bitwise XOR.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpxorq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpxorq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpternlogd_v128`
/// Combine the three 32-bit lane values with the truth table in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpternlogd_v128<const IMM: u8>(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vmovdqu xmm2, [{c}]",
            "vpternlogd xmm0, xmm1, xmm2, {imm}",
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

/// `vpternlogq_v128`
/// Combine the three 64-bit lane values with the truth table in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpternlogq_v128<const IMM: u8>(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vmovdqu xmm2, [{c}]",
            "vpternlogq xmm0, xmm1, xmm2, {imm}",
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

/// `vpternlogd_v256`
/// Combine the three 32-bit lane values with the truth table in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpternlogd_v256<const IMM: u8>(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vmovdqu ymm2, [{c}]",
            "vpternlogd ymm0, ymm1, ymm2, {imm}",
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

/// `vpternlogq_v256`
/// Combine the three 64-bit lane values with the truth table in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpternlogq_v256<const IMM: u8>(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vmovdqu ymm2, [{c}]",
            "vpternlogq ymm0, ymm1, ymm2, {imm}",
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

/// `vpcmpd_v128`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpd_v128<const IMM: u8>(a: V128, b: V128) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpcmpd k1, xmm0, xmm1, {imm}",
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

/// `vpcmpd_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpd_v256<const IMM: u8>(a: V256, b: V256) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpd k1, ymm0, ymm1, {imm}",
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

/// `vpcmpud_v128`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpud_v128<const IMM: u8>(a: V128, b: V128) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpcmpud k1, xmm0, xmm1, {imm}",
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

/// `vpcmpud_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpud_v256<const IMM: u8>(a: V256, b: V256) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpud k1, ymm0, ymm1, {imm}",
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

/// `vpcmpq_v128`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpq_v128<const IMM: u8>(a: V128, b: V128) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpcmpq k1, xmm0, xmm1, {imm}",
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

/// `vpcmpq_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpq_v256<const IMM: u8>(a: V256, b: V256) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpq k1, ymm0, ymm1, {imm}",
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

/// `vpcmpuq_v128`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpuq_v128<const IMM: u8>(a: V128, b: V128) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpcmpuq k1, xmm0, xmm1, {imm}",
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

/// `vpcmpuq_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpuq_v256<const IMM: u8>(a: V256, b: V256) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpuq k1, ymm0, ymm1, {imm}",
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

/// `vpcmpb_v128`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpb_v128<const IMM: u8>(a: V128, b: V128) -> u16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpcmpb k1, xmm0, xmm1, {imm}",
            "kmovw {out:e}, k1",
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
    out as u16
}

/// `vpcmpb_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpb_v256<const IMM: u8>(a: V256, b: V256) -> u32 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpb k1, ymm0, ymm1, {imm}",
            "kmovd {out:e}, k1",
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
    out as u32
}

/// `vpcmpub_v128`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpub_v128<const IMM: u8>(a: V128, b: V128) -> u16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpcmpub k1, xmm0, xmm1, {imm}",
            "kmovw {out:e}, k1",
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
    out as u16
}

/// `vpcmpub_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpub_v256<const IMM: u8>(a: V256, b: V256) -> u32 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpub k1, ymm0, ymm1, {imm}",
            "kmovd {out:e}, k1",
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
    out as u32
}

/// `vpcmpw_v128`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpw_v128<const IMM: u8>(a: V128, b: V128) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpcmpw k1, xmm0, xmm1, {imm}",
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

/// `vpcmpw_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpw_v256<const IMM: u8>(a: V256, b: V256) -> u16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpw k1, ymm0, ymm1, {imm}",
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

/// `vpcmpuw_v128`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpuw_v128<const IMM: u8>(a: V128, b: V128) -> u8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpcmpuw k1, xmm0, xmm1, {imm}",
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

/// `vpcmpuw_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcmpuw_v256<const IMM: u8>(a: V256, b: V256) -> u16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpuw k1, ymm0, ymm1, {imm}",
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

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestmd_v128(a: *const u8, b: *const u8) -> u8;
}

/// `vptestmd_v128`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestmd_v128(a: V128, b: V128) -> u8 {
    unsafe { rust_isa_avx512vl_vptestmd_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestmd_v256(a: *const u8, b: *const u8) -> u8;
}

/// `vptestmd_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestmd_v256(a: V256, b: V256) -> u8 {
    unsafe { rust_isa_avx512vl_vptestmd_v256(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestnmd_v128(a: *const u8, b: *const u8) -> u8;
}

/// `vptestnmd_v128`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestnmd_v128(a: V128, b: V128) -> u8 {
    unsafe { rust_isa_avx512vl_vptestnmd_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestnmd_v256(a: *const u8, b: *const u8) -> u8;
}

/// `vptestnmd_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestnmd_v256(a: V256, b: V256) -> u8 {
    unsafe { rust_isa_avx512vl_vptestnmd_v256(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestmq_v128(a: *const u8, b: *const u8) -> u8;
}

/// `vptestmq_v128`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestmq_v128(a: V128, b: V128) -> u8 {
    unsafe { rust_isa_avx512vl_vptestmq_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestmq_v256(a: *const u8, b: *const u8) -> u8;
}

/// `vptestmq_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestmq_v256(a: V256, b: V256) -> u8 {
    unsafe { rust_isa_avx512vl_vptestmq_v256(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestnmq_v128(a: *const u8, b: *const u8) -> u8;
}

/// `vptestnmq_v128`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestnmq_v128(a: V128, b: V128) -> u8 {
    unsafe { rust_isa_avx512vl_vptestnmq_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestnmq_v256(a: *const u8, b: *const u8) -> u8;
}

/// `vptestnmq_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestnmq_v256(a: V256, b: V256) -> u8 {
    unsafe { rust_isa_avx512vl_vptestnmq_v256(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestmb_v128(a: *const u8, b: *const u8) -> u16;
}

/// `vptestmb_v128`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestmb_v128(a: V128, b: V128) -> u16 {
    unsafe { rust_isa_avx512vl_vptestmb_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestmb_v256(a: *const u8, b: *const u8) -> u32;
}

/// `vptestmb_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestmb_v256(a: V256, b: V256) -> u32 {
    unsafe { rust_isa_avx512vl_vptestmb_v256(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestnmb_v128(a: *const u8, b: *const u8) -> u16;
}

/// `vptestnmb_v128`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestnmb_v128(a: V128, b: V128) -> u16 {
    unsafe { rust_isa_avx512vl_vptestnmb_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestnmb_v256(a: *const u8, b: *const u8) -> u32;
}

/// `vptestnmb_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestnmb_v256(a: V256, b: V256) -> u32 {
    unsafe { rust_isa_avx512vl_vptestnmb_v256(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestmw_v128(a: *const u8, b: *const u8) -> u8;
}

/// `vptestmw_v128`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestmw_v128(a: V128, b: V128) -> u8 {
    unsafe { rust_isa_avx512vl_vptestmw_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestmw_v256(a: *const u8, b: *const u8) -> u16;
}

/// `vptestmw_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestmw_v256(a: V256, b: V256) -> u16 {
    unsafe { rust_isa_avx512vl_vptestmw_v256(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestnmw_v128(a: *const u8, b: *const u8) -> u8;
}

/// `vptestnmw_v128`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestnmw_v128(a: V128, b: V128) -> u8 {
    unsafe { rust_isa_avx512vl_vptestnmw_v128(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vptestnmw_v256(a: *const u8, b: *const u8) -> u16;
}

/// `vptestnmw_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vptestnmw_v256(a: V256, b: V256) -> u16 {
    unsafe { rust_isa_avx512vl_vptestnmw_v256(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovm2b_v128(mask: u16, out: *mut u8);
}

/// `vpmovm2b_v128`
/// Expand the mask bits to all-ones or all-zeros lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovm2b_v128(mask: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmovm2b_v128(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovm2b_v256(mask: u32, out: *mut u8);
}

/// `vpmovm2b_v256`
/// Expand the mask bits to all-ones or all-zeros lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovm2b_v256(mask: u32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmovm2b_v256(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovm2w_v128(mask: u8, out: *mut u8);
}

/// `vpmovm2w_v128`
/// Expand the mask bits to all-ones or all-zeros lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovm2w_v128(mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmovm2w_v128(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovm2w_v256(mask: u16, out: *mut u8);
}

/// `vpmovm2w_v256`
/// Expand the mask bits to all-ones or all-zeros lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovm2w_v256(mask: u16) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmovm2w_v256(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovm2d_v128(mask: u8, out: *mut u8);
}

/// `vpmovm2d_v128`
/// Expand the mask bits to all-ones or all-zeros lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovm2d_v128(mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmovm2d_v128(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovm2d_v256(mask: u8, out: *mut u8);
}

/// `vpmovm2d_v256`
/// Expand the mask bits to all-ones or all-zeros lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovm2d_v256(mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmovm2d_v256(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovm2q_v128(mask: u8, out: *mut u8);
}

/// `vpmovm2q_v128`
/// Expand the mask bits to all-ones or all-zeros lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovm2q_v128(mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmovm2q_v128(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovm2q_v256(mask: u8, out: *mut u8);
}

/// `vpmovm2q_v256`
/// Expand the mask bits to all-ones or all-zeros lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovm2q_v256(mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmovm2q_v256(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovb2m_v128(a: *const u8) -> u16;
}

/// `vpmovb2m_v128`
/// Collect the sign bits of the lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovb2m_v128(a: V128) -> u16 {
    unsafe { rust_isa_avx512vl_vpmovb2m_v128(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovb2m_v256(a: *const u8) -> u32;
}

/// `vpmovb2m_v256`
/// Collect the sign bits of the lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovb2m_v256(a: V256) -> u32 {
    unsafe { rust_isa_avx512vl_vpmovb2m_v256(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovw2m_v128(a: *const u8) -> u8;
}

/// `vpmovw2m_v128`
/// Collect the sign bits of the lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovw2m_v128(a: V128) -> u8 {
    unsafe { rust_isa_avx512vl_vpmovw2m_v128(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovw2m_v256(a: *const u8) -> u16;
}

/// `vpmovw2m_v256`
/// Collect the sign bits of the lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovw2m_v256(a: V256) -> u16 {
    unsafe { rust_isa_avx512vl_vpmovw2m_v256(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovd2m_v128(a: *const u8) -> u8;
}

/// `vpmovd2m_v128`
/// Collect the sign bits of the lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovd2m_v128(a: V128) -> u8 {
    unsafe { rust_isa_avx512vl_vpmovd2m_v128(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovd2m_v256(a: *const u8) -> u8;
}

/// `vpmovd2m_v256`
/// Collect the sign bits of the lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovd2m_v256(a: V256) -> u8 {
    unsafe { rust_isa_avx512vl_vpmovd2m_v256(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovq2m_v128(a: *const u8) -> u8;
}

/// `vpmovq2m_v128`
/// Collect the sign bits of the lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovq2m_v128(a: V128) -> u8 {
    unsafe { rust_isa_avx512vl_vpmovq2m_v128(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmovq2m_v256(a: *const u8) -> u8;
}

/// `vpmovq2m_v256`
/// Collect the sign bits of the lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmovq2m_v256(a: V256) -> u8 {
    unsafe { rust_isa_avx512vl_vpmovq2m_v256(a.as_ptr()) }
}

/// `vpermd_v128`
/// Permute the integer lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermd_v128(a: V128, idx: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{idx}]",
            "vpermd xmm0, xmm1, xmm0",
            "vmovdqu [{out}], xmm0",
            a = in(reg) a.as_ptr(),
            idx = in(reg) idx.as_ptr(),
            out = in(reg) out.as_mut_ptr(),
            out("xmm0") _,
            out("xmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpermd_v256`
/// Permute the integer lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermd_v256(a: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{idx}]",
            "vpermd ymm0, ymm1, ymm0",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            idx = in(reg) idx.as_ptr(),
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpermq_v128`
/// Permute the integer lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermq_v128(a: V128, idx: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{idx}]",
            "vpermq xmm0, xmm1, xmm0",
            "vmovdqu [{out}], xmm0",
            a = in(reg) a.as_ptr(),
            idx = in(reg) idx.as_ptr(),
            out = in(reg) out.as_mut_ptr(),
            out("xmm0") _,
            out("xmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpermq_v256`
/// Permute the integer lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermq_v256(a: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{idx}]",
            "vpermq ymm0, ymm1, ymm0",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            idx = in(reg) idx.as_ptr(),
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpermilps_v128`
/// Permute the float lanes within 128-bit blocks with `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermilps_v128<const IMM: u8>(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vpermilps xmm0, xmm0, {imm}",
            "vmovdqu [{out}], xmm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("xmm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpermilps_v256`
/// Permute the float lanes within 128-bit blocks with `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermilps_v256<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vpermilps ymm0, ymm0, {imm}",
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

/// `vpermilpd_v128`
/// Permute the float lanes within 128-bit blocks with `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermilpd_v128<const IMM: u8>(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vpermilpd xmm0, xmm0, {imm}",
            "vmovdqu [{out}], xmm0",
            a = in(reg) a.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("xmm0") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpermilpd_v256`
/// Permute the float lanes within 128-bit blocks with `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermilpd_v256<const IMM: u8>(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vpermilpd ymm0, ymm0, {imm}",
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

unsafe extern "C" {
    fn rust_isa_avx512vl_vcompressps_v128(a: *const u8, mask: u8, out: *mut u8);
}

/// `vcompressps_v128`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vcompressps_v128(a: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vcompressps_v128(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vcompressps_v256(a: *const u8, mask: u8, out: *mut u8);
}

/// `vcompressps_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vcompressps_v256(a: V256, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vcompressps_v256(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vcompresspd_v128(a: *const u8, mask: u8, out: *mut u8);
}

/// `vcompresspd_v128`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vcompresspd_v128(a: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vcompresspd_v128(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vcompresspd_v256(a: *const u8, mask: u8, out: *mut u8);
}

/// `vcompresspd_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vcompresspd_v256(a: V256, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vcompresspd_v256(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vexpandps_v128(a: *const u8, mask: u8, out: *mut u8);
}

/// `vexpandps_v128`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vexpandps_v128(a: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vexpandps_v128(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vexpandps_v256(a: *const u8, mask: u8, out: *mut u8);
}

/// `vexpandps_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vexpandps_v256(a: V256, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vexpandps_v256(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vexpandpd_v128(a: *const u8, mask: u8, out: *mut u8);
}

/// `vexpandpd_v128`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vexpandpd_v128(a: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vexpandpd_v128(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vexpandpd_v256(a: *const u8, mask: u8, out: *mut u8);
}

/// `vexpandpd_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vexpandpd_v256(a: V256, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vexpandpd_v256(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vblendmps_v128(a: *const u8, b: *const u8, mask: u8, out: *mut u8);
}

/// `vblendmps_v128`
/// Blend the lanes selected by `mask` from `b` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vblendmps_v128(a: V128, b: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vblendmps_v128(a.as_ptr(), b.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vblendmps_v256(a: *const u8, b: *const u8, mask: u8, out: *mut u8);
}

/// `vblendmps_v256`
/// Blend the lanes selected by `mask` from `b` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vblendmps_v256(a: V256, b: V256, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vblendmps_v256(a.as_ptr(), b.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vblendmpd_v128(a: *const u8, b: *const u8, mask: u8, out: *mut u8);
}

/// `vblendmpd_v128`
/// Blend the lanes selected by `mask` from `b` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vblendmpd_v128(a: V128, b: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vblendmpd_v128(a.as_ptr(), b.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vblendmpd_v256(a: *const u8, b: *const u8, mask: u8, out: *mut u8);
}

/// `vblendmpd_v256`
/// Blend the lanes selected by `mask` from `b` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vblendmpd_v256(a: V256, b: V256, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vblendmpd_v256(a.as_ptr(), b.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vmovaps_v128(a: *const u8, src: *const u8, mask: u8, out: *mut u8);
}

/// `vmovaps_v128`
/// Move the lanes selected by `mask` from `src` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vmovaps_v128(a: V128, src: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vmovaps_v128(a.as_ptr(), src.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vmovaps_v256(a: *const u8, src: *const u8, mask: u8, out: *mut u8);
}

/// `vmovaps_v256`
/// Move the lanes selected by `mask` from `src` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vmovaps_v256(a: V256, src: V256, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vmovaps_v256(a.as_ptr(), src.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vmovapd_v128(a: *const u8, src: *const u8, mask: u8, out: *mut u8);
}

/// `vmovapd_v128`
/// Move the lanes selected by `mask` from `src` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vmovapd_v128(a: V128, src: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vmovapd_v128(a.as_ptr(), src.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vmovapd_v256(a: *const u8, src: *const u8, mask: u8, out: *mut u8);
}

/// `vmovapd_v256`
/// Move the lanes selected by `mask` from `src` and the rest from `a`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vmovapd_v256(a: V256, src: V256, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vmovapd_v256(a.as_ptr(), src.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vgatherdps_v128(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherdps_v128`
/// Gather 32-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for reading 4 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vgatherdps_v128(src: V128, base: *const u8, indices: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vgatherdps_v128(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask,
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vgatherdps_v256(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherdps_v256`
/// Gather 32-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for reading 4 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vgatherdps_v256(src: V256, base: *const u8, indices: V256, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vgatherdps_v256(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask,
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vgatherdpd_v128(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherdpd_v128`
/// Gather 64-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for reading 8 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vgatherdpd_v128(src: V128, base: *const u8, indices: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vgatherdpd_v128(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask,
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vgatherdpd_v256(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherdpd_v256`
/// Gather 64-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for reading 8 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vgatherdpd_v256(src: V256, base: *const u8, indices: V128, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vgatherdpd_v256(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask,
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vgatherqps_v128(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherqps_v128`
/// Gather two 32-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for reading 4 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vgatherqps_v128(src: V128, base: *const u8, indices: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vgatherqps_v128(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask,
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vgatherqps_v256(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherqps_v256`
/// Gather four 32-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for reading 4 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vgatherqps_v256(src: V128, base: *const u8, indices: V256, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vgatherqps_v256(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask,
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vgatherqpd_v128(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherqpd_v128`
/// Gather 64-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for reading 8 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vgatherqpd_v128(src: V128, base: *const u8, indices: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vgatherqpd_v128(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask,
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vgatherqpd_v256(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherqpd_v256`
/// Gather 64-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for reading 8 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vgatherqpd_v256(src: V256, base: *const u8, indices: V256, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vgatherqpd_v256(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask,
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vscatterdps_v128(
        base: *mut u8,
        indices: *const u8,
        a: *const u8,
        mask: u8,
    );
}

/// `vscatterdps_v128`
/// Scatter 32-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for writing 4 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vscatterdps_v128(base: *mut u8, indices: V128, a: V128, mask: u8) {
    unsafe { rust_isa_avx512vl_vscatterdps_v128(base, indices.as_ptr(), a.as_ptr(), mask) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vscatterdps_v256(
        base: *mut u8,
        indices: *const u8,
        a: *const u8,
        mask: u8,
    );
}

/// `vscatterdps_v256`
/// Scatter 32-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for writing 4 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vscatterdps_v256(base: *mut u8, indices: V256, a: V256, mask: u8) {
    unsafe { rust_isa_avx512vl_vscatterdps_v256(base, indices.as_ptr(), a.as_ptr(), mask) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vscatterdpd_v128(
        base: *mut u8,
        indices: *const u8,
        a: *const u8,
        mask: u8,
    );
}

/// `vscatterdpd_v128`
/// Scatter 64-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for writing 8 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vscatterdpd_v128(base: *mut u8, indices: V128, a: V128, mask: u8) {
    unsafe { rust_isa_avx512vl_vscatterdpd_v128(base, indices.as_ptr(), a.as_ptr(), mask) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vscatterdpd_v256(
        base: *mut u8,
        indices: *const u8,
        a: *const u8,
        mask: u8,
    );
}

/// `vscatterdpd_v256`
/// Scatter 64-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for writing 8 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vscatterdpd_v256(base: *mut u8, indices: V128, a: V256, mask: u8) {
    unsafe { rust_isa_avx512vl_vscatterdpd_v256(base, indices.as_ptr(), a.as_ptr(), mask) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vscatterqps_v128(
        base: *mut u8,
        indices: *const u8,
        a: *const u8,
        mask: u8,
    );
}

/// `vscatterqps_v128`
/// Scatter two 32-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for writing 4 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vscatterqps_v128(base: *mut u8, indices: V128, a: V128, mask: u8) {
    unsafe { rust_isa_avx512vl_vscatterqps_v128(base, indices.as_ptr(), a.as_ptr(), mask) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vscatterqps_v256(
        base: *mut u8,
        indices: *const u8,
        a: *const u8,
        mask: u8,
    );
}

/// `vscatterqps_v256`
/// Scatter four 32-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for writing 4 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vscatterqps_v256(base: *mut u8, indices: V256, a: V128, mask: u8) {
    unsafe { rust_isa_avx512vl_vscatterqps_v256(base, indices.as_ptr(), a.as_ptr(), mask) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vscatterqpd_v128(
        base: *mut u8,
        indices: *const u8,
        a: *const u8,
        mask: u8,
    );
}

/// `vscatterqpd_v128`
/// Scatter 64-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for writing 8 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vscatterqpd_v128(base: *mut u8, indices: V128, a: V128, mask: u8) {
    unsafe { rust_isa_avx512vl_vscatterqpd_v128(base, indices.as_ptr(), a.as_ptr(), mask) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vscatterqpd_v256(
        base: *mut u8,
        indices: *const u8,
        a: *const u8,
        mask: u8,
    );
}

/// `vscatterqpd_v256`
/// Scatter 64-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL.
///
/// Every selected address must be valid for writing 8 bytes.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vscatterqpd_v256(base: *mut u8, indices: V256, a: V256, mask: u8) {
    unsafe { rust_isa_avx512vl_vscatterqpd_v256(base, indices.as_ptr(), a.as_ptr(), mask) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmullq_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullq_v128`
/// Multiply the 64-bit lanes, keeping the low 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmullq_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmullq_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmullq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullq_v256`
/// Multiply the 64-bit lanes, keeping the low 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512DQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmullq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmullq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmadd52luq_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmadd52luq_v128`
/// Add the low 52 bits of the unsigned 64-bit products to `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512IFMA, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512ifma")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmadd52luq_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmadd52luq_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmadd52huq_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmadd52huq_v128`
/// Add the high 52 bits of the unsigned 64-bit products to `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512IFMA, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512ifma")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmadd52huq_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmadd52huq_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmadd52luq_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmadd52luq_v256`
/// Add the low 52 bits of the unsigned 64-bit products to `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512IFMA, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512ifma")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmadd52luq_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmadd52luq_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmadd52huq_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmadd52huq_v256`
/// Add the high 52 bits of the unsigned 64-bit products to `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512IFMA, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512ifma")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmadd52huq_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmadd52huq_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpermb_v128(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermb_v128`
/// Permute the bytes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermb_v128(a: V128, idx: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpermb_v128(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpermi2b_v128(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2b_v128`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermi2b_v128(a: V128, b: V128, idx: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpermi2b_v128(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpermt2b_v128(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2b_v128`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermt2b_v128(a: V128, b: V128, idx: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpermt2b_v128(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpermb_v256(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermb_v256`
/// Permute the bytes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermb_v256(a: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpermb_v256(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpermi2b_v256(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2b_v256`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermi2b_v256(a: V256, b: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpermi2b_v256(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpermt2b_v256(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2b_v256`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VBMI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpermt2b_v256(a: V256, b: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpermt2b_v256(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpcompressb_v128(a: *const u8, mask: u16, out: *mut u8);
}

/// `vpcompressb_v128`
/// Pack selected lanes to the low end, or expand the low lanes, and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcompressb_v128(a: V128, mask: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpcompressb_v128(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpcompressb_v256(a: *const u8, mask: u32, out: *mut u8);
}

/// `vpcompressb_v256`
/// Pack selected lanes to the low end, or expand the low lanes, and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcompressb_v256(a: V256, mask: u32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpcompressb_v256(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpcompressw_v128(a: *const u8, mask: u8, out: *mut u8);
}

/// `vpcompressw_v128`
/// Pack selected lanes to the low end, or expand the low lanes, and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcompressw_v128(a: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpcompressw_v128(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpcompressw_v256(a: *const u8, mask: u16, out: *mut u8);
}

/// `vpcompressw_v256`
/// Pack selected lanes to the low end, or expand the low lanes, and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpcompressw_v256(a: V256, mask: u16) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpcompressw_v256(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpexpandb_v128(a: *const u8, mask: u16, out: *mut u8);
}

/// `vpexpandb_v128`
/// Pack selected lanes to the low end, or expand the low lanes, and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpexpandb_v128(a: V128, mask: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpexpandb_v128(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpexpandb_v256(a: *const u8, mask: u32, out: *mut u8);
}

/// `vpexpandb_v256`
/// Pack selected lanes to the low end, or expand the low lanes, and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpexpandb_v256(a: V256, mask: u32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpexpandb_v256(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpexpandw_v128(a: *const u8, mask: u8, out: *mut u8);
}

/// `vpexpandw_v128`
/// Pack selected lanes to the low end, or expand the low lanes, and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpexpandw_v128(a: V128, mask: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpexpandw_v128(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpexpandw_v256(a: *const u8, mask: u16, out: *mut u8);
}

/// `vpexpandw_v256`
/// Pack selected lanes to the low end, or expand the low lanes, and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpexpandw_v256(a: V256, mask: u16) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpexpandw_v256(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

/// `vpshldw_v128`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshldw_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpshldw xmm0, xmm0, xmm1, {imm}",
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

/// `vpshldw_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshldw_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshldw ymm0, ymm0, ymm1, {imm}",
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

/// `vpshldd_v128`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshldd_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpshldd xmm0, xmm0, xmm1, {imm}",
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

/// `vpshldd_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshldd_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshldd ymm0, ymm0, ymm1, {imm}",
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

/// `vpshldq_v128`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshldq_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpshldq xmm0, xmm0, xmm1, {imm}",
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

/// `vpshldq_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshldq_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshldq ymm0, ymm0, ymm1, {imm}",
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

/// `vpshrdw_v128`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshrdw_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpshrdw xmm0, xmm0, xmm1, {imm}",
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

/// `vpshrdw_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshrdw_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshrdw ymm0, ymm0, ymm1, {imm}",
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

/// `vpshrdd_v128`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshrdd_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpshrdd xmm0, xmm0, xmm1, {imm}",
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

/// `vpshrdd_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshrdd_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshrdd ymm0, ymm0, ymm1, {imm}",
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

/// `vpshrdq_v128`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshrdq_v128<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        asm!(
            "vmovdqu xmm0, [{a}]",
            "vmovdqu xmm1, [{b}]",
            "vpshrdq xmm0, xmm0, xmm1, {imm}",
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

/// `vpshrdq_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must enable AVX-512VBMI2, AVX-512VL.
#[target_feature(enable = "avx512vbmi2")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshrdq_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshrdq ymm0, ymm0, ymm1, {imm}",
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

unsafe extern "C" {
    fn rust_isa_avx512vl_vpdpbusd_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusd_v128`
/// Multiply lane pairs, add the products, and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpdpbusd_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpdpbusd_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpdpbusd_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusd_v256`
/// Multiply lane pairs, add the products, and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpdpbusd_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpdpbusd_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpdpbusds_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusds_v128`
/// Multiply lane pairs, add the products, and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpdpbusds_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpdpbusds_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpdpbusds_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusds_v256`
/// Multiply lane pairs, add the products, and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpdpbusds_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpdpbusds_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpdpwssd_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssd_v128`
/// Multiply lane pairs, add the products, and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpdpwssd_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpdpwssd_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpdpwssd_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssd_v256`
/// Multiply lane pairs, add the products, and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpdpwssd_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpdpwssd_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpdpwssds_v128(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssds_v128`
/// Multiply lane pairs, add the products, and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpdpwssds_v128(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpdpwssds_v128(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpdpwssds_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssds_v256`
/// Multiply lane pairs, add the products, and accumulate into `acc`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VNNI, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vnni")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpdpwssds_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpdpwssds_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpopcntb_v128(a: *const u8, out: *mut u8);
}

/// `vpopcntb_v128`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must enable AVX-512BITALG, AVX-512VL.
#[target_feature(enable = "avx512bitalg")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpopcntb_v128(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpopcntb_v128(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpopcntb_v256(a: *const u8, out: *mut u8);
}

/// `vpopcntb_v256`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must enable AVX-512BITALG, AVX-512VL.
#[target_feature(enable = "avx512bitalg")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpopcntb_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpopcntb_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpopcntw_v128(a: *const u8, out: *mut u8);
}

/// `vpopcntw_v128`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must enable AVX-512BITALG, AVX-512VL.
#[target_feature(enable = "avx512bitalg")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpopcntw_v128(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpopcntw_v128(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpopcntw_v256(a: *const u8, out: *mut u8);
}

/// `vpopcntw_v256`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must enable AVX-512BITALG, AVX-512VL.
#[target_feature(enable = "avx512bitalg")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpopcntw_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpopcntw_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpopcntd_v128(a: *const u8, out: *mut u8);
}

/// `vpopcntd_v128`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VPOPCNTDQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vpopcntdq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpopcntd_v128(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpopcntd_v128(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpopcntd_v256(a: *const u8, out: *mut u8);
}

/// `vpopcntd_v256`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VPOPCNTDQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vpopcntdq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpopcntd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpopcntd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpopcntq_v128(a: *const u8, out: *mut u8);
}

/// `vpopcntq_v128`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VPOPCNTDQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vpopcntdq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpopcntq_v128(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpopcntq_v128(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpopcntq_v256(a: *const u8, out: *mut u8);
}

/// `vpopcntq_v256`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VPOPCNTDQ, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vpopcntdq")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpopcntq_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpopcntq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpshufbitqmb_v128(a: *const u8, idx: *const u8) -> u16;
}

/// `vpshufbitqmb_v128`
/// Select one bit from each 64-bit group and return the selected bits as a mask.
///
/// # Safety
///
/// The caller must enable AVX-512BITALG, AVX-512VL.
#[target_feature(enable = "avx512bitalg")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshufbitqmb_v128(a: V128, idx: V128) -> u16 {
    unsafe { rust_isa_avx512vl_vpshufbitqmb_v128(a.as_ptr(), idx.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpshufbitqmb_v256(a: *const u8, idx: *const u8) -> u32;
}

/// `vpshufbitqmb_v256`
/// Select one bit from each 64-bit group and return the selected bits as a mask.
///
/// # Safety
///
/// The caller must enable AVX-512BITALG, AVX-512VL.
#[target_feature(enable = "avx512bitalg")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpshufbitqmb_v256(a: V256, idx: V256) -> u32 {
    unsafe { rust_isa_avx512vl_vpshufbitqmb_v256(a.as_ptr(), idx.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddb_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddb_v128`
/// Add the 8-bit lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddb_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpaddb_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddb_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddb_v256`
/// Add the 8-bit lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddb_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpaddb_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddw_v128`
/// Add the 16-bit lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpaddw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddw_v256`
/// Add the 16-bit lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpaddw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubb_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubb_v128`
/// Subtract the 8-bit lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubb_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpsubb_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubb_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubb_v256`
/// Subtract the 8-bit lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubb_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpsubb_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubw_v128`
/// Subtract the 16-bit lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpsubw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubw_v256`
/// Subtract the 16-bit lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpsubw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmullw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullw_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmullw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmullw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmullw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullw_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmullw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmullw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmulhw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulhw_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmulhw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmulhw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmulhw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulhw_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmulhw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmulhw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmulhuw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulhuw_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmulhuw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmulhuw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmulhuw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulhuw_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmulhuw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmulhuw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmaddwd_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaddwd_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmaddwd_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmaddwd_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmaddwd_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaddwd_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmaddwd_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmaddwd_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmaddubsw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaddubsw_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmaddubsw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmaddubsw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmaddubsw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaddubsw_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmaddubsw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmaddubsw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddsb_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddsb_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddsb_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpaddsb_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddsb_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddsb_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddsb_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpaddsb_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddsw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddsw_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddsw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpaddsw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddsw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddsw_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddsw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpaddsw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddusb_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddusb_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddusb_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpaddusb_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddusb_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddusb_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddusb_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpaddusb_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddusw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddusw_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddusw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpaddusw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpaddusw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddusw_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpaddusw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpaddusw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubsb_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubsb_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubsb_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpsubsb_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubsb_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubsb_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubsb_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpsubsb_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubsw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubsw_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubsw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpsubsw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubsw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubsw_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubsw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpsubsw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubusb_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubusb_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubusb_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpsubusb_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubusb_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubusb_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubusb_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpsubusb_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubusw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubusw_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubusw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpsubusw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpsubusw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubusw_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpsubusw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpsubusw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpavgb_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpavgb_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpavgb_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpavgb_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpavgb_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpavgb_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpavgb_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpavgb_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpavgw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpavgw_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpavgw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpavgw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpavgw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpavgw_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpavgw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpavgw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpminsb_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpminsb_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpminsb_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpminsb_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpminsb_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpminsb_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpminsb_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpminsb_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpminsw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpminsw_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpminsw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpminsw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpminsw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpminsw_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpminsw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpminsw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpminub_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpminub_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpminub_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpminub_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpminub_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpminub_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpminub_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpminub_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmaxsb_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaxsb_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmaxsb_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmaxsb_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmaxsb_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaxsb_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmaxsb_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmaxsb_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmaxsw_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaxsw_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmaxsw_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmaxsw_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmaxsw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaxsw_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmaxsw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmaxsw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmaxub_v128(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaxub_v128`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmaxub_v128(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512vl_vpmaxub_v128(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512vl_vpmaxub_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaxub_v256`
/// Apply the byte or word operation to the lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW, AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vpmaxub_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512vl_vpmaxub_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
