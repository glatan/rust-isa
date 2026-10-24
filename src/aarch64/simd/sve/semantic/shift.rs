//! SVE shift operations.

unsafe extern "C" {
    fn rust_isa_sve_lsl_i32(a: *const i32, shift: i32, out: *mut i32, len: usize);
}

/// Logical left shift of `i32` slices.
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
pub unsafe fn lsl_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_lsl_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_lsr_i32(a: *const i32, shift: i32, out: *mut i32, len: usize);
}

/// Logical right shift of `i32` slices.
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
pub unsafe fn lsr_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_lsr_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_asr_i32(a: *const i32, shift: i32, out: *mut i32, len: usize);
}

/// Arithmetic right shift of `i32` slices.
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
pub unsafe fn asr_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_asr_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
