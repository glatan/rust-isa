//! AVX2.
//!
//! AVX2 is not part of the x86_64 baseline. Every operation is `unsafe` and
//! the caller must enable AVX2 with `#[target_feature(enable = "avx2")]` or a
//! global target feature.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).

mod arithmetic;
mod bitwise;
mod blend;
mod broadcast;
mod gather;
mod horizontal;
mod immediate;
mod integer;
mod pack;
mod permute;
mod shift;
mod v256;

pub use arithmetic::*;
pub use bitwise::*;
pub use blend::*;
pub use broadcast::*;
pub use gather::*;
pub use horizontal::*;
pub use immediate::*;
pub use integer::*;
pub use pack::*;
pub use permute::*;
pub use shift::*;
pub use v256::*;
