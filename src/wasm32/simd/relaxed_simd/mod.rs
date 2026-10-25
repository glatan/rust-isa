//! Relaxed SIMD.
//!
//! Instruction behavior follows [Relaxed SIMD]. A relaxed operator allows more than one result. The engine chooses one of those results and uses that choice for the whole program. These functions return the engine's choice. They do not force the deterministic profile.
//!
//! Every value is a [`V128`] from fixed-width SIMD. Every function here is safe. Relaxed SIMD has no memory operations and no lane immediates.
//! This module is compiled only when `simd128` and `relaxed-simd` are both enabled.
//! Dependents do not inherit rustflags from this crate. Set these flags with `RUSTFLAGS` or the target's `rustflags`:
//!
//! ```text
//! -C target-feature=+simd128,+relaxed-simd -C linker-plugin-lto
//! ```
//!
//! `cargo rustc -- -C target-feature=+relaxed-simd` and a `#[target_feature]` attribute on only the caller do not enable this module's `cfg` or its build script.
//! Without `linker-plugin-lto`, the linked binary keeps a call into the compiled C and a `v128.store` / `v128.load` pair for each temporary result.
//!
//! A wasm32 build with this feature also needs `clang`, including a wasm32 backend, `wasm_simd128.h`, and support for `-mrelaxed-simd`, and `llvm-ar`.
//! The build script compiles the bundled C and links it as a static library. Prebuilt objects are not published.
//!
//! ```
//! use rust_isa::wasm32::simd::relaxed_simd::f32x4_relaxed_madd;
//! use rust_isa::wasm32::simd::simd128::{f32x4_extract_lane, f32x4_splat};
//!
//! let product = f32x4_relaxed_madd(
//!     f32x4_splat(2.0),
//!     f32x4_splat(3.0),
//!     f32x4_splat(4.0),
//! );
//! assert_eq!(f32x4_extract_lane::<0>(product), 10.0);
//! ```
//!
//! [Relaxed SIMD]: https://webassembly.github.io/spec/core/exec/numerics.html#relaxed-operations
//!
//! Instructions not listed in this module are not exposed.

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::wasm32::simd::relaxed_simd::V128 as _;
//! use rust_isa::wasm32::simd::relaxed_simd::f32x4_relaxed_madd as _;
//! use rust_isa::wasm32::simd::relaxed_simd::f32x4_relaxed_max as _;
//! use rust_isa::wasm32::simd::relaxed_simd::f32x4_relaxed_min as _;
//! use rust_isa::wasm32::simd::relaxed_simd::f32x4_relaxed_nmadd as _;
//! use rust_isa::wasm32::simd::relaxed_simd::f64x2_relaxed_madd as _;
//! use rust_isa::wasm32::simd::relaxed_simd::f64x2_relaxed_max as _;
//! use rust_isa::wasm32::simd::relaxed_simd::f64x2_relaxed_min as _;
//! use rust_isa::wasm32::simd::relaxed_simd::f64x2_relaxed_nmadd as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i16x8_relaxed_dot_i8x16_i7x16_s as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i16x8_relaxed_laneselect as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i16x8_relaxed_q15mulr_s as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i32x4_relaxed_dot_i8x16_i7x16_add_s as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i32x4_relaxed_laneselect as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i32x4_relaxed_trunc_f32x4_s as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i32x4_relaxed_trunc_f32x4_u as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i32x4_relaxed_trunc_f64x2_s_zero as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i32x4_relaxed_trunc_f64x2_u_zero as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i64x2_relaxed_laneselect as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i8x16_relaxed_laneselect as _;
//! use rust_isa::wasm32::simd::relaxed_simd::i8x16_relaxed_swizzle as _;
//! ```
//! <!-- doctest-inventory:end -->

pub use super::simd128::V128;

mod f32x4;
mod f64x2;
mod i16x8;
mod i32x4;
mod i64x2;
mod i8x16;

pub use f32x4::*;
pub use f64x2::*;
pub use i8x16::*;
pub use i16x8::*;
pub use i32x4::*;
pub use i64x2::*;
