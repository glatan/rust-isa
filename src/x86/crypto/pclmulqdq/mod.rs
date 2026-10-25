//! PCLMULQDQ.
//!
//! PCLMULQDQ is not part of the x86-64 baseline. The operation is `unsafe` and
//! requires the `pclmulqdq` target feature. The values are [`V128`].
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//! The wrapper uses `asm!` and takes an immediate as a const generic.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::pclmulqdq::pclmulqdq;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { pclmulqdq::<0>(v, v) }.to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::pclmulqdq::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { pclmulqdq::<0>(v128, v128) };
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::crypto::pclmulqdq::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = unsafe { pclmulqdq::<0>(v128, v128) };
//! ```
//!
//! </details>
//!
//! <details>
//! <summary>Instruction inventory</summary>
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::crypto::pclmulqdq::pclmulqdq as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use core::arch::asm;

use crate::x86::simd::sse2::V128;

/// `pclmulqdq`
///
/// Multiply 64-bit halves and keep the 128-bit carry-less product. `IMM`
/// selects the halves.
///
/// # Safety
///
/// The caller must enable PCLMULQDQ.
#[target_feature(enable = "pclmulqdq")]
#[inline]
pub unsafe fn pclmulqdq<const IMM: u8>(a: V128, b: V128) -> V128 {
    let a = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "pclmulqdq {output}, {b}, {imm}",
            output = inout(xmm_reg) a => output,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}
