//! NEON.
//!
//! NEON is part of the AArch64 baseline. The arithmetic operations are safe.
//! Loads and stores take raw pointers and are `unsafe`. The values are
//! [`V128`].
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//! The build compiles the bundled C with clang. Const-generic lane and shift
//! immediates document their valid range on each function.
//!
//! Instructions not listed in this module are not exposed.

mod absdiff;
mod compare;
mod convert;
mod crypto;
mod extract;
mod float_arith;
mod float_compare;
mod halving;
mod integer;
mod integer_multiply_accumulate;
mod lane;
mod minmax;
mod narrow;
mod reciprocal;
mod reduce;
mod saturating;
mod scalar;
mod shift;
mod shift_immediate;
mod shuffle;
mod structure;
mod v128;
mod widen;

pub use absdiff::*;
pub use compare::*;
pub use convert::*;
pub use crypto::*;
pub use extract::*;
pub use float_arith::*;
pub use float_compare::*;
pub use halving::*;
pub use integer::*;
pub use integer_multiply_accumulate::*;
pub use lane::*;
pub use minmax::*;
pub use narrow::*;
pub use reciprocal::*;
pub use reduce::*;
pub use saturating::*;
pub use scalar::*;
pub use shift::*;
pub use shift_immediate::*;
pub use shuffle::*;
pub use structure::*;
pub use v128::*;
pub use widen::*;
