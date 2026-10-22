//! NEON crypto instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_pmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_pmull_p8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_pmull2_p8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_pmull_p64(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

binary!(
    /// `pmul` on polynomial bytes.
    pmul,
    rust_isa_neon_pmul
);

binary!(
    /// `pmull` on polynomial bytes, producing 16-bit polynomial lanes.
    pmull_p8,
    rust_isa_neon_pmull_p8
);

binary!(
    /// `pmull2` on the high polynomial bytes, producing 16-bit polynomial lanes.
    pmull2_p8,
    rust_isa_neon_pmull2_p8
);

/// `pmull` on the low 64-bit polynomial lanes.
///
/// # Safety
///
/// The caller must enable the `aes` feature.
#[target_feature(enable = "aes")]
#[inline]
pub unsafe fn pmull_p64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_pmull_p64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
