//! AVX-512 VP2INTERSECT with 128-bit and 256-bit values.
//!
//! The 512-bit forms are in [`avx512vp2intersect`](super::super::avx512vp2intersect).

use crate::x86::simd::avx2::V256;
use crate::x86::simd::sse2::V128;
use crate::x86::simd::{Mask2, Mask4, Mask8};

unsafe extern "C" {
    fn rust_isa_avx512vl_vp2intersectd_v128(
        a: *const u8,
        b: *const u8,
        a_mask: *mut u32,
        b_mask: *mut u32,
    );
    fn rust_isa_avx512vl_vp2intersectq_v128(
        a: *const u8,
        b: *const u8,
        a_mask: *mut u32,
        b_mask: *mut u32,
    );
    fn rust_isa_avx512vl_vp2intersectd_v256(
        a: *const u8,
        b: *const u8,
        a_mask: *mut u32,
        b_mask: *mut u32,
    );
    fn rust_isa_avx512vl_vp2intersectq_v256(
        a: *const u8,
        b: *const u8,
        a_mask: *mut u32,
        b_mask: *mut u32,
    );
}

/// `vp2intersectd` on 128-bit values.
///
/// Return the 4-bit intersection masks for 32-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VP2INTERSECT, and AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vp2intersect")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vp2intersectd_v128(a: V128, b: V128) -> (Mask4, Mask4) {
    let mut a_mask = 0u32;
    let mut b_mask = 0u32;
    unsafe {
        rust_isa_avx512vl_vp2intersectd_v128(a.as_ptr(), b.as_ptr(), &mut a_mask, &mut b_mask);
    }
    (
        Mask4::from_bits(a_mask as u8),
        Mask4::from_bits(b_mask as u8),
    )
}

/// `vp2intersectq` on 128-bit values.
///
/// Return the 2-bit intersection masks for 64-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VP2INTERSECT, and AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vp2intersect")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vp2intersectq_v128(a: V128, b: V128) -> (Mask2, Mask2) {
    let mut a_mask = 0u32;
    let mut b_mask = 0u32;
    unsafe {
        rust_isa_avx512vl_vp2intersectq_v128(a.as_ptr(), b.as_ptr(), &mut a_mask, &mut b_mask);
    }
    (
        Mask2::from_bits(a_mask as u8),
        Mask2::from_bits(b_mask as u8),
    )
}

/// `vp2intersectd` on 256-bit values.
///
/// Return the 8-bit intersection masks for 32-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VP2INTERSECT, and AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vp2intersect")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vp2intersectd_v256(a: V256, b: V256) -> (Mask8, Mask8) {
    let mut a_mask = 0u32;
    let mut b_mask = 0u32;
    unsafe {
        rust_isa_avx512vl_vp2intersectd_v256(a.as_ptr(), b.as_ptr(), &mut a_mask, &mut b_mask);
    }
    (
        Mask8::from_bits(a_mask as u8),
        Mask8::from_bits(b_mask as u8),
    )
}

/// `vp2intersectq` on 256-bit values.
///
/// Return the 4-bit intersection masks for 64-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VP2INTERSECT, and AVX-512VL.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vp2intersect")]
#[target_feature(enable = "avx512vl")]
#[inline]
pub unsafe fn vp2intersectq_v256(a: V256, b: V256) -> (Mask4, Mask4) {
    let mut a_mask = 0u32;
    let mut b_mask = 0u32;
    unsafe {
        rust_isa_avx512vl_vp2intersectq_v256(a.as_ptr(), b.as_ptr(), &mut a_mask, &mut b_mask);
    }
    (
        Mask4::from_bits(a_mask as u8),
        Mask4::from_bits(b_mask as u8),
    )
}
