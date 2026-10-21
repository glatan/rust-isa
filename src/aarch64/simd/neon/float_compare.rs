//! NEON floating-point compare instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_fcmeq_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fcmeq_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fcmge_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fcmge_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fcmgt_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fcmgt_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fcmle_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fcmle_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fcmlt_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fcmlt_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_facge_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_facge_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_facgt_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_facgt_f64(a: *const u8, b: *const u8, out: *mut u8);
}

/// `fcmeq_f32`
///
/// Compare float lanes for equality.
#[inline(always)]
pub fn fcmeq_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fcmeq_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmeq_f64`
///
/// Compare float lanes for equality.
#[inline(always)]
pub fn fcmeq_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fcmeq_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmge_f32`
///
/// Compare float lanes for greater than or equal.
#[inline(always)]
pub fn fcmge_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fcmge_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmge_f64`
///
/// Compare float lanes for greater than or equal.
#[inline(always)]
pub fn fcmge_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fcmge_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmgt_f32`
///
/// Compare float lanes for greater than.
#[inline(always)]
pub fn fcmgt_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fcmgt_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmgt_f64`
///
/// Compare float lanes for greater than.
#[inline(always)]
pub fn fcmgt_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fcmgt_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmle_f32`
///
/// Compare float lanes for less than or equal.
#[inline(always)]
pub fn fcmle_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fcmle_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmle_f64`
///
/// Compare float lanes for less than or equal.
#[inline(always)]
pub fn fcmle_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fcmle_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmlt_f32`
///
/// Compare float lanes for less than.
#[inline(always)]
pub fn fcmlt_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fcmlt_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fcmlt_f64`
///
/// Compare float lanes for less than.
#[inline(always)]
pub fn fcmlt_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fcmlt_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `facge_f32`
///
/// Compare the absolute values of float lanes for greater than or equal.
#[inline(always)]
pub fn facge_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_facge_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `facge_f64`
///
/// Compare the absolute values of float lanes for greater than or equal.
#[inline(always)]
pub fn facge_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_facge_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `facgt_f32`
///
/// Compare the absolute values of float lanes for greater than.
#[inline(always)]
pub fn facgt_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_facgt_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `facgt_f64`
///
/// Compare the absolute values of float lanes for greater than.
#[inline(always)]
pub fn facgt_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_facgt_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
