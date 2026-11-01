//! SVE polynomial multiply and pairwise min/max instructions.

unsafe extern "C" {
    fn rust_isa_sve_pmul_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `pmul`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn pmul_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    unsafe {
        rust_isa_sve_pmul_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_pmullb_u16(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `pmullb`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn pmullb_u16(a: &[u8], b: &[u8], out: &mut [u16]) {
    unsafe {
        rust_isa_sve_pmullb_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_pmullt_u16(a: *const u8, b: *const u8, out: *mut u16, len: usize);
}
/// `pmullt`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn pmullt_u16(a: &[u8], b: &[u8], out: &mut [u16]) {
    unsafe {
        rust_isa_sve_pmullt_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_smaxp_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `smaxp`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn smaxp_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_smaxp_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sminp_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `sminp`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn sminp_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sminp_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_umaxp_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `umaxp`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn umaxp_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_umaxp_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_uminp_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `uminp`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uminp_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_uminp_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
