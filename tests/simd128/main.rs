#![cfg(all(target_arch = "wasm32", target_feature = "simd128"))]

mod bitwise;
mod convert;
mod float;
mod integer;
mod lanes;
mod memory;

#[cfg(target_feature = "relaxed-simd")]
mod relaxed;
