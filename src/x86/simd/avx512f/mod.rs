//! AVX-512 Foundation.
//!
//! AVX-512 is not part of the x86_64 baseline. Every operation is `unsafe`
//! and the caller must enable the required target features.

mod float;
mod integer;
mod memory;
mod v512;

pub use float::*;
pub use integer::*;
pub use memory::*;
pub use v512::*;
