use rust_isa::x86::simd::sse2::{V128, paddd};
use std::hint::black_box;

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel(a: &V128, b: &V128) -> V128 {
    paddd(paddd(*a, *b), *b)
}

fn main() {
    let a = black_box(V128::from_bytes([
        1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0,
    ]));
    let b = black_box(V128::from_bytes([
        10, 0, 0, 0, 20, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0,
    ]));
    let c = unsafe { kernel(&a, &b) };
    black_box(c);
}
