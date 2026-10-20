//! x86 SIMD extensions.
//!
//! The extensions are grouped by instruction set. The instruction name is
//! exposed directly from the extension module.

pub mod avx2;
pub mod avx512bw;
pub mod avx512dq;
pub mod avx512f;
pub mod avx512ifma;
pub mod avx512vbmi;
pub mod avx512vl;
pub mod avx512vnni;
pub mod sse2;
