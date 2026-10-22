//! NEON extract instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_ext(a: *const u8, b: *const u8, index: i32, out: *mut u8);
}

/// `ext`
///
/// Extract 16 bytes starting at byte `INDEX` from the concatenation of `a` and `b`.
/// `INDEX` must be less than 16.
#[inline(always)]
pub fn ext<const INDEX: u8>(a: V128, b: V128) -> V128 {
    const { assert!(INDEX < 16, "NEON ext index must be less than 16") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ext(a.as_ptr(), b.as_ptr(), INDEX as i32, out.as_mut_ptr());
    }
    out
}
