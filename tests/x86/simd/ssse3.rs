use rust_isa::x86::simd::sse2::V128;
use rust_isa::x86::simd::ssse3::{
    pabsb, pabsd, pabsw, palignr, phaddd, phaddsw, phaddw, phsubd, phsubsw, phsubw, pmaddubsw,
    pmulhrsw, pshufb, psignb, psignd, psignw,
};

use super::support::*;

#[test]
fn ssse3_abs_and_shuffle() {
    if !is_x86_feature_detected!("ssse3") {
        return;
    }
    let a = V128::from_bytes([
        0xff, 0xfe, 2, 0x80, 0, 1, 0xff, 0x7f, 3, 4, 5, 6, 7, 8, 9, 10,
    ]);
    assert_eq!(&unsafe { pabsb(a) }.to_bytes()[..4], &[1, 2, 2, 0x80]);
    assert_eq!(
        i16_lanes(unsafe { pabsw(i16x8([-1, -2, 3, -32768, 5, 6, 7, 8])) }),
        [1, 2, 3, -32768, 5, 6, 7, 8]
    );
    assert_eq!(
        i32_lanes(unsafe { pabsd(i32x4([-1, 2, -3, i32::MIN])) }),
        [1, 2, 3, i32::MIN]
    );

    let source = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let identity = V128::from_bytes(core::array::from_fn(|index| index as u8));
    assert_eq!(
        unsafe { pshufb(source, identity) }.to_bytes(),
        source.to_bytes()
    );
    let zero = V128::from_bytes([0x80; 16]);
    assert_eq!(unsafe { pshufb(source, zero) }.to_bytes(), [0; 16]);
    let reverse = V128::from_bytes(core::array::from_fn(|index| 15 - index as u8));
    assert_eq!(
        unsafe { pshufb(source, reverse) }.to_bytes(),
        core::array::from_fn(|index| 15 - index as u8)
    );
}

#[test]
fn ssse3_horizontal_and_saturating() {
    if !is_x86_feature_detected!("ssse3") {
        return;
    }
    let a = i16x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = i16x8([10, 20, 30, 40, 50, 60, 70, 80]);
    assert_eq!(
        i16_lanes(unsafe { phaddw(a, b) }),
        [3, 7, 11, 15, 30, 70, 110, 150]
    );
    assert_eq!(
        i16_lanes(unsafe { phsubw(a, b) }),
        [-1, -1, -1, -1, -10, -10, -10, -10]
    );
    assert_eq!(
        i16_lanes(unsafe { phaddsw(i16x8([32767, 1, 0, 0, 0, 0, 0, 0]), i16x8([0; 8])) }),
        [32767, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        i16_lanes(unsafe { phsubsw(i16x8([-32768, 1, 0, 0, 0, 0, 0, 0]), i16x8([0; 8])) }),
        [-32768, 0, 0, 0, 0, 0, 0, 0]
    );

    let a = i32x4([1, 2, 3, 4]);
    let b = i32x4([10, 20, 30, 40]);
    assert_eq!(i32_lanes(unsafe { phaddd(a, b) }), [3, 7, 30, 70]);
    assert_eq!(i32_lanes(unsafe { phsubd(a, b) }), [-1, -1, -10, -10]);
}

#[test]
fn ssse3_multiply_and_sign() {
    if !is_x86_feature_detected!("ssse3") {
        return;
    }
    let a = V128::from_bytes([
        1, 0xff, 2, 0xfe, 3, 0xff, 4, 0xfe, 5, 0xff, 6, 0xfe, 7, 0xff, 8, 0xfe,
    ]);
    let b = V128::from_bytes([1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8]);
    assert_eq!(
        i16_lanes(unsafe { pmaddubsw(a, b) }),
        [256, 512, 774, 1032, 1300, 1560, 1834, 2096]
    );
    assert_eq!(
        i16_lanes(unsafe { pmulhrsw(i16x8([0x4000; 8]), i16x8([0x4000; 8])) }),
        [8192; 8]
    );
    assert_eq!(
        i16_lanes(unsafe { pmulhrsw(i16x8([i16::MIN; 8]), i16x8([i16::MIN; 8])) }),
        [i16::MIN; 8]
    );

    let a = V128::from_bytes([
        1, 0xff, 2, 0xfe, 3, 0xff, 4, 0xfe, 5, 0xff, 6, 0xfe, 7, 0xff, 8, 0xfe,
    ]);
    let b = V128::from_bytes([
        1, 0xff, 2, 0xfe, 3, 0xff, 4, 0xfe, 5, 0xff, 6, 0xfe, 7, 0xff, 8, 0xfe,
    ]);
    assert_eq!(
        unsafe { psignb(a, b) }.to_bytes(),
        [1, 1, 2, 2, 3, 1, 4, 2, 5, 1, 6, 2, 7, 1, 8, 2]
    );
    let zero = V128::from_bytes([0; 16]);
    assert_eq!(unsafe { psignb(a, zero) }.to_bytes(), [0; 16]);
    assert_eq!(
        i16_lanes(unsafe { psignw(i16x8([1, -2, 3, -4, 5, -6, 7, -8]), i16x8([-1; 8])) }),
        [-1, 2, -3, 4, -5, 6, -7, 8]
    );
    assert_eq!(
        i32_lanes(unsafe { psignd(i32x4([1, -2, 3, -4]), i32x4([-1; 4])) }),
        [-1, 2, -3, 4]
    );
}

#[test]
fn ssse3_palignr() {
    if !is_x86_feature_detected!("ssse3") {
        return;
    }
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8 + 16));
    let b = V128::from_bytes(core::array::from_fn(|index| index as u8));
    assert_eq!(unsafe { palignr::<0>(a, b) }.to_bytes(), b.to_bytes());
    assert_eq!(
        unsafe { palignr::<4>(a, b) }.to_bytes(),
        core::array::from_fn(|index| (index + 4) as u8)
    );
    assert_eq!(unsafe { palignr::<16>(a, b) }.to_bytes(), a.to_bytes());
}
