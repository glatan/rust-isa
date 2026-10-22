//! NEON lane instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_splat_i8(value: i8, out: *mut u8);
    fn rust_isa_arm_neon_splat_i16(value: i16, out: *mut u8);
    fn rust_isa_arm_neon_splat_i32(value: i32, out: *mut u8);
}

/// Broadcast an 8-bit value to all lanes.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn splat_i8(value: i8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_splat_i8(value, out.as_mut_ptr());
    }
    out
}

/// Broadcast a 16-bit value to all lanes.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn splat_i16(value: i16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_splat_i16(value, out.as_mut_ptr());
    }
    out
}

/// Broadcast a 32-bit value to all lanes.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_splat_i32(value, out.as_mut_ptr());
    }
    out
}
