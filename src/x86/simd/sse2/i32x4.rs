//! 32-bit integer instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_paddd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `paddd`
///
/// Add the four 32-bit integer lanes.
///
/// ```
/// use rust_isa::x86::simd::sse2::{paddd, V128};
///
/// let a = V128::from_bytes([
///     1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0,
/// ]);
/// let b = V128::from_bytes([
///     10, 0, 0, 0, 20, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0,
/// ]);
/// let c = paddd(a, b);
/// assert_eq!(c.to_bytes()[0], 11);
/// assert_eq!(c.to_bytes()[4], 22);
/// ```
#[inline(always)]
pub fn paddd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
