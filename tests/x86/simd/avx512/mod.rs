//! AVX-512 test modules.

mod bf16;
mod bitalg;
mod bw;
mod bw_operations;
mod cd;
mod dq;
mod dq_float;
mod foundation;
mod foundation_compress;
mod foundation_float;
mod foundation_gather;
mod foundation_mask;
mod fp16;
mod fp16_wide;
mod ifma;
mod ifma_wide;
pub(crate) mod support;
mod vbmi;
mod vbmi2;
mod vl;
mod vl_128_256;
mod vnni;
mod vnni_wide;
mod vp2intersect;
mod vpopcntdq;
