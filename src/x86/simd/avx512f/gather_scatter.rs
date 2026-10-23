//! Gather and scatter instructions.

use crate::x86::simd::avx2::V256;
use crate::x86::simd::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_avx512f_vgatherdps(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u16,
        out: *mut u8,
    );
}

/// `vgatherdps`
/// Gather 32-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F.
///
/// Every selected address must be valid for reading 4 bytes.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vgatherdps(src: V512, base: *const u8, indices: V512, mask: u16) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vgatherdps(src.as_ptr(), base, indices.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vgatherdpd(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherdpd`
/// Gather 64-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F.
///
/// Every selected address must be valid for reading 8 bytes.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vgatherdpd(src: V512, base: *const u8, indices: V256, mask: u8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vgatherdpd(src.as_ptr(), base, indices.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vgatherqps(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherqps`
/// Gather 32-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F.
///
/// Every selected address must be valid for reading 4 bytes.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vgatherqps(src: V256, base: *const u8, indices: V512, mask: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx512f_vgatherqps(src.as_ptr(), base, indices.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vgatherqpd(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherqpd`
/// Gather 64-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F.
///
/// Every selected address must be valid for reading 8 bytes.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vgatherqpd(src: V512, base: *const u8, indices: V512, mask: u8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vgatherqpd(src.as_ptr(), base, indices.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vscatterdps(base: *mut u8, indices: *const u8, a: *const u8, mask: u16);
}

/// `vscatterdps`
/// Scatter the 32-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F.
///
/// Every selected address must be valid for writing 4 bytes.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vscatterdps(base: *mut u8, indices: V512, a: V512, mask: u16) {
    unsafe { rust_isa_avx512f_vscatterdps(base, indices.as_ptr(), a.as_ptr(), mask) }
}

unsafe extern "C" {
    fn rust_isa_avx512f_vscatterdpd(base: *mut u8, indices: *const u8, a: *const u8, mask: u8);
}

/// `vscatterdpd`
/// Scatter the 64-bit float lanes selected by `mask` with 32-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F.
///
/// Every selected address must be valid for writing 8 bytes.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vscatterdpd(base: *mut u8, indices: V256, a: V512, mask: u8) {
    unsafe { rust_isa_avx512f_vscatterdpd(base, indices.as_ptr(), a.as_ptr(), mask) }
}

unsafe extern "C" {
    fn rust_isa_avx512f_vscatterqps(base: *mut u8, indices: *const u8, a: *const u8, mask: u8);
}

/// `vscatterqps`
/// Scatter the 32-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F.
///
/// Every selected address must be valid for writing 4 bytes.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vscatterqps(base: *mut u8, indices: V512, a: V256, mask: u8) {
    unsafe { rust_isa_avx512f_vscatterqps(base, indices.as_ptr(), a.as_ptr(), mask) }
}

unsafe extern "C" {
    fn rust_isa_avx512f_vscatterqpd(base: *mut u8, indices: *const u8, a: *const u8, mask: u8);
}

/// `vscatterqpd`
/// Scatter the 64-bit float lanes selected by `mask` with 64-bit indices.
///
/// # Safety
///
/// The caller must enable AVX-512F.
///
/// Every selected address must be valid for writing 8 bytes.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vscatterqpd(base: *mut u8, indices: V512, a: V512, mask: u8) {
    unsafe { rust_isa_avx512f_vscatterqpd(base, indices.as_ptr(), a.as_ptr(), mask) }
}
