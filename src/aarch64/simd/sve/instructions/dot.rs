//! SVE dot product instructions.

unsafe extern "C" {
    fn rust_isa_sve_sqrdmlah_i32(
        a: *const i32,
        b: *const i32,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sqrdmlah` on `i32` slices (scalar fallback).
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
pub unsafe fn sqrdmlah_i32(a: &[i32], b: &[i32], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqrdmlah_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqrdmlsh_i32(
        a: *const i32,
        b: *const i32,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sqrdmlsh` on `i32` slices (scalar fallback).
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
pub unsafe fn sqrdmlsh_i32(a: &[i32], b: &[i32], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqrdmlsh_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sdot_i32(
        a: *const i8,
        b: *const i8,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sdot` on `i32` slices (scalar fallback).
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
pub unsafe fn sdot_i32(a: &[i8], b: &[i8], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(
        a.len(),
        acc.len() * 4,
        "SVE input must contain four elements per output"
    );
    assert_eq!(out.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sdot_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_udot_i32(
        a: *const u8,
        b: *const u8,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `udot` on `i32` slices (scalar fallback).
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
pub unsafe fn udot_i32(a: &[u8], b: &[u8], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(
        a.len(),
        acc.len() * 4,
        "SVE input must contain four elements per output"
    );
    assert_eq!(out.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_udot_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sudot_i32(
        a: *const i8,
        b: *const u8,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sudot` on `i32` slices (scalar fallback).
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
pub unsafe fn sudot_i32(a: &[i8], b: &[u8], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(
        a.len(),
        acc.len() * 4,
        "SVE input must contain four elements per output"
    );
    assert_eq!(out.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sudot_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_usdot_i32(
        a: *const u8,
        b: *const i8,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `usdot` on `i32` slices (scalar fallback).
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
pub unsafe fn usdot_i32(a: &[u8], b: &[i8], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(
        a.len(),
        acc.len() * 4,
        "SVE input must contain four elements per output"
    );
    assert_eq!(out.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_usdot_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
