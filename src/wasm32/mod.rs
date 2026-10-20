//! Instruction sets for `wasm32`.
//!
//! Instructions live in an extension module. This module is a namespace.
//! A Rust function name is the WebAssembly text instruction name with `.` written as `_`.

#[cfg(feature = "wide-arithmetic")]
pub mod wide_arithmetic;

pub mod simd;
