//! PowerPC Power9 Altivec aliases.
//!
//! The Power9 vector instructions are exposed in [`power9_vector`](super::power9_vector). This
//! module provides the Altivec-named entry points for the same operations.
//!
//! The module name follows the `power9-altivec` target feature. The re-exported
//! operations are `unsafe` and require the `power9-vector` target feature;
//! `power9-altivec` alone is insufficient. The values are
//! [`V128`](super::altivec::V128).
//!
//! Instruction behavior follows the [Power ISA](https://openpowerfoundation.org/specifications/isa/).
//!
//! This module re-exports existing wrappers and adds no build steps.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::powerpc::simd::altivec::V128;
//! use rust_isa::powerpc::simd::power9_altivec::vctzd;
//!
//! assert_ne!(unsafe { vctzd(V128::from_bytes([0; 16])) }.to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::powerpc::simd::power9_altivec::*;
//! use rust_isa::powerpc::simd::altivec::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { vabsd(v128, v128) };
//! let _ = unsafe { vabsduw(v128, v128) };
//! let _ = unsafe { vctzb(v128) };
//! let _ = unsafe { vctzd(v128) };
//! let _ = unsafe { vctzh(v128) };
//! let _ = unsafe { vctzlsbb(v128) };
//! let _ = unsafe { vctzw(v128) };
//! let _ = unsafe { vextsb2d(v128) };
//! let _ = unsafe { vextsb2w(v128) };
//! let _ = unsafe { vextsh2d(v128) };
//! let _ = unsafe { vextsh2w(v128) };
//! let _ = unsafe { vextsw2d(v128) };
//! let _ = unsafe { vnegd(v128) };
//! let _ = unsafe { vnegw(v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::powerpc::simd::power9_altivec::vabsd as _;
//! use rust_isa::powerpc::simd::power9_altivec::vabsduw as _;
//! use rust_isa::powerpc::simd::power9_altivec::vctzb as _;
//! use rust_isa::powerpc::simd::power9_altivec::vctzd as _;
//! use rust_isa::powerpc::simd::power9_altivec::vctzh as _;
//! use rust_isa::powerpc::simd::power9_altivec::vctzlsbb as _;
//! use rust_isa::powerpc::simd::power9_altivec::vctzw as _;
//! use rust_isa::powerpc::simd::power9_altivec::vextsb2d as _;
//! use rust_isa::powerpc::simd::power9_altivec::vextsb2w as _;
//! use rust_isa::powerpc::simd::power9_altivec::vextsh2d as _;
//! use rust_isa::powerpc::simd::power9_altivec::vextsh2w as _;
//! use rust_isa::powerpc::simd::power9_altivec::vextsw2d as _;
//! use rust_isa::powerpc::simd::power9_altivec::vnegd as _;
//! use rust_isa::powerpc::simd::power9_altivec::vnegw as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

pub use super::power9_vector::{
    vabsd, vabsduw, vctzb, vctzd, vctzh, vctzlsbb, vctzw, vextsb2d, vextsb2w, vextsh2d, vextsh2w,
    vextsw2d, vnegd, vnegw,
};
