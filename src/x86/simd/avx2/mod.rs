//! AVX2.
//!
//! AVX2 is not part of the x86_64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable AVX2 with `#[target_feature(enable = "avx2")]` or a
//! global target feature. The values are [`V256`]; insert, extract, and gather
//! operations also use [`V128`](super::sse2::V128). Immediate operands are
//! const generics unless documented otherwise.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.
//!
//! Instructions not listed in this module are not exposed.

mod arithmetic;
mod bitwise;
mod blendv;
mod broadcast;
mod compare;
mod gather;
mod horizontal;
mod immediate;
mod movemask;
mod pack;
mod permute;
mod shift;
mod shuffle;
mod v256;

pub use arithmetic::*;
pub use bitwise::*;
pub use blendv::*;
pub use broadcast::*;
pub use compare::*;
pub use gather::*;
pub use horizontal::*;
pub use immediate::*;
pub use movemask::*;
pub use pack::*;
pub use permute::*;
pub use shift::*;
pub use shuffle::*;
pub use v256::*;
