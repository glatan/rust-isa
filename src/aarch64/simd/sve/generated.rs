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
    fn rust_isa_sve_abs_i8(a: *const i8, out: *mut i8, len: usize);
}
/// `abs` on `i8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn abs_i8(a: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_abs_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_neg_i8(a: *const i8, out: *mut i8, len: usize);
}
/// `neg` on `i8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn neg_i8(a: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_neg_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_clz_i8(a: *const i8, out: *mut i8, len: usize);
}
/// `clz` on `i8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn clz_i8(a: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_clz_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cnt_i8(a: *const i8, out: *mut i8, len: usize);
}
/// `cnt` on `i8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cnt_i8(a: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cnt_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_not_i8(a: *const i8, out: *mut i8, len: usize);
}
/// `not` on `i8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn not_i8(a: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_not_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_abs_i16(a: *const i16, out: *mut i16, len: usize);
}
/// `abs` on `i16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn abs_i16(a: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_abs_i16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_neg_i16(a: *const i16, out: *mut i16, len: usize);
}
/// `neg` on `i16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn neg_i16(a: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_neg_i16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_clz_i16(a: *const i16, out: *mut i16, len: usize);
}
/// `clz` on `i16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn clz_i16(a: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_clz_i16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cnt_i16(a: *const i16, out: *mut i16, len: usize);
}
/// `cnt` on `i16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cnt_i16(a: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cnt_i16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_not_i16(a: *const i16, out: *mut i16, len: usize);
}
/// `not` on `i16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn not_i16(a: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_not_i16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_abs_i64(a: *const i64, out: *mut i64, len: usize);
}
/// `abs` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn abs_i64(a: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_abs_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_neg_i64(a: *const i64, out: *mut i64, len: usize);
}
/// `neg` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn neg_i64(a: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_neg_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_clz_i64(a: *const i64, out: *mut i64, len: usize);
}
/// `clz` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn clz_i64(a: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_clz_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cnt_i64(a: *const i64, out: *mut i64, len: usize);
}
/// `cnt` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cnt_i64(a: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cnt_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_not_i64(a: *const i64, out: *mut i64, len: usize);
}
/// `not` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn not_i64(a: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_not_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqabs_i8(a: *const i8, out: *mut i8, len: usize);
}
/// `sqabs` on `i8` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqabs_i8(a: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqabs_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqneg_i8(a: *const i8, out: *mut i8, len: usize);
}
/// `sqneg` on `i8` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqneg_i8(a: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqneg_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqabs_i16(a: *const i16, out: *mut i16, len: usize);
}
/// `sqabs` on `i16` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqabs_i16(a: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqabs_i16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqneg_i16(a: *const i16, out: *mut i16, len: usize);
}
/// `sqneg` on `i16` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqneg_i16(a: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqneg_i16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqabs_i32(a: *const i32, out: *mut i32, len: usize);
}
/// `sqabs` on `i32` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqabs_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqabs_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqneg_i32(a: *const i32, out: *mut i32, len: usize);
}
/// `sqneg` on `i32` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqneg_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqneg_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqabs_i64(a: *const i64, out: *mut i64, len: usize);
}
/// `sqabs` on `i64` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqabs_i64(a: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqabs_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqneg_i64(a: *const i64, out: *mut i64, len: usize);
}
/// `sqneg` on `i64` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqneg_i64(a: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqneg_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rbit_i32(a: *const i32, out: *mut i32, len: usize);
}
/// `rbit` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rbit_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rbit_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rev_i32(a: *const i32, out: *mut i32, len: usize);
}
/// `rev` from `i32` to `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rev_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rev_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_revb_i32(a: *const i32, out: *mut i32, len: usize);
}
/// `revb` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn revb_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_revb_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_revh_i32(a: *const i32, out: *mut i32, len: usize);
}
/// `revh` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn revh_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_revh_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_revw_i64(a: *const i64, out: *mut i64, len: usize);
}
/// `revw` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn revw_i64(a: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_revw_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cnot_i32(a: *const i32, out: *mut i32, len: usize);
}
/// `cnot` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cnot_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cnot_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sunpkhi_i8(a: *const i8, out: *mut i16, len: usize);
}
/// `sunpkhi` from `i8` to `i16` slices (scalar widening fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sunpkhi_i8(a: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sunpkhi_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sunpklo_i8(a: *const i8, out: *mut i16, len: usize);
}
/// `sunpklo` from `i8` to `i16` slices (scalar widening fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sunpklo_i8(a: &[i8], out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sunpklo_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uunpkhi_u8(a: *const u8, out: *mut u16, len: usize);
}
/// `uunpkhi` from `u8` to `u16` slices (scalar widening fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uunpkhi_u8(a: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uunpkhi_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uunpklo_u8(a: *const u8, out: *mut u16, len: usize);
}
/// `uunpklo` from `u8` to `u16` slices (scalar widening fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uunpklo_u8(a: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uunpklo_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sxtb_i8(a: *const i8, out: *mut i32, len: usize);
}
/// `sxtb` from `i8` to `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sxtb_i8(a: &[i8], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sxtb_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sxth_i16(a: *const i16, out: *mut i32, len: usize);
}
/// `sxth` from `i16` to `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sxth_i16(a: &[i16], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sxth_i16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sxtw_i32(a: *const i32, out: *mut i64, len: usize);
}
/// `sxtw` from `i32` to `i64` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sxtw_i32(a: &[i32], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sxtw_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uxtb_u8(a: *const u8, out: *mut u32, len: usize);
}
/// `uxtb` from `u8` to `u32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uxtb_u8(a: &[u8], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uxtb_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uxth_u16(a: *const u16, out: *mut u32, len: usize);
}
/// `uxth` from `u16` to `u32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uxth_u16(a: &[u16], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uxth_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uxtw_u32(a: *const u32, out: *mut u64, len: usize);
}
/// `uxtw` from `u32` to `u64` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uxtw_u32(a: &[u32], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uxtw_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_add_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `add` on `i8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sub_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `sub` on `i8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sub_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sub_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_mul_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `mul` on `i8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_add_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `add` on `i16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sub_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `sub` on `i16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sub_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sub_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_mul_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `mul` on `i16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_add_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `add` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sub_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `sub` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sub_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sub_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_mul_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `mul` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_add_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `add` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sub_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `sub` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sub_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sub_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_mul_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `mul` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_add_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `add` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sub_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `sub` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sub_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sub_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_mul_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `mul` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_add_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `add` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sub_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `sub` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sub_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sub_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_mul_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `mul` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_add_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `add` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sub_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `sub` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sub_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sub_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_mul_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `mul` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_smax_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `smax` on `i8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn smax_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smax_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_smin_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `smin` on `i8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn smin_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smin_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_smax_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `smax` on `i16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn smax_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smax_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_smin_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `smin` on `i16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn smin_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smin_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_smax_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `smax` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn smax_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smax_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_smin_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `smin` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn smin_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smin_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_umax_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `umax` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn umax_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umax_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_umin_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `umin` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn umin_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umin_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_umax_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `umax` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn umax_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umax_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_umin_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `umin` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn umin_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umin_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_umax_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `umax` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn umax_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umax_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_umin_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `umin` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn umin_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umin_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_umax_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `umax` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn umax_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umax_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_umin_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `umin` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn umin_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umin_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sabd_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `sabd` on `i8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sabd_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sabd_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sabd_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `sabd` on `i16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sabd_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sabd_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sabd_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `sabd` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sabd_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sabd_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sabd_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `sabd` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sabd_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sabd_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uabd_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `uabd` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uabd_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uabd_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uabd_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `uabd` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uabd_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uabd_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uabd_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `uabd` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uabd_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uabd_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uabd_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `uabd` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uabd_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uabd_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_smulh_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `smulh` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn smulh_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smulh_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_smulh_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `smulh` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn smulh_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smulh_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_umulh_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `umulh` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn umulh_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umulh_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_umulh_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `umulh` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn umulh_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umulh_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_and_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `and` on `i8` slices.
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
    fn rust_isa_sve_subr_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `subr` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn subr_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_subr_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_nand_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `nand` on `i32` slices.
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
    fn rust_isa_sve_addv_i8(a: *const i8, len: usize) -> i8;
}
/// Reduce `add` over a `i8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_i8(a: &[i8]) -> i8 {
    unsafe { rust_isa_sve_addv_i8(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_addv_i16(a: *const i16, len: usize) -> i16;
}
/// Reduce `add` over a `i16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_i16(a: &[i16]) -> i16 {
    unsafe { rust_isa_sve_addv_i16(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_addv_i64(a: *const i64, len: usize) -> i64;
}
/// Reduce `add` over a `i64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_i64(a: &[i64]) -> i64 {
    unsafe { rust_isa_sve_addv_i64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_addv_u8(a: *const u8, len: usize) -> u8;
}
/// Reduce `add` over a `u8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_u8(a: &[u8]) -> u8 {
    unsafe { rust_isa_sve_addv_u8(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_addv_u16(a: *const u16, len: usize) -> u16;
}
/// Reduce `add` over a `u16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_u16(a: &[u16]) -> u16 {
    unsafe { rust_isa_sve_addv_u16(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_addv_u32(a: *const u32, len: usize) -> u32;
}
/// Reduce `add` over a `u32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_u32(a: &[u32]) -> u32 {
    unsafe { rust_isa_sve_addv_u32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_addv_u64(a: *const u64, len: usize) -> u64;
}
/// Reduce `add` over a `u64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_u64(a: &[u64]) -> u64 {
    unsafe { rust_isa_sve_addv_u64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_maxv_i8(a: *const i8, len: usize) -> i8;
}
/// Reduce `max` over a `i8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_i8(a: &[i8]) -> i8 {
    unsafe { rust_isa_sve_maxv_i8(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_minv_i8(a: *const i8, len: usize) -> i8;
}
/// Reduce `min` over a `i8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_i8(a: &[i8]) -> i8 {
    unsafe { rust_isa_sve_minv_i8(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_maxv_i16(a: *const i16, len: usize) -> i16;
}
/// Reduce `max` over a `i16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_i16(a: &[i16]) -> i16 {
    unsafe { rust_isa_sve_maxv_i16(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_minv_i16(a: *const i16, len: usize) -> i16;
}
/// Reduce `min` over a `i16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_i16(a: &[i16]) -> i16 {
    unsafe { rust_isa_sve_minv_i16(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_maxv_i64(a: *const i64, len: usize) -> i64;
}
/// Reduce `max` over a `i64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_i64(a: &[i64]) -> i64 {
    unsafe { rust_isa_sve_maxv_i64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_minv_i64(a: *const i64, len: usize) -> i64;
}
/// Reduce `min` over a `i64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_i64(a: &[i64]) -> i64 {
    unsafe { rust_isa_sve_minv_i64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_maxv_u8(a: *const u8, len: usize) -> u8;
}
/// Reduce `max` over a `u8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_u8(a: &[u8]) -> u8 {
    unsafe { rust_isa_sve_maxv_u8(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_minv_u8(a: *const u8, len: usize) -> u8;
}
/// Reduce `min` over a `u8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_u8(a: &[u8]) -> u8 {
    unsafe { rust_isa_sve_minv_u8(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_maxv_u16(a: *const u16, len: usize) -> u16;
}
/// Reduce `max` over a `u16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_u16(a: &[u16]) -> u16 {
    unsafe { rust_isa_sve_maxv_u16(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_minv_u16(a: *const u16, len: usize) -> u16;
}
/// Reduce `min` over a `u16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_u16(a: &[u16]) -> u16 {
    unsafe { rust_isa_sve_minv_u16(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_maxv_u32(a: *const u32, len: usize) -> u32;
}
/// Reduce `max` over a `u32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_u32(a: &[u32]) -> u32 {
    unsafe { rust_isa_sve_maxv_u32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_minv_u32(a: *const u32, len: usize) -> u32;
}
/// Reduce `min` over a `u32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_u32(a: &[u32]) -> u32 {
    unsafe { rust_isa_sve_minv_u32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_maxv_u64(a: *const u64, len: usize) -> u64;
}
/// Reduce `max` over a `u64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_u64(a: &[u64]) -> u64 {
    unsafe { rust_isa_sve_maxv_u64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_minv_u64(a: *const u64, len: usize) -> u64;
}
/// Reduce `min` over a `u64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_u64(a: &[u64]) -> u64 {
    unsafe { rust_isa_sve_minv_u64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_andv_i32(a: *const i32, len: usize) -> i32;
}
/// Reduce `and` over a `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_and_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_andv_i32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_eorv_i32(a: *const i32, len: usize) -> i32;
}
/// Reduce `eor` over a `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_eor_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_eorv_i32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_lsl_i8(a: *const i8, shift: i32, out: *mut i8, len: usize);
}
/// `lsl` on `i8` slices by a scalar shift.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lsl_i8(a: &[i8], shift: i32, out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_lsl_i8(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_lsr_i8(a: *const i8, shift: i32, out: *mut i8, len: usize);
}
/// `lsr` on `i8` slices by a scalar shift.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lsr_i8(a: &[i8], shift: i32, out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_lsr_i8(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_asr_i8(a: *const i8, shift: i32, out: *mut i8, len: usize);
}
/// `asr` on `i8` slices by a scalar shift.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn asr_i8(a: &[i8], shift: i32, out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_asr_i8(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_lsl_i16(a: *const i16, shift: i32, out: *mut i16, len: usize);
}
/// `lsl` on `i16` slices by a scalar shift.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lsl_i16(a: &[i16], shift: i32, out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_lsl_i16(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_lsr_i16(a: *const i16, shift: i32, out: *mut i16, len: usize);
}
/// `lsr` on `i16` slices by a scalar shift.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lsr_i16(a: &[i16], shift: i32, out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_lsr_i16(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_asr_i16(a: *const i16, shift: i32, out: *mut i16, len: usize);
}
/// `asr` on `i16` slices by a scalar shift.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn asr_i16(a: &[i16], shift: i32, out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_asr_i16(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_lsl_i64(a: *const i64, shift: i32, out: *mut i64, len: usize);
}
/// `lsl` on `i64` slices by a scalar shift.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lsl_i64(a: &[i64], shift: i32, out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_lsl_i64(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_lsr_i64(a: *const i64, shift: i32, out: *mut i64, len: usize);
}
/// `lsr` on `i64` slices by a scalar shift.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lsr_i64(a: &[i64], shift: i32, out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_lsr_i64(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_asr_i64(a: *const i64, shift: i32, out: *mut i64, len: usize);
}
/// `asr` on `i64` slices by a scalar shift.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn asr_i64(a: &[i64], shift: i32, out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_asr_i64(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sli_i32(a: *const i32, shift: i32, out: *mut i32, len: usize);
}
/// `sli` on `i32` slices (scalar shift).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sli_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sli_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sri_i32(a: *const i32, shift: i32, out: *mut i32, len: usize);
}
/// `sri` on `i32` slices (scalar shift).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sri_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sri_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_srshl_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `srshl` on `i32` slices (scalar shift).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn srshl_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_srshl_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_urshl_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `urshl` on `u32` slices (scalar shift).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn urshl_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_urshl_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqshl_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `sqshl` on `i32` slices (scalar shift).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqshl_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqshl_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uqshl_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `uqshl` on `u32` slices (scalar shift).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqshl_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqshl_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqrshl_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `sqrshl` on `i32` slices (scalar shift).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqrshl_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqrshl_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uqrshl_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `uqrshl` on `u32` slices (scalar shift).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqrshl_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqrshl_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_srshr_i32(a: *const i32, shift: i32, out: *mut i32, len: usize);
}
/// `srshr` on `i32` slices (scalar shift).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn srshr_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_srshr_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_urshr_u32(a: *const u32, shift: i32, out: *mut u32, len: usize);
}
/// `urshr` on `u32` slices (scalar shift).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn urshr_u32(a: &[u32], shift: i32, out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_urshr_u32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_srsra_i32(
        a: *const i32,
        shift: i32,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `srsra` on `i32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn srsra_i32(a: &[i32], shift: i32, acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_srsra_i32(a.as_ptr(), shift, acc.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ursra_u32(
        a: *const u32,
        shift: i32,
        acc: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `ursra` on `u32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ursra_u32(a: &[u32], shift: i32, acc: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ursra_u32(a.as_ptr(), shift, acc.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ssra_i32(a: *const i32, shift: i32, acc: *const i32, out: *mut i32, len: usize);
}
/// `ssra` on `i32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ssra_i32(a: &[i32], shift: i32, acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ssra_i32(a.as_ptr(), shift, acc.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_usra_u32(a: *const u32, shift: i32, acc: *const u32, out: *mut u32, len: usize);
}
/// `usra` on `u32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn usra_u32(a: &[u32], shift: i32, acc: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_usra_u32(a.as_ptr(), shift, acc.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_abd_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `abd` on `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn abd_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_abd_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_mulx_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `mulx` on `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mulx_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mulx_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_recps_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `recps` from `f32` to `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn recps_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_recps_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rsqrts_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `rsqrts` from `f32` to `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rsqrts_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsqrts_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_recpe_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `recpe` from `f32` to `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn recpe_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_recpe_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rsqrte_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `rsqrte` from `f32` to `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rsqrte_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsqrte_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_abd_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `abd` on `f64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn abd_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_abd_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_mulx_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `mulx` on `f64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mulx_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mulx_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_recps_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `recps` from `f64` to `f64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn recps_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_recps_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rsqrts_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `rsqrts` from `f64` to `f64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rsqrts_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsqrts_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_recpe_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `recpe` from `f64` to `f64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn recpe_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_recpe_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rsqrte_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `rsqrte` from `f64` to `f64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rsqrte_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsqrte_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmeq_f32(a: *const f32, b: *const f32, out: *mut u32, len: usize);
}
/// `fcmeq` on `f32` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmeq_f32(a: &[f32], b: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmeq_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmeq_f64(a: *const f64, b: *const f64, out: *mut u32, len: usize);
}
/// `fcmeq` on `f64` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmeq_f64(a: &[f64], b: &[f64], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmeq_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmge_f32(a: *const f32, b: *const f32, out: *mut u32, len: usize);
}
/// `fcmge` on `f32` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmge_f32(a: &[f32], b: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmge_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmge_f64(a: *const f64, b: *const f64, out: *mut u32, len: usize);
}
/// `fcmge` on `f64` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmge_f64(a: &[f64], b: &[f64], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmge_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmgt_f32(a: *const f32, b: *const f32, out: *mut u32, len: usize);
}
/// `fcmgt` on `f32` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmgt_f32(a: &[f32], b: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmgt_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmgt_f64(a: *const f64, b: *const f64, out: *mut u32, len: usize);
}
/// `fcmgt` on `f64` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmgt_f64(a: &[f64], b: &[f64], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmgt_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmle_f32(a: *const f32, b: *const f32, out: *mut u32, len: usize);
}
/// `fcmle` on `f32` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmle_f32(a: &[f32], b: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmle_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmle_f64(a: *const f64, b: *const f64, out: *mut u32, len: usize);
}
/// `fcmle` on `f64` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmle_f64(a: &[f64], b: &[f64], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmle_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmlt_f32(a: *const f32, b: *const f32, out: *mut u32, len: usize);
}
/// `fcmlt` on `f32` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmlt_f32(a: &[f32], b: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmlt_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmlt_f64(a: *const f64, b: *const f64, out: *mut u32, len: usize);
}
/// `fcmlt` on `f64` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmlt_f64(a: &[f64], b: &[f64], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmlt_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_facge_f32(a: *const f32, b: *const f32, out: *mut u32, len: usize);
}
/// `facge` on `f32` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn facge_f32(a: &[f32], b: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_facge_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_facge_f64(a: *const f64, b: *const f64, out: *mut u32, len: usize);
}
/// `facge` on `f64` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn facge_f64(a: &[f64], b: &[f64], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_facge_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_facgt_f32(a: *const f32, b: *const f32, out: *mut u32, len: usize);
}
/// `facgt` on `f32` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn facgt_f32(a: &[f32], b: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_facgt_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_facgt_f64(a: *const f64, b: *const f64, out: *mut u32, len: usize);
}
/// `facgt` on `f64` slices; true lanes become `u32::MAX`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn facgt_f64(a: &[f64], b: &[f64], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_facgt_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_maxnmv_f32(a: *const f32, len: usize) -> f32;
}
/// Reduce `maxnm` over a `f32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_maxnm_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_maxnmv_f32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_minnmv_f32(a: *const f32, len: usize) -> f32;
}
/// Reduce `minnm` over a `f32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_minnm_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_minnmv_f32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_maxnmv_f64(a: *const f64, len: usize) -> f64;
}
/// Reduce `maxnm` over a `f64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_maxnm_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_maxnmv_f64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_minnmv_f64(a: *const f64, len: usize) -> f64;
}
/// Reduce `minnm` over a `f64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_minnm_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_minnmv_f64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_mad_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `mad` on `f32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mad_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mad_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_msb_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `msb` on `f32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn msb_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_msb_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmad_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fmad` on `f32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmad_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmad_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmsb_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fmsb` on `f32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmsb_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmsb_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fnmad_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fnmad` on `f32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmad_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmad_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fnmsb_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fnmsb` on `f32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmsb_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmsb_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fnmla_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fnmla` on `f32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmla_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmla_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fnmls_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fnmls` on `f32` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmls_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmls_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_mad_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `mad` on `f64` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mad_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mad_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_msb_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `msb` on `f64` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn msb_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_msb_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmad_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fmad` on `f64` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmad_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmad_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmsb_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fmsb` on `f64` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmsb_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmsb_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fnmad_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fnmad` on `f64` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmad_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmad_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fnmsb_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fnmsb` on `f64` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmsb_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmsb_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fnmla_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fnmla` on `f64` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmla_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmla_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fnmls_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fnmls` on `f64` slices with an accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmls_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmls_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cadd_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// Complex `cadd` on `f32` slices with a 90 degree rotation.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cadd_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cadd_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcadd_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// Complex `fcadd` on `f32` slices with a 90 degree rotation.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcadd_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcadd_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmla_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// Complex `fcmla` on `f32` slices with a 90 degree rotation.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmla_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmla_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cmla_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// Complex `cmla` on `f32` slices with a 90 degree rotation.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cmla_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cmla_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cadd_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// Complex `cadd` on `f64` slices with a 90 degree rotation.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cadd_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cadd_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcadd_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// Complex `fcadd` on `f64` slices with a 90 degree rotation.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcadd_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcadd_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmla_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// Complex `fcmla` on `f64` slices with a 90 degree rotation.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcmla_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmla_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cmla_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// Complex `cmla` on `f64` slices with a 90 degree rotation.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cmla_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cmla_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ftsmul_f32(a: *const f32, b: *const u32, out: *mut f32, len: usize);
}
/// `ftsmul` on `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ftsmul_f32(a: &[f32], b: &[u32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ftsmul_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ftssel_f32(a: *const f32, b: *const u32, out: *mut f32, len: usize);
}
/// `ftssel` on `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ftssel_f32(a: &[f32], b: &[u32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ftssel_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ftmad_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `ftmad` on `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ftmad_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ftmad_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cpy_i32(a: *const i32, out: *mut i32, len: usize);
}
/// `cpy` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cpy_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cpy_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_mov_i32(a: *const i32, out: *mut i32, len: usize);
}
/// `mov` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mov_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mov_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_movprfx_i32(a: *const i32, out: *mut i32, len: usize);
}
/// `movprfx` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn movprfx_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_movprfx_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_compact_i32(a: *const i32, out: *mut i32, len: usize);
}
/// `compact` on `i32` slices with an all-true predicate.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn compact_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_compact_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_trn1_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `trn1` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn trn1_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_trn1_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_trn2_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `trn2` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn trn2_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_trn2_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_zip1_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `zip1` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn zip1_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_zip1_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_zip2_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `zip2` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn zip2_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_zip2_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uzp1_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `uzp1` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uzp1_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uzp1_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uzp2_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `uzp2` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uzp2_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uzp2_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_splice_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `splice` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn splice_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_splice_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ext_i32(a: *const i32, b: *const i32, offset: usize, out: *mut i32, len: usize);
}
/// `ext` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ext_i32(a: &[i32], b: &[i32], offset: usize, out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ext_i32(a.as_ptr(), b.as_ptr(), offset, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_tbl_i32(a: *const i32, idx: *const u32, out: *mut i32, len: usize);
}
/// `tbl` on `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn tbl_i32(a: &[i32], idx: &[u32], out: &mut [i32]) {
    assert_eq!(a.len(), idx.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_tbl_i32(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sel_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `sel` on `i32` slices with an all-true predicate.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sel_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sel_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_csel_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `csel` on `i32` slices with an all-true condition.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn csel_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_csel_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
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
unsafe extern "C" {
    fn rust_isa_sve_clast_i32(a: *const i32, len: usize) -> i32;
}
/// `clast` of an `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn clast_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_clast_i32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_dup_i32(value: i32, out: *mut i32, len: usize);
}
/// Broadcast an `i32` value.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn dup_i32(value: i32, out: &mut [i32]) {
    unsafe {
        rust_isa_sve_dup_i32(value, out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fdup_f32(value: f32, out: *mut f32, len: usize);
}
/// Broadcast an `f32` value.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fdup_f32(value: f32, out: &mut [f32]) {
    unsafe {
        rust_isa_sve_fdup_f32(value, out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_insr_i32(value: i32, out: *mut i32, len: usize);
}
/// Insert an `i32` scalar into a slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn insr_i32(value: i32, out: &mut [i32]) {
    unsafe {
        rust_isa_sve_insr_i32(value, out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_index_i32(start: i32, step: i32, out: *mut i32, len: usize);
}
/// Generate an `i32` index vector.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn index_i32(start: i32, step: i32, out: &mut [i32]) {
    unsafe {
        rust_isa_sve_index_i32(start, step, out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `ld1` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1_u8(a: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `ld1` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1_u16(a: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `ld1` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1_u32(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ld1` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `ld1` on `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `ld1` on `f64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1b_u8(a: *const u8, out: *mut u32, len: usize);
}
/// `ld1b` from `u8` to `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1b_u8(a: &[u8], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1b_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1sb_i8(a: *const i8, out: *mut i32, len: usize);
}
/// `ld1sb` from `i8` to `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1sb_i8(a: &[i8], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1sb_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1h_u16(a: *const u16, out: *mut u32, len: usize);
}
/// `ld1h` from `u16` to `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1h_u16(a: &[u16], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1h_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1sh_i16(a: *const i16, out: *mut i32, len: usize);
}
/// `ld1sh` from `i16` to `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1sh_i16(a: &[i16], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1sh_i16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1w_u32(a: *const u32, out: *mut u64, len: usize);
}
/// `ld1w` from `u32` to `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1w_u32(a: &[u32], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1w_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1sw_i32(a: *const i32, out: *mut i64, len: usize);
}
/// `ld1sw` from `i32` to `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1sw_i32(a: &[i32], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1sw_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1d_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ld1d` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1d_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1d_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldnt1b_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `ldnt1b` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldnt1b_u8(a: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnt1b_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldnt1h_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `ldnt1h` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldnt1h_u16(a: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnt1h_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldnt1w_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `ldnt1w` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldnt1w_u32(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnt1w_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldnt1d_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ldnt1d` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldnt1d_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnt1d_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldff1b_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `ldff1b` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldff1b_u8(a: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldff1b_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldff1h_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `ldff1h` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldff1h_u16(a: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldff1h_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldff1w_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `ldff1w` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldff1w_u32(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldff1w_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldff1d_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ldff1d` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldff1d_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldff1d_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldnf1b_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `ldnf1b` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldnf1b_u8(a: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnf1b_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldnf1h_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `ldnf1h` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldnf1h_u16(a: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnf1h_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldnf1w_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `ldnf1w` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldnf1w_u32(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnf1w_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ldnf1d_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ldnf1d` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldnf1d_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnf1d_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1rqb_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `ld1rqb` replicate from `u8` slices.
///
/// Read the 16-byte quadword at the start of `a` and replicate it across `out`.
///
/// # Panics
///
/// Panics when `a` has fewer than 16 bytes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rqb_u8(a: &[u8], out: &mut [u8]) {
    assert!(a.len() >= 16, "SVE replicate load needs a full quadword");
    unsafe {
        rust_isa_sve_ld1rqb_u8(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1rqh_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `ld1rqh` replicate from `u16` slices.
///
/// Read the 16-byte quadword at the start of `a` and replicate it across `out`.
///
/// # Panics
///
/// Panics when `a` has fewer than 8 elements.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rqh_u16(a: &[u16], out: &mut [u16]) {
    assert!(a.len() >= 8, "SVE replicate load needs a full quadword");
    unsafe {
        rust_isa_sve_ld1rqh_u16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1rqw_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `ld1rqw` replicate from `u32` slices.
///
/// Read the 16-byte quadword at the start of `a` and replicate it across `out`.
///
/// # Panics
///
/// Panics when `a` has fewer than 4 elements.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rqw_u32(a: &[u32], out: &mut [u32]) {
    assert!(a.len() >= 4, "SVE replicate load needs a full quadword");
    unsafe {
        rust_isa_sve_ld1rqw_u32(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1rqd_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ld1rqd` replicate from `u64` slices.
///
/// Read the 16-byte quadword at the start of `a` and replicate it across `out`.
///
/// # Panics
///
/// Panics when `a` has fewer than 2 elements.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rqd_u64(a: &[u64], out: &mut [u64]) {
    assert!(a.len() >= 2, "SVE replicate load needs a full quadword");
    unsafe {
        rust_isa_sve_ld1rqd_u64(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1rb_u8(a: *const u8, out: *mut u32, len: usize);
}
/// `ld1rb` broadcast from `u8` to `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rb_u8(a: &[u8], out: &mut [u32]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rb_u8(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1rsb_i8(a: *const i8, out: *mut i32, len: usize);
}
/// `ld1rsb` broadcast from `i8` to `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rsb_i8(a: &[i8], out: &mut [i32]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rsb_i8(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1rh_u16(a: *const u16, out: *mut u32, len: usize);
}
/// `ld1rh` broadcast from `u16` to `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rh_u16(a: &[u16], out: &mut [u32]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rh_u16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1rsh_i16(a: *const i16, out: *mut i32, len: usize);
}
/// `ld1rsh` broadcast from `i16` to `i32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rsh_i16(a: &[i16], out: &mut [i32]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rsh_i16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1rw_u32(a: *const u32, out: *mut u64, len: usize);
}
/// `ld1rw` broadcast from `u32` to `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rw_u32(a: &[u32], out: &mut [u64]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rw_u32(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1rsw_i32(a: *const i32, out: *mut i64, len: usize);
}
/// `ld1rsw` broadcast from `i32` to `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rsw_i32(a: &[i32], out: &mut [i64]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rsw_i32(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld1rd_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ld1rd` broadcast from `u64` to `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rd_u64(a: &[u64], out: &mut [u64]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rd_u64(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st1_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `st1` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st1_u8(a: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_st1_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st1_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `st1` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st1_u16(a: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_st1_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st1_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `st1` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st1_u32(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_st1_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st1_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `st1` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st1_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_st1_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st1_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `st1` on `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st1_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_st1_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st1_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `st1` on `f64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st1_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_st1_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st1b_u32(a: *const u32, out: *mut u8, len: usize);
}
/// `st1b` from `u32` to `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st1b_u32(a: &[u32], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_st1b_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st1h_u32(a: *const u32, out: *mut u16, len: usize);
}
/// `st1h` from `u32` to `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st1h_u32(a: &[u32], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_st1h_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st1w_u64(a: *const u64, out: *mut u32, len: usize);
}
/// `st1w` from `u64` to `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st1w_u64(a: &[u64], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_st1w_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_stnt1b_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `stnt1b` on `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn stnt1b_u8(a: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_stnt1b_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_stnt1h_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `stnt1h` on `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn stnt1h_u16(a: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_stnt1h_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_stnt1w_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `stnt1w` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn stnt1w_u32(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_stnt1w_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_stnt1d_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `stnt1d` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn stnt1d_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_stnt1d_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld2b_u8(a: *const u8, out0: *mut u8, out1: *mut u8, len: usize);
}
/// `ld2b` deinterleave from `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld2b_u8(a: &[u8], out0: &mut [u8], out1: &mut [u8]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 2,
        "SVE input must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld2b_u8(a.as_ptr(), out0.as_mut_ptr(), out1.as_mut_ptr(), out0.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld2h_u16(a: *const u16, out0: *mut u16, out1: *mut u16, len: usize);
}
/// `ld2h` deinterleave from `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld2h_u16(a: &[u16], out0: &mut [u16], out1: &mut [u16]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 2,
        "SVE input must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld2h_u16(a.as_ptr(), out0.as_mut_ptr(), out1.as_mut_ptr(), out0.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld2w_u32(a: *const u32, out0: *mut u32, out1: *mut u32, len: usize);
}
/// `ld2w` deinterleave from `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld2w_u32(a: &[u32], out0: &mut [u32], out1: &mut [u32]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 2,
        "SVE input must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld2w_u32(a.as_ptr(), out0.as_mut_ptr(), out1.as_mut_ptr(), out0.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld2d_u64(a: *const u64, out0: *mut u64, out1: *mut u64, len: usize);
}
/// `ld2d` deinterleave from `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld2d_u64(a: &[u64], out0: &mut [u64], out1: &mut [u64]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 2,
        "SVE input must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld2d_u64(a.as_ptr(), out0.as_mut_ptr(), out1.as_mut_ptr(), out0.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld3b_u8(a: *const u8, out0: *mut u8, out1: *mut u8, out2: *mut u8, len: usize);
}
/// `ld3b` deinterleave from `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld3b_u8(a: &[u8], out0: &mut [u8], out1: &mut [u8], out2: &mut [u8]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 3,
        "SVE input must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld3b_u8(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld3h_u16(
        a: *const u16,
        out0: *mut u16,
        out1: *mut u16,
        out2: *mut u16,
        len: usize,
    );
}
/// `ld3h` deinterleave from `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld3h_u16(a: &[u16], out0: &mut [u16], out1: &mut [u16], out2: &mut [u16]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 3,
        "SVE input must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld3h_u16(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld3w_u32(
        a: *const u32,
        out0: *mut u32,
        out1: *mut u32,
        out2: *mut u32,
        len: usize,
    );
}
/// `ld3w` deinterleave from `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld3w_u32(a: &[u32], out0: &mut [u32], out1: &mut [u32], out2: &mut [u32]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 3,
        "SVE input must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld3w_u32(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld3d_u64(
        a: *const u64,
        out0: *mut u64,
        out1: *mut u64,
        out2: *mut u64,
        len: usize,
    );
}
/// `ld3d` deinterleave from `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld3d_u64(a: &[u64], out0: &mut [u64], out1: &mut [u64], out2: &mut [u64]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 3,
        "SVE input must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld3d_u64(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld4b_u8(
        a: *const u8,
        out0: *mut u8,
        out1: *mut u8,
        out2: *mut u8,
        out3: *mut u8,
        len: usize,
    );
}
/// `ld4b` deinterleave from `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld4b_u8(
    a: &[u8],
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out3.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 4,
        "SVE input must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld4b_u8(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out3.as_mut_ptr(),
            out0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld4h_u16(
        a: *const u16,
        out0: *mut u16,
        out1: *mut u16,
        out2: *mut u16,
        out3: *mut u16,
        len: usize,
    );
}
/// `ld4h` deinterleave from `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld4h_u16(
    a: &[u16],
    out0: &mut [u16],
    out1: &mut [u16],
    out2: &mut [u16],
    out3: &mut [u16],
) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out3.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 4,
        "SVE input must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld4h_u16(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out3.as_mut_ptr(),
            out0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld4w_u32(
        a: *const u32,
        out0: *mut u32,
        out1: *mut u32,
        out2: *mut u32,
        out3: *mut u32,
        len: usize,
    );
}
/// `ld4w` deinterleave from `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld4w_u32(
    a: &[u32],
    out0: &mut [u32],
    out1: &mut [u32],
    out2: &mut [u32],
    out3: &mut [u32],
) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out3.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 4,
        "SVE input must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld4w_u32(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out3.as_mut_ptr(),
            out0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld4d_u64(
        a: *const u64,
        out0: *mut u64,
        out1: *mut u64,
        out2: *mut u64,
        out3: *mut u64,
        len: usize,
    );
}
/// `ld4d` deinterleave from `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld4d_u64(
    a: &[u64],
    out0: &mut [u64],
    out1: &mut [u64],
    out2: &mut [u64],
    out3: &mut [u64],
) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out3.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 4,
        "SVE input must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld4d_u64(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out3.as_mut_ptr(),
            out0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st2b_u8(a0: *const u8, a1: *const u8, out: *mut u8, len: usize);
}
/// `st2b` interleave to `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st2b_u8(a0: &[u8], a1: &[u8], out: &mut [u8]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 2,
        "SVE output must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st2b_u8(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr(), a0.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st2h_u16(a0: *const u16, a1: *const u16, out: *mut u16, len: usize);
}
/// `st2h` interleave to `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st2h_u16(a0: &[u16], a1: &[u16], out: &mut [u16]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 2,
        "SVE output must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st2h_u16(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr(), a0.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st2w_u32(a0: *const u32, a1: *const u32, out: *mut u32, len: usize);
}
/// `st2w` interleave to `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st2w_u32(a0: &[u32], a1: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 2,
        "SVE output must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st2w_u32(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr(), a0.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st2d_u64(a0: *const u64, a1: *const u64, out: *mut u64, len: usize);
}
/// `st2d` interleave to `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st2d_u64(a0: &[u64], a1: &[u64], out: &mut [u64]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 2,
        "SVE output must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st2d_u64(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr(), a0.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st3b_u8(a0: *const u8, a1: *const u8, a2: *const u8, out: *mut u8, len: usize);
}
/// `st3b` interleave to `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st3b_u8(a0: &[u8], a1: &[u8], a2: &[u8], out: &mut [u8]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 3,
        "SVE output must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st3b_u8(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st3h_u16(
        a0: *const u16,
        a1: *const u16,
        a2: *const u16,
        out: *mut u16,
        len: usize,
    );
}
/// `st3h` interleave to `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st3h_u16(a0: &[u16], a1: &[u16], a2: &[u16], out: &mut [u16]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 3,
        "SVE output must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st3h_u16(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st3w_u32(
        a0: *const u32,
        a1: *const u32,
        a2: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `st3w` interleave to `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st3w_u32(a0: &[u32], a1: &[u32], a2: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 3,
        "SVE output must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st3w_u32(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st3d_u64(
        a0: *const u64,
        a1: *const u64,
        a2: *const u64,
        out: *mut u64,
        len: usize,
    );
}
/// `st3d` interleave to `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st3d_u64(a0: &[u64], a1: &[u64], a2: &[u64], out: &mut [u64]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 3,
        "SVE output must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st3d_u64(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st4b_u8(
        a0: *const u8,
        a1: *const u8,
        a2: *const u8,
        a3: *const u8,
        out: *mut u8,
        len: usize,
    );
}
/// `st4b` interleave to `u8` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st4b_u8(a0: &[u8], a1: &[u8], a2: &[u8], a3: &[u8], out: &mut [u8]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a3.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 4,
        "SVE output must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st4b_u8(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            a3.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st4h_u16(
        a0: *const u16,
        a1: *const u16,
        a2: *const u16,
        a3: *const u16,
        out: *mut u16,
        len: usize,
    );
}
/// `st4h` interleave to `u16` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st4h_u16(a0: &[u16], a1: &[u16], a2: &[u16], a3: &[u16], out: &mut [u16]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a3.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 4,
        "SVE output must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st4h_u16(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            a3.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st4w_u32(
        a0: *const u32,
        a1: *const u32,
        a2: *const u32,
        a3: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `st4w` interleave to `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st4w_u32(a0: &[u32], a1: &[u32], a2: &[u32], a3: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a3.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 4,
        "SVE output must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st4w_u32(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            a3.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st4d_u64(
        a0: *const u64,
        a1: *const u64,
        a2: *const u64,
        a3: *const u64,
        out: *mut u64,
        len: usize,
    );
}
/// `st4d` interleave to `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st4d_u64(a0: &[u64], a1: &[u64], a2: &[u64], a3: &[u64], out: &mut [u64]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a3.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 4,
        "SVE output must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st4d_u64(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            a3.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqadd_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `sqadd` on `i8` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqadd_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqadd_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqsub_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `sqsub` on `i8` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqsub_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqsub_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqadd_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `sqadd` on `i16` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqadd_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqadd_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqsub_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `sqsub` on `i16` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqsub_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqsub_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqadd_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `sqadd` on `i32` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqadd_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqadd_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqsub_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `sqsub` on `i32` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqsub_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqsub_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqadd_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `sqadd` on `i64` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqadd_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqadd_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqsub_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `sqsub` on `i64` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqsub_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqsub_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uqadd_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `uqadd` on `u8` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqadd_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqadd_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uqsub_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `uqsub` on `u8` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqsub_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqsub_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uqadd_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `uqadd` on `u16` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqadd_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqadd_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uqsub_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `uqsub` on `u16` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqsub_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqsub_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uqadd_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `uqadd` on `u32` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqadd_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqadd_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uqsub_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `uqsub` on `u32` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqsub_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqsub_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uqadd_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `uqadd` on `u64` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqadd_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqadd_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uqsub_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `uqsub` on `u64` slices (scalar saturating fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqsub_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uqsub_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqxtnb_i32(a: *const i32, out: *mut i16, len: usize);
}
/// `sqxtnb` from `i32` to `i16` slices (scalar fallback).
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
    fn rust_isa_sve_sqxtunb_i32(a: *const i32, out: *mut u16, len: usize);
}
/// `sqxtunb` from `i32` to `u16` slices (scalar fallback).
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
    fn rust_isa_sve_uqxtnb_u32(a: *const u32, out: *mut u16, len: usize);
}
/// `uqxtnb` from `u32` to `u16` slices (scalar fallback).
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
unsafe extern "C" {
    fn rust_isa_sve_sqdmulh_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `sqdmulh` on `i16` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqdmulh_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqdmulh_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqrdmulh_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `sqrdmulh` on `i16` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqrdmulh_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqrdmulh_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqdmulh_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `sqdmulh` on `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqdmulh_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqdmulh_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqrdmulh_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `sqrdmulh` on `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqrdmulh_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqrdmulh_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_bsl_i32(a: *const i32, b: *const i32, c: *const i32, out: *mut i32, len: usize);
}
/// `bsl` on `i32` slices (scalar fallback).
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
unsafe extern "C" {
    fn rust_isa_sve_cadd_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `cadd` on `i32` slices with a 90 degree rotation (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cadd_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cadd_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sqcadd_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `sqcadd` on `i32` slices with a 90 degree rotation (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqcadd_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqcadd_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sclamp_i32(a: *const i32, min: i32, max: i32, out: *mut i32, len: usize);
}
/// `sclamp` on `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sclamp_i32(a: &[i32], min: i32, max: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sclamp_i32(a.as_ptr(), min, max, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uclamp_u32(a: *const u32, min: u32, max: u32, out: *mut u32, len: usize);
}
/// `uclamp` on `u32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uclamp_u32(a: &[u32], min: u32, max: u32, out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uclamp_u32(a.as_ptr(), min, max, out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_scale_f32(a: *const f32, b: *const i32, out: *mut f32, len: usize);
}
/// `fscale` on `f32` slices with `i32` exponents.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn scale_f32(a: &[f32], b: &[i32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scale_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmov_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `fmov` on `f32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmov_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmov_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_scale_f64(a: *const f64, b: *const i64, out: *mut f64, len: usize);
}
/// `fscale` on `f64` slices with `i64` exponents.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn scale_f64(a: &[f64], b: &[i64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scale_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmov_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `fmov` on `f64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmov_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmov_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_saddlb_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `saddlb` from `i8` to `i16` slices (scalar fallback).
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
    fn rust_isa_sve_usublbt_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `usublbt` from `u8` to `u16` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn usublbt_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_usublbt_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_usubltb_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `usubltb` from `u8` to `u16` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn usubltb_u8(a: &[u8], b: &[u8], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_usubltb_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uaddwb_u8(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `uaddwb` from `u8` to `u16` slices (scalar fallback).
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
    fn rust_isa_sve_saddlh_i16(a: *const i16, b: *const i16, out: *mut i32, len: usize);
}
/// `saddlh` from `i16` to `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn saddlh_i16(a: &[i16], b: &[i16], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_saddlh_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_saddlht_i16(a: *const i16, b: *const i16, out: *mut i32, len: usize);
}
/// `saddlht` from `i16` to `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn saddlht_i16(a: &[i16], b: &[i16], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_saddlht_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uaddlh_u16(a: *const u16, b: *const u16, out: *mut u32, len: usize);
}
/// `uaddlh` from `u16` to `u32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uaddlh_u16(a: &[u16], b: &[u16], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uaddlh_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uaddlht_u16(a: *const u16, b: *const u16, out: *mut u32, len: usize);
}
/// `uaddlht` from `u16` to `u32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uaddlht_u16(a: &[u16], b: &[u16], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uaddlht_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sabdlb_i8(a: *const i8, b: *const i8, out: *mut i16, len: usize);
}
/// `sabdlb` from `i8` to `i16` slices (scalar fallback).
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
    fn rust_isa_sve_saba_i8(a: *const i8, b: *const i8, acc: *const i8, out: *mut i8, len: usize);
}
/// `saba` on `i8` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn saba_i8(a: &[i8], b: &[i8], acc: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_saba_i8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_saba_i16(
        a: *const i16,
        b: *const i16,
        acc: *const i16,
        out: *mut i16,
        len: usize,
    );
}
/// `saba` on `i16` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn saba_i16(a: &[i16], b: &[i16], acc: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_saba_i16(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_saba_i32(
        a: *const i32,
        b: *const i32,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `saba` on `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn saba_i32(a: &[i32], b: &[i32], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_saba_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uaba_u8(a: *const u8, b: *const u8, acc: *const u8, out: *mut u8, len: usize);
}
/// `uaba` on `u8` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uaba_u8(a: &[u8], b: &[u8], acc: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uaba_u8(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uaba_u16(
        a: *const u16,
        b: *const u16,
        acc: *const u16,
        out: *mut u16,
        len: usize,
    );
}
/// `uaba` on `u16` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uaba_u16(a: &[u16], b: &[u16], acc: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uaba_u16(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_uaba_u32(
        a: *const u32,
        b: *const u32,
        acc: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `uaba` on `u32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uaba_u32(a: &[u32], b: &[u32], acc: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uaba_u32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
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
unsafe extern "C" {
    fn rust_isa_sve_srhadd_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `srhadd` on `i8` slices (scalar rounding fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn srhadd_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_srhadd_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_srhadd_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `srhadd` on `i16` slices (scalar rounding fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn srhadd_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_srhadd_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_srhadd_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `srhadd` on `i32` slices (scalar rounding fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn srhadd_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_srhadd_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_urhadd_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `urhadd` on `u8` slices (scalar rounding fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn urhadd_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_urhadd_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_urhadd_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `urhadd` on `u16` slices (scalar rounding fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn urhadd_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_urhadd_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_urhadd_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `urhadd` on `u32` slices (scalar rounding fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn urhadd_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_urhadd_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_xtn_i32(a: *const i32, out: *mut i16, len: usize);
}
/// `xtn` from `i32` to `i16` slices (scalar fallback).
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
    fn rust_isa_sve_sqshlu_i32(a: *const i32, shift: i32, out: *mut i32, len: usize);
}
/// `sqshlu` on `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqshlu_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqshlu_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
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
    fn rust_isa_sve_sdotb_i32(
        a: *const i8,
        b: *const i8,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sdotb` on `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sdotb_i32(a: &[i8], b: &[i8], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(
        a.len(),
        acc.len() * 4,
        "SVE input must contain four elements per output"
    );
    assert_eq!(out.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sdotb_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_sdott_i32(
        a: *const i8,
        b: *const i8,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `sdott` on `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sdott_i32(a: &[i8], b: &[i8], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(
        a.len(),
        acc.len() * 4,
        "SVE input must contain four elements per output"
    );
    assert_eq!(out.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sdott_i32(
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
    fn rust_isa_sve_udotb_i32(
        a: *const u8,
        b: *const u8,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `udotb` on `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn udotb_i32(a: &[u8], b: &[u8], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(
        a.len(),
        acc.len() * 4,
        "SVE input must contain four elements per output"
    );
    assert_eq!(out.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_udotb_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_udott_i32(
        a: *const u8,
        b: *const u8,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `udott` on `i32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn udott_i32(a: &[u8], b: &[u8], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(
        a.len(),
        acc.len() * 4,
        "SVE input must contain four elements per output"
    );
    assert_eq!(out.len(), acc.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_udott_i32(
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
unsafe extern "C" {
    fn rust_isa_sve_urecpe_f32_u32(a: *const u32, out: *mut f32, len: usize);
}
/// `urecpe` from `u32` to `f32` slices (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn urecpe_f32_u32(a: &[u32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_urecpe_f32_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld2_u32(a: *const u32, out0: *mut u32, out1: *mut u32, len: usize);
}
/// `ld2` deinterleave from `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld2_u32(a: &[u32], out0: &mut [u32], out1: &mut [u32]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 2,
        "SVE input must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld2_u32(a.as_ptr(), out0.as_mut_ptr(), out1.as_mut_ptr(), out0.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld3_u32(
        a: *const u32,
        out0: *mut u32,
        out1: *mut u32,
        out2: *mut u32,
        len: usize,
    );
}
/// `ld3` deinterleave from `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld3_u32(a: &[u32], out0: &mut [u32], out1: &mut [u32], out2: &mut [u32]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 3,
        "SVE input must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld3_u32(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ld4_u32(
        a: *const u32,
        out0: *mut u32,
        out1: *mut u32,
        out2: *mut u32,
        out3: *mut u32,
        len: usize,
    );
}
/// `ld4` deinterleave from `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld4_u32(
    a: &[u32],
    out0: &mut [u32],
    out1: &mut [u32],
    out2: &mut [u32],
    out3: &mut [u32],
) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out3.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 4,
        "SVE input must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld4_u32(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out3.as_mut_ptr(),
            out0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st2_u32(a0: *const u32, a1: *const u32, out: *mut u32, len: usize);
}
/// `st2` interleave to `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st2_u32(a0: &[u32], a1: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 2,
        "SVE output must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st2_u32(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr(), a0.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st3_u32(
        a0: *const u32,
        a1: *const u32,
        a2: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `st3` interleave to `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st3_u32(a0: &[u32], a1: &[u32], a2: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 3,
        "SVE output must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st3_u32(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st4_u32(
        a0: *const u32,
        a1: *const u32,
        a2: *const u32,
        a3: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `st4` interleave to `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st4_u32(a0: &[u32], a1: &[u32], a2: &[u32], a3: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a3.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 4,
        "SVE output must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st4_u32(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            a3.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_st1d_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `st1d` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn st1d_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_st1d_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcvt_f32_f64(a: *const f64, out: *mut f32, len: usize);
}
/// Convert `f64` slices to `f32` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcvt_f32_f64(a: &[f64], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvt_f32_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcvt_f64_f32(a: *const f32, out: *mut f64, len: usize);
}
/// Convert `f32` slices to `f64` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcvt_f64_f32(a: &[f32], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvt_f64_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcvtzs_i32_f32(a: *const f32, out: *mut i32, len: usize);
}
/// `fcvtzs` from `f32` to `i32` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcvtzs_i32_f32(a: &[f32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzs_i32_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcvtzs_i32_f64(a: *const f64, out: *mut i32, len: usize);
}
/// `fcvtzs` from `f64` to `i32` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcvtzs_i32_f64(a: &[f64], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzs_i32_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcvtzs_i64_f32(a: *const f32, out: *mut i64, len: usize);
}
/// `fcvtzs` from `f32` to `i64` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcvtzs_i64_f32(a: &[f32], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzs_i64_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcvtzs_i64_f64(a: *const f64, out: *mut i64, len: usize);
}
/// `fcvtzs` from `f64` to `i64` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcvtzs_i64_f64(a: &[f64], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzs_i64_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcvtzu_u32_f32(a: *const f32, out: *mut u32, len: usize);
}
/// `fcvtzu` from `f32` to `u32` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcvtzu_u32_f32(a: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzu_u32_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcvtzu_u32_f64(a: *const f64, out: *mut u32, len: usize);
}
/// `fcvtzu` from `f64` to `u32` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcvtzu_u32_f64(a: &[f64], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzu_u32_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcvtzu_u64_f32(a: *const f32, out: *mut u64, len: usize);
}
/// `fcvtzu` from `f32` to `u64` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcvtzu_u64_f32(a: &[f32], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzu_u64_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcvtzu_u64_f64(a: *const f64, out: *mut u64, len: usize);
}
/// `fcvtzu` from `f64` to `u64` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fcvtzu_u64_f64(a: &[f64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcvtzu_u64_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_scvtf_f32_i32(a: *const i32, out: *mut f32, len: usize);
}
/// `scvtf` from `i32` to `f32` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn scvtf_f32_i32(a: &[i32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scvtf_f32_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_scvtf_f32_i64(a: *const i64, out: *mut f32, len: usize);
}
/// `scvtf` from `i64` to `f32` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn scvtf_f32_i64(a: &[i64], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scvtf_f32_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_scvtf_f64_i32(a: *const i32, out: *mut f64, len: usize);
}
/// `scvtf` from `i32` to `f64` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn scvtf_f64_i32(a: &[i32], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scvtf_f64_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_scvtf_f64_i64(a: *const i64, out: *mut f64, len: usize);
}
/// `scvtf` from `i64` to `f64` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn scvtf_f64_i64(a: &[i64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_scvtf_f64_i64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ucvtf_f32_u32(a: *const u32, out: *mut f32, len: usize);
}
/// `ucvtf` from `u32` to `f32` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ucvtf_f32_u32(a: &[u32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ucvtf_f32_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ucvtf_f32_u64(a: *const u64, out: *mut f32, len: usize);
}
/// `ucvtf` from `u64` to `f32` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ucvtf_f32_u64(a: &[u64], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ucvtf_f32_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ucvtf_f64_u32(a: *const u32, out: *mut f64, len: usize);
}
/// `ucvtf` from `u32` to `f64` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ucvtf_f64_u32(a: &[u32], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ucvtf_f64_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_ucvtf_f64_u64(a: *const u64, out: *mut f64, len: usize);
}
/// `ucvtf` from `u64` to `f64` (scalar fallback).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ucvtf_f64_u64(a: &[u64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ucvtf_f64_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fabs_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `fabs` on `f32` slices (alias of `abs`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fabs_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fabs_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fneg_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `fneg` on `f32` slices (alias of `neg`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fneg_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fneg_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fsqrt_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `fsqrt` on `f32` slices (alias of `sqrt`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fsqrt_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fsqrt_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frecpe_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frecpe` on `f32` slices (alias of `recpe`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frecpe_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frecpe_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frinta_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frinta` on `f32` slices (alias of `rinta`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frinta_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frinta_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frinti_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frinti` on `f32` slices (alias of `rinti`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frinti_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frinti_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frintx_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frintx` on `f32` slices (alias of `rintx`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintx_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintx_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frintm_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frintm` on `f32` slices (alias of `floor`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintm_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintm_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frintn_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frintn` on `f32` slices (alias of `rint`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintn_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintn_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frintp_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frintp` on `f32` slices (alias of `ceil`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintp_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintp_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frintz_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `frintz` on `f32` slices (alias of `trunc`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintz_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintz_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rintm_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `rintm` on `f32` slices (alias of `floor`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintm_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintm_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rintn_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `rintn` on `f32` slices (alias of `rint`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintn_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintn_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rintp_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `rintp` on `f32` slices (alias of `ceil`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintp_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintp_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rintz_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `rintz` on `f32` slices (alias of `trunc`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintz_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintz_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fadd_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fadd` on `f32` slices (alias of `add`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fadd_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fadd_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fsub_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fsub` on `f32` slices (alias of `sub`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fsub_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fsub_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmul_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmul` on `f32` slices (alias of `mul`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmul_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmul_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fdiv_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fdiv` on `f32` slices (alias of `div`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fdiv_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fdiv_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmax_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmax` on `f32` slices (alias of `max`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmax_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmax_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmin_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmin` on `f32` slices (alias of `min`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmin_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmin_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmaxnm_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmaxnm` on `f32` slices (alias of `maxnm`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxnm_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmaxnm_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fminnm_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fminnm` on `f32` slices (alias of `minnm`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminnm_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fminnm_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fabd_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fabd` on `f32` slices (alias of `abd`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fabd_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fabd_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmulx_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmulx` on `f32` slices (alias of `mulx`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmulx_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmulx_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frecps_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `frecps` on `f32` slices (alias of `recps`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frecps_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frecps_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frsqrts_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `frsqrts` on `f32` slices (alias of `rsqrts`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frsqrts_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frsqrts_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fscale_f32(a: *const f32, b: *const i32, out: *mut f32, len: usize);
}
/// `fscale` on `f32` slices with `i32` exponents (alias of `scale`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fscale_f32(a: &[f32], b: &[i32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fscale_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmla_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fmla` on `f32` slices (alias of `fma`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmla_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmla_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmls_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fmls` on `f32` slices (alias of `mls`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmls_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmls_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_faddv_f32(a: *const f32, len: usize) -> f32;
}
/// `faddv` over a `f32` slice (alias of `addv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn faddv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_faddv_f32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_fmaxv_f32(a: *const f32, len: usize) -> f32;
}
/// `fmaxv` over a `f32` slice (alias of `maxv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fmaxv_f32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_fminv_f32(a: *const f32, len: usize) -> f32;
}
/// `fminv` over a `f32` slice (alias of `minv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fminv_f32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_fmaxnmv_f32(a: *const f32, len: usize) -> f32;
}
/// `fmaxnmv` over a `f32` slice (alias of `maxnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxnmv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fmaxnmv_f32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_fminnmv_f32(a: *const f32, len: usize) -> f32;
}
/// `fminnmv` over a `f32` slice (alias of `minnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminnmv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fminnmv_f32(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_fabs_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `fabs` on `f64` slices (alias of `abs`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fabs_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fabs_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fneg_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `fneg` on `f64` slices (alias of `neg`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fneg_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fneg_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fsqrt_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `fsqrt` on `f64` slices (alias of `sqrt`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fsqrt_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fsqrt_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frecpe_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frecpe` on `f64` slices (alias of `recpe`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frecpe_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frecpe_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frinta_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frinta` on `f64` slices (alias of `rinta`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frinta_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frinta_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frinti_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frinti` on `f64` slices (alias of `rinti`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frinti_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frinti_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frintx_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frintx` on `f64` slices (alias of `rintx`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintx_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintx_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frintm_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frintm` on `f64` slices (alias of `floor`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintm_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintm_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frintn_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frintn` on `f64` slices (alias of `rint`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintn_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintn_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frintp_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frintp` on `f64` slices (alias of `ceil`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintp_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintp_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frintz_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `frintz` on `f64` slices (alias of `trunc`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frintz_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frintz_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rintm_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `rintm` on `f64` slices (alias of `floor`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintm_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintm_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rintn_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `rintn` on `f64` slices (alias of `rint`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintn_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintn_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rintp_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `rintp` on `f64` slices (alias of `ceil`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintp_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintp_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_rintz_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `rintz` on `f64` slices (alias of `trunc`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintz_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintz_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fadd_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fadd` on `f64` slices (alias of `add`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fadd_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fadd_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fsub_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fsub` on `f64` slices (alias of `sub`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fsub_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fsub_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmul_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmul` on `f64` slices (alias of `mul`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmul_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmul_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fdiv_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fdiv` on `f64` slices (alias of `div`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fdiv_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fdiv_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmax_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmax` on `f64` slices (alias of `max`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmax_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmax_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmin_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmin` on `f64` slices (alias of `min`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmin_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmin_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmaxnm_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmaxnm` on `f64` slices (alias of `maxnm`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxnm_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmaxnm_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fminnm_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fminnm` on `f64` slices (alias of `minnm`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminnm_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fminnm_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fabd_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fabd` on `f64` slices (alias of `abd`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fabd_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fabd_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmulx_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmulx` on `f64` slices (alias of `mulx`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmulx_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmulx_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frecps_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `frecps` on `f64` slices (alias of `recps`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frecps_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frecps_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_frsqrts_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `frsqrts` on `f64` slices (alias of `rsqrts`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frsqrts_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frsqrts_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fscale_f64(a: *const f64, b: *const i64, out: *mut f64, len: usize);
}
/// `fscale` on `f64` slices with `i64` exponents (alias of `scale`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fscale_f64(a: &[f64], b: &[i64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fscale_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmla_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fmla` on `f64` slices (alias of `fma`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmla_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmla_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmls_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fmls` on `f64` slices (alias of `mls`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmls_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmls_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_faddv_f64(a: *const f64, len: usize) -> f64;
}
/// `faddv` over a `f64` slice (alias of `addv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn faddv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_faddv_f64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_fmaxv_f64(a: *const f64, len: usize) -> f64;
}
/// `fmaxv` over a `f64` slice (alias of `maxv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fmaxv_f64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_fminv_f64(a: *const f64, len: usize) -> f64;
}
/// `fminv` over a `f64` slice (alias of `minv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fminv_f64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_fmaxnmv_f64(a: *const f64, len: usize) -> f64;
}
/// `fmaxnmv` over a `f64` slice (alias of `maxnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxnmv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fmaxnmv_f64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_fminnmv_f64(a: *const f64, len: usize) -> f64;
}
/// `fminnmv` over a `f64` slice (alias of `minnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminnmv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fminnmv_f64(a.as_ptr(), a.len()) }
}
unsafe extern "C" {
    fn rust_isa_sve_div_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `sdiv`/`udiv` on `i64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn div_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_div_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_div_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `sdiv`/`udiv` on `u32` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn div_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_div_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_div_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `sdiv`/`udiv` on `u64` slices.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn div_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_div_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
unsafe extern "C" {
    fn rust_isa_sve_punpkhi_u8(a: *const u8, out: *mut u16, len: usize);
}
/// `punpkhi` on `u8` slices (scalar boolean widening fallback).
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
