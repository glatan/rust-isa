//! NEON memory load instructions.
//!
//! The caller must compile the whole crate with `-C target-feature=+neon`.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_vld1_u8(p: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vld2_u8(p: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_arm_neon_vld3_u8(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8);
    fn rust_isa_arm_neon_vld4_u8(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8, o3: *mut u8);
}

/// `vld1.8`. Load 16 bytes.
///
/// # Safety
///
/// `p` must be valid for reading 16 bytes, and the caller must compile with
/// NEON enabled.
#[inline(always)]
pub unsafe fn vld1(p: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vld1_u8(p, out.as_mut_ptr());
    }
    out
}

/// `vld2.8`. Load 32 bytes and deinterleave them into two vectors.
///
/// # Safety
///
/// `p` must be valid for reading 32 bytes, and the caller must compile with
/// NEON enabled.
#[inline(always)]
pub unsafe fn vld2(p: *const u8) -> (V128, V128) {
    let mut a = V128::from_bytes([0; 16]);
    let mut b = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vld2_u8(p, a.as_mut_ptr(), b.as_mut_ptr());
    }
    (a, b)
}

/// `vld3.8`. Load 48 bytes and deinterleave them into three vectors.
///
/// # Safety
///
/// `p` must be valid for reading 48 bytes, and the caller must compile with
/// NEON enabled.
#[inline(always)]
pub unsafe fn vld3(p: *const u8) -> (V128, V128, V128) {
    let mut a = V128::from_bytes([0; 16]);
    let mut b = V128::from_bytes([0; 16]);
    let mut c = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vld3_u8(p, a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    }
    (a, b, c)
}

/// `vld4.8`. Load 64 bytes and deinterleave them into four vectors.
///
/// # Safety
///
/// `p` must be valid for reading 64 bytes, and the caller must compile with
/// NEON enabled.
#[inline(always)]
pub unsafe fn vld4(p: *const u8) -> (V128, V128, V128, V128) {
    let mut a = V128::from_bytes([0; 16]);
    let mut b = V128::from_bytes([0; 16]);
    let mut c = V128::from_bytes([0; 16]);
    let mut d = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vld4_u8(
            p,
            a.as_mut_ptr(),
            b.as_mut_ptr(),
            c.as_mut_ptr(),
            d.as_mut_ptr(),
        );
    }
    (a, b, c, d)
}
