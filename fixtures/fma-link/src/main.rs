use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::fma::vfmadd132ps;
use std::hint::black_box;

#[inline(never)]
#[target_feature(enable = "avx2")]
#[target_feature(enable = "fma")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_fma(a: &V256, b: &V256, c: &V256) -> V256 {
    unsafe { vfmadd132ps(vfmadd132ps(*a, *b, *c), *b, *c) }
}

fn main() {
    let a = black_box(V256::from_bytes([1; 32]));
    let b = black_box(V256::from_bytes([2; 32]));
    let c = black_box(V256::from_bytes([3; 32]));
    let fma = unsafe { kernel_fma(&a, &b, &c) };
    black_box(fma);
}
