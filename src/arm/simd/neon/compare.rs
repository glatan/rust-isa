//! NEON compare instructions.

use super::V128;
use super::raw::raw_binary;

unsafe extern "C" {
    fn rust_isa_arm_neon_cmpeq_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_cmpeq_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_cmpgt_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_cmpgt_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vceq_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcge_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcgt_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcle_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vclt_s8(a: *const u8, b: *const u8, out: *mut u8);
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
    /// `vceq.i8`. A true lane is all bits one.
    cmpeq_i8,
    rust_isa_arm_neon_cmpeq_i8
);

binary!(
    /// `vceq.i16`. A true lane is all bits one.
    cmpeq_i16,
    rust_isa_arm_neon_cmpeq_i16
);

binary!(
    /// `vceq.i32`. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_arm_neon_cmpeq_i32
);

binary!(
    /// `vcgt.s8`. A true lane is all bits one.
    cmpgt_i8,
    rust_isa_arm_neon_cmpgt_i8
);

binary!(
    /// `vcgt.s16`. A true lane is all bits one.
    cmpgt_i16,
    rust_isa_arm_neon_cmpgt_i16
);

binary!(
    /// `vcgt.s32`. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_arm_neon_cmpgt_i32
);

raw_binary!(
    /// `vceq.i8`. A true byte lane is all bits one.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vceq,
    rust_isa_arm_neon_vceq_s8
);

raw_binary!(
    /// `vcge.s8`. A true byte lane is all bits one.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcge,
    rust_isa_arm_neon_vcge_s8
);

raw_binary!(
    /// `vcgt.s8`. A true byte lane is all bits one.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcgt,
    rust_isa_arm_neon_vcgt_s8
);

raw_binary!(
    /// `vcle.s8`. A true byte lane is all bits one.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcle,
    rust_isa_arm_neon_vcle_s8
);

raw_binary!(
    /// `vclt.s8`. A true byte lane is all bits one.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vclt,
    rust_isa_arm_neon_vclt_s8
);
