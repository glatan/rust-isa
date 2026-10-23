use super::V256;

unsafe extern "C" {
    fn rust_isa_avx2_vpsllvd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsllvq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsrlvd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsrlvq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsravd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpslld(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_avx2_vpsrld(a: *const u8, count: u32, out: *mut u8);
}

/// `vpsllvd`
///
/// Shift the eight 32-bit lanes left by the per-lane counts in `b`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsllvd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsllvd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsllvq`
///
/// Shift the four 64-bit lanes left by the per-lane counts in `b`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsllvq(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsllvq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsrlvd`
///
/// Shift the eight 32-bit lanes right by the per-lane counts in `b`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsrlvd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsrlvd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsrlvq`
///
/// Shift the four 64-bit lanes right by the per-lane counts in `b`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsrlvq(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsrlvq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsravd`
///
/// Arithmetically shift the eight 32-bit lanes right by the per-lane counts in `b`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsravd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsravd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
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
