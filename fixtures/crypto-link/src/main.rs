use rust_isa::x86::crypto::aes::aesenc;
use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::fma::vfmadd132ps;
use rust_isa::x86::simd::sse2::V128;
use std::hint::black_box;

#[inline(never)]
#[target_feature(enable = "aes")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_aes(a: &V128, b: &V128) -> V128 {
    aesenc(aesenc(*a, *b), *b)
}

#[inline(never)]
#[target_feature(enable = "avx2")]
#[target_feature(enable = "fma")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_fma(a: &V256, b: &V256, c: &V256) -> V256 {
    vfmadd132ps(vfmadd132ps(*a, *b, *c), *b, *c)
}

fn main() {
    let a = black_box(V128::from_bytes([1; 16]));
    let b = black_box(V128::from_bytes([2; 16]));
    let aes = unsafe { kernel_aes(&a, &b) };
    black_box(aes);

    let a = black_box(V256::from_bytes([1; 32]));
    let b = black_box(V256::from_bytes([2; 32]));
    let c = black_box(V256::from_bytes([3; 32]));
    let fma = unsafe { kernel_fma(&a, &b, &c) };
    black_box(fma);
}
