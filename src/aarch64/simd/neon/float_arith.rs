//! NEON floating-point arithmetic.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_faddp_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_faddp_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmaxnm_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmaxnm_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fminnm_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fminnm_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmulx_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmulx_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fabd_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fabd_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_faddv_f32(a: *const u8) -> f32;
    fn rust_isa_neon_faddv_f64(a: *const u8) -> f64;
    fn rust_isa_neon_fmaxnmv_f32(a: *const u8) -> f32;
    fn rust_isa_neon_fmaxnmv_f64(a: *const u8) -> f64;
    fn rust_isa_neon_fmaxv_f32(a: *const u8) -> f32;
    fn rust_isa_neon_fmaxv_f64(a: *const u8) -> f64;
    fn rust_isa_neon_fminnmv_f32(a: *const u8) -> f32;
    fn rust_isa_neon_fminnmv_f64(a: *const u8) -> f64;
    fn rust_isa_neon_fminv_f32(a: *const u8) -> f32;
    fn rust_isa_neon_fminv_f64(a: *const u8) -> f64;
}

unsafe extern "C" {
    fn rust_isa_neon_fmla_f32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_fmla_f64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_fmls_f32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_fmls_f64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `faddp_f32`
///
/// Add adjacent float lane pairs.
#[inline(always)]
pub fn faddp_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_faddp_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `faddp_f64`
///
/// Add adjacent float lane pairs.
#[inline(always)]
pub fn faddp_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_faddp_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmaxnm_f32`
///
/// Take the maximum of float lanes, preferring numeric values.
#[inline(always)]
pub fn fmaxnm_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmaxnm_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmaxnm_f64`
///
/// Take the maximum of float lanes, preferring numeric values.
#[inline(always)]
pub fn fmaxnm_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmaxnm_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fminnm_f32`
///
/// Take the minimum of float lanes, preferring numeric values.
#[inline(always)]
pub fn fminnm_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fminnm_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fminnm_f64`
///
/// Take the minimum of float lanes, preferring numeric values.
#[inline(always)]
pub fn fminnm_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fminnm_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmla_f32`
///
/// Multiply f32 lanes and add the result to the accumulator.
#[inline(always)]
pub fn fmla_f32(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmla_f32(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmla_f64`
///
/// Multiply f64 lanes and add the result to the accumulator.
#[inline(always)]
pub fn fmla_f64(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmla_f64(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmls_f32`
///
/// Multiply f32 lanes and subtract the result from the accumulator.
#[inline(always)]
pub fn fmls_f32(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmls_f32(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmls_f64`
///
/// Multiply f64 lanes and subtract the result from the accumulator.
#[inline(always)]
pub fn fmls_f64(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmls_f64(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmulx_f32`
///
/// Multiply float lanes with the extended multiply behavior.
#[inline(always)]
pub fn fmulx_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmulx_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmulx_f64`
///
/// Multiply float lanes with the extended multiply behavior.
#[inline(always)]
pub fn fmulx_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmulx_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fabd_f32`
///
/// Take the absolute difference of float lanes.
#[inline(always)]
pub fn fabd_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fabd_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fabd_f64`
///
/// Take the absolute difference of float lanes.
#[inline(always)]
pub fn fabd_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fabd_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `faddv_f32`
///
/// Add all float lanes.
#[inline(always)]
pub fn faddv_f32(a: V128) -> f32 {
    unsafe { rust_isa_neon_faddv_f32(a.as_ptr()) }
}

/// `faddv_f64`
///
/// Add all float lanes.
#[inline(always)]
pub fn faddv_f64(a: V128) -> f64 {
    unsafe { rust_isa_neon_faddv_f64(a.as_ptr()) }
}

/// `fmaxnmv_f32`
///
/// Take the maximum of all float lanes, preferring numeric values.
#[inline(always)]
pub fn fmaxnmv_f32(a: V128) -> f32 {
    unsafe { rust_isa_neon_fmaxnmv_f32(a.as_ptr()) }
}

/// `fmaxnmv_f64`
///
/// Take the maximum of all float lanes, preferring numeric values.
#[inline(always)]
pub fn fmaxnmv_f64(a: V128) -> f64 {
    unsafe { rust_isa_neon_fmaxnmv_f64(a.as_ptr()) }
}

/// `fmaxv_f32`
///
/// Take the maximum of all float lanes.
#[inline(always)]
pub fn fmaxv_f32(a: V128) -> f32 {
    unsafe { rust_isa_neon_fmaxv_f32(a.as_ptr()) }
}

/// `fmaxv_f64`
///
/// Take the maximum of all float lanes.
#[inline(always)]
pub fn fmaxv_f64(a: V128) -> f64 {
    unsafe { rust_isa_neon_fmaxv_f64(a.as_ptr()) }
}

/// `fminnmv_f32`
///
/// Take the minimum of all float lanes, preferring numeric values.
#[inline(always)]
pub fn fminnmv_f32(a: V128) -> f32 {
    unsafe { rust_isa_neon_fminnmv_f32(a.as_ptr()) }
}

/// `fminnmv_f64`
///
/// Take the minimum of all float lanes, preferring numeric values.
#[inline(always)]
pub fn fminnmv_f64(a: V128) -> f64 {
    unsafe { rust_isa_neon_fminnmv_f64(a.as_ptr()) }
}

/// `fminv_f32`
///
/// Take the minimum of all float lanes.
#[inline(always)]
pub fn fminv_f32(a: V128) -> f32 {
    unsafe { rust_isa_neon_fminv_f32(a.as_ptr()) }
}

/// `fminv_f64`
///
/// Take the minimum of all float lanes.
#[inline(always)]
pub fn fminv_f64(a: V128) -> f64 {
    unsafe { rust_isa_neon_fminv_f64(a.as_ptr()) }
}
