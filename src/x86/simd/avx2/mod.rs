//! AVX2.
//!
//! AVX2 is not part of the x86_64 baseline. Every operation is `unsafe` and
//! the caller must enable AVX2 with `#[target_feature(enable = "avx2")]` or a
//! global target feature.

mod arithmetic;
mod bitwise;
mod horizontal;
mod integer;
mod pack;
mod v256;

pub use arithmetic::*;
pub use bitwise::*;
pub use horizontal::*;
pub use integer::*;
pub use pack::*;
pub use v256::*;
