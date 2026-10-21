//! Instruction wrappers.

use crate::x86::simd::avx2::V256;
use crate::x86::simd::avx512f::V512;
use crate::x86::simd::sse2::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx512bw_vpaddb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddb`
/// Add the 64 byte lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpaddb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpaddb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpsubb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubb`
/// Subtract the 64 byte lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpsubb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpsubb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpaddw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddw`
/// Add the 32 16-bit integer lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpaddw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpaddw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpsubw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubw`
/// Subtract the 32 16-bit integer lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpsubw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpsubw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpaddsb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddsb`
/// Add the 64 signed byte lanes with signed saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpaddsb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpaddsb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpaddsw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddsw`
/// Add the 32 signed 16-bit lanes with signed saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpaddsw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpaddsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpaddusb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddusb`
/// Add the 64 unsigned byte lanes with unsigned saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpaddusb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpaddusb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpaddusw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddusw`
/// Add the 32 unsigned 16-bit lanes with unsigned saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpaddusw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpaddusw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpsubsb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubsb`
/// Subtract the 64 signed byte lanes with signed saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpsubsb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpsubsb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpsubsw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubsw`
/// Subtract the 32 signed 16-bit lanes with signed saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpsubsw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpsubsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpsubusb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubusb`
/// Subtract the 64 unsigned byte lanes with unsigned saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpsubusb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpsubusb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpsubusw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubusw`
/// Subtract the 32 unsigned 16-bit lanes with unsigned saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpsubusw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpsubusw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpavgb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpavgb`
/// Average the 64 unsigned byte lanes, rounding up.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpavgb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpavgb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpavgw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpavgw`
/// Average the 32 unsigned 16-bit lanes, rounding up.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpavgw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpavgw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpminub(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpminub`
/// Take the unsigned minimum of the 64 byte lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpminub(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpminub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpminsb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpminsb`
/// Take the signed minimum of the 64 byte lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpminsb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpminsb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpminsw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpminsw`
/// Take the signed minimum of the 32 16-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpminsw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpminsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmaxub(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaxub`
/// Take the unsigned maximum of the 64 byte lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmaxub(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpmaxub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmaxsb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaxsb`
/// Take the signed maximum of the 64 byte lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmaxsb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpmaxsb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmaxsw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaxsw`
/// Take the signed maximum of the 32 16-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmaxsw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpmaxsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmulhw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulhw`
/// Multiply the 32 signed 16-bit lanes and keep the high 16 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmulhw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpmulhw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmulhuw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulhuw`
/// Multiply the 32 unsigned 16-bit lanes and keep the high 16 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmulhuw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpmulhuw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmullw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullw`
/// Multiply the 32 16-bit integer lanes, keeping the low 16 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmullw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpmullw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmaddwd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaddwd`
/// Multiply signed 16-bit lane pairs and add each pair to a 32-bit lane.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmaddwd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpmaddwd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmaddubsw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaddubsw`
/// Multiply unsigned bytes by signed bytes, add adjacent products, and saturate to 16 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmaddubsw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpmaddubsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpacksswb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpacksswb`
/// Pack signed 16-bit lanes to signed bytes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpacksswb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpacksswb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpackssdw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpackssdw`
/// Pack signed 32-bit lanes to signed 16-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpackssdw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpackssdw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpackuswb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpackuswb`
/// Pack signed 16-bit lanes to unsigned bytes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpackuswb(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpackuswb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpackusdw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpackusdw`
/// Pack signed 32-bit lanes to unsigned 16-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpackusdw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpackusdw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpunpcklbw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpunpcklbw`
/// Interleave the low bytes of `a` and `b`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpunpcklbw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpunpcklbw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpunpckhbw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpunpckhbw`
/// Interleave the high bytes of `a` and `b`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpunpckhbw(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpunpckhbw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpunpcklwd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpunpcklwd`
/// Interleave the low 16-bit lanes of `a` and `b`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpunpcklwd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpunpcklwd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpunpckhwd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpunpckhwd`
/// Interleave the high 16-bit lanes of `a` and `b`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpunpckhwd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpunpckhwd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpsllw(a: *const u8, count: u32, out: *mut u8);
}

