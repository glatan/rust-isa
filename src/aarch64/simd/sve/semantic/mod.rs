//! Curated SVE operations with semantic names.
//!
//! The per-mnemonic wrappers live in the `mnemonics` submodule; both are
//! re-exported from the parent module.

mod bitwise;
mod float_arith;
mod float_round;
mod integer_arith;
mod reduce;
mod shift;

pub use bitwise::*;
pub use float_arith::*;
pub use float_round::*;
pub use integer_arith::*;
pub use reduce::*;
pub use shift::*;
