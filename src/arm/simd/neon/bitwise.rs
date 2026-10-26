//! NEON bitwise instructions.

use super::V128;
use super::raw::{raw_binary, raw_ternary, raw_unary};

unsafe extern "C" {
    fn rust_isa_arm_neon_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_orr(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_eor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vbic_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vbsl_u8(mask: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmvn_u8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vorn_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vtst_u8(a: *const u8, b: *const u8, out: *mut u8);
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
    /// Bitwise AND.
    and,
    rust_isa_arm_neon_and
);

binary!(
    /// Bitwise OR.
    orr,
    rust_isa_arm_neon_orr
);

binary!(
    /// Bitwise XOR.
    eor,
    rust_isa_arm_neon_eor
);

raw_binary!(
    /// `vbic.i8`. Clear the bits of `a` that are set in `b`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vbic,
    rust_isa_arm_neon_vbic_u8
);

raw_ternary!(
    /// `vbsl.i8`. Select the bits of `a` where `mask` is set and the bits of
    /// `b` elsewhere.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vbsl,
    rust_isa_arm_neon_vbsl_u8
);

raw_unary!(
    /// `vmvn.i8`. Invert every bit.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmvn,
    rust_isa_arm_neon_vmvn_u8
);

raw_binary!(
    /// `vorn.i8`. Bitwise OR of `a` with the inverse of `b`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vorn,
    rust_isa_arm_neon_vorn_u8
);

raw_binary!(
    /// `vtst.i8`. A byte lane is all bits one when the bitwise AND of the
    /// operands is nonzero.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vtst,
    rust_isa_arm_neon_vtst_u8
);
