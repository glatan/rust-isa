use super::V256;

unsafe extern "C" {
    fn rust_isa_avx2_vphaddw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vphaddd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vphaddsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vphsubw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vphsubd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vphsubsw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vphaddw`
///
/// Add adjacent signed 16-bit pairs within each 128-bit lane, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vphaddw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vphaddw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vphaddd`
///
/// Add adjacent signed 32-bit pairs within each 128-bit lane, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vphaddd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vphaddd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vphaddsw`
///
/// Add adjacent signed 16-bit pairs within each 128-bit lane with saturation.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vphaddsw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vphaddsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vphsubw`
///
/// Subtract adjacent signed 16-bit pairs within each 128-bit lane, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vphsubw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vphsubw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vphsubd`
///
/// Subtract adjacent signed 32-bit pairs within each 128-bit lane, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vphsubd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vphsubd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vphsubsw`
///
/// Subtract adjacent signed 16-bit pairs within each 128-bit lane with saturation.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vphsubsw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vphsubsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
