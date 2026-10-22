use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512vl::{vpmullq_v128, vpmullq_v256};
use rust_isa::x86::simd::sse2::V128;

use super::support::*;

#[test]
fn vl_mullo() {
    if !vl_enabled() {
        return;
    }
    let a = V128::from_bytes([3, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0]);
    let b = V128::from_bytes([7, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0]);
    let c = unsafe { vpmullq_v128(a, b) };
    assert_eq!(&c.to_bytes()[0..8], &21i64.to_le_bytes());
    assert_eq!(&c.to_bytes()[8..16], &55i64.to_le_bytes());

    let mut bytes = [0u8; 32];
    for index in 0..4 {
        bytes[index * 8..index * 8 + 8].copy_from_slice(&(index as i64 + 2).to_le_bytes());
    }
    let a = V256::from_bytes(bytes);
    for index in 0..4 {
        bytes[index * 8..index * 8 + 8].copy_from_slice(&(index as i64 + 3).to_le_bytes());
    }
    let b = V256::from_bytes(bytes);
    let c = unsafe { vpmullq_v256(a, b) };
    let out = c.to_bytes();
    for index in 0..4 {
        assert_eq!(
            i64::from_le_bytes(out[index * 8..index * 8 + 8].try_into().unwrap()),
            (index as i64 + 2) * (index as i64 + 3)
        );
    }
}
