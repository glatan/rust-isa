#![cfg(target_arch = "wasm32")]

#[cfg(target_feature = "simd128")]
mod simd;

#[cfg(feature = "wide-arithmetic")]
mod wide_arithmetic;
