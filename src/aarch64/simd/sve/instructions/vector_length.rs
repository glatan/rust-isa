//! SVE vector length instructions.

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