/// `vpsllw`
/// Shift the 32 16-bit integer lanes left by `count`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpsllw(a: V512, count: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpsllw(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpsrlw(a: *const u8, count: u32, out: *mut u8);
}

/// `vpsrlw`
/// Shift the 32 16-bit integer lanes right by `count`.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpsrlw(a: V512, count: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpsrlw(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpsraw(a: *const u8, count: u32, out: *mut u8);
}

/// `vpsraw`
/// Shift the 32 signed 16-bit lanes right by `count`, keeping the sign.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpsraw(a: V512, count: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpsraw(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `vpslldq`
/// Shift each 128-bit block left by `IMM` bytes, filling with zeros.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpslldq<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vpslldq zmm0, zmm0, {imm}",
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

/// `vpsrldq`
/// Shift each 128-bit block right by `IMM` bytes, filling with zeros.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpsrldq<const IMM: u8>(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vpsrldq zmm0, zmm0, {imm}",
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
    fn rust_isa_avx512bw_vpmovwb(a: *const u8, out: *mut u8);
}

/// `vpmovwb`
/// Truncate the 32 16-bit lanes to bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovwb(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512bw_vpmovwb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovswb(a: *const u8, out: *mut u8);
}

/// `vpmovswb`
/// Pack the 32 signed 16-bit lanes to signed bytes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovswb(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512bw_vpmovswb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovuswb(a: *const u8, out: *mut u8);
}

/// `vpmovuswb`
/// Pack the 32 unsigned 16-bit lanes to unsigned bytes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovuswb(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512bw_vpmovuswb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovdb(a: *const u8, out: *mut u8);
}

/// `vpmovdb`
/// Truncate the 16 32-bit lanes to bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovdb(a: V512) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512bw_vpmovdb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovsdb(a: *const u8, out: *mut u8);
}

/// `vpmovsdb`
/// Pack the 16 signed 32-bit lanes to signed bytes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovsdb(a: V512) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512bw_vpmovsdb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovusdb(a: *const u8, out: *mut u8);
}

/// `vpmovusdb`
/// Pack the 16 unsigned 32-bit lanes to unsigned bytes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovusdb(a: V512) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512bw_vpmovusdb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovdw(a: *const u8, out: *mut u8);
}

/// `vpmovdw`
/// Truncate the 16 32-bit lanes to 16-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovdw(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512bw_vpmovdw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovsdw(a: *const u8, out: *mut u8);
}

/// `vpmovsdw`
/// Pack the 16 signed 32-bit lanes to signed 16-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovsdw(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512bw_vpmovsdw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovusdw(a: *const u8, out: *mut u8);
}

/// `vpmovusdw`
/// Pack the 16 unsigned 32-bit lanes to unsigned 16-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovusdw(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512bw_vpmovusdw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovqw(a: *const u8, out: *mut u8);
}

/// `vpmovqw`
/// Truncate the eight 64-bit lanes to 16-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovqw(a: V512) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512bw_vpmovqw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovsqw(a: *const u8, out: *mut u8);
}

/// `vpmovsqw`
/// Pack the eight signed 64-bit lanes to signed 16-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovsqw(a: V512) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512bw_vpmovsqw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovusqw(a: *const u8, out: *mut u8);
}

/// `vpmovusqw`
/// Pack the eight unsigned 64-bit lanes to unsigned 16-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovusqw(a: V512) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512bw_vpmovusqw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovqb(a: *const u8, out: *mut u8);
}

/// `vpmovqb`
/// Truncate the eight 64-bit lanes to bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovqb(a: V512) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512bw_vpmovqb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovsqb(a: *const u8, out: *mut u8);
}

/// `vpmovsqb`
/// Pack the eight signed 64-bit lanes to signed bytes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovsqb(a: V512) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512bw_vpmovsqb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovusqb(a: *const u8, out: *mut u8);
}

