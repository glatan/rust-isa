//! 32-bit ARM FP16.
//!
//! FP16 is unstable on this target. Every operation is `unsafe` and the caller
//! must compile the whole crate with the FP16 feature enabled.
//!
//! The values are [`V128`] of IEEE 754 binary16 lanes. The result of arithmetic
//! is rounded according to the FPCR rounding mode.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).

use super::neon::V128;

unsafe extern "C" {
    fn rust_isa_arm_fp16_abs(a: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_cmp(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_cvt_f32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_cvt_high_f32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_cvt_f16(a: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_cvt_high_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_div(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_fma(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_fms(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_max(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_min(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_mul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_neg(a: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_sqrt(a: *const u8, out: *mut u8);
    fn rust_isa_arm_fp16_sub(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable FP16.
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
        /// The caller must enable FP16.
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
        /// The caller must enable FP16.
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
    /// `vabs.f16`.
    ///
    /// Take the absolute value of each binary16 lane.
    abs,
    rust_isa_arm_fp16_abs
);
binary!(
    /// `vadd.f16`.
    ///
    /// Add binary16 lanes.
    add,
    rust_isa_arm_fp16_add
);

/// `vcmpeq.f16`
///
/// Compare binary16 lanes for equality. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable FP16 on the CPU and in the target features.
#[inline(always)]
pub unsafe fn cmp(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_fp16_cmp(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unary!(
    /// `vcvt.f32.f16` on the low four lanes.
    ///
    /// Convert the low four binary16 lanes to binary32 lanes.
    cvt_f32,
    rust_isa_arm_fp16_cvt_f32
);
unary!(
    /// `vcvt.f32.f16` on the high four lanes.
    ///
    /// Convert the high four binary16 lanes to binary32 lanes.
    cvt_high_f32,
    rust_isa_arm_fp16_cvt_high_f32
);
unary!(
    /// `vcvt.f16.f32` on four lanes.
    ///
    /// Convert four binary32 lanes to the low four binary16 lanes. The other
    /// lanes are zero.
    cvt_f16,
    rust_isa_arm_fp16_cvt_f16
);

/// `vcvt.f16.f32` into the high four lanes.
///
/// Convert the four binary32 lanes in `b` and insert them into the high half
/// of `a`.
///
/// # Safety
///
/// The caller must enable FP16 on the CPU and in the target features.
#[inline(always)]
pub unsafe fn cvt_high_f16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_fp16_cvt_high_f16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

binary!(
    /// `vdiv.f16`.
    div,
    rust_isa_arm_fp16_div
);
ternary!(
    /// `vfma.f16`.
    ///
    /// Multiply `a` and `b`, add the products to `acc`, and round once.
    fma,
    rust_isa_arm_fp16_fma
);
ternary!(
    /// `vfms.f16`.
    ///
    /// Multiply `a` and `b`, subtract the products from `acc`, and round once.
    fms,
    rust_isa_arm_fp16_fms
);
binary!(
    /// `vmax.f16`.
    max,
    rust_isa_arm_fp16_max
);
binary!(
    /// `vmin.f16`.
    min,
    rust_isa_arm_fp16_min
);
binary!(
    /// `vmul.f16`.
    mul,
    rust_isa_arm_fp16_mul
);
unary!(
    /// `vneg.f16`.
    neg,
    rust_isa_arm_fp16_neg
);
unary!(
    /// `vsqrt.f16`.
    sqrt,
    rust_isa_arm_fp16_sqrt
);
binary!(
    /// `vsub.f16`.
    sub,
    rust_isa_arm_fp16_sub
);
