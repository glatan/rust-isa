//! NEON structure instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_vtrn_u8(a: *const u8, b: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_arm_neon_vuzp_u8(a: *const u8, b: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_arm_neon_vzip_u8(a: *const u8, b: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_arm_neon_vrev16_u8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrev32_u8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrev64_u8(a: *const u8, out: *mut u8);
}

/// `vtrn.8`. Transpose the byte pairs of two vectors.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vtrn(a: V128, b: V128) -> (V128, V128) {
    let mut out_a = V128::from_bytes([0; 16]);
    let mut out_b = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vtrn_u8(
            a.as_ptr(),
            b.as_ptr(),
            out_a.as_mut_ptr(),
            out_b.as_mut_ptr(),
        );
    }
    (out_a, out_b)
}

/// `vuzp.8`. Deinterleave the even and odd bytes of two vectors.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vuzp(a: V128, b: V128) -> (V128, V128) {
    let mut out_a = V128::from_bytes([0; 16]);
    let mut out_b = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vuzp_u8(
            a.as_ptr(),
            b.as_ptr(),
            out_a.as_mut_ptr(),
            out_b.as_mut_ptr(),
        );
    }
    (out_a, out_b)
}

/// `vzip.8`. Interleave the bytes of two vectors.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vzip(a: V128, b: V128) -> (V128, V128) {
    let mut out_a = V128::from_bytes([0; 16]);
    let mut out_b = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vzip_u8(
            a.as_ptr(),
            b.as_ptr(),
            out_a.as_mut_ptr(),
            out_b.as_mut_ptr(),
        );
    }
    (out_a, out_b)
}

/// `vrev16.8`. Reverse the byte pairs in each 16-bit element.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vrev16(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vrev16_u8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrev32.8`. Reverse the bytes in each 32-bit element.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vrev32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vrev32_u8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrev64.8`. Reverse the bytes in each 64-bit element.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vrev64(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vrev64_u8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
