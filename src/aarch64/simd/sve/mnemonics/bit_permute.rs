//! SVE bit deposit/extract/group and byte-wise ternary instructions.

unsafe extern "C" {
    fn rust_isa_sve_bdep_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `bdep`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn bdep_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_bdep_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_bext_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `bext`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn bext_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_bext_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_bgrp_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `bgrp`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn bgrp_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_bgrp_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_eorbt_u32(
        a: *const u32,
        b: *const u32,
        c: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `eorbt`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn eorbt_u32(a: &[u32], b: &[u32], c: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), c.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_eorbt_u32(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_eortb_u32(
        a: *const u32,
        b: *const u32,
        c: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `eortb`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn eortb_u32(a: &[u32], b: &[u32], c: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), c.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_eortb_u32(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
