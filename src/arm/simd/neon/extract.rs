//! NEON extract instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_vext_u8(a: *const u8, b: *const u8, n: i32, out: *mut u8);
}

/// `vext.8`. Extract 16 bytes starting at byte `N` of the concatenation of `a`
/// and `b`. `N` must be in `0..=15`.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vext<const N: u32>(a: V128, b: V128) -> V128 {
    const { assert!(N <= 15, "extract immediate must be in 0..=15") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vext_u8(a.as_ptr(), b.as_ptr(), N as i32, out.as_mut_ptr());
    }
    out
}
