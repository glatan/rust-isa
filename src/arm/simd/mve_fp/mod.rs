//! 32-bit ARM MVE floating point.
//!
//! MVE-FP is an M-profile extension and is not available on
//! `armv7-unknown-linux-gnueabihf`. Every operation is `unsafe` and the caller
//! must run on a CPU with MVE-FP. The C side is compiled with clang and
//! `-march=armv8.1-m.main+mve.fp+fp.dp`.
//!
//! The values are [`V128`] of little-endian binary32 lanes. Results follow
//! the FPSCR rounding mode and the default NaN behaviour of the target.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! Instructions not listed in this module are not exposed.

use super::neon::V128;

unsafe extern "C" {
    fn rust_isa_arm_mve_fp_abs(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_cmp(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_cvt(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_cvt_f32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_div(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_fma(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_fms(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_max(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_min(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_mul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_neg(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_sqrt(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_fp_sub(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable MVE and MVE-FP.
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
        ///
        /// # Safety
        ///
        /// The caller must enable MVE and MVE-FP.
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
        ///
        /// # Safety
        ///
        /// The caller must enable MVE and MVE-FP.
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

unary!(
    /// `vabs.f32`.
    abs,
    rust_isa_arm_mve_fp_abs
);
binary!(
    /// `vadd.f32`.
    add,
    rust_isa_arm_mve_fp_add
);

/// `vcmpeq.f32`
///
/// Compare binary32 lanes for equality. A true lane is all bits one and a
/// false lane is zero. A NaN lane compares false.
///
/// # Safety
///
/// The caller must enable MVE-FP.
#[inline(always)]
pub unsafe fn cmp(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_fp_cmp(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvt.s32.f32`
///
/// Convert binary32 lanes to signed 32-bit integers, rounding toward zero.
///
/// # Safety
///
/// The caller must enable MVE-FP.
#[inline(always)]
pub unsafe fn cvt(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_fp_cvt(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvt.f32.s32`
///
/// Convert signed 32-bit integers to binary32 lanes.
///
/// # Safety
///
/// The caller must enable MVE-FP.
#[inline(always)]
pub unsafe fn cvt_f32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_fp_cvt_f32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

binary!(
    /// `vdiv.f32`.
    div,
    rust_isa_arm_mve_fp_div
);
ternary!(
    /// `vfma.f32`.
    ///
    /// Multiply `a` and `b`, add the products to `acc`, and round once.
    fma,
    rust_isa_arm_mve_fp_fma
);
ternary!(
    /// `vfms.f32`.
    ///
    /// Multiply `a` and `b`, subtract the products from `acc`, and round once.
    fms,
    rust_isa_arm_mve_fp_fms
);
binary!(
    /// `vmaxnm.f32`.
    ///
    /// Take the maximum number of each lane. A quiet NaN operand is ignored.
    max,
    rust_isa_arm_mve_fp_max
);
binary!(
    /// `vminnm.f32`.
    ///
    /// Take the minimum number of each lane. A quiet NaN operand is ignored.
    min,
    rust_isa_arm_mve_fp_min
);
binary!(
    /// `vmul.f32`.
    mul,
    rust_isa_arm_mve_fp_mul
);
unary!(
    /// `vneg.f32`.
    neg,
    rust_isa_arm_mve_fp_neg
);
unary!(
    /// `vsqrt.f32`.
    ///
    /// Take the square root of each lane.
    sqrt,
    rust_isa_arm_mve_fp_sqrt
);
binary!(
    /// `vsub.f32`.
    sub,
    rust_isa_arm_mve_fp_sub
);
