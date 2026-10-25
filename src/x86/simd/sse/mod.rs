//! SSE.
//!
//! SSE is part of the x86-64 baseline. The operations are safe.
//!
//! The values are [`V128`]. SSE covers the 32-bit float operations that SSE2
//! keeps unchanged. This module re-exports the SSE operations implemented by
//! the SSE2 backend so that the public path `x86::simd::sse::*` exists.
//!
//! [`V128`]: super::sse2::V128
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! This module re-exports existing wrappers and adds no build steps.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::x86::simd::sse::addps;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(addps(v, v).to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::x86::simd::sse::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = addps(v128, v128);
//! let _ = cmpeqps(v128, v128);
//! let _ = cmpleps(v128, v128);
//! let _ = cmpltps(v128, v128);
//! let _ = divps(v128, v128);
//! let _ = maxps(v128, v128);
//! let _ = minps(v128, v128);
//! let _ = mulps(v128, v128);
//! #[cfg(target_arch = "x86_64")] let _ = shufps::<0>(v128, v128);
//! let _ = sqrtps(v128);
//! let _ = subps(v128, v128);
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
//! use rust_isa::x86::simd::sse::addps as _;
//! use rust_isa::x86::simd::sse::cmpeqps as _;
//! use rust_isa::x86::simd::sse::cmpleps as _;
//! use rust_isa::x86::simd::sse::cmpltps as _;
//! use rust_isa::x86::simd::sse::divps as _;
//! use rust_isa::x86::simd::sse::maxps as _;
//! use rust_isa::x86::simd::sse::minps as _;
//! use rust_isa::x86::simd::sse::mulps as _;
//! use rust_isa::x86::simd::sse::shufps as _;
//! use rust_isa::x86::simd::sse::sqrtps as _;
//! use rust_isa::x86::simd::sse::subps as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

pub use super::sse2::{
    addps, cmpeqps, cmpleps, cmpltps, divps, maxps, minps, mulps, shufps, sqrtps, subps,
};
