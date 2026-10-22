use rust_isa::x86::simd::avx2::{vpackssdw, vpacksswb, vpackusdw, vpackuswb};

use super::support::*;

#[test]
fn pack_operations() {
    if !enabled() {
        return;
    }
    let a = i16x16(core::array::from_fn(|lane| lane as i16 + 1));
    let b = i16x16(core::array::from_fn(|lane| lane as i16 + 17));
    assert_eq!(
        unsafe { vpacksswb(a, b) }.to_bytes(),
        [
            1, 2, 3, 4, 5, 6, 7, 8, 17, 18, 19, 20, 21, 22, 23, 24, 9, 10, 11, 12, 13, 14, 15, 16,
            25, 26, 27, 28, 29, 30, 31, 32
        ]
    );
    assert_eq!(
        unsafe { vpackuswb(a, b) }.to_bytes(),
        [
            1, 2, 3, 4, 5, 6, 7, 8, 17, 18, 19, 20, 21, 22, 23, 24, 9, 10, 11, 12, 13, 14, 15, 16,
            25, 26, 27, 28, 29, 30, 31, 32
        ]
    );

    let a = i32x8(core::array::from_fn(|lane| lane as i32 + 1));
    let b = i32x8(core::array::from_fn(|lane| lane as i32 + 9));
    assert_eq!(
        i16_lanes(unsafe { vpackssdw(a, b) }),
        [1, 2, 3, 4, 9, 10, 11, 12, 5, 6, 7, 8, 13, 14, 15, 16]
    );
    assert_eq!(
        i16_lanes(unsafe { vpackusdw(a, b) }),
        [1, 2, 3, 4, 9, 10, 11, 12, 5, 6, 7, 8, 13, 14, 15, 16]
    );

    let a = i16x16([-200, 200, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    let saturated = unsafe { vpacksswb(a, a) }.to_bytes();
    assert_eq!(saturated[0], 0x80);
    assert_eq!(saturated[1], 0x7f);
    let unsigned = unsafe {
        vpackuswb(
            i16x16([-1, 256, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]),
            a,
        )
    }
    .to_bytes();
    assert_eq!(unsigned[0], 0);
    assert_eq!(unsigned[1], 0xff);
    let saturated =
        unsafe { vpackssdw(i32x8([i32::MIN, i32::MAX, 0, 1, 2, 3, 4, 5]), i32x8([0; 8])) }
            .to_bytes();
    assert_eq!(&saturated[0..4], &[0x00, 0x80, 0xff, 0x7f]);
    let unsigned =
        unsafe { vpackusdw(i32x8([-1, 65_536, 0, 1, 2, 3, 4, 5]), i32x8([0; 8])) }.to_bytes();
    assert_eq!(&unsigned[0..4], &[0x00, 0x00, 0xff, 0xff]);
}
