use rust_isa::x86::simd::avx2::{vpbroadcastb, vpbroadcastq, vpbroadcastw};

use super::support::*;

#[test]
fn broadcast() {
    if !enabled() {
        return;
    }
    assert_eq!(unsafe { vpbroadcastb(0xab) }.to_bytes(), [0xab; 32]);
    assert_eq!(u16_lanes(unsafe { vpbroadcastw(0x1234) }), [0x1234; 16]);
    assert_eq!(
        i64_lanes(unsafe { vpbroadcastq(0x0123_4567_89ab_cdef) }),
        [0x0123_4567_89ab_cdef; 4]
    );
}
