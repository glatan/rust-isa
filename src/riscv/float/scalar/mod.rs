//! RISC-V scalar floating-point instructions from F, D, Zfh, and Zfhmin.
//!
//! The half-precision operations come from Zfh and Zfhmin. The single- and
//! double-precision operations are the F and D instructions.
//!
//! These extensions are not part of the baseline. Operations that use an
//! extension are `unsafe` and the caller must enable the extensions used by the
//! operation. `fclass*` is safe because it operates on values and does not
//! require a target feature. The wrappers do not carry `#[target_feature]`
//! because rustc does not expose it on stable.
//!
//! Half-precision values are binary16 bit patterns in `u16`. Single- and
//! double-precision values are bit patterns in `u32` and `u64`.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.

//! # Examples
//!
//! ```
//! use rust_isa::riscv::float::scalar::fclass_h;
//!
//! assert_eq!(fclass_h(0x3c00), 1 << 6);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::riscv::float::scalar::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { fadd_d(0, 0) };
//! let _ = unsafe { fadd_h(0, 0) };
//! let _ = unsafe { fadd_s(0, 0) };
//! let _ = fclass_h(0);
//! let _ = unsafe { fcvt_d_h(0) };
//! let _ = unsafe { fcvt_d_l(0) };
//! let _ = unsafe { fcvt_d_lu(0) };
//! let _ = unsafe { fcvt_d_s(0) };
//! let _ = unsafe { fcvt_d_w(0) };
//! let _ = unsafe { fcvt_d_wu(0) };
//! let _ = unsafe { fcvt_h_d(0) };
//! let _ = unsafe { fcvt_h_l(0) };
//! let _ = unsafe { fcvt_h_lu(0) };
//! let _ = unsafe { fcvt_h_s(0) };
//! let _ = unsafe { fcvt_h_w(0) };
//! let _ = unsafe { fcvt_h_wu(0) };
//! let _ = unsafe { fcvt_l_d(0) };
//! let _ = unsafe { fcvt_l_h(0) };
//! let _ = unsafe { fcvt_l_s(0) };
//! let _ = unsafe { fcvt_lu_d(0) };
//! let _ = unsafe { fcvt_lu_h(0) };
//! let _ = unsafe { fcvt_lu_s(0) };
//! let _ = unsafe { fcvt_s_d(0) };
//! let _ = unsafe { fcvt_s_h(0) };
//! let _ = unsafe { fcvt_s_l(0) };
//! let _ = unsafe { fcvt_s_lu(0) };
//! let _ = unsafe { fcvt_s_w(0) };
//! let _ = unsafe { fcvt_s_wu(0) };
//! let _ = unsafe { fcvt_w_d(0) };
//! let _ = unsafe { fcvt_w_h(0) };
//! let _ = unsafe { fcvt_w_s(0) };
//! let _ = unsafe { fcvt_wu_d(0) };
//! let _ = unsafe { fcvt_wu_h(0) };
//! let _ = unsafe { fcvt_wu_s(0) };
//! let _ = unsafe { fdiv_d(0, 0) };
//! let _ = unsafe { fdiv_h(0, 0) };
//! let _ = unsafe { fdiv_s(0, 0) };
//! let _ = unsafe { feq_d(0, 0) };
//! let _ = unsafe { feq_h(0, 0) };
//! let _ = unsafe { feq_s(0, 0) };
//! let _ = unsafe { fle_d(0, 0) };
//! let _ = unsafe { fle_h(0, 0) };
//! let _ = unsafe { fle_s(0, 0) };
//! let _ = unsafe { flt_d(0, 0) };
//! let _ = unsafe { flt_h(0, 0) };
//! let _ = unsafe { flt_s(0, 0) };
//! let _ = unsafe { fmax_d(0, 0) };
//! let _ = unsafe { fmax_h(0, 0) };
//! let _ = unsafe { fmax_s(0, 0) };
//! let _ = unsafe { fmin_d(0, 0) };
//! let _ = unsafe { fmin_h(0, 0) };
//! let _ = unsafe { fmin_s(0, 0) };
//! let _ = unsafe { fmul_d(0, 0) };
//! let _ = unsafe { fmul_h(0, 0) };
//! let _ = unsafe { fmul_s(0, 0) };
//! let _ = unsafe { fmv_d_x(0) };
//! let _ = unsafe { fmv_h_x(0) };
//! let _ = unsafe { fmv_w_x(0) };
//! let _ = unsafe { fmv_x_d(0) };
//! let _ = unsafe { fmv_x_h(0) };
//! let _ = unsafe { fmv_x_w(0) };
//! let _ = unsafe { fsqrt_d(0) };
//! let _ = unsafe { fsqrt_h(0) };
//! let _ = unsafe { fsqrt_s(0) };
//! let _ = unsafe { fsub_d(0, 0) };
//! let _ = unsafe { fsub_h(0, 0) };
//! let _ = unsafe { fsub_s(0, 0) };
//! ```
//!
//! </details>
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::riscv::float::scalar::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { flh(storage.0.as_ptr().cast()) };
//! let _ = unsafe { fsh(storage.0.as_mut_ptr().cast(), 0) };
//! ```
//!
//! </details>
//!
//! <details>
//! <summary>Instruction inventory</summary>
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::riscv::float::scalar::fadd_d as _;
//! use rust_isa::riscv::float::scalar::fadd_h as _;
//! use rust_isa::riscv::float::scalar::fadd_s as _;
//! use rust_isa::riscv::float::scalar::fclass_h as _;
//! use rust_isa::riscv::float::scalar::fcvt_d_h as _;
//! use rust_isa::riscv::float::scalar::fcvt_d_l as _;
//! use rust_isa::riscv::float::scalar::fcvt_d_lu as _;
//! use rust_isa::riscv::float::scalar::fcvt_d_s as _;
//! use rust_isa::riscv::float::scalar::fcvt_d_w as _;
//! use rust_isa::riscv::float::scalar::fcvt_d_wu as _;
//! use rust_isa::riscv::float::scalar::fcvt_h_d as _;
//! use rust_isa::riscv::float::scalar::fcvt_h_l as _;
//! use rust_isa::riscv::float::scalar::fcvt_h_lu as _;
//! use rust_isa::riscv::float::scalar::fcvt_h_s as _;
//! use rust_isa::riscv::float::scalar::fcvt_h_w as _;
//! use rust_isa::riscv::float::scalar::fcvt_h_wu as _;
//! use rust_isa::riscv::float::scalar::fcvt_l_d as _;
//! use rust_isa::riscv::float::scalar::fcvt_l_h as _;
//! use rust_isa::riscv::float::scalar::fcvt_l_s as _;
//! use rust_isa::riscv::float::scalar::fcvt_lu_d as _;
//! use rust_isa::riscv::float::scalar::fcvt_lu_h as _;
//! use rust_isa::riscv::float::scalar::fcvt_lu_s as _;
//! use rust_isa::riscv::float::scalar::fcvt_s_d as _;
//! use rust_isa::riscv::float::scalar::fcvt_s_h as _;
//! use rust_isa::riscv::float::scalar::fcvt_s_l as _;
//! use rust_isa::riscv::float::scalar::fcvt_s_lu as _;
//! use rust_isa::riscv::float::scalar::fcvt_s_w as _;
//! use rust_isa::riscv::float::scalar::fcvt_s_wu as _;
//! use rust_isa::riscv::float::scalar::fcvt_w_d as _;
//! use rust_isa::riscv::float::scalar::fcvt_w_h as _;
//! use rust_isa::riscv::float::scalar::fcvt_w_s as _;
//! use rust_isa::riscv::float::scalar::fcvt_wu_d as _;
//! use rust_isa::riscv::float::scalar::fcvt_wu_h as _;
//! use rust_isa::riscv::float::scalar::fcvt_wu_s as _;
//! use rust_isa::riscv::float::scalar::fdiv_d as _;
//! use rust_isa::riscv::float::scalar::fdiv_h as _;
//! use rust_isa::riscv::float::scalar::fdiv_s as _;
//! use rust_isa::riscv::float::scalar::feq_d as _;
//! use rust_isa::riscv::float::scalar::feq_h as _;
//! use rust_isa::riscv::float::scalar::feq_s as _;
//! use rust_isa::riscv::float::scalar::fle_d as _;
//! use rust_isa::riscv::float::scalar::fle_h as _;
//! use rust_isa::riscv::float::scalar::fle_s as _;
//! use rust_isa::riscv::float::scalar::flh as _;
//! use rust_isa::riscv::float::scalar::flt_d as _;
//! use rust_isa::riscv::float::scalar::flt_h as _;
//! use rust_isa::riscv::float::scalar::flt_s as _;
//! use rust_isa::riscv::float::scalar::fmax_d as _;
//! use rust_isa::riscv::float::scalar::fmax_h as _;
//! use rust_isa::riscv::float::scalar::fmax_s as _;
//! use rust_isa::riscv::float::scalar::fmin_d as _;
//! use rust_isa::riscv::float::scalar::fmin_h as _;
//! use rust_isa::riscv::float::scalar::fmin_s as _;
//! use rust_isa::riscv::float::scalar::fmul_d as _;
//! use rust_isa::riscv::float::scalar::fmul_h as _;
//! use rust_isa::riscv::float::scalar::fmul_s as _;
//! use rust_isa::riscv::float::scalar::fmv_d_x as _;
//! use rust_isa::riscv::float::scalar::fmv_h_x as _;
//! use rust_isa::riscv::float::scalar::fmv_w_x as _;
//! use rust_isa::riscv::float::scalar::fmv_x_d as _;
//! use rust_isa::riscv::float::scalar::fmv_x_h as _;
//! use rust_isa::riscv::float::scalar::fmv_x_w as _;
//! use rust_isa::riscv::float::scalar::fsh as _;
//! use rust_isa::riscv::float::scalar::fsqrt_d as _;
//! use rust_isa::riscv::float::scalar::fsqrt_h as _;
//! use rust_isa::riscv::float::scalar::fsqrt_s as _;
//! use rust_isa::riscv::float::scalar::fsub_d as _;
//! use rust_isa::riscv::float::scalar::fsub_h as _;
//! use rust_isa::riscv::float::scalar::fsub_s as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

