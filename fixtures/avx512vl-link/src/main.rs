use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512vl::{vpaddb_v256, vpermb_v256};
use std::hint::black_box;

#[inline(never)]
#[target_feature(enable = "avx512f,avx512bw,avx512vl")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_add(a: &V256, b: &V256) -> V256 {
    vpaddb_v256(vpaddb_v256(*a, *b), *b)
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512vl,avx512vbmi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_perm(a: &V256, idx: &V256) -> V256 {
    vpermb_v256(vpermb_v256(*a, *idx), *idx)
}

fn main() {
    let a = black_box(V256::from_bytes([1; 32]));
    let b = black_box(V256::from_bytes([2; 32]));
    let idx = black_box(V256::from_bytes(core::array::from_fn(|i| i as u8)));
    let add = unsafe { kernel_add(&a, &b) };
    let perm = unsafe { kernel_perm(&a, &idx) };
    black_box(add);
    black_box(perm);
}
