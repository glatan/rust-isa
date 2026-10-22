use rust_isa::x86::simd::avx2::{V256, vpand, vpcmpeqb, vpor, vpxor};

use super::support::*;

#[test]
fn bitwise_and_compare() {
    if !enabled() {
        return;
    }
    let a = V256::from_bytes([0b1010_1010; 32]);
    let b = V256::from_bytes([0b1100_1100; 32]);
    assert_eq!(unsafe { vpand(a, b) }.to_bytes(), [0b1000_1000; 32]);
    assert_eq!(unsafe { vpcmpeqb(a, a) }.to_bytes(), [0xff; 32]);
    assert_eq!(unsafe { vpcmpeqb(a, b) }.to_bytes(), [0; 32]);
}

#[test]
fn bitwise_or_and_xor() {
    if !enabled() {
        return;
    }
    let a = V256::from_bytes([0b1010_1010; 32]);
    let b = V256::from_bytes([0b1100_1100; 32]);
    assert_eq!(unsafe { vpor(a, b) }.to_bytes(), [0b1110_1110; 32]);
    assert_eq!(unsafe { vpxor(a, b) }.to_bytes(), [0b0110_0110; 32]);
}
