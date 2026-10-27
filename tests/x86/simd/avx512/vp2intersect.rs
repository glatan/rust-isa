use rust_isa::x86::simd::avx512vp2intersect::vp2intersectd;

use super::support::*;

#[test]
fn avx512vp2intersect_masks() {
    if !is_x86_feature_detected!("avx512vp2intersect") {
        return;
    }
    let a = i32x16(core::array::from_fn(|index| {
        if index < 4 {
            index as i32 + 1
        } else {
            100 + index as i32
        }
    }));
    let b = i32x16(core::array::from_fn(|index| match index {
        0 => 2,
        1 => 4,
        _ => 200 + index as i32,
    }));
    assert_eq!(unsafe { vp2intersectd(a, b) }, (0b1010, 0b0011));
}
