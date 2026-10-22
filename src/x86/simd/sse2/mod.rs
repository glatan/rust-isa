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

// Internal modules are grouped by operation.
mod arithmetic;
mod bitwise;
mod compare;
mod convert;
mod memory;
mod movemask;
mod moves;
mod nontemporal;
mod pack;
mod shift;
mod shuffle;
mod unpack;
mod v128;

pub use arithmetic::*;
pub use bitwise::*;
pub use compare::*;
pub use convert::*;
pub use memory::*;
pub use movemask::*;
pub use moves::*;
pub use nontemporal::*;
pub use pack::*;
pub use shift::*;
pub use shuffle::*;
pub use unpack::*;
pub use v128::*;
