use rust_isa::x86::simd::sse3::{
    addsubpd, addsubps, haddpd, haddps, hsubpd, hsubps, lddqu, movddup, movshdup, movsldup,
};

use super::support::*;

#[test]
fn sse3_addsub_and_horizontal() {
    if !is_x86_feature_detected!("sse3") {
        return;
    }
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    assert_eq!(
        f32_lanes(unsafe { addsubps(a, b) }),
        [-9.0, 22.0, -27.0, 44.0]
    );
    assert_eq!(f32_lanes(unsafe { haddps(a, b) }), [3.0, 7.0, 30.0, 70.0]);
    assert_eq!(
        f32_lanes(unsafe { hsubps(a, b) }),
        [-1.0, -1.0, -10.0, -10.0]
    );
    assert_eq!(f32_lanes(unsafe { movsldup(a) }), [1.0, 1.0, 3.0, 3.0]);
    assert_eq!(f32_lanes(unsafe { movshdup(a) }), [2.0, 2.0, 4.0, 4.0]);

    let a = f64x2([1.0, 2.0]);
    let b = f64x2([10.0, 20.0]);
    assert_eq!(f64_lanes(unsafe { addsubpd(a, b) }), [-9.0, 22.0]);
    assert_eq!(f64_lanes(unsafe { haddpd(a, b) }), [3.0, 30.0]);
    assert_eq!(f64_lanes(unsafe { hsubpd(a, b) }), [-1.0, -10.0]);
    assert_eq!(f64_lanes(unsafe { movddup(a) }), [1.0, 1.0]);
}

#[test]
fn sse3_lddqu_loads_unaligned() {
    if !is_x86_feature_detected!("sse3") {
        return;
    }
    let memory = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
    let value = unsafe { lddqu(memory.as_ptr().add(1)) };
    assert_eq!(
        value.to_bytes(),
        [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
    );
}
