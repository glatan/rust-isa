//! Compare instructions.

use super::V256;
unsafe extern "C" {
    fn rust_isa_avx2_vpcmpeqb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpcmpeqw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpcmpeqd(a: *const u8, b: *const u8, out: *mut u8);
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
