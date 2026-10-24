use rust_isa::x86::crypto::aes::aesenc;
use rust_isa::x86::simd::sse2::V128;
use std::hint::black_box;

#[inline(never)]
#[target_feature(enable = "aes")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_aes(a: &V128, b: &V128) -> V128 {
    unsafe { aesenc(aesenc(*a, *b), *b) }
}

fn main() {
    let a = black_box(V128::from_bytes([1; 16]));
    let b = black_box(V128::from_bytes([2; 16]));
    let aes = unsafe { kernel_aes(&a, &b) };
    black_box(aes);
}
