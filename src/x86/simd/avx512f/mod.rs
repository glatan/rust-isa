//! AVX-512 Foundation.
//!
//! AVX-512 is not part of the x86_64 baseline. Every operation is `unsafe`
//! and the caller must enable the required target features.

mod integer;
mod v512;

pub use integer::*;
pub use v512::*;