unsafe extern "C" {
    fn rust_isa_riscv_scalar_fadd(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_scalar_fsub(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_scalar_fmul(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_scalar_fdiv(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_scalar_fsqrt(a: u16) -> u16;
    fn rust_isa_riscv_scalar_fmin(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_scalar_fmax(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_scalar_feq(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_scalar_flt(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_scalar_fle(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_scalar_flh(p: *const u8, out: *mut u8);
    fn rust_isa_riscv_scalar_fsh(p: *mut u8, a: *const u8);
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
    rust_isa_riscv_scalar_fadd
);
binary!(
    /// `fsub.h`.
    fsub_h,
    rust_isa_riscv_scalar_fsub
);
binary!(
    /// `fmul.h`.
    fmul_h,
    rust_isa_riscv_scalar_fmul
);
binary!(
    /// `fdiv.h`.
    fdiv_h,
    rust_isa_riscv_scalar_fdiv
);
binary!(
    /// `fmin.h`.
    fmin_h,
    rust_isa_riscv_scalar_fmin
);
binary!(
    /// `fmax.h`.
    fmax_h,
    rust_isa_riscv_scalar_fmax
);
compare!(
    /// `feq.h`.
    feq_h,
    rust_isa_riscv_scalar_feq
);
compare!(
    /// `flt.h`.
    flt_h,
    rust_isa_riscv_scalar_flt
);
compare!(
    /// `fle.h`.
    fle_h,
    rust_isa_riscv_scalar_fle
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
    unsafe { rust_isa_riscv_scalar_fsqrt(a) }
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
    unsafe { rust_isa_riscv_scalar_flh(p, out.as_mut_ptr()) };
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
    unsafe { rust_isa_riscv_scalar_fsh(p, bytes.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_riscv_scalar_fcvt_h_s(value: u32) -> u16;
    fn rust_isa_riscv_scalar_fcvt_s_h(value: u16) -> u32;
    fn rust_isa_riscv_scalar_fcvt_h_d(value: u64) -> u16;
    fn rust_isa_riscv_scalar_fcvt_d_h(value: u16) -> u64;
    fn rust_isa_riscv_scalar_fcvt_w_h(value: u16) -> i32;
    fn rust_isa_riscv_scalar_fcvt_wu_h(value: u16) -> u32;
    fn rust_isa_riscv_scalar_fcvt_l_h(value: u16) -> i64;
    fn rust_isa_riscv_scalar_fcvt_lu_h(value: u16) -> u64;
    fn rust_isa_riscv_scalar_fcvt_h_w(value: i32) -> u16;
    fn rust_isa_riscv_scalar_fcvt_h_wu(value: u32) -> u16;
    fn rust_isa_riscv_scalar_fcvt_h_l(value: i64) -> u16;
    fn rust_isa_riscv_scalar_fcvt_h_lu(value: u64) -> u16;
    fn rust_isa_riscv_scalar_fmv_x_h(value: u16) -> i16;
    fn rust_isa_riscv_scalar_fmv_h_x(value: i16) -> u16;
}

macro_rules! convert {
    ($(#[$meta:meta])* $name:ident, $c:ident, $from:ty, $to:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable F, D, Zfh, or Zfhmin as required by the operation.
        #[inline]
        pub unsafe fn $name(value: $from) -> $to {
            unsafe { $c(value) }
        }
    };
}
convert!(
    /// `fcvt.h.s`.
    fcvt_h_s,
    rust_isa_riscv_scalar_fcvt_h_s,
    u32,
    u16
);
convert!(
    /// `fcvt.s.h`.
    fcvt_s_h,
    rust_isa_riscv_scalar_fcvt_s_h,
    u16,
    u32
);
convert!(
    /// `fcvt.h.d`.
    fcvt_h_d,
    rust_isa_riscv_scalar_fcvt_h_d,
    u64,
    u16
);
convert!(
    /// `fcvt.d.h`.
    fcvt_d_h,
    rust_isa_riscv_scalar_fcvt_d_h,
    u16,
    u64
);
convert!(
    /// `fcvt.w.h`.
    fcvt_w_h,
    rust_isa_riscv_scalar_fcvt_w_h,
    u16,
    i32
);
convert!(
    /// `fcvt.wu.h`.
    fcvt_wu_h,
    rust_isa_riscv_scalar_fcvt_wu_h,
    u16,
    u32
);
convert!(
    /// `fcvt.l.h`.
    fcvt_l_h,
    rust_isa_riscv_scalar_fcvt_l_h,
    u16,
    i64
);
convert!(
    /// `fcvt.lu.h`.
    fcvt_lu_h,
    rust_isa_riscv_scalar_fcvt_lu_h,
    u16,
    u64
);
convert!(
    /// `fcvt.h.w`.
    fcvt_h_w,
    rust_isa_riscv_scalar_fcvt_h_w,
    i32,
    u16
);
convert!(
    /// `fcvt.h.wu`.
    fcvt_h_wu,
    rust_isa_riscv_scalar_fcvt_h_wu,
    u32,
    u16
);
convert!(
    /// `fcvt.h.l`.
    fcvt_h_l,
    rust_isa_riscv_scalar_fcvt_h_l,
    i64,
    u16
);
convert!(
    /// `fcvt.h.lu`.
    fcvt_h_lu,
    rust_isa_riscv_scalar_fcvt_h_lu,
    u64,
    u16
);
convert!(
    /// `fmv.x.h`.
    fmv_x_h,
    rust_isa_riscv_scalar_fmv_x_h,
    u16,
    i16
);
convert!(
    /// `fmv.h.x`.
    fmv_h_x,
    rust_isa_riscv_scalar_fmv_h_x,
    i16,
    u16
);

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

unsafe extern "C" {
    fn rust_isa_riscv_scalar_fadd_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_scalar_fsub_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_scalar_fmul_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_scalar_fdiv_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_scalar_fsqrt_s(a: u32) -> u32;
    fn rust_isa_riscv_scalar_fmin_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_scalar_fmax_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_scalar_feq_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_scalar_flt_s(a: u32, b: u32) -> u32;
    fn rust_isa_riscv_scalar_fle_s(a: u32, b: u32) -> u32;

    fn rust_isa_riscv_scalar_fadd_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_scalar_fsub_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_scalar_fmul_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_scalar_fdiv_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_scalar_fsqrt_d(a: u64) -> u64;
    fn rust_isa_riscv_scalar_fmin_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_scalar_fmax_d(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_scalar_feq_d(a: u64, b: u64) -> u32;
    fn rust_isa_riscv_scalar_flt_d(a: u64, b: u64) -> u32;
    fn rust_isa_riscv_scalar_fle_d(a: u64, b: u64) -> u32;

    fn rust_isa_riscv_scalar_fcvt_s_d(value: u64) -> u32;
    fn rust_isa_riscv_scalar_fcvt_d_s(value: u32) -> u64;
    fn rust_isa_riscv_scalar_fcvt_w_s(value: u32) -> i32;
    fn rust_isa_riscv_scalar_fcvt_wu_s(value: u32) -> u32;
    fn rust_isa_riscv_scalar_fcvt_l_s(value: u32) -> i64;
    fn rust_isa_riscv_scalar_fcvt_lu_s(value: u32) -> u64;
    fn rust_isa_riscv_scalar_fcvt_s_w(value: i32) -> u32;
    fn rust_isa_riscv_scalar_fcvt_s_wu(value: u32) -> u32;
    fn rust_isa_riscv_scalar_fcvt_s_l(value: i64) -> u32;
    fn rust_isa_riscv_scalar_fcvt_s_lu(value: u64) -> u32;
    fn rust_isa_riscv_scalar_fcvt_w_d(value: u64) -> i32;
    fn rust_isa_riscv_scalar_fcvt_wu_d(value: u64) -> u32;
    fn rust_isa_riscv_scalar_fcvt_l_d(value: u64) -> i64;
    fn rust_isa_riscv_scalar_fcvt_lu_d(value: u64) -> u64;
    fn rust_isa_riscv_scalar_fcvt_d_w(value: i32) -> u64;
    fn rust_isa_riscv_scalar_fcvt_d_wu(value: u32) -> u64;
    fn rust_isa_riscv_scalar_fcvt_d_l(value: i64) -> u64;
    fn rust_isa_riscv_scalar_fcvt_d_lu(value: u64) -> u64;
    fn rust_isa_riscv_scalar_fmv_x_w(value: u32) -> i32;
    fn rust_isa_riscv_scalar_fmv_w_x(value: i32) -> u32;
    fn rust_isa_riscv_scalar_fmv_x_d(value: u64) -> i64;
    fn rust_isa_riscv_scalar_fmv_d_x(value: i64) -> u64;
}

macro_rules! binary_fmt {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable F, D, Zfh, or Zfhmin as required by the operation.
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
        /// The caller must enable F, D, Zfh, or Zfhmin as required by the operation.
        #[inline]
        pub unsafe fn $name(value: $ty) -> $ty {
            unsafe { $c(value) }
        }
    };
}

binary_fmt!(
    /// `fadd.s`.
    fadd_s,
    rust_isa_riscv_scalar_fadd_s,
    u32
);
binary_fmt!(
    /// `fsub.s`.
    fsub_s,
    rust_isa_riscv_scalar_fsub_s,
    u32
);
binary_fmt!(
    /// `fmul.s`.
    fmul_s,
    rust_isa_riscv_scalar_fmul_s,
    u32
);
binary_fmt!(
    /// `fdiv.s`.
    fdiv_s,
    rust_isa_riscv_scalar_fdiv_s,
    u32
);
binary_fmt!(
    /// `fmin.s`.
    fmin_s,
    rust_isa_riscv_scalar_fmin_s,
    u32
);
binary_fmt!(
    /// `fmax.s`.
    fmax_s,
    rust_isa_riscv_scalar_fmax_s,
    u32
);
binary_fmt!(
    /// `fadd.d`.
    fadd_d,
    rust_isa_riscv_scalar_fadd_d,
    u64
);
binary_fmt!(
    /// `fsub.d`.
    fsub_d,
    rust_isa_riscv_scalar_fsub_d,
    u64
);
binary_fmt!(
    /// `fmul.d`.
    fmul_d,
    rust_isa_riscv_scalar_fmul_d,
    u64
);
binary_fmt!(
    /// `fdiv.d`.
    fdiv_d,
    rust_isa_riscv_scalar_fdiv_d,
    u64
);
binary_fmt!(
    /// `fmin.d`.
    fmin_d,
    rust_isa_riscv_scalar_fmin_d,
    u64
);
binary_fmt!(
    /// `fmax.d`.
    fmax_d,
    rust_isa_riscv_scalar_fmax_d,
    u64
);
unary_fmt!(
    /// `fsqrt.s`.
    fsqrt_s,
    rust_isa_riscv_scalar_fsqrt_s,
    u32
);
unary_fmt!(
    /// `fsqrt.d`.
    fsqrt_d,
    rust_isa_riscv_scalar_fsqrt_d,
    u64
);
compare!(
    /// `feq.s`.
    feq_s,
    rust_isa_riscv_scalar_feq_s,
    u32
);
compare!(
    /// `flt.s`.
    flt_s,
    rust_isa_riscv_scalar_flt_s,
    u32
);
compare!(
    /// `fle.s`.
    fle_s,
    rust_isa_riscv_scalar_fle_s,
    u32
);
compare!(
    /// `feq.d`.
    feq_d,
    rust_isa_riscv_scalar_feq_d,
    u64
);
compare!(
    /// `flt.d`.
    flt_d,
    rust_isa_riscv_scalar_flt_d,
    u64
);
compare!(
    /// `fle.d`.
    fle_d,
    rust_isa_riscv_scalar_fle_d,
    u64
);
convert!(
    /// `fcvt.s.d`.
    fcvt_s_d,
    rust_isa_riscv_scalar_fcvt_s_d,
    u64,
    u32
);
convert!(
    /// `fcvt.d.s`.
    fcvt_d_s,
    rust_isa_riscv_scalar_fcvt_d_s,
    u32,
    u64
);
convert!(
    /// `fcvt.w.s`.
    fcvt_w_s,
    rust_isa_riscv_scalar_fcvt_w_s,
    u32,
    i32
);
convert!(
    /// `fcvt.wu.s`.
    fcvt_wu_s,
    rust_isa_riscv_scalar_fcvt_wu_s,
    u32,
    u32
);
convert!(
    /// `fcvt.l.s`.
    fcvt_l_s,
    rust_isa_riscv_scalar_fcvt_l_s,
    u32,
    i64
);
convert!(
    /// `fcvt.lu.s`.
    fcvt_lu_s,
    rust_isa_riscv_scalar_fcvt_lu_s,
    u32,
    u64
);
convert!(
    /// `fcvt.s.w`.
    fcvt_s_w,
    rust_isa_riscv_scalar_fcvt_s_w,
    i32,
    u32
);
convert!(
    /// `fcvt.s.wu`.
    fcvt_s_wu,
    rust_isa_riscv_scalar_fcvt_s_wu,
    u32,
    u32
);
convert!(
    /// `fcvt.s.l`.
    fcvt_s_l,
    rust_isa_riscv_scalar_fcvt_s_l,
    i64,
    u32
);
convert!(
    /// `fcvt.s.lu`.
    fcvt_s_lu,
    rust_isa_riscv_scalar_fcvt_s_lu,
    u64,
    u32
);
convert!(
    /// `fcvt.w.d`.
    fcvt_w_d,
    rust_isa_riscv_scalar_fcvt_w_d,
    u64,
    i32
);
convert!(
    /// `fcvt.wu.d`.
    fcvt_wu_d,
    rust_isa_riscv_scalar_fcvt_wu_d,
    u64,
    u32
);
convert!(
    /// `fcvt.l.d`.
    fcvt_l_d,
    rust_isa_riscv_scalar_fcvt_l_d,
    u64,
    i64
);
convert!(
    /// `fcvt.lu.d`.
    fcvt_lu_d,
    rust_isa_riscv_scalar_fcvt_lu_d,
    u64,
    u64
);
convert!(
    /// `fcvt.d.w`.
    fcvt_d_w,
    rust_isa_riscv_scalar_fcvt_d_w,
    i32,
    u64
);
convert!(
    /// `fcvt.d.wu`.
    fcvt_d_wu,
    rust_isa_riscv_scalar_fcvt_d_wu,
    u32,
    u64
);
convert!(
    /// `fcvt.d.l`.
    fcvt_d_l,
    rust_isa_riscv_scalar_fcvt_d_l,
    i64,
    u64
);
convert!(
    /// `fcvt.d.lu`.
    fcvt_d_lu,
    rust_isa_riscv_scalar_fcvt_d_lu,
    u64,
    u64
);
convert!(
    /// `fmv.x.w`.
    fmv_x_w,
    rust_isa_riscv_scalar_fmv_x_w,
    u32,
    i32
);
convert!(
    /// `fmv.w.x`.
    fmv_w_x,
    rust_isa_riscv_scalar_fmv_w_x,
    i32,
    u32
);
convert!(
    /// `fmv.x.d`.
    fmv_x_d,
    rust_isa_riscv_scalar_fmv_x_d,
    u64,
    i64
);
convert!(
    /// `fmv.d.x`.
    fmv_d_x,
    rust_isa_riscv_scalar_fmv_d_x,
    i64,
    u64
);
