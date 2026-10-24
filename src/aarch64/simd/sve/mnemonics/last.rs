//! SVE last-active element instructions.

unsafe extern "C" {
    fn rust_isa_sve_lasta_i32(a: *const i32, len: usize) -> i32;
}
/// `lasta` of an `i32` slice (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lasta_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_lasta_i32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_lastb_i32(a: *const i32, len: usize) -> i32;
}
/// `lastb` of an `i32` slice (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lastb_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_lastb_i32(a.as_ptr(), a.len()) }
}
