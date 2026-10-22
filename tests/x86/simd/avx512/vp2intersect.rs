use rust_isa::x86::simd::avx512vp2intersect::vp2intersectd;

use super::support::*;

#[test]
fn avx512vp2intersect_masks() {
    if !is_x86_feature_detected!("avx512vp2intersect") {
        return;
    }
    let a = i32x16(core::array::from_fn(|index| index as i32));
    let b = i32x16(core::array::from_fn(|index| 15 - index as i32));
    assert_eq!(unsafe { vp2intersectd(a, b) }, (0xffff, 0xffff));
}
