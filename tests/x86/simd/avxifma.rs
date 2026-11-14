use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avxifma::{vpmadd52huq_v128, vpmadd52luq, vpmadd52luq_v128};

use crate::simd::avx512::support::*;

fn i64x4(values: [i64; 4]) -> V256 {
    let mut bytes = [0u8; 32];
    for (chunk, value) in bytes.as_chunks_mut::<8>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V256::from_bytes(bytes)
}

fn i64x4_lanes(value: V256) -> [i64; 4] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<8>().0.iter();
    core::array::from_fn(|_| i64::from_le_bytes(*lanes.next().unwrap()))
}

#[test]
fn avx_ifma() {
    if !is_x86_feature_detected!("avxifma") {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = i64x4([3; 4]);
    let b = i64x4([5; 4]);
    let zero = V256::from_bytes([0; 32]);
    assert_eq!(i64x4_lanes(unsafe { vpmadd52luq(zero, a, b) }), [15; 4]);

    assert_eq!(
        i64x2_lanes(unsafe { vpmadd52luq_v128(i64x2([0; 2]), i64x2([3; 2]), i64x2([5; 2])) }),
        [15; 2]
    );
    assert_eq!(
        i64x2_lanes(unsafe {
            vpmadd52huq_v128(
                i64x2([0; 2]),
                i64x2([1i64 << 26; 2]),
                i64x2([1i64 << 27; 2]),
            )
        }),
        [2; 2]
    );
}
