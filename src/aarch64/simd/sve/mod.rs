//! AArch64 SVE slice API.
//!
//! SVE is not part of the AArch64 baseline. Every operation is `unsafe` and the
//! caller must enable SVE with `#[target_feature(enable = "sve")]` or a global
//! target feature.
//!
//! SVE vectors have a runtime vector length, so this module exposes slice
//! operations rather than per-register values. The C loop chooses the vector
//! length at run time. All slices must have the same length; a different length
//! panics. The functions do not allocate.
//!
//! The per-instruction API is intentionally omitted because a fixed-length
//! [`V128`](crate::aarch64::simd::neon::V128) cannot represent an SVE register.

unsafe extern "C" {
    fn rust_isa_sve_add_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_mul_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_add_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_mul_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_add_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_mul_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}

/// Add two `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Multiply two `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Add two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Multiply two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Add two `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Multiply two `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
