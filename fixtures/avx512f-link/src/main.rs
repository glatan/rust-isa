use rust_isa::x86::simd::avx512f::{V512, vpaddd};
use std::hint::black_box;

#[inline(never)]
#[target_feature(enable = "avx512f")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel(a: &V512, b: &V512) -> V512 {
    unsafe { vpaddd(vpaddd(*a, *b), *b) }
}

fn main() {
    let a = black_box(V512::from_bytes([1; 64]));
    let b = black_box(V512::from_bytes([2; 64]));
    let c = unsafe { kernel(&a, &b) };
    black_box(c);
}
