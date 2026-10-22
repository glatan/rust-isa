use rust_isa::x86::simd::avx2::{V256, vpblendvb, vpbroadcastb, vpbroadcastq, vpbroadcastw};

use super::support::*;

#[test]
fn broadcast_and_blend() {
    if !enabled() {
        return;
    }
    assert_eq!(unsafe { vpbroadcastb(0xab) }.to_bytes(), [0xab; 32]);
    assert_eq!(u16_lanes(unsafe { vpbroadcastw(0x1234) }), [0x1234; 16]);
    assert_eq!(
        i64_lanes(unsafe { vpbroadcastq(0x0123_4567_89ab_cdef) }),
        [0x0123_4567_89ab_cdef; 4]
    );

    let mask = V256::from_bytes(core::array::from_fn(|index| {
        if index % 2 == 0 { 0x80 } else { 0x00 }
    }));
    assert_eq!(
        unsafe { vpblendvb(V256::from_bytes([1; 32]), V256::from_bytes([2; 32]), mask) }.to_bytes(),
        core::array::from_fn(|index| if index % 2 == 0 { 2 } else { 1 })
    );
}
