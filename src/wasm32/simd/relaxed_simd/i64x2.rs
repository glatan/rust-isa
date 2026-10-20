//! `i64x2.relaxed_*` instructions.
//!
//! Rules shared by Relaxed SIMD are documented on the [parent module](super).

use super::V128;

unsafe extern "C" {
    fn rust_isa_relaxed_simd_i64x2_relaxed_laneselect(
        a: *const u8,
        b: *const u8,
        c: *const u8,
        out: *mut u8,
    );
}
/// `i64x2.relaxed_laneselect`
///
/// Select bits from `a` and `b`. The mask is either `c`, or each `i64` lane of `c` expanded from its sign bit to all bits one or all bits zero. The bits then follow `v128.bitselect`: a `1` selects `a` and a `0` selects `b`.
#[inline(always)]
pub fn i64x2_relaxed_laneselect(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_relaxed_simd_i64x2_relaxed_laneselect(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}
