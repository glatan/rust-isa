//! WebAssembly SIMD128.
//!
//! Every value is a [`V128`]. The instruction decides what the bits mean.
//! `i32x4.add` and `f32x4.add` are different instructions, so [`V128`] does not implement `Add`.
//!
//! Behavior follows [Fixed-width SIMD]. The lane operations are specified with the [vector operations].
//!
//! [Fixed-width SIMD]: https://webassembly.github.io/spec/core/syntax/instructions.html#vector-instructions
//! [vector operations]: https://webassembly.github.io/spec/core/exec/numerics.html#vector-operations
//!
//! This module is compiled only when `target_feature = "simd128"` is enabled.
//! Dependents do not inherit rustflags from this crate. Set both flags with `RUSTFLAGS` or the target's `rustflags`:
//!
//! ```text
//! -C target-feature=+simd128 -C linker-plugin-lto
//! ```
//!
//! `cargo rustc -- -C target-feature=+simd128` and a `#[target_feature]` attribute on only the caller do not enable this module's `cfg` or its build script.
//! Without `linker-plugin-lto`, the linked binary keeps a call into the compiled C and a `v128.store` / `v128.load` pair for each temporary result.
//!
//! A wasm32 build with this feature also needs `clang`, including a wasm32 backend and `wasm_simd128.h`, and `llvm-ar`.
//! The build script compiles the bundled C and links it as a static library. Prebuilt objects are not published.
//!
//! Arithmetic and comparisons are safe. Loads, stores, and lane loads and stores are `unsafe`.
//! A misaligned address does not trap. An out-of-bounds address traps.
//!
//! An immediate lane index is `const LANE: u32`. A constant outside the lane count does not compile.
//!
//! A comparison writes all bits one to a true lane and all bits zero to a false lane.
//! For `f32` and `f64`, a comparison involving NaN is false, except `ne`, which is true.
//!
//! `i8x16.shuffle` is not provided: its 16 lane indices are an immediate in the instruction encoding.
//! [`i8x16_swizzle`] takes those indices from a register.
//!
//! ```
//! use rust_isa::wasm32::simd::simd128::{
//!     f32x4_add, f32x4_extract_lane, f32x4_splat, i8x16_eq, i8x16_splat,
//! };
//!
//! let sum = f32x4_add(f32x4_splat(1.5), f32x4_splat(2.5));
//! assert_eq!(f32x4_extract_lane::<0>(sum), 4.0);
//! assert_eq!(
//!     i8x16_eq(i8x16_splat(1), i8x16_splat(1)).to_bytes(),
//!     [0xff; 16]
//! );
//! assert_eq!(i8x16_eq(i8x16_splat(1), i8x16_splat(2)).to_bytes(), [0; 16]);
//! ```

mod f32x4;
mod f64x2;
mod i16x8;
mod i32x4;
mod i64x2;
mod i8x16;
mod v128;

pub use f32x4::*;
pub use f64x2::*;
pub use i8x16::*;
pub use i16x8::*;
pub use i32x4::*;
pub use i64x2::*;
pub use v128::*;
