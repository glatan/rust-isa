//! x86 matrix extensions.
//!
//! The public path is `x86::matrix::{extension}::{instruction}`.

#[cfg(target_arch = "x86_64")]
pub mod amx_avx512;
#[cfg(target_arch = "x86_64")]
pub mod amx_bf16;
mod amx_common;
#[cfg(target_arch = "x86_64")]
pub mod amx_complex;
#[cfg(target_arch = "x86_64")]
pub mod amx_fp16;
#[cfg(target_arch = "x86_64")]
pub mod amx_fp8;
#[cfg(target_arch = "x86_64")]
pub mod amx_int8;
#[cfg(target_arch = "x86_64")]
pub mod amx_movrs;
#[cfg(target_arch = "x86_64")]
pub mod amx_tile;
