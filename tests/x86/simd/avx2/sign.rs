use rust_isa::x86::simd::avx2::{vpsignb, vpsignd, vpsignw};

use super::support::*;

#[test]
fn sign() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let signs = i8x32(core::array::from_fn(|lane| match lane % 3 {
        0 => 1,
        1 => -1,
        _ => 0,
    }));
    let expected = i8x32(core::array::from_fn(|lane| match lane % 3 {
        0 => 5,
        1 => -5,
        _ => 0,
    }));
    assert_eq!(
        i8_lanes(unsafe { vpsignb(i8x32([5; 32]), signs) }),
        i8_lanes(expected)
    );

    let signs = i16x16(core::array::from_fn(|lane| match lane % 3 {
        0 => 1,
        1 => -1,
        _ => 0,
    }));
    assert_eq!(
        i16_lanes(unsafe { vpsignw(i16x16([100; 16]), signs) }),
        [
            100, -100, 0, 100, -100, 0, 100, -100, 0, 100, -100, 0, 100, -100, 0, 100
        ]
    );

    let signs = i32x8(core::array::from_fn(|lane| match lane % 3 {
        0 => 1,
        1 => -1,
        _ => 0,
    }));
    assert_eq!(
        i32_lanes(unsafe { vpsignd(i32x8([100; 8]), signs) }),
        [100, -100, 0, 100, -100, 0, 100, -100]
    );
}
