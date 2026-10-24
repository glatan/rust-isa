//! SVE narrowing instructions.

unsafe extern "C" {
    fn rust_isa_sve_sqxtunb_i32(a: *const i32, out: *mut u16, len: usize);
}
/// `sqxtunb` from `i32` to `u16` slices (scalar fallback).
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
pub unsafe fn sqxtunb_i32(a: &[i32], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqxtunb_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqxtunt_i32(acc: *const u16, a: *const i32, out: *mut u16, len: usize);
}
/// `sqxtunt` from `i32` to `u16` slices (scalar fallback).
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
pub unsafe fn sqxtunt_i32(acc: &[u16], a: &[i32], out: &mut [u16]) {
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqxtunt_i32(acc.as_ptr(), a.as_ptr(), out.as_mut_ptr(), acc.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_xtn_i32(a: *const i32, out: *mut i16, len: usize);
}
/// `xtn` from `i32` to `i16` slices (scalar fallback).
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
pub unsafe fn xtn_i32(a: &[i32], out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_xtn_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_raddhnb_i32(a: *const i32, b: *const i32, out: *mut i16, len: usize);
}
/// `raddhnb` from `i32` to `i16` slices (scalar fallback).
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
pub unsafe fn raddhnb_i32(a: &[i32], b: &[i32], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_raddhnb_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_raddhnt_i32(a: *const i32, b: *const i32, out: *mut i16, len: usize);
}
/// `raddhnt` from `i32` to `i16` slices (scalar fallback).
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
pub unsafe fn raddhnt_i32(a: &[i32], b: &[i32], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_raddhnt_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rsubhnb_i32(a: *const i32, b: *const i32, out: *mut i16, len: usize);
}
/// `rsubhnb` from `i32` to `i16` slices (scalar fallback).
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
pub unsafe fn rsubhnb_i32(a: &[i32], b: &[i32], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsubhnb_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rsubhnt_i32(a: *const i32, b: *const i32, out: *mut i16, len: usize);
}
/// `rsubhnt` from `i32` to `i16` slices (scalar fallback).
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
pub unsafe fn rsubhnt_i32(a: &[i32], b: &[i32], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsubhnt_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqshrnb_i32(a: *const i32, shift: i32, out: *mut i16, len: usize);
}
/// `sqshrnb` from `i32` to `i16` slices (scalar fallback).
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
pub unsafe fn sqshrnb_i32(a: &[i32], shift: i32, out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqshrnb_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqshrnt_i32(
        acc: *const i16,
        a: *const i32,
        shift: i32,
        out: *mut i16,
        len: usize,
    );
}
/// `sqshrnt` from `i32` to `i16` slices (scalar fallback).
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
pub unsafe fn sqshrnt_i32(acc: &[i16], a: &[i32], shift: i32, out: &mut [i16]) {
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqshrnt_i32(acc.as_ptr(), a.as_ptr(), shift, out.as_mut_ptr(), acc.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqrshrnb_i32(a: *const i32, shift: i32, out: *mut i16, len: usize);
}
/// `sqrshrnb` from `i32` to `i16` slices (scalar fallback).
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
pub unsafe fn sqrshrnb_i32(a: &[i32], shift: i32, out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqrshrnb_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqrshrnt_i32(
        acc: *const i16,
        a: *const i32,
        shift: i32,
        out: *mut i16,
        len: usize,
    );
}
/// `sqrshrnt` from `i32` to `i16` slices (scalar fallback).
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
pub unsafe fn sqrshrnt_i32(acc: &[i16], a: &[i32], shift: i32, out: &mut [i16]) {
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqrshrnt_i32(acc.as_ptr(), a.as_ptr(), shift, out.as_mut_ptr(), acc.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqshrunb_i32(a: *const i32, shift: i32, out: *mut u16, len: usize);
}
/// `sqshrunb` from `i32` to `u16` slices (scalar fallback).
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
pub unsafe fn sqshrunb_i32(a: &[i32], shift: i32, out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqshrunb_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqshrunt_i32(
        acc: *const u16,
        a: *const i32,
        shift: i32,
        out: *mut u16,
        len: usize,
    );
}
/// `sqshrunt` from `i32` to `u16` slices (scalar fallback).
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
pub unsafe fn sqshrunt_i32(acc: &[u16], a: &[i32], shift: i32, out: &mut [u16]) {
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqshrunt_i32(acc.as_ptr(), a.as_ptr(), shift, out.as_mut_ptr(), acc.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqrshrunb_i32(a: *const i32, shift: i32, out: *mut u16, len: usize);
}
/// `sqrshrunb` from `i32` to `u16` slices (scalar fallback).
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
pub unsafe fn sqrshrunb_i32(a: &[i32], shift: i32, out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqrshrunb_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqrshrunt_i32(
        acc: *const u16,
        a: *const i32,
        shift: i32,
        out: *mut u16,
        len: usize,
    );
}
/// `sqrshrunt` from `i32` to `u16` slices (scalar fallback).
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
pub unsafe fn sqrshrunt_i32(acc: &[u16], a: &[i32], shift: i32, out: &mut [u16]) {
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqrshrunt_i32(acc.as_ptr(), a.as_ptr(), shift, out.as_mut_ptr(), acc.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uqshrnb_u32(a: *const u32, shift: i32, out: *mut u16, len: usize);
}
/// `uqshrnb` from `u32` to `u16` slices (scalar fallback).
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
pub unsafe fn uqshrnb_u32(a: &[u32], shift: i32, out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqshrnb_u32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uqshrnt_u32(
        acc: *const u16,
        a: *const u32,
        shift: i32,
        out: *mut u16,
        len: usize,
    );
}
/// `uqshrnt` from `u32` to `u16` slices (scalar fallback).
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
pub unsafe fn uqshrnt_u32(acc: &[u16], a: &[u32], shift: i32, out: &mut [u16]) {
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqshrnt_u32(acc.as_ptr(), a.as_ptr(), shift, out.as_mut_ptr(), acc.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uqrshrnb_u32(a: *const u32, shift: i32, out: *mut u16, len: usize);
}
/// `uqrshrnb` from `u32` to `u16` slices (scalar fallback).
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
pub unsafe fn uqrshrnb_u32(a: &[u32], shift: i32, out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqrshrnb_u32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uqrshrnt_u32(
        acc: *const u16,
        a: *const u32,
        shift: i32,
        out: *mut u16,
        len: usize,
    );
}
/// `uqrshrnt` from `u32` to `u16` slices (scalar fallback).
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
pub unsafe fn uqrshrnt_u32(acc: &[u16], a: &[u32], shift: i32, out: &mut [u16]) {
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqrshrnt_u32(acc.as_ptr(), a.as_ptr(), shift, out.as_mut_ptr(), acc.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqxtnb_i32(a: *const i32, out: *mut i16, len: usize);
}
/// `sqxtnb` from `i32` to `i16` slices (scalar fallback).
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
pub unsafe fn sqxtnb_i32(a: &[i32], out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqxtnb_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqxtnt_i32(acc: *const i16, a: *const i32, out: *mut i16, len: usize);
}
/// `sqxtnt` from `i32` to `i16` slices (scalar fallback).
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
pub unsafe fn sqxtnt_i32(acc: &[i16], a: &[i32], out: &mut [i16]) {
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqxtnt_i32(acc.as_ptr(), a.as_ptr(), out.as_mut_ptr(), acc.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uqxtnb_u32(a: *const u32, out: *mut u16, len: usize);
}
/// `uqxtnb` from `u32` to `u16` slices (scalar fallback).
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
pub unsafe fn uqxtnb_u32(a: &[u32], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqxtnb_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uqxtnt_u32(acc: *const u16, a: *const u32, out: *mut u16, len: usize);
}
/// `uqxtnt` from `u32` to `u16` slices (scalar fallback).
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
pub unsafe fn uqxtnt_u32(acc: &[u16], a: &[u32], out: &mut [u16]) {
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqxtnt_u32(acc.as_ptr(), a.as_ptr(), out.as_mut_ptr(), acc.len());
    }
}
