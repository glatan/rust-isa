//! AArch64 SVE2 slice API.
//!
//! SVE2 is not part of the AArch64 baseline. Every wrapper is `unsafe` and
//! requires `#[target_feature(enable = "sve2")]` or a global target feature.
//!
//! The module exposes slice operations. Each function processes the input
//! slices and writes the same number of lanes to the output slice.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! Instructions not listed in this module are not exposed.
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::aarch64::simd::sve2::*;
//! let _ = unsafe { addp_i8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { addp_i16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { addp_i32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { addp_i64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { addp_u8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { addp_u16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { addp_u32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { addp_u64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { addp_f32(&[0.0; 4], &[0.0; 4], &mut [0.0; 4]) };
//! let _ = unsafe { addp_f64(&[0.0; 4], &[0.0; 4], &mut [0.0; 4]) };
//! let _ = unsafe { shadd_i8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shadd_i16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shadd_i32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shadd_i64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shadd_u8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shadd_u16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shadd_u32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shadd_u64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shsub_i8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shsub_i16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shsub_i32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shsub_i64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shsub_u8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shsub_u16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shsub_u32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { shsub_u64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhadd_i8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhadd_i16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhadd_i32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhadd_i64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhadd_u8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhadd_u16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhadd_u32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhadd_u64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhsub_i8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhsub_i16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhsub_i32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhsub_i64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhsub_u8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhsub_u16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhsub_u32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uhsub_u64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { smaxp_i8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { smaxp_i16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { smaxp_i32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { smaxp_i64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { sminp_i8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { sminp_i16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { sminp_i32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { sminp_i64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { umaxp_u8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { umaxp_u16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { umaxp_u32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { umaxp_u64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uminp_u8(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uminp_u16(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uminp_u32(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { uminp_u64(&[0; 4], &[0; 4], &mut [0; 4]) };
//! let _ = unsafe { faddp_f32(&[0.0f32; 4], &[0.0f32; 4], &mut [0.0f32; 4]) };
//! let _ = unsafe { faddp_f64(&[0.0f64; 4], &[0.0f64; 4], &mut [0.0f64; 4]) };
//! let _ = unsafe { fmaxp_f32(&[0.0f32; 4], &[0.0f32; 4], &mut [0.0f32; 4]) };
//! let _ = unsafe { fmaxp_f64(&[0.0f64; 4], &[0.0f64; 4], &mut [0.0f64; 4]) };
//! let _ = unsafe { fminp_f32(&[0.0f32; 4], &[0.0f32; 4], &mut [0.0f32; 4]) };
//! let _ = unsafe { fminp_f64(&[0.0f64; 4], &[0.0f64; 4], &mut [0.0f64; 4]) };
//! let _ = unsafe { fmaxnmp_f32(&[0.0f32; 4], &[0.0f32; 4], &mut [0.0f32; 4]) };
//! let _ = unsafe { fmaxnmp_f64(&[0.0f64; 4], &[0.0f64; 4], &mut [0.0f64; 4]) };
//! let _ = unsafe { fminnmp_f32(&[0.0f32; 4], &[0.0f32; 4], &mut [0.0f32; 4]) };
//! let _ = unsafe { fminnmp_f64(&[0.0f64; 4], &[0.0f64; 4], &mut [0.0f64; 4]) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::aarch64::simd::sve2::addp_f32 as _;
//! use rust_isa::aarch64::simd::sve2::addp_f64 as _;
//! use rust_isa::aarch64::simd::sve2::addp_i16 as _;
//! use rust_isa::aarch64::simd::sve2::addp_i32 as _;
//! use rust_isa::aarch64::simd::sve2::addp_i64 as _;
//! use rust_isa::aarch64::simd::sve2::addp_i8 as _;
//! use rust_isa::aarch64::simd::sve2::addp_u16 as _;
//! use rust_isa::aarch64::simd::sve2::addp_u32 as _;
//! use rust_isa::aarch64::simd::sve2::addp_u64 as _;
//! use rust_isa::aarch64::simd::sve2::addp_u8 as _;
//! use rust_isa::aarch64::simd::sve2::faddp_f32 as _;
//! use rust_isa::aarch64::simd::sve2::faddp_f64 as _;
//! use rust_isa::aarch64::simd::sve2::fmaxnmp_f32 as _;
//! use rust_isa::aarch64::simd::sve2::fmaxnmp_f64 as _;
//! use rust_isa::aarch64::simd::sve2::fmaxp_f32 as _;
//! use rust_isa::aarch64::simd::sve2::fmaxp_f64 as _;
//! use rust_isa::aarch64::simd::sve2::fminnmp_f32 as _;
//! use rust_isa::aarch64::simd::sve2::fminnmp_f64 as _;
//! use rust_isa::aarch64::simd::sve2::fminp_f32 as _;
//! use rust_isa::aarch64::simd::sve2::fminp_f64 as _;
//! use rust_isa::aarch64::simd::sve2::shadd_i16 as _;
//! use rust_isa::aarch64::simd::sve2::shadd_i32 as _;
//! use rust_isa::aarch64::simd::sve2::shadd_i64 as _;
//! use rust_isa::aarch64::simd::sve2::shadd_i8 as _;
//! use rust_isa::aarch64::simd::sve2::shadd_u16 as _;
//! use rust_isa::aarch64::simd::sve2::shadd_u32 as _;
//! use rust_isa::aarch64::simd::sve2::shadd_u64 as _;
//! use rust_isa::aarch64::simd::sve2::shadd_u8 as _;
//! use rust_isa::aarch64::simd::sve2::shsub_i16 as _;
//! use rust_isa::aarch64::simd::sve2::shsub_i32 as _;
//! use rust_isa::aarch64::simd::sve2::shsub_i64 as _;
//! use rust_isa::aarch64::simd::sve2::shsub_i8 as _;
//! use rust_isa::aarch64::simd::sve2::shsub_u16 as _;
//! use rust_isa::aarch64::simd::sve2::shsub_u32 as _;
//! use rust_isa::aarch64::simd::sve2::shsub_u64 as _;
//! use rust_isa::aarch64::simd::sve2::shsub_u8 as _;
//! use rust_isa::aarch64::simd::sve2::smaxp_i16 as _;
//! use rust_isa::aarch64::simd::sve2::smaxp_i32 as _;
//! use rust_isa::aarch64::simd::sve2::smaxp_i64 as _;
//! use rust_isa::aarch64::simd::sve2::smaxp_i8 as _;
//! use rust_isa::aarch64::simd::sve2::sminp_i16 as _;
//! use rust_isa::aarch64::simd::sve2::sminp_i32 as _;
//! use rust_isa::aarch64::simd::sve2::sminp_i64 as _;
//! use rust_isa::aarch64::simd::sve2::sminp_i8 as _;
//! use rust_isa::aarch64::simd::sve2::uhadd_i16 as _;
//! use rust_isa::aarch64::simd::sve2::uhadd_i32 as _;
//! use rust_isa::aarch64::simd::sve2::uhadd_i64 as _;
//! use rust_isa::aarch64::simd::sve2::uhadd_i8 as _;
//! use rust_isa::aarch64::simd::sve2::uhadd_u16 as _;
//! use rust_isa::aarch64::simd::sve2::uhadd_u32 as _;
//! use rust_isa::aarch64::simd::sve2::uhadd_u64 as _;
//! use rust_isa::aarch64::simd::sve2::uhadd_u8 as _;
//! use rust_isa::aarch64::simd::sve2::uhsub_i16 as _;
//! use rust_isa::aarch64::simd::sve2::uhsub_i32 as _;
//! use rust_isa::aarch64::simd::sve2::uhsub_i64 as _;
//! use rust_isa::aarch64::simd::sve2::uhsub_i8 as _;
//! use rust_isa::aarch64::simd::sve2::uhsub_u16 as _;
//! use rust_isa::aarch64::simd::sve2::uhsub_u32 as _;
//! use rust_isa::aarch64::simd::sve2::uhsub_u64 as _;
//! use rust_isa::aarch64::simd::sve2::uhsub_u8 as _;
//! use rust_isa::aarch64::simd::sve2::umaxp_u16 as _;
//! use rust_isa::aarch64::simd::sve2::umaxp_u32 as _;
//! use rust_isa::aarch64::simd::sve2::umaxp_u64 as _;
//! use rust_isa::aarch64::simd::sve2::umaxp_u8 as _;
//! use rust_isa::aarch64::simd::sve2::uminp_u16 as _;
//! use rust_isa::aarch64::simd::sve2::uminp_u32 as _;
//! use rust_isa::aarch64::simd::sve2::uminp_u64 as _;
//! use rust_isa::aarch64::simd::sve2::uminp_u8 as _;
//! ```
//! <!-- doctest-inventory:end -->

