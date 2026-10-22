//! SM4 instructions.
//!
//! SM4 is not part of the x86-64 baseline. Every operation is `unsafe` and the
//! caller must enable SM4 with `#[target_feature(enable = "sm4")]` or a global
//! target feature.
//!
//! The values are [`V128`]. The caller combines the key schedule and round
//! instructions with the SM4 specification.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_sm4_vsm4key4(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sm4_vsm4rnds4(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vsm4key4`
///
/// Perform four SM4 key-schedule rounds.
///
/// # Safety
///
/// The caller must enable SM4.
#[target_feature(enable = "sm4")]
#[inline]
pub unsafe fn vsm4key4(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sm4_vsm4key4(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsm4rnds4`
///
/// Perform four SM4 encryption rounds.
///
/// # Safety
///
/// The caller must enable SM4.
#[target_feature(enable = "sm4")]
#[inline]
pub unsafe fn vsm4rnds4(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sm4_vsm4rnds4(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
