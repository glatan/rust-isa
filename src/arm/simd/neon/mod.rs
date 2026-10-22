//! NEON.
//!
//! NEON is unstable on this target. The caller must compile with
//! `-C target-feature=+neon`.

mod arith;
mod ops;
mod v128;
mod widen;

pub use arith::*;
pub use ops::*;
pub use v128::*;
pub use widen::*;

unsafe extern "C" {
    fn rust_isa_arm_neon_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vadd.i32`
///
/// Add the four 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmul.i32`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
