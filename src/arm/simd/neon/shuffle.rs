//! NEON table lookup instructions.

use super::{V64, V128};

unsafe extern "C" {
    fn rust_isa_arm_neon_vtbl_u8(table: *const u8, idx: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vtbx_u8(table: *const u8, idx: *const u8, src: *const u8, out: *mut u8);
}

/// `vtbl.8`. Look up 8 bytes in the 16-byte table `table` with `idx`. An index
/// outside `0..=15` produces zero.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vtbl(table: V128, idx: V64) -> V64 {
    let mut out = V64::from_bytes([0; 8]);
    unsafe {
        rust_isa_arm_neon_vtbl_u8(table.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vtbx.8`. Look up 8 bytes in the 16-byte table `table` with `idx`, keeping
/// the bytes of `src` where the index is outside `0..=15`.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vtbx(table: V128, idx: V64, src: V64) -> V64 {
    let mut out = V64::from_bytes([0; 8]);
    unsafe {
        rust_isa_arm_neon_vtbx_u8(table.as_ptr(), idx.as_ptr(), src.as_ptr(), out.as_mut_ptr());
    }
    out
}
