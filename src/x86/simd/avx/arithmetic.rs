//! Floating-point arithmetic and bitwise instructions.

use super::V256;

unsafe extern "C" {
    fn rust_isa_avx_vaddsubps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vaddsubpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vandpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vandnps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vandnpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vxorpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vmaxps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vmaxpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vminps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vminpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vsqrtps(a: *const u8, out: *mut u8);
    fn rust_isa_avx_vsqrtpd(a: *const u8, out: *mut u8);
    fn rust_isa_avx_vrcpps(a: *const u8, out: *mut u8);
}

/// `vaddsubps`
///
/// Subtract even 32-bit float lanes and add odd lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vaddsubps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vaddsubps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddsubpd`
///
/// Subtract even 64-bit float lanes and add odd lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vaddsubpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vaddsubpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vandpd`
///
/// Bitwise AND of the 256-bit values.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vandpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vandpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vandnps`
///
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vandnps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vandnps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vandnpd`
///
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vandnpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vandnpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vxorpd`
///
/// Bitwise XOR of the 256-bit values.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vxorpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vxorpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaxps`
///
/// Take the maximum of each 32-bit float lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmaxps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vmaxps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaxpd`
///
/// Take the maximum of each 64-bit float lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmaxpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vmaxpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vminps`
///
/// Take the minimum of each 32-bit float lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vminps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vminps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vminpd`
///
/// Take the minimum of each 64-bit float lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vminpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vminpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsqrtps`
///
/// Take the square root of each 32-bit float lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vsqrtps(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vsqrtps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsqrtpd`
///
/// Take the square root of each 64-bit float lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vsqrtpd(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vsqrtpd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrcpps`
///
/// Approximate the reciprocal of each 32-bit float lane.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vrcpps(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vrcpps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
