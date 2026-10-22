//! RISC-V Zfh, Zfhmin, and Zfa scalar floating-point instructions.
//!
//! The half-precision operations come from Zfh and Zfhmin. The single- and
//! double-precision operations are the F and D instructions together with the
//! Zfa additions (immediate loads, `fminm`, `fmaxm`, rounding, quiet
//! comparisons, and the modular conversion).
//!
//! These extensions are not part of the baseline. Operations that use an
//! extension are `unsafe` and the caller must enable the extensions used by the
//! operation. `fclass*` and `fli*` are safe because they operate on values and
//! do not require a target feature. The wrappers do not carry
//! `#[target_feature]` because rustc does not expose it on stable.
//!
//! Half-precision values are binary16 bit patterns in `u16`. Single- and
//! double-precision values are bit patterns in `u32` and `u64`.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.

unsafe extern "C" {
    fn rust_isa_riscv_zfh_fadd(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fsub(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fmul(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fdiv(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fsqrt(a: u16) -> u16;
    fn rust_isa_riscv_zfh_fmin(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fmax(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_feq(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_zfh_flt(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_zfh_fle(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_zfh_flh(p: *const u8, out: *mut u8);
    fn rust_isa_riscv_zfh_fsh(p: *mut u8, a: *const u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zfh.
        #[inline]
        pub unsafe fn $name(a: u16, b: u16) -> u16 {
            unsafe { $c(a, b) }
        }
    };
}
macro_rules! compare {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        compare!($(#[$meta])* $name, $c, u16);
    };
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zfh.
        #[inline]
        pub unsafe fn $name(a: $ty, b: $ty) -> bool {
            unsafe { $c(a, b) != 0 }
        }
    };
}

binary!(
    /// `fadd.h`.
    fadd_h,
    rust_isa_riscv_zfh_fadd
);
binary!(
    /// `fsub.h`.
    fsub_h,
    rust_isa_riscv_zfh_fsub
);
binary!(
    /// `fmul.h`.
    fmul_h,
    rust_isa_riscv_zfh_fmul
);
binary!(
    /// `fdiv.h`.
    fdiv_h,
    rust_isa_riscv_zfh_fdiv
);
binary!(
    /// `fmin.h`.
    fmin_h,
    rust_isa_riscv_zfh_fmin
);
binary!(
    /// `fmax.h`.
    fmax_h,
    rust_isa_riscv_zfh_fmax
);
compare!(
    /// `feq.h`.
    feq_h,
    rust_isa_riscv_zfh_feq
);
compare!(
    /// `flt.h`.
    flt_h,
    rust_isa_riscv_zfh_flt
);
compare!(
    /// `fle.h`.
    fle_h,
    rust_isa_riscv_zfh_fle
);

/// `fsqrt.h`.
///
/// Compute the square root of a binary16 value.
///
/// # Safety
///
/// The caller must enable Zfh.
#[inline]
pub unsafe fn fsqrt_h(a: u16) -> u16 {
    unsafe { rust_isa_riscv_zfh_fsqrt(a) }
}

/// `flh`.
///
/// Load a binary16 value from 2 bytes at `p`.
///
/// # Safety
///
/// The caller must enable Zfh. `p` must be valid for reading 2 bytes.
#[inline]
pub unsafe fn flh(p: *const u8) -> u16 {
    let mut out = [0u8; 2];
    unsafe { rust_isa_riscv_zfh_flh(p, out.as_mut_ptr()) };
    u16::from_le_bytes(out)
}

/// `fsh`.
///
/// Store a binary16 value as 2 bytes at `p`.
///
/// # Safety
///
/// The caller must enable Zfh. `p` must be valid for writing 2 bytes.
#[inline]
pub unsafe fn fsh(p: *mut u8, value: u16) {
    let bytes = value.to_le_bytes();
    unsafe { rust_isa_riscv_zfh_fsh(p, bytes.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_riscv_zfh_fcvt_h_s(value: u32) -> u16;
    fn rust_isa_riscv_zfh_fcvt_s_h(value: u16) -> u32;
    fn rust_isa_riscv_zfh_fcvt_h_d(value: u64) -> u16;
    fn rust_isa_riscv_zfh_fcvt_d_h(value: u16) -> u64;
    fn rust_isa_riscv_zfh_fcvt_w_h(value: u16) -> i32;
    fn rust_isa_riscv_zfh_fcvt_wu_h(value: u16) -> u32;
    fn rust_isa_riscv_zfh_fcvt_l_h(value: u16) -> i64;
    fn rust_isa_riscv_zfh_fcvt_lu_h(value: u16) -> u64;
    fn rust_isa_riscv_zfh_fcvt_h_w(value: i32) -> u16;
    fn rust_isa_riscv_zfh_fcvt_h_wu(value: u32) -> u16;
    fn rust_isa_riscv_zfh_fcvt_h_l(value: i64) -> u16;
    fn rust_isa_riscv_zfh_fcvt_h_lu(value: u64) -> u16;
    fn rust_isa_riscv_zfh_fmv_x_h(value: u16) -> i16;
    fn rust_isa_riscv_zfh_fmv_h_x(value: i16) -> u16;
}

macro_rules! convert {
    ($(#[$meta:meta])* $name:ident, $c:ident, $from:ty, $to:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zfh, F, D, or Zfa as required by the operation.
        #[inline]
        pub unsafe fn $name(value: $from) -> $to {
            unsafe { $c(value) }
        }
    };
}
convert!(
    /// `fcvt.h.s`.
    fcvt_h_s,
    rust_isa_riscv_zfh_fcvt_h_s,
    u32,
    u16
);
convert!(
    /// `fcvt.s.h`.
    fcvt_s_h,
    rust_isa_riscv_zfh_fcvt_s_h,
    u16,
    u32
);
convert!(
    /// `fcvt.h.d`.
    fcvt_h_d,
    rust_isa_riscv_zfh_fcvt_h_d,
    u64,
    u16
);
convert!(
    /// `fcvt.d.h`.
    fcvt_d_h,
    rust_isa_riscv_zfh_fcvt_d_h,
    u16,
    u64
);
convert!(
    /// `fcvt.w.h`.
    fcvt_w_h,
    rust_isa_riscv_zfh_fcvt_w_h,
    u16,
    i32
);
convert!(
    /// `fcvt.wu.h`.
    fcvt_wu_h,
    rust_isa_riscv_zfh_fcvt_wu_h,
    u16,
    u32
);
convert!(
    /// `fcvt.l.h`.
    fcvt_l_h,
    rust_isa_riscv_zfh_fcvt_l_h,
    u16,
    i64
);
convert!(
    /// `fcvt.lu.h`.
    fcvt_lu_h,
    rust_isa_riscv_zfh_fcvt_lu_h,
    u16,
    u64
);
convert!(
    /// `fcvt.h.w`.
    fcvt_h_w,
    rust_isa_riscv_zfh_fcvt_h_w,
    i32,
    u16
);
convert!(
    /// `fcvt.h.wu`.
    fcvt_h_wu,
    rust_isa_riscv_zfh_fcvt_h_wu,
    u32,
    u16
);
convert!(
    /// `fcvt.h.l`.
    fcvt_h_l,
    rust_isa_riscv_zfh_fcvt_h_l,
    i64,
    u16
);
convert!(
    /// `fcvt.h.lu`.
    fcvt_h_lu,
    rust_isa_riscv_zfh_fcvt_h_lu,
    u64,
    u16
);
convert!(
    /// `fmv.x.h`.
    fmv_x_h,
    rust_isa_riscv_zfh_fmv_x_h,
    u16,
    i16
);
convert!(
    /// `fmv.h.x`.
    fmv_h_x,
    rust_isa_riscv_zfh_fmv_h_x,
    i16,
    u16
);

unsafe extern "C" {
    fn rust_isa_riscv_zfh_fminm(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fmaxm(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fround(a: u16) -> u16;
    fn rust_isa_riscv_zfh_froundnx(a: u16) -> u16;
    fn rust_isa_riscv_zfh_fleq(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_zfh_fltq(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_zfh_fcvtmod_w_h(a: u16) -> i32;
}

binary!(
    /// `fminm.h`.
    fminm_h,
    rust_isa_riscv_zfh_fminm
);
binary!(
    /// `fmaxm.h`.
    fmaxm_h,
    rust_isa_riscv_zfh_fmaxm
);
compare!(
    /// `fleq.h`.
    fleq_h,
    rust_isa_riscv_zfh_fleq
);
compare!(
    /// `fltq.h`.
    fltq_h,
    rust_isa_riscv_zfh_fltq
);

/// `fround.h`.
///
/// Round a binary16 value to an integral value using the current rounding mode.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fround_h(a: u16) -> u16 {
    unsafe { rust_isa_riscv_zfh_fround(a) }
}

/// `froundnx.h`.
///
/// Round a binary16 value to an integral value and report whether the result is
/// inexact.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn froundnx_h(a: u16) -> u16 {
    unsafe { rust_isa_riscv_zfh_froundnx(a) }
}

/// `fcvtmod.w.h`.
///
/// Convert a binary16 value to a signed 32-bit integer with modulo semantics.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fcvtmod_w_h(a: u16) -> i32 {
    unsafe { rust_isa_riscv_zfh_fcvtmod_w_h(a) }
}

/// `fclass.h`.
///
/// Return the RISC-V floating-point class mask for a binary16 value.
#[inline]
pub fn fclass_h(value: u16) -> u32 {
    let sign = value >> 15;
    let exponent = (value >> 10) & 0x1f;
    let mantissa = value & 0x03ff;
    if exponent == 0x1f {
        if mantissa == 0 {
            return if sign == 0 { 1 << 7 } else { 1 << 0 };
        }
        return if mantissa & 0x0200 == 0 {
            1 << 8
        } else {
            1 << 9
        };
    }
    if exponent == 0 {
        if mantissa == 0 {
            return if sign == 0 { 1 << 4 } else { 1 << 3 };
        }
        return if sign == 0 { 1 << 5 } else { 1 << 2 };
    }
    if sign == 0 { 1 << 6 } else { 1 << 1 }
}

/// `fli.h`.
///
/// Load one of the 32 Zfa half-precision constants selected by `imm`.
#[inline]
pub fn fli_h(imm: u8) -> u16 {
    const VALUES: [u16; 32] = [
        0xbc00, 0x0400, 0x0100, 0x0200, 0x3800, 0x2000, 0x2c00, 0x3000, 0x3400, 0x3500, 0x3600,
        0x3700, 0x3800, 0x3900, 0x3a00, 0x3b00, 0x3c00, 0x3d00, 0x3e00, 0x3f00, 0x4000, 0x4100,
        0x4200, 0x4400, 0x4800, 0x4c00, 0x5800, 0x5c00, 0x7800, 0x7c00, 0x7c00, 0x7e00,
    ];
    VALUES[usize::from(imm & 31)]
}

unsafe extern "C" {
    fn rust_isa_riscv_zfh_fadd_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_fsub_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_fmul_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_fdiv_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_fsqrt_s(a: u32) -> u32;
    fn rust_isa_riscv_zfh_fmin_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_fmax_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_fminm_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_fmaxm_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_fround_s(a: u32) -> u32;
    fn rust_isa_riscv_zfh_froundnx_s(a: u32) -> u32;
    fn rust_isa_riscv_zfh_feq_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_flt_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_fle_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_fleq_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_zfh_fltq_s(a: u32, b: u32) -> u32;

    fn rust_isa_riscv_zfh_fadd_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zfh_fsub_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zfh_fmul_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zfh_fdiv_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zfh_fsqrt_d(a: u64) -> u64;
    fn rust_isa_riscv_zfh_fmin_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zfh_fmax_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zfh_fminm_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zfh_fmaxm_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zfh_fround_d(a: u64) -> u64;
    fn rust_isa_riscv_zfh_froundnx_d(a: u64) -> u64;
    fn rust_isa_riscv_zfh_feq_d(a: u64, b: u64) -> u32;
    fn rust_isa_riscv_zfh_flt_d(a: u64, b: u64) -> u32;
    fn rust_isa_riscv_zfh_fle_d(a: u64, b: u64) -> u32;
    fn rust_isa_riscv_zfh_fleq_d(a: u64, b: u64) -> u32;
    fn rust_isa_riscv_zfh_fltq_d(a: u64, b: u64) -> u32;

    fn rust_isa_riscv_zfh_fcvt_s_d(value: u64) -> u32;
    fn rust_isa_riscv_zfh_fcvt_d_s(value: u32) -> u64;
    fn rust_isa_riscv_zfh_fcvt_w_s(value: u32) -> i32;
    fn rust_isa_riscv_zfh_fcvt_wu_s(value: u32) -> u32;
    fn rust_isa_riscv_zfh_fcvt_l_s(value: u32) -> i64;
    fn rust_isa_riscv_zfh_fcvt_lu_s(value: u32) -> u64;
    fn rust_isa_riscv_zfh_fcvt_s_w(value: i32) -> u32;
    fn rust_isa_riscv_zfh_fcvt_s_wu(value: u32) -> u32;
    fn rust_isa_riscv_zfh_fcvt_s_l(value: i64) -> u32;
    fn rust_isa_riscv_zfh_fcvt_s_lu(value: u64) -> u32;
    fn rust_isa_riscv_zfh_fcvt_w_d(value: u64) -> i32;
    fn rust_isa_riscv_zfh_fcvt_wu_d(value: u64) -> u32;
    fn rust_isa_riscv_zfh_fcvt_l_d(value: u64) -> i64;
    fn rust_isa_riscv_zfh_fcvt_lu_d(value: u64) -> u64;
    fn rust_isa_riscv_zfh_fcvt_d_w(value: i32) -> u64;
    fn rust_isa_riscv_zfh_fcvt_d_wu(value: u32) -> u64;
    fn rust_isa_riscv_zfh_fcvt_d_l(value: i64) -> u64;
    fn rust_isa_riscv_zfh_fcvt_d_lu(value: u64) -> u64;
    fn rust_isa_riscv_zfh_fcvtmod_w_d(value: u64) -> i32;
    fn rust_isa_riscv_zfh_fmv_x_w(value: u32) -> i32;
    fn rust_isa_riscv_zfh_fmv_w_x(value: i32) -> u32;
    fn rust_isa_riscv_zfh_fmv_x_d(value: u64) -> i64;
    fn rust_isa_riscv_zfh_fmv_d_x(value: i64) -> u64;
    fn rust_isa_riscv_zfh_fmvh_x_d(value: u64) -> u32;
    fn rust_isa_riscv_zfh_fmvp_d_x(low: u32, high: u32) -> u64;
}

macro_rules! binary_fmt {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zfh, F, D, or Zfa as required by the operation.
        #[inline]
        pub unsafe fn $name(a: $ty, b: $ty) -> $ty {
            unsafe { $c(a, b) }
        }
    };
}

macro_rules! unary_fmt {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zfh, F, D, or Zfa as required by the operation.
        #[inline]
        pub unsafe fn $name(value: $ty) -> $ty {
            unsafe { $c(value) }
        }
    };
}

binary_fmt!(
    /// `fadd.s`.
    fadd_s,
    rust_isa_riscv_zfh_fadd_s,
    u32
);
binary_fmt!(
    /// `fsub.s`.
    fsub_s,
    rust_isa_riscv_zfh_fsub_s,
    u32
);
binary_fmt!(
    /// `fmul.s`.
    fmul_s,
    rust_isa_riscv_zfh_fmul_s,
    u32
);
binary_fmt!(
    /// `fdiv.s`.
    fdiv_s,
    rust_isa_riscv_zfh_fdiv_s,
    u32
);
binary_fmt!(
    /// `fmin.s`.
    fmin_s,
    rust_isa_riscv_zfh_fmin_s,
    u32
);
binary_fmt!(
    /// `fmax.s`.
    fmax_s,
    rust_isa_riscv_zfh_fmax_s,
    u32
);
binary_fmt!(
    /// `fminm.s`.
    fminm_s,
    rust_isa_riscv_zfh_fminm_s,
    u32
);
binary_fmt!(
    /// `fmaxm.s`.
    fmaxm_s,
    rust_isa_riscv_zfh_fmaxm_s,
    u32
);
binary_fmt!(
    /// `fadd.d`.
    fadd_d,
    rust_isa_riscv_zfh_fadd_d,
    u64
);
binary_fmt!(
    /// `fsub.d`.
    fsub_d,
    rust_isa_riscv_zfh_fsub_d,
    u64
);
binary_fmt!(
    /// `fmul.d`.
    fmul_d,
    rust_isa_riscv_zfh_fmul_d,
    u64
);
binary_fmt!(
    /// `fdiv.d`.
    fdiv_d,
    rust_isa_riscv_zfh_fdiv_d,
    u64
);
binary_fmt!(
    /// `fmin.d`.
    fmin_d,
    rust_isa_riscv_zfh_fmin_d,
    u64
);
binary_fmt!(
    /// `fmax.d`.
    fmax_d,
    rust_isa_riscv_zfh_fmax_d,
    u64
);
binary_fmt!(
    /// `fminm.d`.
    fminm_d,
    rust_isa_riscv_zfh_fminm_d,
    u64
);
binary_fmt!(
    /// `fmaxm.d`.
    fmaxm_d,
    rust_isa_riscv_zfh_fmaxm_d,
    u64
);

unary_fmt!(
    /// `fsqrt.s`.
    fsqrt_s,
    rust_isa_riscv_zfh_fsqrt_s,
    u32
);
unary_fmt!(
    /// `fround.s`.
    fround_s,
    rust_isa_riscv_zfh_fround_s,
    u32
);
unary_fmt!(
    /// `froundnx.s`.
    froundnx_s,
    rust_isa_riscv_zfh_froundnx_s,
    u32
);
unary_fmt!(
    /// `fsqrt.d`.
    fsqrt_d,
    rust_isa_riscv_zfh_fsqrt_d,
    u64
);
unary_fmt!(
    /// `fround.d`.
    fround_d,
    rust_isa_riscv_zfh_fround_d,
    u64
);
unary_fmt!(
    /// `froundnx.d`.
    froundnx_d,
    rust_isa_riscv_zfh_froundnx_d,
    u64
);

compare!(
    /// `feq.s`.
    feq_s,
    rust_isa_riscv_zfh_feq_s,
    u32
);
compare!(
    /// `flt.s`.
    flt_s,
    rust_isa_riscv_zfh_flt_s,
    u32
);
compare!(
    /// `fle.s`.
    fle_s,
    rust_isa_riscv_zfh_fle_s,
    u32
);
compare!(
    /// `fleq.s`.
    fleq_s,
    rust_isa_riscv_zfh_fleq_s,
    u32
);
compare!(
    /// `fltq.s`.
    fltq_s,
    rust_isa_riscv_zfh_fltq_s,
    u32
);
compare!(
    /// `feq.d`.
    feq_d,
    rust_isa_riscv_zfh_feq_d,
    u64
);
compare!(
    /// `flt.d`.
    flt_d,
    rust_isa_riscv_zfh_flt_d,
    u64
);
compare!(
    /// `fle.d`.
    fle_d,
    rust_isa_riscv_zfh_fle_d,
    u64
);
compare!(
    /// `fleq.d`.
    fleq_d,
    rust_isa_riscv_zfh_fleq_d,
    u64
);
compare!(
    /// `fltq.d`.
    fltq_d,
    rust_isa_riscv_zfh_fltq_d,
    u64
);

convert!(
    /// `fcvt.s.d`.
    fcvt_s_d,
    rust_isa_riscv_zfh_fcvt_s_d,
    u64,
    u32
);
convert!(
    /// `fcvt.d.s`.
    fcvt_d_s,
    rust_isa_riscv_zfh_fcvt_d_s,
    u32,
    u64
);
convert!(
    /// `fcvt.w.s`.
    fcvt_w_s,
    rust_isa_riscv_zfh_fcvt_w_s,
    u32,
    i32
);
convert!(
    /// `fcvt.wu.s`.
    fcvt_wu_s,
    rust_isa_riscv_zfh_fcvt_wu_s,
    u32,
    u32
);
convert!(
    /// `fcvt.l.s`.
    fcvt_l_s,
    rust_isa_riscv_zfh_fcvt_l_s,
    u32,
    i64
);
convert!(
    /// `fcvt.lu.s`.
    fcvt_lu_s,
    rust_isa_riscv_zfh_fcvt_lu_s,
    u32,
    u64
);
convert!(
    /// `fcvt.s.w`.
    fcvt_s_w,
    rust_isa_riscv_zfh_fcvt_s_w,
    i32,
    u32
);
convert!(
    /// `fcvt.s.wu`.
    fcvt_s_wu,
    rust_isa_riscv_zfh_fcvt_s_wu,
    u32,
    u32
);
convert!(
    /// `fcvt.s.l`.
    fcvt_s_l,
    rust_isa_riscv_zfh_fcvt_s_l,
    i64,
    u32
);
convert!(
    /// `fcvt.s.lu`.
    fcvt_s_lu,
    rust_isa_riscv_zfh_fcvt_s_lu,
    u64,
    u32
);
convert!(
    /// `fcvt.w.d`.
    fcvt_w_d,
    rust_isa_riscv_zfh_fcvt_w_d,
    u64,
    i32
);
convert!(
    /// `fcvt.wu.d`.
    fcvt_wu_d,
    rust_isa_riscv_zfh_fcvt_wu_d,
    u64,
    u32
);
convert!(
    /// `fcvt.l.d`.
    fcvt_l_d,
    rust_isa_riscv_zfh_fcvt_l_d,
    u64,
    i64
);
convert!(
    /// `fcvt.lu.d`.
    fcvt_lu_d,
    rust_isa_riscv_zfh_fcvt_lu_d,
    u64,
    u64
);
convert!(
    /// `fcvt.d.w`.
    fcvt_d_w,
    rust_isa_riscv_zfh_fcvt_d_w,
    i32,
    u64
);
convert!(
    /// `fcvt.d.wu`.
    fcvt_d_wu,
    rust_isa_riscv_zfh_fcvt_d_wu,
    u32,
    u64
);
convert!(
    /// `fcvt.d.l`.
    fcvt_d_l,
    rust_isa_riscv_zfh_fcvt_d_l,
    i64,
    u64
);
convert!(
    /// `fcvt.d.lu`.
    fcvt_d_lu,
    rust_isa_riscv_zfh_fcvt_d_lu,
    u64,
    u64
);
convert!(
    /// `fcvtmod.w.d`.
    fcvtmod_w_d,
    rust_isa_riscv_zfh_fcvtmod_w_d,
    u64,
    i32
);
convert!(
    /// `fmv.x.w`.
    fmv_x_w,
    rust_isa_riscv_zfh_fmv_x_w,
    u32,
    i32
);
convert!(
    /// `fmv.w.x`.
    fmv_w_x,
    rust_isa_riscv_zfh_fmv_w_x,
    i32,
    u32
);
convert!(
    /// `fmv.x.d`.
    fmv_x_d,
    rust_isa_riscv_zfh_fmv_x_d,
    u64,
    i64
);
convert!(
    /// `fmv.d.x`.
    fmv_d_x,
    rust_isa_riscv_zfh_fmv_d_x,
    i64,
    u64
);

/// `fmvh.x.d`.
///
/// Return bits 63:32. This instruction is only encodable on RV32; the value
/// API is also provided on RV64 for symmetry with [`fmvp_d_x`].
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fmvh_x_d(value: u64) -> u32 {
    unsafe { rust_isa_riscv_zfh_fmvh_x_d(value) }
}

/// `fmvp.d.x`.
///
/// Combine `low` (bits 31:0) and `high` (bits 63:32). This instruction is only
/// encodable on RV32; the value API is also provided on RV64.
///
/// # Safety
///
/// The caller must enable Zfa.
#[inline]
pub unsafe fn fmvp_d_x(low: u32, high: u32) -> u64 {
    unsafe { rust_isa_riscv_zfh_fmvp_d_x(low, high) }
}

/// `fli.s`.
///
/// Load one of the 32 Zfa single-precision constants selected by `imm`.
#[inline]
pub fn fli_s(imm: u8) -> u32 {
    const VALUES: [u32; 32] = [
        0xbf80_0000,
        0x0080_0000,
        0x3780_0000,
        0x3800_0000,
        0x3b80_0000,
        0x3c00_0000,
        0x3d80_0000,
        0x3e00_0000,
        0x3e80_0000,
        0x3ea0_0000,
        0x3ec0_0000,
        0x3ee0_0000,
        0x3f00_0000,
        0x3f20_0000,
        0x3f40_0000,
        0x3f60_0000,
        0x3f80_0000,
        0x3fa0_0000,
        0x3fc0_0000,
        0x3fe0_0000,
        0x4000_0000,
        0x4020_0000,
        0x4040_0000,
        0x4080_0000,
        0x4100_0000,
        0x4180_0000,
        0x4300_0000,
        0x4380_0000,
        0x4700_0000,
        0x4780_0000,
        0x7f80_0000,
        0x7fc0_0000,
    ];
    VALUES[usize::from(imm & 31)]
}

/// `fli.d`.
///
/// Load one of the 32 Zfa double-precision constants selected by `imm`.
#[inline]
pub fn fli_d(imm: u8) -> u64 {
    const VALUES: [u64; 32] = [
        0xbff0_0000_0000_0000,
        0x0010_0000_0000_0000,
        0x3ef0_0000_0000_0000,
        0x3f00_0000_0000_0000,
        0x3f70_0000_0000_0000,
        0x3f80_0000_0000_0000,
        0x3fb0_0000_0000_0000,
        0x3fc0_0000_0000_0000,
        0x3fd0_0000_0000_0000,
        0x3fd4_0000_0000_0000,
        0x3fd8_0000_0000_0000,
        0x3fdc_0000_0000_0000,
        0x3fe0_0000_0000_0000,
        0x3fe4_0000_0000_0000,
        0x3fe8_0000_0000_0000,
        0x3fec_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x3ff4_0000_0000_0000,
        0x3ff8_0000_0000_0000,
        0x3ffc_0000_0000_0000,
        0x4000_0000_0000_0000,
        0x4004_0000_0000_0000,
        0x4008_0000_0000_0000,
        0x4010_0000_0000_0000,
        0x4020_0000_0000_0000,
        0x4030_0000_0000_0000,
        0x4060_0000_0000_0000,
        0x4070_0000_0000_0000,
        0x40e0_0000_0000_0000,
        0x40f0_0000_0000_0000,
        0x7ff0_0000_0000_0000,
        0x7ff8_0000_0000_0000,
    ];
    VALUES[usize::from(imm & 31)]
}
