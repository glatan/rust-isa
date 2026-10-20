//! Architecture-specific ISA extensions.
//!
//! This crate is `no_std` and does not call `core::arch`. The public path is
//! `{arch}::{family}::{extension}::{instruction}`. `family` is a namespace.
//! Instructions are re-exported from the extension, so a call looks like
//! `wasm32::simd::simd128::f32x4_add`.
//!
//! The behavior of an instruction, and the flags required to enable its extension,
//! are documented on that extension.

#![no_std]

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub mod x86;

#[cfg(target_arch = "wasm32")]
pub mod wasm32;
