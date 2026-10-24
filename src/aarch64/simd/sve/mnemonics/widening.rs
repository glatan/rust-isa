//! SVE widening instructions.

unsafe extern "C" {
    fn rust_isa_sve_saddlb_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `saddlb` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn saddlb_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_saddlb_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_saddlt_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `saddlt` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn saddlt_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_saddlt_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_saddlbt_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `saddlbt` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn saddlbt_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_saddlbt_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ssublb_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `ssublb` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn ssublb_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ssublb_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ssublt_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `ssublt` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn ssublt_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ssublt_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ssublbt_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `ssublbt` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn ssublbt_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ssublbt_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ssubltb_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `ssubltb` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn ssubltb_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ssubltb_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_saddwb_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `saddwb` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn saddwb_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_saddwb_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_saddwt_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `saddwt` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn saddwt_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_saddwt_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ssubwb_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `ssubwb` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn ssubwb_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ssubwb_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ssubwt_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `ssubwt` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn ssubwt_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ssubwt_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uaddlb_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `uaddlb` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn uaddlb_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uaddlb_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uaddlt_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `uaddlt` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn uaddlt_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uaddlt_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_usublb_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `usublb` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn usublb_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_usublb_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_usublt_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `usublt` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn usublt_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_usublt_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uaddwb_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `uaddwb` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn uaddwb_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uaddwb_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uaddwt_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `uaddwt` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn uaddwt_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uaddwt_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_usubwb_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `usubwb` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn usubwb_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_usubwb_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_usubwt_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `usubwt` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn usubwt_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_usubwt_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sabdlb_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `sabdlb` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn sabdlb_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sabdlb_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sabdlt_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `sabdlt` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn sabdlt_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sabdlt_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uabdlb_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `uabdlb` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn uabdlb_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uabdlb_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uabdlt_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `uabdlt` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn uabdlt_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uabdlt_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_smullb_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `smullb` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn smullb_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smullb_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_smullt_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `smullt` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn smullt_i8(a: &[i8], b: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smullt_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_umullb_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `umullb` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn umullb_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umullb_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_umullt_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `umullt` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn umullt_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umullt_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqdmullb_i16(a: *const i16, b: *const i16, out: *mut i32, len: usize);
}
/// `sqdmullb` from `i16` to `i32` slices (scalar fallback).
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
pub unsafe fn sqdmullb_i16(a: &[i16], b: &[i16], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqdmullb_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqdmullt_i16(a: *const i16, b: *const i16, out: *mut i32, len: usize);
}
/// `sqdmullt` from `i16` to `i32` slices (scalar fallback).
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
pub unsafe fn sqdmullt_i16(a: &[i16], b: &[i16], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqdmullt_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_smlalb_i8(
        a: *const i8,
        b: *const i8,
        acc: *const i16,
        out: *mut i16,
        len: usize,
    );
}
/// `smlalb` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn smlalb_i8(a: &[i8], b: &[i8], acc: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smlalb_i8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_smlalt_i8(
        a: *const i8,
        b: *const i8,
        acc: *const i16,
        out: *mut i16,
        len: usize,
    );
}
/// `smlalt` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn smlalt_i8(a: &[i8], b: &[i8], acc: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smlalt_i8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_smlslb_i8(
        a: *const i8,
        b: *const i8,
        acc: *const i16,
        out: *mut i16,
        len: usize,
    );
}
/// `smlslb` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn smlslb_i8(a: &[i8], b: &[i8], acc: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smlslb_i8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_smlslt_i8(
        a: *const i8,
        b: *const i8,
        acc: *const i16,
        out: *mut i16,
        len: usize,
    );
}
/// `smlslt` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn smlslt_i8(a: &[i8], b: &[i8], acc: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smlslt_i8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_umlalb_u8(
        a: *const u8,
        b: *const u8,
        acc: *const u16,
        out: *mut u16,
        len: usize,
    );
}
/// `umlalb` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn umlalb_u8(a: &[u8], b: &[u8], acc: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umlalb_u8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_umlalt_u8(
        a: *const u8,
        b: *const u8,
        acc: *const u16,
        out: *mut u16,
        len: usize,
    );
}
/// `umlalt` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn umlalt_u8(a: &[u8], b: &[u8], acc: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umlalt_u8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_umlslb_u8(
        a: *const u8,
        b: *const u8,
        acc: *const u16,
        out: *mut u16,
        len: usize,
    );
}
/// `umlslb` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn umlslb_u8(a: &[u8], b: &[u8], acc: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umlslb_u8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_umlslt_u8(
        a: *const u8,
        b: *const u8,
        acc: *const u16,
        out: *mut u16,
        len: usize,
    );
}
/// `umlslt` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn umlslt_u8(a: &[u8], b: &[u8], acc: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umlslt_u8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqdmlalb_i16(
        a: *const i16,
        b: *const i16,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sqdmlalb` from `i16` to `i32` slices (scalar fallback).
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
pub unsafe fn sqdmlalb_i16(a: &[i16], b: &[i16], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqdmlalb_i16(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqdmlalt_i16(
        a: *const i16,
        b: *const i16,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sqdmlalt` from `i16` to `i32` slices (scalar fallback).
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
pub unsafe fn sqdmlalt_i16(a: &[i16], b: &[i16], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqdmlalt_i16(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqdmlalbt_i16(
        a: *const i16,
        b: *const i16,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sqdmlalbt` from `i16` to `i32` slices (scalar fallback).
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
pub unsafe fn sqdmlalbt_i16(a: &[i16], b: &[i16], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqdmlalbt_i16(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqdmlslb_i16(
        a: *const i16,
        b: *const i16,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sqdmlslb` from `i16` to `i32` slices (scalar fallback).
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
pub unsafe fn sqdmlslb_i16(a: &[i16], b: &[i16], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqdmlslb_i16(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqdmlslt_i16(
        a: *const i16,
        b: *const i16,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sqdmlslt` from `i16` to `i32` slices (scalar fallback).
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
pub unsafe fn sqdmlslt_i16(a: &[i16], b: &[i16], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqdmlslt_i16(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sqdmlslbt_i16(
        a: *const i16,
        b: *const i16,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sqdmlslbt` from `i16` to `i32` slices (scalar fallback).
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
pub unsafe fn sqdmlslbt_i16(a: &[i16], b: &[i16], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqdmlslbt_i16(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sadalp_i16(a: *const i16, out: *mut i32, len: usize);
}
/// `sadalp` from `i16` to `i32` slices (scalar fallback).
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
pub unsafe fn sadalp_i16(a: &[i16], out: &mut [i32]) {
    assert_eq!(
        a.len(),
        out.len() * 2,
        "SVE input must contain two elements per output"
    );
    unsafe {
        rust_isa_sve_sadalp_i16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uadalp_u16(a: *const u16, out: *mut u32, len: usize);
}
/// `uadalp` from `u16` to `u32` slices (scalar fallback).
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
pub unsafe fn uadalp_u16(a: &[u16], out: &mut [u32]) {
    assert_eq!(
        a.len(),
        out.len() * 2,
        "SVE input must contain two elements per output"
    );
    unsafe {
        rust_isa_sve_uadalp_u16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sabalb_i8(
        a: *const i8,
        b: *const i8,
        acc: *const i16,
        out: *mut i16,
        len: usize,
    );
}
/// `sabalb` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn sabalb_i8(a: &[i8], b: &[i8], acc: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sabalb_i8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sabalt_i8(
        a: *const i8,
        b: *const i8,
        acc: *const i16,
        out: *mut i16,
        len: usize,
    );
}
/// `sabalt` from `i8` to `i16` slices (scalar fallback).
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
pub unsafe fn sabalt_i8(a: &[i8], b: &[i8], acc: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sabalt_i8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uabalb_u8(
        a: *const u8,
        b: *const u8,
        acc: *const u16,
        out: *mut u16,
        len: usize,
    );
}
/// `uabalb` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn uabalb_u8(a: &[u8], b: &[u8], acc: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uabalb_u8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uabalt_u8(
        a: *const u8,
        b: *const u8,
        acc: *const u16,
        out: *mut u16,
        len: usize,
    );
}
/// `uabalt` from `u8` to `u16` slices (scalar fallback).
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
pub unsafe fn uabalt_u8(a: &[u8], b: &[u8], acc: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uabalt_u8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
