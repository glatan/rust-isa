//! WebAssembly SIMD extensions.
//!
//! [Fixed-width SIMD] is implemented by `simd128`.
//! [Relaxed SIMD] is implemented by `relaxed_simd`.
//!
//! `simd128` is compiled only when `target_feature = "simd128"` is enabled.
//! Without that feature this module is empty, and the crate does not reference the SIMD128 C symbols.
//! `relaxed_simd` is compiled only when `simd128` and `relaxed-simd` are both enabled.
//!
//! [Fixed-width SIMD]: https://webassembly.github.io/spec/core/syntax/instructions.html#vector-instructions
//! [Relaxed SIMD]: https://webassembly.github.io/spec/core/exec/numerics.html#relaxed-operations

#[cfg(target_feature = "simd128")]
pub mod simd128;

#[cfg(all(target_feature = "simd128", target_feature = "relaxed-simd"))]
pub mod relaxed_simd;
