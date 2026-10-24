//! SVE conversion instructions.

unsafe extern "C" {
    fn rust_isa_sve_fcvt_f32_f64(a: *const f64, out: *mut f32, len: usize);
}
/// Convert `f64` slices to `f32` (scalar fallback).
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
pub unsafe fn fcvt_f32_f64(a: &[f64], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvt_f32_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fcvt_f64_f32(a: *const f32, out: *mut f64, len: usize);
}
/// Convert `f32` slices to `f64` (scalar fallback).
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
pub unsafe fn fcvt_f64_f32(a: &[f32], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvt_f64_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fcvtzs_i32_f32(a: *const f32, out: *mut i32, len: usize);
}
/// `fcvtzs` from `f32` to `i32` (scalar fallback).
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
pub unsafe fn fcvtzs_i32_f32(a: &[f32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzs_i32_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fcvtzs_i32_f64(a: *const f64, out: *mut i32, len: usize);
}
/// `fcvtzs` from `f64` to `i32` (scalar fallback).
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
pub unsafe fn fcvtzs_i32_f64(a: &[f64], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzs_i32_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fcvtzs_i64_f32(a: *const f32, out: *mut i64, len: usize);
}
/// `fcvtzs` from `f32` to `i64` (scalar fallback).
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
pub unsafe fn fcvtzs_i64_f32(a: &[f32], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzs_i64_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fcvtzs_i64_f64(a: *const f64, out: *mut i64, len: usize);
}
/// `fcvtzs` from `f64` to `i64` (scalar fallback).
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
pub unsafe fn fcvtzs_i64_f64(a: &[f64], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzs_i64_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fcvtzu_u32_f32(a: *const f32, out: *mut u32, len: usize);
}
/// `fcvtzu` from `f32` to `u32` (scalar fallback).
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
pub unsafe fn fcvtzu_u32_f32(a: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzu_u32_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fcvtzu_u32_f64(a: *const f64, out: *mut u32, len: usize);
}
/// `fcvtzu` from `f64` to `u32` (scalar fallback).
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
pub unsafe fn fcvtzu_u32_f64(a: &[f64], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzu_u32_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fcvtzu_u64_f32(a: *const f32, out: *mut u64, len: usize);
}
/// `fcvtzu` from `f32` to `u64` (scalar fallback).
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
pub unsafe fn fcvtzu_u64_f32(a: &[f32], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzu_u64_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fcvtzu_u64_f64(a: *const f64, out: *mut u64, len: usize);
}
/// `fcvtzu` from `f64` to `u64` (scalar fallback).
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
pub unsafe fn fcvtzu_u64_f64(a: &[f64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzu_u64_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_scvtf_f32_i32(a: *const i32, out: *mut f32, len: usize);
}
/// `scvtf` from `i32` to `f32` (scalar fallback).
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
pub unsafe fn scvtf_f32_i32(a: &[i32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scvtf_f32_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_scvtf_f32_i64(a: *const i64, out: *mut f32, len: usize);
}
/// `scvtf` from `i64` to `f32` (scalar fallback).
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
pub unsafe fn scvtf_f32_i64(a: &[i64], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scvtf_f32_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_scvtf_f64_i32(a: *const i32, out: *mut f64, len: usize);
}
/// `scvtf` from `i32` to `f64` (scalar fallback).
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
pub unsafe fn scvtf_f64_i32(a: &[i32], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scvtf_f64_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_scvtf_f64_i64(a: *const i64, out: *mut f64, len: usize);
}
/// `scvtf` from `i64` to `f64` (scalar fallback).
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
pub unsafe fn scvtf_f64_i64(a: &[i64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scvtf_f64_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ucvtf_f32_u32(a: *const u32, out: *mut f32, len: usize);
}
/// `ucvtf` from `u32` to `f32` (scalar fallback).
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
pub unsafe fn ucvtf_f32_u32(a: &[u32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ucvtf_f32_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ucvtf_f32_u64(a: *const u64, out: *mut f32, len: usize);
}
/// `ucvtf` from `u64` to `f32` (scalar fallback).
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
pub unsafe fn ucvtf_f32_u64(a: &[u64], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ucvtf_f32_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ucvtf_f64_u32(a: *const u32, out: *mut f64, len: usize);
}
/// `ucvtf` from `u32` to `f64` (scalar fallback).
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
pub unsafe fn ucvtf_f64_u32(a: &[u32], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ucvtf_f64_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ucvtf_f64_u64(a: *const u64, out: *mut f64, len: usize);
}
/// `ucvtf` from `u64` to `f64` (scalar fallback).
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
pub unsafe fn ucvtf_f64_u64(a: &[u64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ucvtf_f64_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
