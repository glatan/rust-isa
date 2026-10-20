use rust_isa::x86::simd::avx2::{V256, vpaddd, vpmulld};
use std::hint::black_box;

#[inline(never)]
#[target_feature(enable = "avx2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel(a: &V256, b: &V256) -> V256 {
    vpmulld(vpaddd(*a, *b), *b)
}

fn main() {
    let a = black_box(V256::from_bytes([1; 32]));
    let b = black_box(V256::from_bytes([2; 32]));
    let c = unsafe { kernel(&a, &b) };
    black_box(c);
}
