//! RISC-V vector bit manipulation: Zvbb.
//!
//! Zvbb is not part of the RISC-V vector baseline. The caller must compile
//! with the `zvbb` target feature enabled.
//!
//! The values are [`V128`]. The instructions operate on 64-bit lanes, except
//! `vwsll`, which widens 32-bit lanes to 64-bit lanes.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! Instructions not listed in this module are not exposed.

//! # Compile-time immediate check
//!
//! ```compile_fail
//! use rust_isa::riscv::simd::rvv::V128;
//! use rust_isa::riscv::simd::zv::vror_vi;
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vror_vi::<64>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::riscv::simd::rvv::V128;
//! use rust_isa::riscv::simd::zv::vwsll_vi;
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vwsll_vi::<32>(v) };
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::riscv::simd::rvv::V128;
//! use rust_isa::riscv::simd::zv::*;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let _ = unsafe { vandn_vv(v128, v128) };
//! let _ = unsafe { vandn_vx(v128, 1) };
//! let _ = unsafe { vbrev(v128) };
//! let _ = unsafe { vbrev8(v128) };
//! let _ = unsafe { vrev8(v128) };
//! let _ = unsafe { vclz(v128) };
//! let _ = unsafe { vctz(v128) };
//! let _ = unsafe { vcpop(v128) };
//! let _ = unsafe { vrol_vv(v128, v128) };
//! let _ = unsafe { vrol_vx(v128, 1) };
//! let _ = unsafe { vror_vv(v128, v128) };
//! let _ = unsafe { vror_vx(v128, 1) };
//! let _ = unsafe { vror_vi::<1>(v128) };
//! let _ = unsafe { vwsll_vv(v128, v128) };
//! let _ = unsafe { vwsll_vx(v128, 1) };
//! let _ = unsafe { vwsll_vi::<1>(v128) };
//! ```

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::riscv::simd::zv::vandn_vv as _;
//! use rust_isa::riscv::simd::zv::vandn_vx as _;
//! use rust_isa::riscv::simd::zv::vbrev as _;
//! use rust_isa::riscv::simd::zv::vbrev8 as _;
//! use rust_isa::riscv::simd::zv::vclz as _;
//! use rust_isa::riscv::simd::zv::vcpop as _;
//! use rust_isa::riscv::simd::zv::vctz as _;
//! use rust_isa::riscv::simd::zv::vrev8 as _;
//! use rust_isa::riscv::simd::zv::vrol_vv as _;
//! use rust_isa::riscv::simd::zv::vrol_vx as _;
//! use rust_isa::riscv::simd::zv::vror_vv as _;
//! use rust_isa::riscv::simd::zv::vror_vi as _;
//! use rust_isa::riscv::simd::zv::vror_vx as _;
//! use rust_isa::riscv::simd::zv::vwsll_vv as _;
//! use rust_isa::riscv::simd::zv::vwsll_vi as _;
//! use rust_isa::riscv::simd::zv::vwsll_vx as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::riscv::simd::rvv::V128;

