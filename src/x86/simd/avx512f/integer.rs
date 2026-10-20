//! Integer instructions.

use super::V512;

unsafe extern "C" {
    fn rust_isa_avx512f_vpaddd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddd`
///
/// Add the 16 32-bit integer lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpaddd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpaddd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpsubd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubd`
///
/// Subtract the 16 32-bit integer lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpsubd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpsubd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpmulld(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulld`
///
/// Multiply the 16 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpmulld(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpmulld(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpaddq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddq`
///
/// Add the eight 64-bit integer lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpaddq(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpaddq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpsubq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubq`
///
/// Subtract the eight 64-bit integer lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpsubq(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpsubq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpandd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandd`
///
/// Bitwise AND of the 64 bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpandd(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpandd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpord(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpord`
///
/// Bitwise OR of the 64 bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpord(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpord(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpxord(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpxord`
///
/// Bitwise XOR of the 64 bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpxord(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpxord(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx512f_vpermd(a: *const u8, idx: *const u8, out: *mut u8);
    fn rust_isa_avx512f_vpbroadcastd(value: u32, out: *mut u8);
    fn rust_isa_avx512f_vpslld(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_avx512f_vpsrld(a: *const u8, count: u32, out: *mut u8);
}

/// `vpermd`
///
/// Permute the 16 32-bit integer lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpermd(a: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpermd(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpbroadcastd`
///
/// Broadcast a 32-bit integer to the 16 lanes.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpbroadcastd(value: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpbroadcastd(value, out.as_mut_ptr());
    }
    out
}

/// `vpslld`
///
/// Shift the 16 32-bit integer lanes left by `count`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpslld(a: V512, count: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpslld(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `vpsrld`
///
/// Shift the 16 32-bit integer lanes right by `count`.
///
/// # Safety
///
/// The caller must enable AVX-512F.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn vpsrld(a: V512, count: u32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_vpsrld(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}
