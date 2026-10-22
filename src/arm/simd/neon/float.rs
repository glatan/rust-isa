//! NEON floating-point instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_fsub(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with NEON enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `vadd.f32`.
    fadd,
    rust_isa_arm_neon_fadd
);

binary!(
    /// `vmul.f32`.
    fmul,
    rust_isa_arm_neon_fmul
);

binary!(
    /// `vsub.f32`.
    fsub,
    rust_isa_arm_neon_fsub
);
