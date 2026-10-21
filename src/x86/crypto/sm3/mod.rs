//! SM3 instructions.
//!
//! SM3 is not part of the x86-64 baseline. Every operation is `unsafe` and the
//! caller must enable SM3 with `#[target_feature(enable = "sm3")]` or a global
//! target feature.
//!
//! The values are [`V128`]. The round instruction takes a two-bit immediate;
//! `vsm3rnds2::<IMM>` accepts `0..=3` and a larger value is a compile-time
//! error. The caller combines the message schedule and round instructions with
//! the SM3 specification.

use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_sm3_vsm3msg1(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sm3_vsm3msg2(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sm3_vsm3rnds2_0(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sm3_vsm3rnds2_1(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sm3_vsm3rnds2_2(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_sm3_vsm3rnds2_3(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vsm3msg1`
///
/// Perform the first part of the SM3 message expansion.
///
/// # Safety
///
/// The caller must enable SM3.
#[target_feature(enable = "sm3")]
#[inline]
pub unsafe fn vsm3msg1(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sm3_vsm3msg1(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsm3msg2`
///
/// Perform the second part of the SM3 message expansion.
///
/// # Safety
///
/// The caller must enable SM3.
#[target_feature(enable = "sm3")]
#[inline]
pub unsafe fn vsm3msg2(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sm3_vsm3msg2(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsm3rnds2::<IMM>`
///
/// Perform two SM3 rounds. `IMM` selects the round group and must be in
/// `0..=3`.
///
/// # Safety
///
/// The caller must enable SM3.
#[target_feature(enable = "sm3")]
#[inline]
pub unsafe fn vsm3rnds2<const IMM: u8>(a: V128, b: V128, c: V128) -> V128 {
    const { assert!(IMM <= 3, "vsm3rnds2 immediate must be in 0..=3") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match IMM {
        0 => call!(rust_isa_sm3_vsm3rnds2_0),
        1 => call!(rust_isa_sm3_vsm3rnds2_1),
        2 => call!(rust_isa_sm3_vsm3rnds2_2),
        3 => call!(rust_isa_sm3_vsm3rnds2_3),
        _ => unreachable!(),
    }
}
