//! VPCLMULQDQ.
//!
//! VPCLMULQDQ is not part of the x86-64 baseline. Every instruction wrapper is `unsafe`
//! and the required target features are documented on each function. The values
//! are [`V256`] and [`V512`].
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The build compiles the bundled C with clang. The immediate is a runtime
//! value because this API does not claim zero cost for these forms.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::vpclmulqdq::vpclmulqdq_v256;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vpclmulqdq_v256(v, v, 0) };
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::vpclmulqdq::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { vpclmulqdq_v256(v256, v256, 0) };
//! let _ = unsafe { vpclmulqdq_v512(v512, v512, 0) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::crypto::vpclmulqdq::vpclmulqdq_v256 as _;
//! use rust_isa::x86::crypto::vpclmulqdq::vpclmulqdq_v512 as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::x86::simd::avx2::V256;
use crate::x86::simd::avx512f::V512;

unsafe extern "C" {
    fn rust_isa_vpclmulqdq_vpclmulqdq_v256(a: *const u8, b: *const u8, imm: u8, out: *mut u8);
    fn rust_isa_vpclmulqdq_vpclmulqdq_v512(a: *const u8, b: *const u8, imm: u8, out: *mut u8);
}

/// `vpclmulqdq` on 256-bit values.
///
/// The immediate is a runtime value. This function does not claim zero cost.
///
/// # Safety
///
/// The caller must enable AVX2, AVX-512F, AVX-512VL, and VPCLMULQDQ.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[target_feature(enable = "vpclmulqdq")]
#[inline]
pub unsafe fn vpclmulqdq_v256(a: V256, b: V256, imm: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_vpclmulqdq_vpclmulqdq_v256(a.as_ptr(), b.as_ptr(), imm, out.as_mut_ptr());
    }
    out
}

/// `vpclmulqdq` on 512-bit values.
///
/// The immediate is a runtime value. This function does not claim zero cost.
///
/// # Safety
///
/// The caller must enable AVX-512F, AVX-512VL, and VPCLMULQDQ.
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vl")]
#[target_feature(enable = "vpclmulqdq")]
#[inline]
pub unsafe fn vpclmulqdq_v512(a: V512, b: V512, imm: u8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_vpclmulqdq_vpclmulqdq_v512(a.as_ptr(), b.as_ptr(), imm, out.as_mut_ptr());
    }
    out
}
