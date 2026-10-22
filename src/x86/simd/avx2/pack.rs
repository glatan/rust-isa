use super::V256;

unsafe extern "C" {
    fn rust_isa_avx2_vpacksswb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpackssdw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpackuswb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpackusdw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpacksswb`
///
/// Pack 16-bit signed lanes into 8-bit signed lanes with saturation, within each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpacksswb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpacksswb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpackssdw`
///
/// Pack 32-bit signed lanes into 16-bit signed lanes with saturation, within each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpackssdw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpackssdw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpackuswb`
///
/// Pack 16-bit signed lanes into 8-bit unsigned lanes with saturation, within each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpackuswb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpackuswb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpackusdw`
///
/// Pack 32-bit signed lanes into 16-bit unsigned lanes with saturation, within each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpackusdw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpackusdw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
