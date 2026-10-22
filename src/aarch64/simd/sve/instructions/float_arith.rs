//! SVE floating-point arithmetic instructions.

unsafe extern "C" {
    fn rust_isa_sve_recps_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `recps` from `f32` to `f32` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn recps_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_recps_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rsqrts_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `rsqrts` from `f32` to `f32` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rsqrts_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsqrts_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_recpe_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `recpe` from `f32` to `f32` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn recpe_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_recpe_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rsqrte_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `rsqrte` from `f32` to `f32` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rsqrte_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsqrte_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_recps_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `recps` from `f64` to `f64` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn recps_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_recps_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rsqrts_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `rsqrts` from `f64` to `f64` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rsqrts_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsqrts_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_recpe_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `recpe` from `f64` to `f64` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn recpe_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_recpe_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rsqrte_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `rsqrte` from `f64` to `f64` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rsqrte_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsqrte_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_scale_f32(a: *const f32, b: *const i32, out: *mut f32, len: usize);
}
/// `fscale` on `f32` slices with `i32` exponents.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn scale_f32(a: &[f32], b: &[i32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scale_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmov_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `fmov` on `f32` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmov_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmov_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_scale_f64(a: *const f64, b: *const i64, out: *mut f64, len: usize);
}
/// `fscale` on `f64` slices with `i64` exponents.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn scale_f64(a: &[f64], b: &[i64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scale_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmov_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `fmov` on `f64` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmov_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmov_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_urecpe_f32_u32(a: *const u32, out: *mut f32, len: usize);
}
/// `urecpe` from `u32` to `f32` slices (scalar fallback).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn urecpe_f32_u32(a: &[u32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_urecpe_f32_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fabs_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `fabs` on `f32` slices (alias of `abs`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fabs_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fabs_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fneg_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `fneg` on `f32` slices (alias of `neg`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fneg_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fneg_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fsqrt_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `fsqrt` on `f32` slices (alias of `sqrt`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fsqrt_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fsqrt_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frecpe_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frecpe` on `f32` slices (alias of `recpe`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frecpe_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frecpe_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frinta_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frinta` on `f32` slices (alias of `rinta`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frinta_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frinta_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frinti_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frinti` on `f32` slices (alias of `rinti`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frinti_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frinti_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frintx_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frintx` on `f32` slices (alias of `rintx`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintx_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintx_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frintm_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frintm` on `f32` slices (alias of `floor`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintm_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintm_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frintn_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frintn` on `f32` slices (alias of `rint`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintn_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintn_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frintp_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frintp` on `f32` slices (alias of `ceil`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintp_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintp_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frintz_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frintz` on `f32` slices (alias of `trunc`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintz_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintz_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rintm_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `rintm` on `f32` slices (alias of `floor`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintm_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintm_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rintn_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `rintn` on `f32` slices (alias of `rint`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintn_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintn_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rintp_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `rintp` on `f32` slices (alias of `ceil`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintp_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintp_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rintz_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `rintz` on `f32` slices (alias of `trunc`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintz_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintz_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fadd_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fadd` on `f32` slices (alias of `add`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fadd_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fadd_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fsub_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fsub` on `f32` slices (alias of `sub`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fsub_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fsub_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmul_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmul` on `f32` slices (alias of `mul`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmul_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmul_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fdiv_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fdiv` on `f32` slices (alias of `div`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fdiv_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fdiv_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmax_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmax` on `f32` slices (alias of `max`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmax_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmax_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmin_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmin` on `f32` slices (alias of `min`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmin_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmin_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmaxnm_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmaxnm` on `f32` slices (alias of `maxnm`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxnm_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmaxnm_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fminnm_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fminnm` on `f32` slices (alias of `minnm`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminnm_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fminnm_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fabd_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fabd` on `f32` slices (alias of `abd`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fabd_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fabd_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmulx_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmulx` on `f32` slices (alias of `mulx`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmulx_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmulx_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frecps_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `frecps` on `f32` slices (alias of `recps`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frecps_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frecps_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frsqrts_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `frsqrts` on `f32` slices (alias of `rsqrts`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frsqrts_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frsqrts_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fscale_f32(a: *const f32, b: *const i32, out: *mut f32, len: usize);
}
/// `fscale` on `f32` slices with `i32` exponents (alias of `scale`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fscale_f32(a: &[f32], b: &[i32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fscale_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmla_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fmla` on `f32` slices (alias of `fma`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmla_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmla_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmls_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fmls` on `f32` slices (alias of `mls`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmls_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmls_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_faddv_f32(a: *const f32, len: usize) -> f32;
}
/// `faddv` over a `f32` slice (alias of `addv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn faddv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_faddv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fmaxv_f32(a: *const f32, len: usize) -> f32;
}
/// `fmaxv` over a `f32` slice (alias of `maxv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fmaxv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fminv_f32(a: *const f32, len: usize) -> f32;
}
/// `fminv` over a `f32` slice (alias of `minv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fminv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fmaxnmv_f32(a: *const f32, len: usize) -> f32;
}
/// `fmaxnmv` over a `f32` slice (alias of `maxnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxnmv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fmaxnmv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fminnmv_f32(a: *const f32, len: usize) -> f32;
}
/// `fminnmv` over a `f32` slice (alias of `minnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminnmv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fminnmv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fabs_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `fabs` on `f64` slices (alias of `abs`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fabs_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fabs_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fneg_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `fneg` on `f64` slices (alias of `neg`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fneg_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fneg_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fsqrt_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `fsqrt` on `f64` slices (alias of `sqrt`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fsqrt_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fsqrt_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frecpe_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frecpe` on `f64` slices (alias of `recpe`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frecpe_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frecpe_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frinta_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frinta` on `f64` slices (alias of `rinta`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frinta_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frinta_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frinti_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frinti` on `f64` slices (alias of `rinti`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frinti_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frinti_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frintx_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frintx` on `f64` slices (alias of `rintx`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintx_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintx_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frintm_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frintm` on `f64` slices (alias of `floor`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintm_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintm_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frintn_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frintn` on `f64` slices (alias of `rint`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintn_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintn_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frintp_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frintp` on `f64` slices (alias of `ceil`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintp_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintp_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frintz_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frintz` on `f64` slices (alias of `trunc`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintz_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintz_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rintm_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `rintm` on `f64` slices (alias of `floor`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintm_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintm_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rintn_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `rintn` on `f64` slices (alias of `rint`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintn_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintn_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rintp_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `rintp` on `f64` slices (alias of `ceil`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintp_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintp_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rintz_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `rintz` on `f64` slices (alias of `trunc`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintz_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintz_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fadd_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fadd` on `f64` slices (alias of `add`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fadd_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fadd_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fsub_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fsub` on `f64` slices (alias of `sub`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fsub_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fsub_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmul_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmul` on `f64` slices (alias of `mul`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmul_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmul_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fdiv_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fdiv` on `f64` slices (alias of `div`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fdiv_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fdiv_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmax_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmax` on `f64` slices (alias of `max`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmax_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmax_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmin_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmin` on `f64` slices (alias of `min`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmin_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmin_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmaxnm_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmaxnm` on `f64` slices (alias of `maxnm`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxnm_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmaxnm_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fminnm_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fminnm` on `f64` slices (alias of `minnm`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminnm_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fminnm_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fabd_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fabd` on `f64` slices (alias of `abd`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fabd_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fabd_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmulx_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmulx` on `f64` slices (alias of `mulx`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmulx_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmulx_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frecps_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `frecps` on `f64` slices (alias of `recps`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frecps_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frecps_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frsqrts_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `frsqrts` on `f64` slices (alias of `rsqrts`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frsqrts_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frsqrts_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fscale_f64(a: *const f64, b: *const i64, out: *mut f64, len: usize);
}
/// `fscale` on `f64` slices with `i64` exponents (alias of `scale`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fscale_f64(a: &[f64], b: &[i64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fscale_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmla_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fmla` on `f64` slices (alias of `fma`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmla_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmla_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmls_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fmls` on `f64` slices (alias of `mls`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmls_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmls_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_faddv_f64(a: *const f64, len: usize) -> f64;
}
/// `faddv` over a `f64` slice (alias of `addv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn faddv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_faddv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fmaxv_f64(a: *const f64, len: usize) -> f64;
}
/// `fmaxv` over a `f64` slice (alias of `maxv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fmaxv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fminv_f64(a: *const f64, len: usize) -> f64;
}
/// `fminv` over a `f64` slice (alias of `minv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fminv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fmaxnmv_f64(a: *const f64, len: usize) -> f64;
}
/// `fmaxnmv` over a `f64` slice (alias of `maxnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxnmv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fmaxnmv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fminnmv_f64(a: *const f64, len: usize) -> f64;
}
/// `fminnmv` over a `f64` slice (alias of `minnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminnmv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fminnmv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_abd_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `abd` on `f32` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn abd_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_abd_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_mulx_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `mulx` on `f32` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mulx_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mulx_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_abd_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `abd` on `f64` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn abd_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_abd_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_mulx_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `mulx` on `f64` slices.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mulx_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mulx_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