/// `vpmovusqb`
/// Pack the eight unsigned 64-bit lanes to unsigned bytes with saturation.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovusqb(a: V512) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx512bw_vpmovusqb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpcmpb`
/// Compare the 64 signed byte lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
#[cfg(target_arch = "x86_64")]
pub unsafe fn vpcmpb<const IMM: u8>(a: V512, b: V512) -> u64 {
    let out: u64;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpb k1, zmm0, zmm1, {imm}",
            "kmovq {out:r}, k1",
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
    out
}

/// `vpcmpw`
/// Compare the 32 signed 16-bit lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpcmpw<const IMM: u8>(a: V512, b: V512) -> u32 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpw k1, zmm0, zmm1, {imm}",
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

/// `vpcmpub`
/// Compare the 64 unsigned byte lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
#[cfg(target_arch = "x86_64")]
pub unsafe fn vpcmpub<const IMM: u8>(a: V512, b: V512) -> u64 {
    let out: u64;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpub k1, zmm0, zmm1, {imm}",
            "kmovq {out:r}, k1",
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
    out
}

/// `vpcmpuw`
/// Compare the 32 unsigned 16-bit lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpcmpuw<const IMM: u8>(a: V512, b: V512) -> u32 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpuw k1, zmm0, zmm1, {imm}",
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
    fn rust_isa_avx512bw_vptestmb(a: *const u8, b: *const u8) -> u64;
}

/// `vptestmb`
/// Test the bitwise AND of the byte lanes and return the nonzero-lane mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vptestmb(a: V512, b: V512) -> u64 {
    unsafe { rust_isa_avx512bw_vptestmb(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vptestmw(a: *const u8, b: *const u8) -> u32;
}

/// `vptestmw`
/// Test the bitwise AND of the 16-bit lanes and return the nonzero-lane mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vptestmw(a: V512, b: V512) -> u32 {
    unsafe { rust_isa_avx512bw_vptestmw(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vptestnmb(a: *const u8, b: *const u8) -> u64;
}

/// `vptestnmb`
/// Test the bitwise AND of the byte lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vptestnmb(a: V512, b: V512) -> u64 {
    unsafe { rust_isa_avx512bw_vptestnmb(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vptestnmw(a: *const u8, b: *const u8) -> u32;
}

/// `vptestnmw`
/// Test the bitwise AND of the 16-bit lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vptestnmw(a: V512, b: V512) -> u32 {
    unsafe { rust_isa_avx512bw_vptestnmw(a.as_ptr(), b.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovm2b(mask: u64, out: *mut u8);
}

/// `vpmovm2b`
/// Expand the 64 mask bits to byte lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovm2b(mask: u64) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpmovm2b(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovm2w(mask: u32, out: *mut u8);
}

/// `vpmovm2w`
/// Expand the 32 mask bits to 16-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovm2w(mask: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpmovm2w(mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovb2m(a: *const u8) -> u64;
}

/// `vpmovb2m`
/// Collect the sign bits of the 64 byte lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovb2m(a: V512) -> u64 {
    unsafe { rust_isa_avx512bw_vpmovb2m(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpmovw2m(a: *const u8) -> u32;
}

/// `vpmovw2m`
/// Collect the sign bits of the 32 16-bit lanes into a mask.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpmovw2m(a: V512) -> u32 {
    unsafe { rust_isa_avx512bw_vpmovw2m(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpbroadcastb(value: u8, out: *mut u8);
}

/// `vpbroadcastb`
/// Broadcast a byte to the 64 byte lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpbroadcastb(value: u8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpbroadcastb(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512bw_vpbroadcastw(value: u16, out: *mut u8);
}

/// `vpbroadcastw`
/// Broadcast a 16-bit integer to the 32 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512BW.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[inline]
pub unsafe fn vpbroadcastw(value: u16) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512bw_vpbroadcastw(value, out.as_mut_ptr());
    }
    out
}
