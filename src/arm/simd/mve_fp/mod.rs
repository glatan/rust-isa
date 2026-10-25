//! 32-bit ARM MVE floating point.
//!
//! MVE-FP is an M-profile extension and is not available on
//! `armv7-unknown-linux-gnueabihf`. Every instruction wrapper is `unsafe` and the caller
//! must run on a CPU with MVE-FP. The C side is compiled with clang and
//! `-march=armv8.1-m.main+mve.fp+fp.dp`.
//!
//! The values are [`V128`] of little-endian binary32 lanes. Results follow
//! the FPSCR rounding mode and the default NaN behaviour of the target.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::arm::simd::mve_fp::add;
//! use rust_isa::arm::simd::neon::V128;
//!
//! let one = V128::from_bytes([
//!     0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f,
//!     0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f,
//! ]);
//! let two = V128::from_bytes([
//!     0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40,
//!     0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40,
//! ]);
//! let sum = unsafe { add(one, two) };
//! assert_eq!(
//!     sum.to_bytes(),
//!     [
//!         0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x40, 0x40,
//!         0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x40, 0x40,
//!     ]
//! );
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::arm::simd::mve_fp::*;
//! use rust_isa::arm::simd::neon::V128;
//! use rust_isa::arm::simd::neon::V64;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { abs(v128) };
//! let _ = unsafe { add(v128, v128) };
//! let _ = unsafe { cmp(v128, v128) };
//! let _ = unsafe { cvt(v128) };
//! let _ = unsafe { cvt_f32(v128) };
//! let _ = unsafe { div(v128, v128) };
//! let _ = unsafe { fma(v128, v128, v128) };
//! let _ = unsafe { fms(v128, v128, v128) };
//! let _ = unsafe { max(v128, v128) };
//! let _ = unsafe { min(v128, v128) };
//! let _ = unsafe { mul(v128, v128) };
//! let _ = unsafe { neg(v128) };
//! let _ = unsafe { sqrt(v128) };
//! let _ = unsafe { sub(v128, v128) };
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::arm::simd::mve_fp::*;
//! use rust_isa::arm::simd::neon::V128;
//! use rust_isa::arm::simd::neon::V64;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { abs(v128) };
//! let _ = unsafe { add(v128, v128) };
//! let _ = unsafe { cmp(v128, v128) };
//! let _ = unsafe { cvt(v128) };
//! let _ = unsafe { cvt_f32(v128) };
//! let _ = unsafe { div(v128, v128) };
//! let _ = unsafe { fma(v128, v128, v128) };
//! let _ = unsafe { fms(v128, v128, v128) };
//! let _ = unsafe { max(v128, v128) };
//! let _ = unsafe { min(v128, v128) };
//! let _ = unsafe { mul(v128, v128) };
//! let _ = unsafe { neg(v128) };
//! let _ = unsafe { sqrt(v128) };
//! let _ = unsafe { sub(v128, v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::arm::simd::mve_fp::abs as _;
//! use rust_isa::arm::simd::mve_fp::add as _;
//! use rust_isa::arm::simd::mve_fp::cmp as _;
//! use rust_isa::arm::simd::mve_fp::cvt as _;
//! use rust_isa::arm::simd::mve_fp::cvt_f32 as _;
//! use rust_isa::arm::simd::mve_fp::div as _;
//! use rust_isa::arm::simd::mve_fp::fma as _;
//! use rust_isa::arm::simd::mve_fp::fms as _;
//! use rust_isa::arm::simd::mve_fp::max as _;
//! use rust_isa::arm::simd::mve_fp::min as _;
//! use rust_isa::arm::simd::mve_fp::mul as _;
//! use rust_isa::arm::simd::mve_fp::neg as _;
//! use rust_isa::arm::simd::mve_fp::sqrt as _;
//! use rust_isa::arm::simd::mve_fp::sub as _;
//! ```
//! <!-- doctest-inventory:end -->

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
