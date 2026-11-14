//! SVE halving/subtract-narrow instructions.

unsafe extern "C" {
    fn rust_isa_sve_subhnb_i16(a: *const i16, b: *const i16, out: *mut i8, len: usize);
}
/// `subhnb`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn subhnb_i16(a: &[i16], b: &[i16], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    assert_eq!(b.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_subhnb_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_subhnt_i16(a: *const i16, b: *const i16, out: *mut i8, len: usize);
}
/// `subhnt`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn subhnt_i16(a: &[i16], b: &[i16], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    assert_eq!(b.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_subhnt_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

/// `shsubr` on `i32` lanes.
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
pub unsafe fn shsubr_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for ((out_elem, b_elem), a_elem) in out.iter_mut().zip(b.iter()).zip(a.iter()) {
        *out_elem = b_elem.wrapping_sub(*a_elem) >> 1;
    }
}

/// `uhsubr` on `u32` lanes.
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
pub unsafe fn uhsubr_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for ((out_elem, b_elem), a_elem) in out.iter_mut().zip(b.iter()).zip(a.iter()) {
        *out_elem = b_elem.wrapping_sub(*a_elem) >> 1;
    }
}
