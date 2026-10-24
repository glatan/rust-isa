//! AArch64 SVE slice API.
//!
//! SVE is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable SVE with `#[target_feature(enable = "sve")]` or a global
//! target feature.
//!
//! SVE vectors have a runtime vector length, so this module exposes slice
//! operations rather than per-register values. The slices use the element type
//! in each function name. The C loop chooses the vector length at run time.
//! Most functions that take multiple slices require them to have the same
//! length; a different length panics. Broadcast and replicate loads are the
//! exception: they read a fixed-size input and write an output slice whose
//! length is independent. The functions do not allocate.
//!
//! The per-register API is intentionally omitted because a fixed-length
//! [`V128`](crate::aarch64::simd::neon::V128) cannot represent an SVE register.
//! Each instruction is exposed as a slice operation instead.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.

mod mnemonics;
mod semantic;
pub use mnemonics::*;
pub use semantic::*;
