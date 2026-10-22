//! SVE trigonometric multiply instructions.

unsafe extern "C" {
    fn rust_isa_sve_ftsmul_f32(a: *const f32, b: *const u32, out: *mut f32, len: usize);
}
/// `ftsmul` on `f32` slices.
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
pub unsafe fn ftsmul_f32(a: &[f32], b: &[u32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ftsmul_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ftssel_f32(a: *const f32, b: *const u32, out: *mut f32, len: usize);
}
/// `ftssel` on `f32` slices.
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
pub unsafe fn ftssel_f32(a: &[f32], b: &[u32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ftssel_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ftmad_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `ftmad` on `f32` slices.
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
pub unsafe fn ftmad_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ftmad_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