unsafe extern "C" {
    fn rust_isa_zvbb_vandn_vv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vandn_vx(a: *const u8, x: u64, out: *mut u8);
    fn rust_isa_zvbb_vbrev(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vbrev8(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vrev8(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vclz(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vctz(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vcpop(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vrol_vv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vrol_vx(a: *const u8, x: u64, out: *mut u8);
    fn rust_isa_zvbb_vror_vv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vx(a: *const u8, x: u64, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vx(a: *const u8, x: u32, out: *mut u8);
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the `zvbb` target feature enabled.
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
        /// The caller must compile with the `zvbb` target feature enabled.
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

macro_rules! binary_scalar {
    ($(#[$meta:meta])* $name:ident, $c:ident, $scalar:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the `zvbb` target feature enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, x: $scalar) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), x, out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `vandn.vv` on 64-bit lanes.
    vandn_vv,
    rust_isa_zvbb_vandn_vv
);
binary_scalar!(
    /// `vandn.vx` on 64-bit lanes.
    vandn_vx,
    rust_isa_zvbb_vandn_vx,
    u64
);
unary!(
    /// `vbrev.v` on 64-bit lanes.
    vbrev,
    rust_isa_zvbb_vbrev
);
unary!(
    /// `vbrev8.v` on 64-bit lanes.
    vbrev8,
    rust_isa_zvbb_vbrev8
);
unary!(
    /// `vrev8.v` on 64-bit lanes.
    vrev8,
    rust_isa_zvbb_vrev8
);
unary!(
    /// `vclz.v` on 64-bit lanes.
    vclz,
    rust_isa_zvbb_vclz
);
unary!(
    /// `vctz.v` on 64-bit lanes.
    vctz,
    rust_isa_zvbb_vctz
);
unary!(
    /// `vcpop.v` on 64-bit lanes.
    vcpop,
    rust_isa_zvbb_vcpop
);
binary!(
    /// `vrol.vv` on 64-bit lanes.
    vrol_vv,
    rust_isa_zvbb_vrol_vv
);
binary_scalar!(
    /// `vrol.vx` on 64-bit lanes.
    vrol_vx,
    rust_isa_zvbb_vrol_vx,
    u64
);
binary!(
    /// `vror.vv` on 64-bit lanes.
    vror_vv,
    rust_isa_zvbb_vror_vv
);
binary_scalar!(
    /// `vror.vx` on 64-bit lanes.
    vror_vx,
    rust_isa_zvbb_vror_vx,
    u64
);
binary!(
    /// `vwsll.vv` widening 32-bit lanes to 64-bit lanes.
    vwsll_vv,
    rust_isa_zvbb_vwsll_vv
);
binary_scalar!(
    /// `vwsll.vx` widening 32-bit lanes to 64-bit lanes.
    vwsll_vx,
    rust_isa_zvbb_vwsll_vx,
    u32
);

unsafe extern "C" {
    fn rust_isa_zvbb_vror_vi_0(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_1(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_2(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_3(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_4(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_5(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_6(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_7(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_8(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_9(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_10(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_11(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_12(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_13(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_14(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_15(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_16(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_17(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_18(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_19(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_20(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_21(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_22(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_23(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_24(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_25(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_26(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_27(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_28(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_29(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_30(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_31(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_32(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_33(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_34(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_35(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_36(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_37(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_38(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_39(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_40(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_41(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_42(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_43(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_44(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_45(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_46(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_47(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_48(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_49(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_50(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_51(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_52(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_53(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_54(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_55(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_56(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_57(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_58(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_59(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_60(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_61(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_62(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vror_vi_63(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_0(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_1(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_2(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_3(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_4(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_5(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_6(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_7(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_8(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_9(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_10(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_11(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_12(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_13(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_14(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_15(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_16(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_17(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_18(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_19(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_20(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_21(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_22(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_23(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_24(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_25(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_26(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_27(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_28(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_29(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_30(a: *const u8, out: *mut u8);
    fn rust_isa_zvbb_vwsll_vi_31(a: *const u8, out: *mut u8);
}

/// `vror.vi` on 64-bit lanes.
///
/// `IMM` must be in `0..=63`.
///
/// # Safety
///
/// The caller must compile with the `zvbb` target feature enabled.
#[inline(always)]
pub unsafe fn vror_vi<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 63, "vror.vi immediate must be in 0..=63") };
    macro_rules! arm {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match IMM {
        0 => arm!(rust_isa_zvbb_vror_vi_0),
        1 => arm!(rust_isa_zvbb_vror_vi_1),
        2 => arm!(rust_isa_zvbb_vror_vi_2),
        3 => arm!(rust_isa_zvbb_vror_vi_3),
        4 => arm!(rust_isa_zvbb_vror_vi_4),
        5 => arm!(rust_isa_zvbb_vror_vi_5),
        6 => arm!(rust_isa_zvbb_vror_vi_6),
        7 => arm!(rust_isa_zvbb_vror_vi_7),
        8 => arm!(rust_isa_zvbb_vror_vi_8),
        9 => arm!(rust_isa_zvbb_vror_vi_9),
        10 => arm!(rust_isa_zvbb_vror_vi_10),
        11 => arm!(rust_isa_zvbb_vror_vi_11),
        12 => arm!(rust_isa_zvbb_vror_vi_12),
        13 => arm!(rust_isa_zvbb_vror_vi_13),
        14 => arm!(rust_isa_zvbb_vror_vi_14),
        15 => arm!(rust_isa_zvbb_vror_vi_15),
        16 => arm!(rust_isa_zvbb_vror_vi_16),
        17 => arm!(rust_isa_zvbb_vror_vi_17),
        18 => arm!(rust_isa_zvbb_vror_vi_18),
        19 => arm!(rust_isa_zvbb_vror_vi_19),
        20 => arm!(rust_isa_zvbb_vror_vi_20),
        21 => arm!(rust_isa_zvbb_vror_vi_21),
        22 => arm!(rust_isa_zvbb_vror_vi_22),
        23 => arm!(rust_isa_zvbb_vror_vi_23),
        24 => arm!(rust_isa_zvbb_vror_vi_24),
        25 => arm!(rust_isa_zvbb_vror_vi_25),
        26 => arm!(rust_isa_zvbb_vror_vi_26),
        27 => arm!(rust_isa_zvbb_vror_vi_27),
        28 => arm!(rust_isa_zvbb_vror_vi_28),
        29 => arm!(rust_isa_zvbb_vror_vi_29),
        30 => arm!(rust_isa_zvbb_vror_vi_30),
        31 => arm!(rust_isa_zvbb_vror_vi_31),
        32 => arm!(rust_isa_zvbb_vror_vi_32),
        33 => arm!(rust_isa_zvbb_vror_vi_33),
        34 => arm!(rust_isa_zvbb_vror_vi_34),
        35 => arm!(rust_isa_zvbb_vror_vi_35),
        36 => arm!(rust_isa_zvbb_vror_vi_36),
        37 => arm!(rust_isa_zvbb_vror_vi_37),
        38 => arm!(rust_isa_zvbb_vror_vi_38),
        39 => arm!(rust_isa_zvbb_vror_vi_39),
        40 => arm!(rust_isa_zvbb_vror_vi_40),
        41 => arm!(rust_isa_zvbb_vror_vi_41),
        42 => arm!(rust_isa_zvbb_vror_vi_42),
        43 => arm!(rust_isa_zvbb_vror_vi_43),
        44 => arm!(rust_isa_zvbb_vror_vi_44),
        45 => arm!(rust_isa_zvbb_vror_vi_45),
        46 => arm!(rust_isa_zvbb_vror_vi_46),
        47 => arm!(rust_isa_zvbb_vror_vi_47),
        48 => arm!(rust_isa_zvbb_vror_vi_48),
        49 => arm!(rust_isa_zvbb_vror_vi_49),
        50 => arm!(rust_isa_zvbb_vror_vi_50),
        51 => arm!(rust_isa_zvbb_vror_vi_51),
        52 => arm!(rust_isa_zvbb_vror_vi_52),
        53 => arm!(rust_isa_zvbb_vror_vi_53),
        54 => arm!(rust_isa_zvbb_vror_vi_54),
        55 => arm!(rust_isa_zvbb_vror_vi_55),
        56 => arm!(rust_isa_zvbb_vror_vi_56),
        57 => arm!(rust_isa_zvbb_vror_vi_57),
        58 => arm!(rust_isa_zvbb_vror_vi_58),
        59 => arm!(rust_isa_zvbb_vror_vi_59),
        60 => arm!(rust_isa_zvbb_vror_vi_60),
        61 => arm!(rust_isa_zvbb_vror_vi_61),
        62 => arm!(rust_isa_zvbb_vror_vi_62),
        63 => arm!(rust_isa_zvbb_vror_vi_63),
        _ => unreachable!(),
    }
}

/// `vwsll.vi` widening 32-bit lanes to 64-bit lanes.
///
/// `IMM` must be in `0..=31`.
///
/// # Safety
///
/// The caller must compile with the `zvbb` target feature enabled.
#[inline(always)]
pub unsafe fn vwsll_vi<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "vwsll.vi immediate must be in 0..=31") };
    macro_rules! arm {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match IMM {
        0 => arm!(rust_isa_zvbb_vwsll_vi_0),
        1 => arm!(rust_isa_zvbb_vwsll_vi_1),
        2 => arm!(rust_isa_zvbb_vwsll_vi_2),
        3 => arm!(rust_isa_zvbb_vwsll_vi_3),
        4 => arm!(rust_isa_zvbb_vwsll_vi_4),
        5 => arm!(rust_isa_zvbb_vwsll_vi_5),
        6 => arm!(rust_isa_zvbb_vwsll_vi_6),
        7 => arm!(rust_isa_zvbb_vwsll_vi_7),
        8 => arm!(rust_isa_zvbb_vwsll_vi_8),
        9 => arm!(rust_isa_zvbb_vwsll_vi_9),
        10 => arm!(rust_isa_zvbb_vwsll_vi_10),
        11 => arm!(rust_isa_zvbb_vwsll_vi_11),
        12 => arm!(rust_isa_zvbb_vwsll_vi_12),
        13 => arm!(rust_isa_zvbb_vwsll_vi_13),
        14 => arm!(rust_isa_zvbb_vwsll_vi_14),
        15 => arm!(rust_isa_zvbb_vwsll_vi_15),
        16 => arm!(rust_isa_zvbb_vwsll_vi_16),
        17 => arm!(rust_isa_zvbb_vwsll_vi_17),
        18 => arm!(rust_isa_zvbb_vwsll_vi_18),
        19 => arm!(rust_isa_zvbb_vwsll_vi_19),
        20 => arm!(rust_isa_zvbb_vwsll_vi_20),
        21 => arm!(rust_isa_zvbb_vwsll_vi_21),
        22 => arm!(rust_isa_zvbb_vwsll_vi_22),
        23 => arm!(rust_isa_zvbb_vwsll_vi_23),
        24 => arm!(rust_isa_zvbb_vwsll_vi_24),
        25 => arm!(rust_isa_zvbb_vwsll_vi_25),
        26 => arm!(rust_isa_zvbb_vwsll_vi_26),
        27 => arm!(rust_isa_zvbb_vwsll_vi_27),
        28 => arm!(rust_isa_zvbb_vwsll_vi_28),
        29 => arm!(rust_isa_zvbb_vwsll_vi_29),
        30 => arm!(rust_isa_zvbb_vwsll_vi_30),
        31 => arm!(rust_isa_zvbb_vwsll_vi_31),
        _ => unreachable!(),
    }
}
