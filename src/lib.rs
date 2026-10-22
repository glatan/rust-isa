//! Architecture-specific ISA extensions.
//!
//! This crate is `no_std` and does not call `core::arch`. The public path is
//! `{arch}::{family}::{extension}::{instruction}`. `family` is a namespace.
//! `wasm32::wide_arithmetic` is the exception: it sits directly under the arch
//! module because it has no family.
//! Instructions are re-exported from the extension, so a call looks like
//! `wasm32::simd::simd128::f32x4_add`.
//!
//! The behavior of an instruction, and the flags required to enable its extension,
//! are documented on that extension.

#![no_std]

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "loongarch64")]
pub mod loongarch;

#[cfg(target_arch = "arm")]
pub mod arm;

#[cfg(target_arch = "mips64")]
pub mod mips;

#[cfg(target_arch = "powerpc64")]
pub mod powerpc;

#[cfg(target_arch = "s390x")]
pub mod s390x;

#[cfg(target_arch = "riscv64")]
pub mod riscv;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub mod x86;

#[cfg(target_arch = "wasm32")]
pub mod wasm32;
