//! SSE2.
//!
//! SSE2 is part of the x86_64 baseline. The arithmetic operations are safe.
//! Loads and stores take raw pointers and are `unsafe`.
//!
//! The values are [`V128`]. The instruction decides what the lanes mean.

mod bitwise;
mod i8x16;
mod i16x8;
mod i32x4;
mod i64x2;
mod v128;

pub use bitwise::*;
pub use i8x16::*;
pub use i16x8::*;
pub use i32x4::*;
pub use i64x2::*;
pub use v128::*;
