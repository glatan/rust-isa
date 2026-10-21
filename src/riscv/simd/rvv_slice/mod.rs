//! RISC-V V slice API.
//!
//! The V extension is unstable. The caller must compile the whole crate with
//! `-C target-feature=+v`; the wrappers do not carry `#[target_feature]`.
//!
//! SVE-like runtime vector lengths do not fit [`V128`]. These functions take
//! slices and let the C loop choose VL with `vsetvl`. All slices must have the
//! same length; a different length panics. The functions do not allocate.
//!
//! The fixed-length [`V128`](crate::riscv::simd::rv_vector::V128) API is
//! separate from this slice API.

unsafe extern "C" {
    fn rust_isa_rvv_slice_add_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_rvv_slice_mul_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_rvv_slice_add_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_mul_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}

/// Add two `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn add_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "RVV slices must have the same length");
    assert_eq!(a.len(), out.len(), "RVV slices must have the same length");
    unsafe {
        rust_isa_rvv_slice_add_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
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
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn mul_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "RVV slices must have the same length");
    assert_eq!(a.len(), out.len(), "RVV slices must have the same length");
    unsafe {
        rust_isa_rvv_slice_mul_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
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
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn add_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "RVV slices must have the same length");
    assert_eq!(a.len(), out.len(), "RVV slices must have the same length");
    unsafe {
        rust_isa_rvv_slice_add_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
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
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn mul_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "RVV slices must have the same length");
    assert_eq!(a.len(), out.len(), "RVV slices must have the same length");
    unsafe {
        rust_isa_rvv_slice_mul_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
