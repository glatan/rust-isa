//! NEON arithmetic instructions.
//!
//! Each function exposes one representative data width of the instruction.
//! The caller must compile the whole crate with `-C target-feature=+neon`.

use super::V128;
unsafe extern "C" {
    fn rust_isa_arm_neon_vaba_s8(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vabd_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vabs_s32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcls_s8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vclz_s8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcnt_s8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vcvt_s32_f32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vhadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vhsub_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmax_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmin_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmla_s32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmls_s32(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vneg_s32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqabs_s32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqdmulh_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqneg_s32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqrdmulh_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqsub_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
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

macro_rules! ternary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(acc: V128, a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

ternary!(
    /// `vaba.s8`. Absolute difference of `a` and `b`, accumulated into `acc`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vaba,
    rust_isa_arm_neon_vaba_s8
);
binary!(
    /// `vabd.s8`. Absolute difference of the signed byte lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vabd,
    rust_isa_arm_neon_vabd_s8
);
unary!(
    /// `vabs.s32`. Absolute value of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vabs,
    rust_isa_arm_neon_vabs_s32
);
unary!(
    /// `vcls.s8`. Count the leading sign bits of each byte lane.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcls,
    rust_isa_arm_neon_vcls_s8
);
unary!(
    /// `vclz.i8`. Count the leading zeros of each byte lane.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vclz,
    rust_isa_arm_neon_vclz_s8
);
unary!(
    /// `vcnt.8`. Count the set bits of each byte lane.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcnt,
    rust_isa_arm_neon_vcnt_s8
);
unary!(
    /// `vcvt.s32.f32`. Convert the 32-bit float lanes to signed integers,
    /// rounding toward zero.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vcvt,
    rust_isa_arm_neon_vcvt_s32_f32
);
binary!(
    /// `vhadd.s32`. Halving add of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vhadd,
    rust_isa_arm_neon_vhadd_s32
);
binary!(
    /// `vhsub.s32`. Halving subtract of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vhsub,
    rust_isa_arm_neon_vhsub_s32
);
binary!(
    /// `vmax.s32`. Maximum of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmax,
    rust_isa_arm_neon_vmax_s32
);
binary!(
    /// `vmin.s32`. Minimum of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmin,
    rust_isa_arm_neon_vmin_s32
);
ternary!(
    /// `vmla.s32`. Multiply `a` and `b` and accumulate into `acc`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmla,
    rust_isa_arm_neon_vmla_s32
);
ternary!(
    /// `vmls.s32`. Multiply `a` and `b` and subtract from `acc`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmls,
    rust_isa_arm_neon_vmls_s32
);
unary!(
    /// `vneg.s32`. Negate the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vneg,
    rust_isa_arm_neon_vneg_s32
);
unary!(
    /// `vqabs.s32`. Saturating absolute value of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqabs,
    rust_isa_arm_neon_vqabs_s32
);
binary!(
    /// `vqadd.s32`. Saturating add of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqadd,
    rust_isa_arm_neon_vqadd_s32
);
binary!(
    /// `vqdmulh.s16`. Doubling multiply of the signed 16-bit lanes, keeping the
    /// high half and saturating.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqdmulh,
    rust_isa_arm_neon_vqdmulh_s16
);
unary!(
    /// `vqneg.s32`. Saturating negate of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqneg,
    rust_isa_arm_neon_vqneg_s32
);
binary!(
    /// `vqrdmulh.s16`. Rounding doubling multiply of the signed 16-bit lanes,
    /// keeping the high half and saturating.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqrdmulh,
    rust_isa_arm_neon_vqrdmulh_s16
);
binary!(
    /// `vqsub.s32`. Saturating subtract of the signed 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqsub,
    rust_isa_arm_neon_vqsub_s32
);

macro_rules! binary_unsafe {
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

binary_unsafe!(
    /// `vadd.i8`.
    add_i8,
    rust_isa_arm_neon_add_i8
);

binary_unsafe!(
    /// `vadd.i16`.
    add_i16,
    rust_isa_arm_neon_add_i16
);

binary_unsafe!(
    /// `vadd.i64`.
    add_i64,
    rust_isa_arm_neon_add_i64
);

binary_unsafe!(
    /// `vsub.i8`.
    sub_i8,
    rust_isa_arm_neon_sub_i8
);

binary_unsafe!(
    /// `vsub.i16`.
    sub_i16,
    rust_isa_arm_neon_sub_i16
);

binary_unsafe!(
    /// `vsub.i32`.
    sub_i32,
    rust_isa_arm_neon_sub_i32
);

binary_unsafe!(
    /// `vsub.i64`.
    sub_i64,
    rust_isa_arm_neon_sub_i64
);

binary_unsafe!(
    /// `vmul.i16`.
    mul_i16,
    rust_isa_arm_neon_mul_i16
);

binary_unsafe!(
    /// `vmul.i32`.
    mul_i32,
    rust_isa_arm_neon_mul_i32
);
