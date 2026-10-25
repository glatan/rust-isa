//! 32-bit ARM MVE.
//!
//! MVE is an M-profile extension and is not available on
//! `armv7-unknown-linux-gnueabihf`. Every instruction wrapper is `unsafe` and the caller
//! must run on a CPU with MVE. The C side is compiled with clang and
//! `-march=armv8.1-m.main+mve`.
//!
//! The values are [`V128`] of little-endian lanes. Unless stated otherwise,
//! the integer operations use signed 32-bit lanes. The reduction operations
//! return scalars.
//!
//! The MVE predicate is a 16-bit value. [`ctp`] returns the raw predicate
//! register value; [`cmp`] returns a vector mask with all bits one or zero per
//! lane.
//!
//! [`fma`] and [`fms`] are the MVE-FP forms and require MVE-FP.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::arm::simd::mve::add;
//! use rust_isa::arm::simd::neon::V128;
//!
//! let a = V128::from_bytes([1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
//! let b = V128::from_bytes([10, 0, 0, 0, 20, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0]);
//! let sum = unsafe { add(a, b) };
//! assert_eq!(
//!     sum.to_bytes(),
//!     [11, 0, 0, 0, 22, 0, 0, 0, 33, 0, 0, 0, 44, 0, 0, 0]
//! );
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::arm::simd::mve::*;
//! use rust_isa::arm::simd::neon::V128;
//! use rust_isa::arm::simd::neon::V64;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { abav(0, v128, v128) };
//! let _ = unsafe { abd(v128, v128) };
//! let _ = unsafe { abs(v128) };
//! let _ = unsafe { add(v128, v128) };
//! let _ = unsafe { addlv(v128) };
//! let _ = unsafe { addv(v128) };
//! let _ = unsafe { and(v128, v128) };
//! let _ = unsafe { bic(v128, v128) };
//! let _ = unsafe { brsr(v128, 0) };
//! let _ = unsafe { clz(v128) };
//! let _ = unsafe { cmp(v128, v128) };
//! let _ = unsafe { ctp(0) };
//! let _ = unsafe { dup(0) };
//! let _ = unsafe { eor(v128, v128) };
//! let _ = unsafe { fma(v128, v128, v128) };
//! let _ = unsafe { fms(v128, v128, v128) };
//! let _ = unsafe { hadd(v128, v128) };
//! let _ = unsafe { hsub(v128, v128) };
//! let _ = unsafe { idup(0, 0) };
//! let _ = unsafe { iwdup(0, 0, 0) };
//! let _ = unsafe { ldr(storage.0.as_ptr().cast()) };
//! let _ = unsafe { ldrb(storage.0.as_ptr().cast()) };
//! let _ = unsafe { max(v128, v128) };
//! let _ = unsafe { maxv(v128) };
//! let _ = unsafe { min(v128, v128) };
//! let _ = unsafe { minv(v128) };
//! let _ = unsafe { mla(v128, v128, v128) };
//! let _ = unsafe { mlal(v128, v128, v128) };
//! let _ = unsafe { mls(v128, v128, v128) };
//! let _ = unsafe { mlsl(v128, v128, v128) };
//! let _ = unsafe { mov(v128) };
//! let _ = unsafe { mul(v128, v128) };
//! let _ = unsafe { mull(v128, v128) };
//! let _ = unsafe { mvn(v128) };
//! let _ = unsafe { neg(v128) };
//! let _ = unsafe { orn(v128, v128) };
//! let _ = unsafe { orr(v128, v128) };
//! let _ = unsafe { qabs(v128) };
//! let _ = unsafe { qadd(v128, v128) };
//! let _ = unsafe { qdmulh(v128, v128) };
//! let _ = unsafe { qdmull(v128, v128) };
//! let _ = unsafe { qmovn(v128) };
//! let _ = unsafe { qneg(v128) };
//! let _ = unsafe { qrdmulh(v128, v128) };
//! let _ = unsafe { qshl(v128, 0) };
//! let _ = unsafe { qsub(v128, v128) };
//! let _ = unsafe { rev16(v128) };
//! let _ = unsafe { rev32(v128) };
//! let _ = unsafe { rev64(v128) };
//! let _ = unsafe { rmulh(v128, v128) };
//! let _ = unsafe { shl(v128, 0) };
//! let _ = unsafe { shr(v128, 0) };
//! let _ = unsafe { sli(v128, v128, 0) };
//! let _ = unsafe { sri(v128, v128, 0) };
//! let _ = unsafe { str(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = unsafe { strb(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = unsafe { sub(v128, v128) };
//! let _ = unsafe { subn(v128, v128) };
//! let _ = unsafe { subw(v128, v128) };
//! let _ = unsafe { uzp(v128, v128) };
//! let _ = unsafe { zip(v128, v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::arm::simd::mve::abav as _;
//! use rust_isa::arm::simd::mve::abd as _;
//! use rust_isa::arm::simd::mve::abs as _;
//! use rust_isa::arm::simd::mve::add as _;
//! use rust_isa::arm::simd::mve::addlv as _;
//! use rust_isa::arm::simd::mve::addv as _;
//! use rust_isa::arm::simd::mve::and as _;
//! use rust_isa::arm::simd::mve::bic as _;
//! use rust_isa::arm::simd::mve::brsr as _;
//! use rust_isa::arm::simd::mve::clz as _;
//! use rust_isa::arm::simd::mve::cmp as _;
//! use rust_isa::arm::simd::mve::ctp as _;
//! use rust_isa::arm::simd::mve::dup as _;
//! use rust_isa::arm::simd::mve::eor as _;
//! use rust_isa::arm::simd::mve::fma as _;
//! use rust_isa::arm::simd::mve::fms as _;
//! use rust_isa::arm::simd::mve::hadd as _;
//! use rust_isa::arm::simd::mve::hsub as _;
//! use rust_isa::arm::simd::mve::idup as _;
//! use rust_isa::arm::simd::mve::iwdup as _;
//! use rust_isa::arm::simd::mve::ldr as _;
//! use rust_isa::arm::simd::mve::ldrb as _;
//! use rust_isa::arm::simd::mve::max as _;
//! use rust_isa::arm::simd::mve::maxv as _;
//! use rust_isa::arm::simd::mve::min as _;
//! use rust_isa::arm::simd::mve::minv as _;
//! use rust_isa::arm::simd::mve::mla as _;
//! use rust_isa::arm::simd::mve::mlal as _;
//! use rust_isa::arm::simd::mve::mls as _;
//! use rust_isa::arm::simd::mve::mlsl as _;
//! use rust_isa::arm::simd::mve::mov as _;
//! use rust_isa::arm::simd::mve::mul as _;
//! use rust_isa::arm::simd::mve::mull as _;
//! use rust_isa::arm::simd::mve::mvn as _;
//! use rust_isa::arm::simd::mve::neg as _;
//! use rust_isa::arm::simd::mve::orn as _;
//! use rust_isa::arm::simd::mve::orr as _;
//! use rust_isa::arm::simd::mve::qabs as _;
//! use rust_isa::arm::simd::mve::qadd as _;
//! use rust_isa::arm::simd::mve::qdmulh as _;
//! use rust_isa::arm::simd::mve::qdmull as _;
//! use rust_isa::arm::simd::mve::qmovn as _;
//! use rust_isa::arm::simd::mve::qneg as _;
//! use rust_isa::arm::simd::mve::qrdmulh as _;
//! use rust_isa::arm::simd::mve::qshl as _;
//! use rust_isa::arm::simd::mve::qsub as _;
//! use rust_isa::arm::simd::mve::rev16 as _;
//! use rust_isa::arm::simd::mve::rev32 as _;
//! use rust_isa::arm::simd::mve::rev64 as _;
//! use rust_isa::arm::simd::mve::rmulh as _;
//! use rust_isa::arm::simd::mve::shl as _;
//! use rust_isa::arm::simd::mve::shr as _;
//! use rust_isa::arm::simd::mve::sli as _;
//! use rust_isa::arm::simd::mve::sri as _;
//! use rust_isa::arm::simd::mve::str as _;
//! use rust_isa::arm::simd::mve::strb as _;
//! use rust_isa::arm::simd::mve::sub as _;
//! use rust_isa::arm::simd::mve::subn as _;
//! use rust_isa::arm::simd::mve::subw as _;
//! use rust_isa::arm::simd::mve::uzp as _;
//! use rust_isa::arm::simd::mve::zip as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::neon::V128;

