//! SVE predicate instructions.

unsafe extern "C" {
    fn rust_isa_sve_cntb() -> u64;
}
/// Number of bytes in an SVE vector for `cntb`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cntb() -> usize {
    unsafe { rust_isa_sve_cntb() as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_cnth() -> u64;
}
/// Number of 16-bit lanes in an SVE vector for `cnth`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cnth() -> usize {
    unsafe { rust_isa_sve_cnth() as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_cntw() -> u64;
}
/// Number of 32-bit lanes in an SVE vector for `cntw`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cntw() -> usize {
    unsafe { rust_isa_sve_cntw() as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_cntd() -> u64;
}
/// Number of 64-bit lanes in an SVE vector for `cntd`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cntd() -> usize {
    unsafe { rust_isa_sve_cntd() as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_rdffr() -> u64;
}
/// Read the first-fault register; predicates are all-true in the slice API.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rdffr() -> u64 {
    unsafe { rust_isa_sve_rdffr() }
}

unsafe extern "C" {
    fn rust_isa_sve_ptest_i32(a: *const i32, len: usize) -> i32;
}
/// Test whether any `i32` lane is active.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ptest_i32(a: &[i32]) -> bool {
    unsafe { rust_isa_sve_ptest_i32(a.as_ptr(), a.len()) != 0 }
}

unsafe extern "C" {
    fn rust_isa_sve_whilelt_i32(start: usize, end: usize) -> u64;
}
/// `whilelt` for `i32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilelt_i32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilelt_i32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilelt_u32(start: usize, end: usize) -> u64;
}
/// `whilelt` for `u32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilelt_u32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilelt_u32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilele_i32(start: usize, end: usize) -> u64;
}
/// `whilele` for `i32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilele_i32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilele_i32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilele_u32(start: usize, end: usize) -> u64;
}
/// `whilele` for `u32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilele_u32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilele_u32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilelo_i32(start: usize, end: usize) -> u64;
}
/// `whilelo` for `i32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilelo_i32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilelo_i32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilelo_u32(start: usize, end: usize) -> u64;
}
/// `whilelo` for `u32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilelo_u32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilelo_u32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilels_i32(start: usize, end: usize) -> u64;
}
/// `whilels` for `i32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilels_i32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilels_i32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilels_u32(start: usize, end: usize) -> u64;
}
/// `whilels` for `u32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilels_u32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilels_u32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilegt_i32(start: usize, end: usize) -> u64;
}
/// `whilegt` for `i32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilegt_i32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilegt_i32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilegt_u32(start: usize, end: usize) -> u64;
}
/// `whilegt` for `u32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilegt_u32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilegt_u32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilege_i32(start: usize, end: usize) -> u64;
}
/// `whilege` for `i32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilege_i32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilege_i32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilege_u32(start: usize, end: usize) -> u64;
}
/// `whilege` for `u32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilege_u32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilege_u32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilehi_i32(start: usize, end: usize) -> u64;
}
/// `whilehi` for `i32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilehi_i32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilehi_i32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilehi_u32(start: usize, end: usize) -> u64;
}
/// `whilehi` for `u32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilehi_u32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilehi_u32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilehs_i32(start: usize, end: usize) -> u64;
}
/// `whilehs` for `i32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilehs_i32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilehs_i32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilehs_u32(start: usize, end: usize) -> u64;
}
/// `whilehs` for `u32` lanes; returns the number of active lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilehs_u32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilehs_u32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilerw_i32(start: usize, end: usize) -> u64;
}
/// Pointer-range predicate; the slice API treats the whole range as active.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilerw_i32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilerw_i32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_whilewr_i32(start: usize, end: usize) -> u64;
}
/// Pointer-range predicate; the slice API treats the whole range as active.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilewr_i32(start: usize, end: usize) -> usize {
    unsafe { rust_isa_sve_whilewr_i32(start, end) as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_punpkhi_u8(a: *const u8, out: *mut u16, len: usize);
}
/// `punpkhi` on `u8` slices (scalar boolean widening fallback).
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
pub unsafe fn punpkhi_u8(a: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_punpkhi_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_punpklo_u8(a: *const u8, out: *mut u16, len: usize);
}
/// `punpklo` on `u8` slices (scalar boolean widening fallback).
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
pub unsafe fn punpklo_u8(a: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_punpklo_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
