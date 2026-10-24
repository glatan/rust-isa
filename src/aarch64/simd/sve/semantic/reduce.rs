//! SVE reduction operations.

unsafe extern "C" {
    fn rust_isa_sve_addv_f32(a: *const f32, len: usize) -> f32;
}

/// Sum an `f32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_addv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_addv_i32(a: *const i32, len: usize) -> i32;
}

/// Sum an `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_addv_i32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_addv_f64(a: *const f64, len: usize) -> f64;
}

/// Sum an `f64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_addv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minv_f32(a: *const f32, len: usize) -> f32;
}

/// Minimum of an `f32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_minv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxv_f32(a: *const f32, len: usize) -> f32;
}

/// Maximum of an `f32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_maxv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minv_i32(a: *const i32, len: usize) -> i32;
}

/// Minimum of an `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_minv_i32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxv_i32(a: *const i32, len: usize) -> i32;
}

/// Maximum of an `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_maxv_i32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minv_f64(a: *const f64, len: usize) -> f64;
}

/// Minimum of an `f64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_minv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxv_f64(a: *const f64, len: usize) -> f64;
}

/// Maximum of an `f64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_maxv_f64(a.as_ptr(), a.len()) }
}
