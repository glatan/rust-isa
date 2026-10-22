//! SSE2.
//!
//! SSE2 is part of the x86_64 baseline. The arithmetic operations are safe.
//! Loads and stores take raw pointers and are `unsafe`.
//!
//! The values are [`V128`]. The instruction decides what the lanes mean.
//! Immediate operands are const generics unless documented otherwise.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.
//!
//! Instructions not listed in this module are not exposed.

mod bitwise;
mod double_convert;
mod f32x4;
mod f64x2;
mod i16x8;
mod i32x4;
mod i64x2;
mod i8x16;
mod scalar_move;
mod shift;
mod shuffle;
mod unpack;
mod v128;

pub use bitwise::*;
pub use double_convert::*;
pub use f32x4::*;
pub use f64x2::*;
pub use i8x16::*;
pub use i16x8::*;
pub use i32x4::*;
pub use i64x2::*;
pub use scalar_move::*;
pub use shift::*;
pub use shuffle::*;
pub use unpack::*;
pub use v128::*;
