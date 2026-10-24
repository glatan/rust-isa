//! x86 matrix extensions.
//!
//! The public path is `x86::matrix::{extension}::{instruction}`.

#[cfg(target_arch = "x86_64")]
pub mod amx_avx512;
#[cfg(target_arch = "x86_64")]
pub mod amx_movrs;
#[cfg(target_arch = "x86_64")]
pub mod amx_tile;
