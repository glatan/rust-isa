//! SVE bitwise instructions.

unsafe extern "C" {
    fn rust_isa_sve_and_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `and` on `i8` slices.
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
pub unsafe fn and_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_and_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_orr_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `orr` on `i8` slices.
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
pub unsafe fn orr_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_orr_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_eor_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `eor` on `i8` slices.
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
pub unsafe fn eor_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_eor_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_and_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `and` on `i16` slices.
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
pub unsafe fn and_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_and_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_orr_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `orr` on `i16` slices.
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
pub unsafe fn orr_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_orr_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_eor_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `eor` on `i16` slices.
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
pub unsafe fn eor_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_eor_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_and_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `and` on `i64` slices.
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
pub unsafe fn and_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_and_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_orr_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `orr` on `i64` slices.
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
pub unsafe fn orr_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_orr_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_eor_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `eor` on `i64` slices.
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
pub unsafe fn eor_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_eor_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_bic_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `bic` on `i32` slices.
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
pub unsafe fn bic_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_bic_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_nand_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `nand` on `i32` slices.
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
pub unsafe fn nand_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_nand_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_nor_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `nor` on `i32` slices.
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
pub unsafe fn nor_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_nor_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_orn_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `orn` on `i32` slices.
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
pub unsafe fn orn_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_orn_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_bsl_i32(a: *const i32, b: *const i32, c: *const i32, out: *mut i32, len: usize);
}
/// `bsl` on `i32` slices (scalar fallback).
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
pub unsafe fn bsl_i32(a: &[i32], b: &[i32], c: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), c.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_bsl_i32(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
