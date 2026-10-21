use super::V256;

unsafe extern "C" {
    fn rust_isa_avx2_vpacksswb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpackssdw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpackuswb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpackusdw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpunpcklbw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpunpckhbw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpunpcklwd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpunpckhwd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpunpckldq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpunpckhdq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpunpcklqdq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpunpckhqdq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmovmskb(a: *const u8) -> u32;
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

/// `vpunpcklbw`
///
/// Interleave the low 8 bytes of each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpunpcklbw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpunpcklbw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpunpckhbw`
///
/// Interleave the high 8 bytes of each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpunpckhbw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpunpckhbw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpunpcklwd`
///
/// Interleave the low 4 16-bit lanes of each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpunpcklwd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpunpcklwd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpunpckhwd`
///
/// Interleave the high 4 16-bit lanes of each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpunpckhwd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpunpckhwd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpunpckldq`
///
/// Interleave the low 2 32-bit lanes of each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpunpckldq(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpunpckldq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpunpckhdq`
///
/// Interleave the high 2 32-bit lanes of each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpunpckhdq(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpunpckhdq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpunpcklqdq`
///
/// Interleave the low 64-bit lane of each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpunpcklqdq(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpunpcklqdq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpunpckhqdq`
///
/// Interleave the high 64-bit lane of each 128-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpunpckhqdq(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpunpckhqdq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmovmskb`
///
/// Extract the high bit of each byte into a 32-bit mask.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmovmskb(a: V256) -> u32 {
    unsafe { rust_isa_avx2_vpmovmskb(a.as_ptr()) }
}
