use rust_isa::x86::simd::avx::{vbroadcastsd, vbroadcastss};

use crate::simd::support::*;

#[test]
fn broadcast_values() {
    if !is_x86_feature_detected!("avx") {
        return;
    }
    assert_eq!(f32x8_lanes(unsafe { vbroadcastss(1.5) }), [1.5; 8]);
    assert_eq!(f64x4_lanes(unsafe { vbroadcastsd(2.5) }), [2.5; 4]);
}
