//! 32-bit ARM DOTPROD.
//!
//! DOTPROD is unstable on this target. Every operation is `unsafe` and the
//! caller must compile the whole crate with the DOTPROD feature enabled.
//!
//! The values are [`V128`]. The dot-product instructions multiply bytes, add
//! adjacent products, and accumulate into 32-bit lanes.

use super::neon::V128;

unsafe extern "C" {
    fn rust_isa_arm_dotprod_sdot(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_dotprod_udot(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vdot.s8`
///
/// Multiply signed bytes and accumulate into the 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable DOTPROD on the CPU and in the target features.
#[inline(always)]
pub unsafe fn sdot(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_dotprod_sdot(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdot.u8`
///
/// Multiply unsigned bytes and accumulate into the 32-bit lanes of `acc`.
///
/// # Safety
///
/// The caller must enable DOTPROD on the CPU and in the target features.
#[inline(always)]
pub unsafe fn udot(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_dotprod_udot(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
