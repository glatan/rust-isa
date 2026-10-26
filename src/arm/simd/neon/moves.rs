//! NEON move and swap instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_vmov_u8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vswp_u8(a: *const u8, b: *const u8, o0: *mut u8, o1: *mut u8);
}

/// `vmov`. Copy a 128-bit value.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vmov(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vmov_u8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vswp`. Swap two 128-bit values.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vswp(a: V128, b: V128) -> (V128, V128) {
    let mut out_a = V128::from_bytes([0; 16]);
    let mut out_b = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vswp_u8(
            a.as_ptr(),
            b.as_ptr(),
            out_a.as_mut_ptr(),
            out_b.as_mut_ptr(),
        );
    }
    (out_a, out_b)
}
