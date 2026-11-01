//! SVE narrowing shift instructions.

unsafe extern "C" {
    fn rust_isa_sve_shrnb_i16(a: *const i16, out: *mut i8, len: usize);
}
/// `shrnb` on `i16` sources narrowing to `i8`.
///
/// # Panics
///
/// Panics when the source slice is too short.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shrnb_i16(a: &[i16], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_shrnb_i16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_shrnt_i16(a: *const i16, out: *mut i8, len: usize);
}
/// `shrnt` on `i16` sources narrowing to `i8`.
///
/// # Panics
///
/// Panics when the source slice is too short.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shrnt_i16(a: &[i16], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_shrnt_i16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rshrnb_i16(a: *const i16, out: *mut i8, len: usize);
}
/// `rshrnb` on `i16` sources narrowing to `i8`.
///
/// # Panics
///
/// Panics when the source slice is too short.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn rshrnb_i16(a: &[i16], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rshrnb_i16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rshrnt_i16(a: *const i16, out: *mut i8, len: usize);
}
/// `rshrnt` on `i16` sources narrowing to `i8`.
///
/// # Panics
///
/// Panics when the source slice is too short.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn rshrnt_i16(a: &[i16], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rshrnt_i16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

/// `sshllb` on `i8` sources widening to `i16`.
///
/// # Panics
///
/// Panics when the source slice is too short.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sshllb_i16(a: &[i8], shift: i32, out: &mut [i16]) {
    assert!(a.len() >= out.len() * 2, "SVE source is too short");
    assert!((0..16).contains(&shift), "shift must be in 0..15");
    for lane in 0..out.len() {
        out[lane] = (a[lane * 2] as i16) << shift;
    }
}

/// `sshllt` on `i8` sources widening to `i16`.
///
/// # Panics
///
/// Panics when the source slice is too short.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sshllt_i16(a: &[i8], shift: i32, out: &mut [i16]) {
    assert!(a.len() >= out.len() * 2, "SVE source is too short");
    assert!((0..16).contains(&shift), "shift must be in 0..15");
    for lane in 0..out.len() {
        out[lane] = (a[lane * 2 + 1] as i16) << shift;
    }
}

/// `ushllb` on `u8` sources widening to `u16`.
///
/// # Panics
///
/// Panics when the source slice is too short.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ushllb_u16(a: &[u8], shift: i32, out: &mut [u16]) {
    assert!(a.len() >= out.len() * 2, "SVE source is too short");
    assert!((0..16).contains(&shift), "shift must be in 0..15");
    for lane in 0..out.len() {
        out[lane] = (a[lane * 2] as u16) << shift;
    }
}

/// `ushllt` on `u8` sources widening to `u16`.
///
/// # Panics
///
/// Panics when the source slice is too short.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ushllt_u16(a: &[u8], shift: i32, out: &mut [u16]) {
    assert!(a.len() >= out.len() * 2, "SVE source is too short");
    assert!((0..16).contains(&shift), "shift must be in 0..15");
    for lane in 0..out.len() {
        out[lane] = (a[lane * 2 + 1] as u16) << shift;
    }
}
