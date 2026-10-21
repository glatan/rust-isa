//! SSE.
//!
//! SSE is part of the x86-64 baseline. The operations are safe.
//!
//! The values are [`V128`]. SSE covers the 32-bit float operations that SSE2
//! keeps unchanged. This module re-exports the SSE operations implemented by
//! the SSE2 backend so that the public path `x86::simd::sse::*` exists.
//!
//! [`V128`]: super::sse2::V128

pub use super::sse2::{
    addps, cmpeqps, cmpleps, cmpltps, divps, maxps, minps, mulps, shufps, sqrtps, subps,
};
