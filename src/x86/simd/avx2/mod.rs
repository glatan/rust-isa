//! AVX2.
//!
//! AVX2 is not part of the x86_64 baseline. Every operation is `unsafe` and
//! the caller must enable AVX2 with `#[target_feature(enable = "avx2")]` or a
//! global target feature.

mod bitwise;
mod integer;
mod v256;

pub use bitwise::*;
pub use integer::*;
pub use v256::*;
