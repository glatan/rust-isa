use super::V256;
use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_avx2_vpgatherdd(
        src: *const u8,
        base: *const u8,
        index: *const u8,
        mask: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avx2_vpgatherqd(
        src: *const u8,
        base: *const u8,
        index: *const u8,
        mask: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avx2_vpgatherdq(
        src: *const u8,
        base: *const u8,
        index: *const u8,
        mask: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avx2_vpgatherqq(
        src: *const u8,
        base: *const u8,
        index: *const u8,
        mask: *const u8,
        out: *mut u8,
    );
    fn rust_isa_avx2_vpmaskmovd_load(p: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmaskmovq_load(p: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmaskmovd_store(p: *mut u8, mask: *const u8, value: *const u8);
    fn rust_isa_avx2_vpmaskmovq_store(p: *mut u8, mask: *const u8, value: *const u8);
}

/// `vpgatherdd`
///
/// Gather eight 32-bit values using byte offsets. `index` holds the byte
/// offsets and `mask` selects the lanes with its sign bit. Unselected lanes
/// are copied from `src`.
///
/// # Safety
///
/// The caller must enable AVX2. Every selected address must be valid for
/// reading 4 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpgatherdd(src: V256, base: *const u8, index: V256, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpgatherdd(
            src.as_ptr(),
            base,
            index.as_ptr(),
            mask.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `vpgatherqd`
///
/// Gather four 32-bit values using 64-bit byte offsets. Unselected lanes are
/// copied from `src`.
///
/// # Safety
///
/// The caller must enable AVX2. Every selected address must be valid for
/// reading 4 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpgatherqd(src: V128, base: *const u8, index: V256, mask: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx2_vpgatherqd(
            src.as_ptr(),
            base,
            index.as_ptr(),
            mask.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `vpgatherdq`
///
/// Gather four 64-bit values using 32-bit byte offsets. Unselected lanes are
/// copied from `src`.
///
/// # Safety
///
/// The caller must enable AVX2. Every selected address must be valid for
/// reading 8 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpgatherdq(src: V256, base: *const u8, index: V128, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpgatherdq(
            src.as_ptr(),
            base,
            index.as_ptr(),
            mask.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `vpgatherqq`
///
/// Gather four 64-bit values using 64-bit byte offsets. Unselected lanes are
/// copied from `src`.
///
/// # Safety
///
/// The caller must enable AVX2. Every selected address must be valid for
/// reading 8 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpgatherqq(src: V256, base: *const u8, index: V256, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpgatherqq(
            src.as_ptr(),
            base,
            index.as_ptr(),
            mask.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `vpmaskmovd` load form
///
/// Load the selected 32-bit lanes. Unselected lanes are zeroed.
///
/// # Safety
///
/// The caller must enable AVX2 and `p` must be valid for reading 32 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaskmovd_load(p: *const u8, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmaskmovd_load(p, mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmaskmovd` store form
///
/// Store the selected 32-bit lanes. Unselected lanes are not written.
///
/// # Safety
///
/// The caller must enable AVX2 and `p` must be valid for writing 32 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaskmovd_store(p: *mut u8, mask: V256, value: V256) {
    unsafe {
        rust_isa_avx2_vpmaskmovd_store(p, mask.as_ptr(), value.as_ptr());
    }
}

/// `vpmaskmovq` load form
///
/// Load the selected 64-bit lanes. Unselected lanes are zeroed.
///
/// # Safety
///
/// The caller must enable AVX2 and `p` must be valid for reading 32 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaskmovq_load(p: *const u8, mask: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmaskmovq_load(p, mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmaskmovq` store form
///
/// Store the selected 64-bit lanes. Unselected lanes are not written.
///
/// # Safety
///
/// The caller must enable AVX2 and `p` must be valid for writing 32 bytes.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaskmovq_store(p: *mut u8, mask: V256, value: V256) {
    unsafe {
        rust_isa_avx2_vpmaskmovq_store(p, mask.as_ptr(), value.as_ptr());
    }
}
