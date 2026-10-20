//! Integer instructions.

use super::V256;

unsafe extern "C" {
    fn rust_isa_avx2_vpaddb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddb`
///
/// Add the 32 byte lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpsubb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubb`
///
/// Subtract the 32 byte lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpaddw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddw`
///
/// Add the 16 16-bit integer lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpsubw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubw`
///
/// Subtract the 16 16-bit integer lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpaddd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddd`
///
/// Add the eight 32-bit integer lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpsubd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubd`
///
/// Subtract the eight 32-bit integer lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpaddq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddq`
///
/// Add the four 64-bit integer lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddq(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpsubq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubq`
///
/// Subtract the four 64-bit integer lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubq(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpmulld(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulld`
///
/// Multiply the eight 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmulld(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmulld(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpmullw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullw`
///
/// Multiply the 16 16-bit integer lanes, keeping the low 16 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmullw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmullw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpcmpeqb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpcmpeqb`
///
/// Compare the 32 byte lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpcmpeqb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpcmpeqb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpcmpeqw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpcmpeqw`
///
/// Compare the 16 16-bit integer lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpcmpeqw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpcmpeqw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpcmpeqd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpcmpeqd`
///
/// Compare the eight 32-bit integer lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpcmpeqd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpcmpeqd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpshufb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpshufb`
///
/// Shuffle the bytes of `a` with the indices in `b`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpshufb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpshufb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpbroadcastd(value: u32, out: *mut u8);
    fn rust_isa_avx2_vpslld(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_avx2_vpsrld(a: *const u8, count: u32, out: *mut u8);
}

/// `vpbroadcastd`
///
/// Broadcast a 32-bit integer to the eight lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpbroadcastd(value: u32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpbroadcastd(value, out.as_mut_ptr());
    }
    out
}

/// `vpslld`
///
/// Shift the eight 32-bit integer lanes left by `count`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpslld(a: V256, count: u32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpslld(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `vpsrld`
///
/// Shift the eight 32-bit integer lanes right by `count`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsrld(a: V256, count: u32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsrld(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx2_vpermd(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermd`
///
/// Permute the eight 32-bit integer lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpermd(a: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpermd(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}
