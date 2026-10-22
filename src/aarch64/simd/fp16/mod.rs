//! AArch64 FP16.
//!
//! FP16 is not part of the AArch64 baseline. Every operation is `unsafe` and
//! the caller must enable FP16 with `#[target_feature(enable = "fp16")]` or a
//! global target feature.
//!
//! The values are [`V128`] of IEEE 754 binary16 lanes. The result of arithmetic
//! is rounded according to the FPCR rounding mode.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).

use crate::aarch64::simd::neon::V128;

unsafe extern "C" {
    fn rust_isa_fp16_vaddq_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_vmulq_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_vsubq_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_vdivq_f16(a: *const u8, b: *const u8, out: *mut u8);
}

/// `fadd` on binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fadd_f16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_vaddq_f16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmul` on binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fmul_f16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_vmulq_f16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fsub` on binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fsub_f16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_vsubq_f16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fdiv` on binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fdiv_f16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_vdivq_f16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
pub use crate::aarch64::simd::fhm::{fmla_f16, fmls_f16};

unsafe extern "C" {
    fn rust_isa_fp16_fabd_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_facge_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_facgt_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcmeq_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcmge_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcmgt_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcmle_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcmlt_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fmax_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fmaxnm_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fmin_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fminnm_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_frecps_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_frsqrts_f16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fabs_f16(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_fneg_f16(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_fsqrt_f16(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_frecpe_f16(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_frsqrte_f16(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_fmlal_f16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fmlal2_f16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fmlsl_f16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fmlsl2_f16(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcvtl_f32_f16(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcvtl2_f32_f16(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcvtn_f16_f32(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcvtn2_f16_f32(a: *const u8, low: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcvtxn_f16_f64(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcvtzs_f16_s32(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcvtzs_f16_s64(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcvtzu_f16_u32(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcvtzu_f16_u64(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_scvtf_f16_s32(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_scvtf_f16_s64(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_ucvtf_f16_u32(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_ucvtf_f16_u64(a: *const u8, out: *mut u8);
    fn rust_isa_fp16_fcvt_f16_f32(a: f32) -> u16;
    fn rust_isa_fp16_fcvt_f32_f16(a: u16) -> f32;
    fn rust_isa_fp16_fcvt_f16_f64(a: f64) -> u16;
    fn rust_isa_fp16_fcvt_f64_f16(a: u16) -> f64;
    fn rust_isa_fp16_frecpx_f16(a: u16) -> u16;
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        /// # Safety
        ///
        /// The caller must enable FP16.
        #[target_feature(enable = "fp16")]
        #[inline]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        /// # Safety
        ///
        /// The caller must enable FP16.
        #[target_feature(enable = "fp16")]
        #[inline]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! ternary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        /// # Safety
        ///
        /// The caller must enable FHM.
        #[target_feature(enable = "fhm")]
        #[inline]
        pub unsafe fn $name(acc: V128, a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `fabd` on binary16 lanes.
    fabd_f16,
    rust_isa_fp16_fabd_f16
);
binary!(
    /// `facge` on binary16 lanes. NaN lanes compare false.
    facge_f16,
    rust_isa_fp16_facge_f16
);
binary!(
    /// `facgt` on binary16 lanes. NaN lanes compare false.
    facgt_f16,
    rust_isa_fp16_facgt_f16
);
binary!(
    /// `fcmeq` on binary16 lanes. NaN lanes compare false.
    fcmeq_f16,
    rust_isa_fp16_fcmeq_f16
);
binary!(
    /// `fcmge` on binary16 lanes. NaN lanes compare false.
    fcmge_f16,
    rust_isa_fp16_fcmge_f16
);
binary!(
    /// `fcmgt` on binary16 lanes. NaN lanes compare false.
    fcmgt_f16,
    rust_isa_fp16_fcmgt_f16
);
binary!(
    /// `fcmle` on binary16 lanes. NaN lanes compare false.
    fcmle_f16,
    rust_isa_fp16_fcmle_f16
);
binary!(
    /// `fcmlt` on binary16 lanes. NaN lanes compare false.
    fcmlt_f16,
    rust_isa_fp16_fcmlt_f16
);
binary!(
    /// `fmax` on binary16 lanes, propagating NaN.
    fmax_f16,
    rust_isa_fp16_fmax_f16
);
binary!(
    /// `fmaxnm` on binary16 lanes, preferring numeric values.
    fmaxnm_f16,
    rust_isa_fp16_fmaxnm_f16
);
binary!(
    /// `fmin` on binary16 lanes, propagating NaN.
    fmin_f16,
    rust_isa_fp16_fmin_f16
);
binary!(
    /// `fminnm` on binary16 lanes, preferring numeric values.
    fminnm_f16,
    rust_isa_fp16_fminnm_f16
);
binary!(
    /// `frecps` step on binary16 lanes.
    frecps_f16,
    rust_isa_fp16_frecps_f16
);
binary!(
    /// `frsqrts` step on binary16 lanes.
    frsqrts_f16,
    rust_isa_fp16_frsqrts_f16
);
unary!(
    /// `fabs` on binary16 lanes.
    fabs_f16,
    rust_isa_fp16_fabs_f16
);
unary!(
    /// `fneg` on binary16 lanes.
    fneg_f16,
    rust_isa_fp16_fneg_f16
);
unary!(
    /// `fsqrt` on binary16 lanes.
    fsqrt_f16,
    rust_isa_fp16_fsqrt_f16
);
unary!(
    /// `frecpe` estimate on binary16 lanes.
    frecpe_f16,
    rust_isa_fp16_frecpe_f16
);
unary!(
    /// `frsqrte` estimate on binary16 lanes.
    frsqrte_f16,
    rust_isa_fp16_frsqrte_f16
);
ternary!(
    /// `fmlal` multiplies the low four binary16 lanes and accumulates into f32 lanes.
    fmlal_f16,
    rust_isa_fp16_fmlal_f16
);
ternary!(
    /// `fmlal2` multiplies the high four binary16 lanes and accumulates into f32 lanes.
    fmlal2_f16,
    rust_isa_fp16_fmlal2_f16
);
ternary!(
    /// `fmlsl` multiplies the low four binary16 lanes and subtracts from f32 lanes.
    fmlsl_f16,
    rust_isa_fp16_fmlsl_f16
);
ternary!(
    /// `fmlsl2` multiplies the high four binary16 lanes and subtracts from f32 lanes.
    fmlsl2_f16,
    rust_isa_fp16_fmlsl2_f16
);

/// `fcvtl` converts the low four binary16 lanes to f32 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fcvtl_f32_f16(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_fcvtl_f32_f16(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcvtl2` converts the high four binary16 lanes to f32 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fcvtl2_f32_f16(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_fcvtl2_f32_f16(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcvtn` converts the low four f32 lanes to binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fcvtn_f16_f32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_fcvtn_f16_f32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcvtn2` converts the high four f32 lanes and writes them into the upper half.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fcvtn2_f16_f32(a: V128, low: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_fcvtn2_f16_f32(a.as_ptr(), low.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcvtxn` converts f64 lanes to binary16 lanes.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fcvtxn_f16_f64(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_fp16_fcvtxn_f16_f64(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

macro_rules! convert {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        /// # Safety
        ///
        /// The caller must enable FP16.
        #[target_feature(enable = "fp16")]
        #[inline]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

convert!(
    /// `fcvtzs` from binary16 lanes to signed 32-bit lanes.
    fcvtzs_f16_s32,
    rust_isa_fp16_fcvtzs_f16_s32
);
convert!(
    /// `fcvtzs` from binary16 lanes to signed 64-bit lanes.
    fcvtzs_f16_s64,
    rust_isa_fp16_fcvtzs_f16_s64
);
convert!(
    /// `fcvtzu` from binary16 lanes to unsigned 32-bit lanes.
    fcvtzu_f16_u32,
    rust_isa_fp16_fcvtzu_f16_u32
);
convert!(
    /// `fcvtzu` from binary16 lanes to unsigned 64-bit lanes.
    fcvtzu_f16_u64,
    rust_isa_fp16_fcvtzu_f16_u64
);
convert!(
    /// `scvtf` from signed 32-bit lanes to binary16 lanes.
    scvtf_f16_s32,
    rust_isa_fp16_scvtf_f16_s32
);
convert!(
    /// `scvtf` from signed 64-bit lanes to binary16 lanes.
    scvtf_f16_s64,
    rust_isa_fp16_scvtf_f16_s64
);
convert!(
    /// `ucvtf` from unsigned 32-bit lanes to binary16 lanes.
    ucvtf_f16_u32,
    rust_isa_fp16_ucvtf_f16_u32
);
convert!(
    /// `ucvtf` from unsigned 64-bit lanes to binary16 lanes.
    ucvtf_f16_u64,
    rust_isa_fp16_ucvtf_f16_u64
);

/// `fcvt` from a binary32 scalar to a binary16 value.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fcvt_f16_f32(a: f32) -> u16 {
    unsafe { rust_isa_fp16_fcvt_f16_f32(a) }
}

/// `fcvt` from a binary16 value to a binary32 scalar.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fcvt_f32_f16(a: u16) -> f32 {
    unsafe { rust_isa_fp16_fcvt_f32_f16(a) }
}

/// `fcvt` from a binary64 scalar to a binary16 value.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fcvt_f16_f64(a: f64) -> u16 {
    unsafe { rust_isa_fp16_fcvt_f16_f64(a) }
}

/// `fcvt` from a binary16 value to a binary64 scalar.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn fcvt_f64_f16(a: u16) -> f64 {
    unsafe { rust_isa_fp16_fcvt_f64_f16(a) }
}

/// `frecpx` on a binary16 scalar.
///
/// # Safety
///
/// The caller must enable FP16.
#[target_feature(enable = "fp16")]
#[inline]
pub unsafe fn frecpx_f16(a: u16) -> u16 {
    unsafe { rust_isa_fp16_frecpx_f16(a) }
}
