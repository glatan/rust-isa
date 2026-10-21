//! AVX-512 VBMI2.
//!
//! AVX-512 VBMI2 is not part of the x86-64 baseline. Every operation is
//! `unsafe` and the caller must enable AVX-512 VBMI2 with
//! `#[target_feature(enable = "avx512vbmi2")]` (which implies AVX-512 F and BW)
//! or a global target feature.
//!
//! The values are [`V512`]. Compress packs the selected elements to the low
//! end of the result; expand writes the source elements into the selected
//! positions and zeroes the rest.

use super::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512vbmi2_vpcompressb(a: *const u8, mask: u64, out: *mut u8);
    fn rust_isa_avx512vbmi2_vpexpandb(a: *const u8, mask: u64, out: *mut u8);
    fn rust_isa_avx512vbmi2_vpcompressw(a: *const u8, mask: u32, out: *mut u8);
    fn rust_isa_avx512vbmi2_vpexpandw(a: *const u8, mask: u32, out: *mut u8);
}

/// `vpcompressb`
///
/// Pack the selected bytes to the low end of the result and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512 VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpcompressb(a: V512, mask: u64) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi2_vpcompressb(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

/// `vpexpandb`
///
/// Write source bytes into the selected positions and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512 VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpexpandb(a: V512, mask: u64) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi2_vpexpandb(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

/// `vpcompressw`
///
/// Pack the selected 16-bit lanes to the low end of the result and zero the
/// rest.
///
/// # Safety
///
/// The caller must enable AVX-512 VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpcompressw(a: V512, mask: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi2_vpcompressw(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

/// `vpexpandw`
///
/// Write source 16-bit lanes into the selected positions and zero the rest.
///
/// # Safety
///
/// The caller must enable AVX-512 VBMI2.
#[target_feature(enable = "avx512vbmi2")]
#[inline]
pub unsafe fn vpexpandw(a: V512, mask: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512vbmi2_vpexpandw(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}
