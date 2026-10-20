//! s390x vector facility.
//!
//! The lane order is big-endian.

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_s390x_vector_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// Vector integer add.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_vector_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// Vector integer multiply.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_vector_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
