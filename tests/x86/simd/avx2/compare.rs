use rust_isa::x86::simd::avx2::{V256, vpcmpeqb, vpcmpeqd, vpcmpeqw};

use super::support::*;

#[test]
fn compare() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = V256::from_bytes([0b1010_1010; 32]);
    let b = V256::from_bytes([0b1100_1100; 32]);
    assert_eq!(unsafe { vpcmpeqb(a, a) }.to_bytes(), [0xff; 32]);
    assert_eq!(unsafe { vpcmpeqb(a, b) }.to_bytes(), [0; 32]);
    assert_eq!(
        unsafe { vpcmpeqw(i16x16([5; 16]), i16x16([5; 16])) }.to_bytes(),
        [0xff; 32]
    );
    assert_eq!(
        unsafe { vpcmpeqd(i32x8([5; 8]), i32x8([6; 8])) }.to_bytes(),
        [0; 32]
    );
}
