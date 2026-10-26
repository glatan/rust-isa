//! NEON floating-point instructions.

use super::V128;
use super::raw::{raw_binary, raw_ternary, raw_unary};

unsafe extern "C" {
    fn rust_isa_arm_neon_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vfma_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vfms_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vfnma_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vfnms_f32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrecpe_f32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrecps_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrsqrte_f32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrsqrts_f32(a: *const u8, b: *const u8, out: *mut u8);
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

raw_ternary!(
    /// `vfma.f32`. Fused multiply-add: `acc + a * b`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON and VFPv4 enabled.
    vfma,
    rust_isa_arm_neon_vfma_f32
);

raw_ternary!(
    /// `vfms.f32`. Fused multiply-subtract: `acc - a * b`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON and VFPv4 enabled.
    vfms,
    rust_isa_arm_neon_vfms_f32
);

raw_ternary!(
    /// `vfnma.f32`. Fused negated multiply-add: `-acc + a * b`.
    ///
    /// AArch32 has no vector form of `VFNMA`, so this negates `acc` and uses
    /// `VFMA`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON and VFPv4 enabled.
    vfnma,
    rust_isa_arm_neon_vfnma_f32
);

raw_ternary!(
    /// `vfnms.f32`. Fused negated multiply-subtract: `-acc - a * b`.
    ///
    /// AArch32 has no vector form of `VFNMS`, so this negates `acc` and uses
    /// `VFMS`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON and VFPv4 enabled.
    vfnms,
    rust_isa_arm_neon_vfnms_f32
);

raw_unary!(
    /// `vrecpe.f32`. Reciprocal estimate of the 32-bit float lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrecpe,
    rust_isa_arm_neon_vrecpe_f32
);

raw_binary!(
    /// `vrecps.f32`. Reciprocal step: `2 - a * b`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrecps,
    rust_isa_arm_neon_vrecps_f32
);

raw_unary!(
    /// `vrsqrte.f32`. Reciprocal square root estimate of the 32-bit float
    /// lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrsqrte,
    rust_isa_arm_neon_vrsqrte_f32
);

raw_binary!(
    /// `vrsqrts.f32`. Reciprocal square root step: `(3 - a * b) / 2`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrsqrts,
    rust_isa_arm_neon_vrsqrts_f32
);
