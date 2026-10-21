//! Conversion and GPR move instructions.

use super::V128;
use super::V256;

unsafe extern "C" {
    fn rust_isa_avx_vcvtps2pd(a: *const u8, out: *mut u8);
    fn rust_isa_avx_vcvtpd2ps(a: *const u8, out: *mut u8);
    fn rust_isa_avx_vcvtdq2ps(a: *const u8, out: *mut u8);
    fn rust_isa_avx_vcvtps2dq(a: *const u8, out: *mut u8);
    fn rust_isa_avx_vcvttps2dq(a: *const u8, out: *mut u8);
    fn rust_isa_avx_vcvtss2sd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vcvtsd2ss(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vcvtps2pd`
///
/// Convert four 32-bit float lanes to four 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcvtps2pd(a: V128) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vcvtps2pd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtpd2ps`
///
/// Convert four 64-bit float lanes to four 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcvtpd2ps(a: V256) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx_vcvtpd2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtss2sd`
///
/// Convert the low 32-bit float lane of `b` to a 64-bit float and preserve the
/// upper lane of `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcvtss2sd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx_vcvtss2sd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtsd2ss`
///
/// Convert the low 64-bit float lane of `b` to a 32-bit float and preserve the
/// upper lanes of `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcvtsd2ss(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx_vcvtsd2ss(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtdq2ps`
///
/// Convert eight signed 32-bit integer lanes to float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcvtdq2ps(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vcvtdq2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtps2dq`
///
/// Convert eight float lanes to signed 32-bit integers using the current
/// rounding mode.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcvtps2dq(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vcvtps2dq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvttps2dq`
///
/// Convert eight float lanes to signed 32-bit integers with truncation.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vcvttps2dq(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vcvttps2dq(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn rust_isa_avx_vmovq(value: u64, out: *mut u8);
    fn rust_isa_avx_vmovq_to_u64(a: *const u8) -> u64;
}

/// `vmovq` from a general-purpose register
///
/// Move a 64-bit integer into the low lane and zero the upper lanes.
///
/// # Safety
///
/// The caller must enable AVX and target x86-64.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovq(value: u64) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx_vmovq(value, out.as_mut_ptr());
    }
    out
}

/// `vmovq` to a general-purpose register
///
/// Extract the low 64-bit lane.
///
/// # Safety
///
/// The caller must enable AVX and target x86-64.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovq_to_u64(a: V128) -> u64 {
    unsafe { rust_isa_avx_vmovq_to_u64(a.as_ptr()) }
}
