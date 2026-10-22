//! NEON widening, narrowing, saturating, and shift operations.
//!
//! The AArch32 widening forms read the low 8 bytes of each narrowing source
//! operand. The narrowing forms write the low 8 bytes of the result and leave
//! the upper 8 bytes zero. Each function exposes one representative data width
//! of the instruction.
//!
//! The caller must compile the whole crate with `-C target-feature=+neon`.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_vabal_s8(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vabdl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vaddhn_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vaddl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vaddw_s8(acc: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmlal_s16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmlsl_s16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmovn_s16(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vmull_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vpadal_s16(acc: *const u8, a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqdmlal_s16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqdmlsl_s16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqdmull_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqmovn_s16(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqmovun_s16(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqshrn_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vqshrun_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vqshlu_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vqrshl_s16(a: *const u8, shift: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vqrshrn_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vqrshrun_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vraddhn_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrshr_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vrshrn_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vrsra_s16(acc: *const u8, a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vrsubhn_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vshll_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vshrn_s16(a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vsli_s16(a: *const u8, b: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vsra_s16(acc: *const u8, a: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vsri_s16(a: *const u8, b: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vsubhn_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vsubl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vsubw_s8(acc: *const u8, b: *const u8, out: *mut u8);
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

macro_rules! accumulate {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(acc: V128, a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(acc.as_ptr(), a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! multiply_accumulate {
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

macro_rules! shift_unary {
    ($(#[$meta:meta])* $name:ident, $c:ident, 0, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(a: V128) -> V128 {
            const { assert!(N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
    ($(#[$meta:meta])* $name:ident, $c:ident, $min:expr, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(a: V128) -> V128 {
            const { assert!(N >= $min && N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! shift_accumulate {
    ($(#[$meta:meta])* $name:ident, $c:ident, 0, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(acc: V128, a: V128) -> V128 {
            const { assert!(N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(acc.as_ptr(), a.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
    ($(#[$meta:meta])* $name:ident, $c:ident, $min:expr, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(acc: V128, a: V128) -> V128 {
            const { assert!(N >= $min && N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(acc.as_ptr(), a.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! shift_binary {
    ($(#[$meta:meta])* $name:ident, $c:ident, 0, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(a: V128, b: V128) -> V128 {
            const { assert!(N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
    ($(#[$meta:meta])* $name:ident, $c:ident, $min:expr, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(a: V128, b: V128) -> V128 {
            const { assert!(N >= $min && N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
}

multiply_accumulate!(
    /// `vabal.s8`. Absolute difference of the low 8 bytes, accumulated into
    /// 16-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vabal,
    rust_isa_arm_neon_vabal_s8
);
binary!(
    /// `vabdl.s8`. Absolute difference of the low 8 bytes, widened to 16 bits.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vabdl,
    rust_isa_arm_neon_vabdl_s8
);
binary!(
    /// `vaddhn.s16`. Add adjacent 16-bit lanes and keep the high byte of each
    /// result.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vaddhn,
    rust_isa_arm_neon_vaddhn_s16
);
binary!(
    /// `vaddl.s8`. Add the low 8 bytes, widened to 16 bits.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vaddl,
    rust_isa_arm_neon_vaddl_s8
);
accumulate!(
    /// `vaddw.s8`. Widen the low 8 bytes and add them to 16-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vaddw,
    rust_isa_arm_neon_vaddw_s8
);
multiply_accumulate!(
    /// `vmlal.s16`. Multiply the low four 16-bit lanes and accumulate into
    /// 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmlal,
    rust_isa_arm_neon_vmlal_s16
);
multiply_accumulate!(
    /// `vmlsl.s16`. Multiply the low four 16-bit lanes and subtract from
    /// 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmlsl,
    rust_isa_arm_neon_vmlsl_s16
);
unary!(
    /// `vmovn.s16`. Keep the low byte of each 16-bit lane.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmovn,
    rust_isa_arm_neon_vmovn_s16
);
binary!(
    /// `vmull.s16`. Multiply the low four 16-bit lanes, widened to 32 bits.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vmull,
    rust_isa_arm_neon_vmull_s16
);
accumulate!(
    /// `vpadal.s16`. Add adjacent 16-bit lane pairs and accumulate into 32-bit
    /// lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vpadal,
    rust_isa_arm_neon_vpadal_s16
);
multiply_accumulate!(
    /// `vqdmlal.s16`. Saturating doubling multiply of the low four 16-bit
    /// lanes, accumulated into 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqdmlal,
    rust_isa_arm_neon_vqdmlal_s16
);
multiply_accumulate!(
    /// `vqdmlsl.s16`. Saturating doubling multiply of the low four 16-bit
    /// lanes, subtracted from 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqdmlsl,
    rust_isa_arm_neon_vqdmlsl_s16
);
binary!(
    /// `vqdmull.s16`. Saturating doubling multiply of the low four 16-bit
    /// lanes, widened to 32 bits.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqdmull,
    rust_isa_arm_neon_vqdmull_s16
);
unary!(
    /// `vqmovn.s16`. Saturating narrow of each 16-bit lane to a signed byte.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqmovn,
    rust_isa_arm_neon_vqmovn_s16
);
unary!(
    /// `vqmovun.s16`. Saturating narrow of each 16-bit lane to an unsigned
    /// byte.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqmovun,
    rust_isa_arm_neon_vqmovun_s16
);
shift_unary!(
    /// `vqshrn.s16`. Saturating shift right by `N` and narrow each 16-bit lane
    /// to a signed byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqshrn,
    rust_isa_arm_neon_vqshrn_s16,
    1,
    8
);
shift_unary!(
    /// `vqshrun.s16`. Saturating shift right by `N` and narrow each 16-bit
    /// lane to an unsigned byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqshrun,
    rust_isa_arm_neon_vqshrun_s16,
    1,
    8
);
shift_unary!(
    /// `vqshlu.s16`. Saturating shift left by `N` into unsigned 16-bit lanes.
    /// `N` must be in `0..=15`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqshlu,
    rust_isa_arm_neon_vqshlu_s16,
    0,
    15
);
binary!(
    /// `vqrshl.s16`. Saturating rounding shift left by the signed lane counts
    /// in `shift`. A negative count shifts right.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqrshl,
    rust_isa_arm_neon_vqrshl_s16
);
shift_unary!(
    /// `vqrshrn.s16`. Saturating rounding shift right by `N` and narrow each
    /// 16-bit lane to a signed byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqrshrn,
    rust_isa_arm_neon_vqrshrn_s16,
    1,
    8
);
shift_unary!(
    /// `vqrshrun.s16`. Saturating rounding shift right by `N` and narrow each
    /// 16-bit lane to an unsigned byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vqrshrun,
    rust_isa_arm_neon_vqrshrun_s16,
    1,
    8
);
binary!(
    /// `vraddhn.s16`. Add adjacent 16-bit lanes with rounding and keep the high
    /// byte of each result.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vraddhn,
    rust_isa_arm_neon_vraddhn_s16
);
shift_unary!(
    /// `vrshr.s16`. Rounding shift right by `N`. `N` must be in `1..=16`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrshr,
    rust_isa_arm_neon_vrshr_s16,
    1,
    16
);
shift_unary!(
    /// `vrshrn.s16`. Rounding shift right by `N` and narrow each 16-bit lane to
    /// a signed byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrshrn,
    rust_isa_arm_neon_vrshrn_s16,
    1,
    8
);
shift_accumulate!(
    /// `vrsra.s16`. Rounding shift right by `N` and accumulate. `N` must be in
    /// `1..=16`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrsra,
    rust_isa_arm_neon_vrsra_s16,
    1,
    16
);
binary!(
    /// `vrsubhn.s16`. Subtract adjacent 16-bit lanes with rounding and keep the
    /// high byte of each result.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vrsubhn,
    rust_isa_arm_neon_vrsubhn_s16
);
shift_unary!(
    /// `vshll.s16`. Shift the low four 16-bit lanes left by `N` and widen to
    /// 32 bits. `N` must be in `0..=15`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vshll,
    rust_isa_arm_neon_vshll_s16,
    0,
    15
);
shift_unary!(
    /// `vshrn.s16`. Shift right by `N` and narrow each 16-bit lane to a signed
    /// byte. `N` must be in `1..=8`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vshrn,
    rust_isa_arm_neon_vshrn_s16,
    1,
    8
);
shift_binary!(
    /// `vsli.s16`. Shift `b` left by `N` and insert it into `a`. `N` must be in
    /// `0..=15`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsli,
    rust_isa_arm_neon_vsli_s16,
    0,
    15
);
shift_accumulate!(
    /// `vsra.s16`. Shift `a` right by `N` and accumulate into `acc`. `N` must
    /// be in `1..=16`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsra,
    rust_isa_arm_neon_vsra_s16,
    1,
    16
);
shift_binary!(
    /// `vsri.s16`. Shift `b` right by `N` and insert it into `a`. `N` must be
    /// in `1..=16`.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsri,
    rust_isa_arm_neon_vsri_s16,
    1,
    16
);
binary!(
    /// `vsubhn.s16`. Subtract adjacent 16-bit lanes and keep the high byte of
    /// each result.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsubhn,
    rust_isa_arm_neon_vsubhn_s16
);
binary!(
    /// `vsubl.s8`. Subtract the low 8 bytes, widened to 16 bits.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsubl,
    rust_isa_arm_neon_vsubl_s8
);
accumulate!(
    /// `vsubw.s8`. Widen the low 8 bytes and subtract them from 16-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must compile with NEON enabled.
    vsubw,
    rust_isa_arm_neon_vsubw_s8
);