unsafe extern "C" {
    fn rust_isa_arm_mve_ldr(p: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_ldrb(p: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_str(p: *mut u8, value: *const u8);
    fn rust_isa_arm_mve_strb(p: *mut u8, value: *const u8);
    fn rust_isa_arm_mve_abs(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_addlv(a: *const u8) -> i64;
    fn rust_isa_arm_mve_addv(a: *const u8) -> i32;
    fn rust_isa_arm_mve_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_abd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_abav(acc: u32, a: *const u8, b: *const u8) -> u32;
    fn rust_isa_arm_mve_bic(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_brsr(a: *const u8, bits: u32, out: *mut u8);
    fn rust_isa_arm_mve_clz(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_cmp(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_ctp(count: u32) -> u16;
    fn rust_isa_arm_mve_dup(value: i32, out: *mut u8);
    fn rust_isa_arm_mve_eor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_hadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_hsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_idup(base: u32, step: u32, out: *mut u8);
    fn rust_isa_arm_mve_iwdup(base: u32, wrap: u32, step: u32, out: *mut u8);
    fn rust_isa_arm_mve_max(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_maxv(a: *const u8) -> i32;
    fn rust_isa_arm_mve_min(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_minv(a: *const u8) -> i32;
    fn rust_isa_arm_mve_mla(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_mlal(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_mls(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_mlsl(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_mov(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_mul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_mull(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_mvn(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_neg(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_orn(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_orr(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_qabs(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_qadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_qdmulh(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_qdmull(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_qmovn(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_qneg(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_qrdmulh(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_qshl(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_arm_mve_qsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_rev16(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_rev32(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_rev64(a: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_rmulh(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_shl(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_arm_mve_shr(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_arm_mve_sli(a: *const u8, b: *const u8, count: u32, out: *mut u8);
    fn rust_isa_arm_mve_sri(a: *const u8, b: *const u8, count: u32, out: *mut u8);
    fn rust_isa_arm_mve_sub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_subn(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_subw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_uzp(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_mve_zip(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable MVE.
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
        /// The caller must enable MVE.
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
        /// The caller must enable MVE.
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

/// `vldrw.u32`
///
/// Load four 32-bit lanes from `p`.
///
/// # Safety
///
/// The caller must enable MVE. `p` must be valid for reading 16 bytes.
#[inline(always)]
pub unsafe fn ldr(p: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_ldr(p, out.as_mut_ptr());
    }
    out
}

/// `vldrb.u8`
///
/// Load 16 bytes from `p`.
///
/// # Safety
///
/// The caller must enable MVE. `p` must be valid for reading 16 bytes.
#[inline(always)]
pub unsafe fn ldrb(p: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_ldrb(p, out.as_mut_ptr());
    }
    out
}

/// `vstrw.u32`
///
/// Store four 32-bit lanes to `p`.
///
/// # Safety
///
/// The caller must enable MVE. `p` must be valid for writing 16 bytes and
/// four-byte aligned.
#[inline(always)]
pub unsafe fn str(p: *mut u8, value: V128) {
    unsafe {
        rust_isa_arm_mve_str(p, value.as_ptr());
    }
}

/// `vstrb.u8`
///
/// Store 16 bytes to `p`.
///
/// # Safety
///
/// The caller must enable MVE. `p` must be valid for writing 16 bytes.
#[inline(always)]
pub unsafe fn strb(p: *mut u8, value: V128) {
    unsafe {
        rust_isa_arm_mve_strb(p, value.as_ptr());
    }
}

unary!(
    /// `vabs.s32`.
    ///
    /// Take the saturating absolute value of each signed 32-bit lane.
    abs,
    rust_isa_arm_mve_abs
);
binary!(
    /// `vadd.i32`.
    add,
    rust_isa_arm_mve_add
);

/// `vaddlv.s32`
///
/// Add all signed 32-bit lanes and widen the result to 64 bits.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn addlv(a: V128) -> i64 {
    unsafe { rust_isa_arm_mve_addlv(a.as_ptr()) }
}

/// `vaddv.s32`
///
/// Add all signed 32-bit lanes.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn addv(a: V128) -> i32 {
    unsafe { rust_isa_arm_mve_addv(a.as_ptr()) }
}

binary!(
    /// Bitwise AND.
    and,
    rust_isa_arm_mve_and
);
binary!(
    /// `vabd.s32`.
    ///
    /// Take the absolute difference of each signed 32-bit lane.
    abd,
    rust_isa_arm_mve_abd
);

/// `vabav.s8`
///
/// Add the absolute differences of signed byte lanes to `acc`.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn abav(acc: u32, a: V128, b: V128) -> u32 {
    unsafe { rust_isa_arm_mve_abav(acc, a.as_ptr(), b.as_ptr()) }
}

binary!(
    /// Bitwise clear: `a & !b`.
    bic,
    rust_isa_arm_mve_bic
);

/// `vbrsr.s32`
///
/// Reverse all 32 bits of each lane and shift the result right by
/// `32 - bits`. `bits` is normally in `1..=32`; a value of 0 produces zero.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn brsr(a: V128, bits: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_brsr(a.as_ptr(), bits, out.as_mut_ptr());
    }
    out
}

unary!(
    /// `vclz.i32`.
    ///
    /// Count leading zero bits in each 32-bit lane.
    clz,
    rust_isa_arm_mve_clz
);

/// `vcmpeq.i32`
///
/// Compare signed 32-bit lanes for equality. A true lane is all bits one and a
/// false lane is zero.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn cmp(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_cmp(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vctp.8`
///
/// Create a tail predicate with `count` active byte lanes and return the raw
/// 16-bit predicate register value. `count` is in `0..=16`.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn ctp(count: u32) -> u16 {
    unsafe { rust_isa_arm_mve_ctp(count) }
}

/// `vdup.32`
///
/// Broadcast a 32-bit value to all lanes.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn dup(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_dup(value, out.as_mut_ptr());
    }
    out
}

binary!(
    /// Bitwise XOR.
    eor,
    rust_isa_arm_mve_eor
);
binary!(
    /// `vhadd.s32`.
    ///
    /// Add signed 32-bit lanes, shift right by one, and keep the low 32 bits.
    hadd,
    rust_isa_arm_mve_hadd
);
binary!(
    /// `vhsub.s32`.
    ///
    /// Subtract signed 32-bit lanes, shift right by one, and keep the low 32
    /// bits.
    hsub,
    rust_isa_arm_mve_hsub
);

/// `vidup.u32`
///
/// Create a vector of four 32-bit lanes where lane `i` is `base + i * step`.
/// `step` must be 1, 2, 4, or 8; other values produce zero.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn idup(base: u32, step: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_idup(base, step, out.as_mut_ptr());
    }
    out
}

/// `viwdup.u32`
///
/// Create a vector of four wrapping 32-bit increments starting at `base` and
/// wrapping at `wrap`. `step` must be 1, 2, 4, or 8; other values produce
/// zero.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn iwdup(base: u32, wrap: u32, step: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_iwdup(base, wrap, step, out.as_mut_ptr());
    }
    out
}

binary!(
    /// `vmax.s32`.
    max,
    rust_isa_arm_mve_max
);

/// `vmaxv.s32`
///
/// Take the maximum signed 32-bit lane.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn maxv(a: V128) -> i32 {
    unsafe { rust_isa_arm_mve_maxv(a.as_ptr()) }
}

binary!(
    /// `vmin.s32`.
    min,
    rust_isa_arm_mve_min
);

/// `vminv.s32`
///
/// Take the minimum signed 32-bit lane.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn minv(a: V128) -> i32 {
    unsafe { rust_isa_arm_mve_minv(a.as_ptr()) }
}

ternary!(
    /// `vmla.i32`.
    ///
    /// Multiply signed 32-bit lanes and add the products to `acc`.
    mla,
    rust_isa_arm_mve_mla
);
ternary!(
    /// `vmlal.s16`.
    ///
    /// Multiply the even-numbered signed 16-bit lanes of `a` and `b`, widen
    /// the products to 32 bits, and add them to `acc`.
    mlal,
    rust_isa_arm_mve_mlal
);
ternary!(
    /// `vmls.i32`.
    ///
    /// Multiply signed 32-bit lanes and subtract the products from `acc`.
    mls,
    rust_isa_arm_mve_mls
);
ternary!(
    /// `vmlsl.s16`.
    ///
    /// Multiply the even-numbered signed 16-bit lanes of `a` and `b`, widen
    /// the products to 32 bits, and subtract them from `acc`.
    mlsl,
    rust_isa_arm_mve_mlsl
);
unary!(
    /// `vmov`.
    ///
    /// Copy a vector register.
    mov,
    rust_isa_arm_mve_mov
);
binary!(
    /// `vmul.i32`, keeping the low 32 bits of each product.
    mul,
    rust_isa_arm_mve_mul
);
binary!(
    /// `vmull.s16`.
    ///
    /// Multiply the even-numbered signed 16-bit lanes of `a` and `b` and widen
    /// the products to 32 bits.
    mull,
    rust_isa_arm_mve_mull
);
unary!(
    /// `vmvn`.
    mvn,
    rust_isa_arm_mve_mvn
);
unary!(
    /// `vneg.s32`.
    neg,
    rust_isa_arm_mve_neg
);
binary!(
    /// Bitwise OR-NOT: `a | !b`.
    orn,
    rust_isa_arm_mve_orn
);
binary!(
    /// Bitwise OR.
    orr,
    rust_isa_arm_mve_orr
);
unary!(
    /// `vqabs.s32`.
    ///
    /// Take the saturating absolute value of each signed 32-bit lane.
    qabs,
    rust_isa_arm_mve_qabs
);
binary!(
    /// `vqadd.s32`.
    ///
    /// Add signed 32-bit lanes with saturation.
    qadd,
    rust_isa_arm_mve_qadd
);
binary!(
    /// `vqdmulh.s32`.
    ///
    /// Multiply signed 32-bit lanes, double the products, and keep the high
    /// 32 bits with saturation.
    qdmulh,
    rust_isa_arm_mve_qdmulh
);
binary!(
    /// `vqdmull.s16`.
    ///
    /// Multiply the even-numbered signed 16-bit lanes of `a` and `b`, double
    /// the products, and widen them to 32 bits with saturation.
    qdmull,
    rust_isa_arm_mve_qdmull
);
unary!(
    /// `vqmovn.s32`.
    ///
    /// Saturate signed 32-bit lanes to signed 16 bits and write each result to
    /// the low 16 bits of the corresponding 32-bit lane. The high 16 bits of
    /// each lane are zero.
    qmovn,
    rust_isa_arm_mve_qmovn
);
unary!(
    /// `vqneg.s32`.
    ///
    /// Negate signed 32-bit lanes with saturation.
    qneg,
    rust_isa_arm_mve_qneg
);
binary!(
    /// `vqrdmulh.s32`.
    ///
    /// Multiply signed 32-bit lanes, double the products, and keep the high
    /// 32 bits with rounding and saturation.
    qrdmulh,
    rust_isa_arm_mve_qrdmulh
);

/// `vqshl.s32`
///
/// Shift signed 32-bit lanes left by `count` bits with saturation.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn qshl(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_qshl(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

binary!(
    /// `vqsub.s32`.
    ///
    /// Subtract signed 32-bit lanes with saturation.
    qsub,
    rust_isa_arm_mve_qsub
);
unary!(
    /// `vrev16.8`.
    ///
    /// Reverse the bytes in each 16-bit lane.
    rev16,
    rust_isa_arm_mve_rev16
);
unary!(
    /// `vrev32.8`.
    ///
    /// Reverse the bytes in each 32-bit lane.
    rev32,
    rust_isa_arm_mve_rev32
);
unary!(
    /// `vrev64.8`.
    ///
    /// Reverse the bytes in each 64-bit lane.
    rev64,
    rust_isa_arm_mve_rev64
);
binary!(
    /// `vrmulh.s32`.
    ///
    /// Multiply signed 32-bit lanes, round, and keep the high 32 bits.
    rmulh,
    rust_isa_arm_mve_rmulh
);

/// `vshl.s32`
///
/// Shift signed 32-bit lanes left by `count` bits.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn shl(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_shl(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `vshr.s32`
///
/// Shift signed 32-bit lanes right by `count` bits, filling with the sign.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn shr(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_shr(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `vsli.32`
///
/// Shift `b` left by `count` bits and insert the low `32 - count` bits into
/// `a`.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn sli(a: V128, b: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_sli(a.as_ptr(), b.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `vsri.32`
///
/// Shift `b` right by `count` bits and insert the high `32 - count` bits into
/// `a`.
///
/// # Safety
///
/// The caller must enable MVE.
#[inline(always)]
pub unsafe fn sri(a: V128, b: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_mve_sri(a.as_ptr(), b.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

binary!(
    /// `vsub.i32`.
    sub,
    rust_isa_arm_mve_sub
);
binary!(
    /// `vsubhn.s32`.
    ///
    /// Subtract signed 32-bit lanes and keep the high 16 bits of each result,
    /// packed into the low four 16-bit lanes.
    subn,
    rust_isa_arm_mve_subn
);
binary!(
    /// `vsubw.s16`.
    ///
    /// Subtract the even-numbered signed 16-bit lanes of `b`, widened to 32
    /// bits, from the signed 32-bit lanes of `a`.
    subw,
    rust_isa_arm_mve_subw
);
binary!(
    /// `vuzp.8`
    ///
    /// Take the even bytes of `a` followed by the even bytes of `b`.
    uzp,
    rust_isa_arm_mve_uzp
);
binary!(
    /// `vzip.8`
    ///
    /// Interleave the low eight bytes of `a` and `b`.
    zip,
    rust_isa_arm_mve_zip
);

pub use super::mve_fp::{fma, fms};
