//! SVE matrix multiply-accumulate instructions.

unsafe extern "C" {
    fn rust_isa_sve_smmla_i32(
        a: *const i8,
        b: *const i8,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `smmla` on `i32` slices (scalar fallback).
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
pub unsafe fn smmla_i32(a: &[i8], b: &[i8], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(
        a.len(),
        acc.len() * 4,
        "SVE input must contain four elements per output"
    );
    assert_eq!(out.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smmla_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ummla_u32(
        a: *const u8,
        b: *const u8,
        acc: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `ummla` on `u32` slices (scalar fallback).
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
pub unsafe fn ummla_u32(a: &[u8], b: &[u8], acc: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(
        a.len(),
        acc.len() * 4,
        "SVE input must contain four elements per output"
    );
    assert_eq!(out.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ummla_u32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_usmmla_i32(
        a: *const u8,
        b: *const i8,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `usmmla` on `i32` slices (scalar fallback).
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
pub unsafe fn usmmla_i32(a: &[u8], b: &[i8], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(
        a.len(),
        acc.len() * 4,
        "SVE input must contain four elements per output"
    );
    assert_eq!(out.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_usmmla_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