unsafe extern "C" {
    fn rust_isa_sve2_addp_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `addp` on `i8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addp_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_addp_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_addp_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `addp` on `i16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addp_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_addp_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_addp_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `addp` on `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addp_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_addp_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_addp_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `addp` on `i64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addp_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_addp_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_addp_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `addp` on `u8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addp_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_addp_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_addp_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `addp` on `u16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addp_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_addp_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_addp_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `addp` on `u32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addp_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_addp_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_addp_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `addp` on `u64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addp_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_addp_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_addp_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `addp` on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addp_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_addp_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_addp_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `addp` on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addp_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_addp_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shadd_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `shadd` on `i8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shadd_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shadd_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shadd_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `shadd` on `i16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shadd_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shadd_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shadd_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `shadd` on `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shadd_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shadd_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shadd_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `shadd` on `i64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shadd_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shadd_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shadd_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `shadd` on `u8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shadd_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shadd_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shadd_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `shadd` on `u16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shadd_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shadd_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shadd_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `shadd` on `u32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shadd_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shadd_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shadd_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `shadd` on `u64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shadd_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shadd_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shsub_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `shsub` on `i8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shsub_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shsub_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shsub_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `shsub` on `i16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shsub_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shsub_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shsub_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `shsub` on `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shsub_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shsub_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shsub_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `shsub` on `i64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shsub_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shsub_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shsub_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `shsub` on `u8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shsub_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shsub_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shsub_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `shsub` on `u16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shsub_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shsub_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shsub_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `shsub` on `u32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shsub_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shsub_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_shsub_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `shsub` on `u64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn shsub_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_shsub_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhadd_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `uhadd` on `i8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhadd_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhadd_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhadd_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `uhadd` on `i16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhadd_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhadd_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhadd_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `uhadd` on `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhadd_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhadd_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhadd_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `uhadd` on `i64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhadd_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhadd_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhadd_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `uhadd` on `u8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhadd_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhadd_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhadd_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `uhadd` on `u16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhadd_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhadd_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhadd_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `uhadd` on `u32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhadd_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhadd_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhadd_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `uhadd` on `u64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhadd_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhadd_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhsub_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `uhsub` on `i8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhsub_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhsub_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhsub_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `uhsub` on `i16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhsub_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhsub_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhsub_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `uhsub` on `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhsub_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhsub_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhsub_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `uhsub` on `i64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhsub_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhsub_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhsub_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `uhsub` on `u8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhsub_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhsub_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhsub_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `uhsub` on `u16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhsub_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhsub_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhsub_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `uhsub` on `u32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhsub_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhsub_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uhsub_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `uhsub` on `u64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uhsub_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uhsub_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_smaxp_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `smaxp` on `i8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn smaxp_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_smaxp_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_smaxp_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `smaxp` on `i16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn smaxp_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_smaxp_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_smaxp_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `smaxp` on `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn smaxp_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_smaxp_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_smaxp_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `smaxp` on `i64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn smaxp_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_smaxp_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_sminp_i8(a: *const i8, b: *const i8, out: *mut i8, len: usize);
}
/// `sminp` on `i8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn sminp_i8(a: &[i8], b: &[i8], out: &mut [i8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_sminp_i8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_sminp_i16(a: *const i16, b: *const i16, out: *mut i16, len: usize);
}
/// `sminp` on `i16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn sminp_i16(a: &[i16], b: &[i16], out: &mut [i16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_sminp_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_sminp_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
}
/// `sminp` on `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn sminp_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_sminp_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_sminp_i64(a: *const i64, b: *const i64, out: *mut i64, len: usize);
}
/// `sminp` on `i64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn sminp_i64(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_sminp_i64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_umaxp_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `umaxp` on `u8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn umaxp_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_umaxp_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_umaxp_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `umaxp` on `u16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn umaxp_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_umaxp_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_umaxp_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `umaxp` on `u32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn umaxp_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_umaxp_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_umaxp_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `umaxp` on `u64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn umaxp_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_umaxp_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uminp_u8(a: *const u8, b: *const u8, out: *mut u8, len: usize);
}
/// `uminp` on `u8` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uminp_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uminp_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uminp_u16(a: *const u16, b: *const u16, out: *mut u16, len: usize);
}
/// `uminp` on `u16` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uminp_u16(a: &[u16], b: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uminp_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uminp_u32(a: *const u32, b: *const u32, out: *mut u32, len: usize);
}
/// `uminp` on `u32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uminp_u32(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uminp_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_uminp_u64(a: *const u64, b: *const u64, out: *mut u64, len: usize);
}
/// `uminp` on `u64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn uminp_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_uminp_u64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_faddp_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `faddp` on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn faddp_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_faddp_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_faddp_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `faddp` on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn faddp_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_faddp_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_fmaxp_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmaxp` on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fmaxp_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_fmaxp_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_fmaxp_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmaxp` on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fmaxp_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_fmaxp_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_fminp_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fminp` on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fminp_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_fminp_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_fminp_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fminp` on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fminp_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_fminp_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_fmaxnmp_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fmaxnmp` on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fmaxnmp_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_fmaxnmp_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_fmaxnmp_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fmaxnmp` on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fmaxnmp_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_fmaxnmp_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_fminnmp_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}
/// `fminnmp` on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fminnmp_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_fminnmp_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve2_fminnmp_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// `fminnmp` on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fminnmp_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE2 slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE2 slices must have the same length");
    unsafe {
        rust_isa_sve2_fminnmp_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
