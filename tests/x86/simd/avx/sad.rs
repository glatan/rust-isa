use rust_isa::x86::simd::avx::vmpsadbw;
use rust_isa::x86::simd::avx2::V256;

use super::support::*;

#[test]
fn sum_of_absolute_differences() {
    if !enabled() {
        return;
    }
    let sad = unsafe { vmpsadbw::<0>(V256::from_bytes([1; 32]), V256::from_bytes([0; 32])) };
    let sad_bytes = sad.to_bytes();
    for lane in 0..16 {
        assert_eq!(
            i16::from_le_bytes(sad_bytes[lane * 2..lane * 2 + 2].try_into().unwrap()),
            4
        );
    }
}
