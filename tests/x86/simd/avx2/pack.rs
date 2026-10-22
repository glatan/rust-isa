use rust_isa::x86::simd::avx2::{
    V256, vpackssdw, vpacksswb, vpackusdw, vpackuswb, vpunpckhbw, vpunpckhdq, vpunpckhqdq,
    vpunpckhwd, vpunpcklbw, vpunpckldq, vpunpcklqdq, vpunpcklwd,
};

use super::support::*;

#[test]
fn pack_and_unpack() {
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

    let a = V256::from_bytes(core::array::from_fn(|index| index as u8));
    let b = V256::from_bytes(core::array::from_fn(|index| index as u8 + 100));
    assert_eq!(
        unsafe { vpunpcklbw(a, b) }.to_bytes(),
        [
            0, 100, 1, 101, 2, 102, 3, 103, 4, 104, 5, 105, 6, 106, 7, 107, 16, 116, 17, 117, 18,
            118, 19, 119, 20, 120, 21, 121, 22, 122, 23, 123
        ]
    );
    assert_eq!(
        unsafe { vpunpckhbw(a, b) }.to_bytes(),
        [
            8, 108, 9, 109, 10, 110, 11, 111, 12, 112, 13, 113, 14, 114, 15, 115, 24, 124, 25, 125,
            26, 126, 27, 127, 28, 128, 29, 129, 30, 130, 31, 131
        ]
    );

    let a = i16x16(core::array::from_fn(|lane| lane as i16));
    let b = i16x16(core::array::from_fn(|lane| lane as i16 + 100));
    assert_eq!(
        i16_lanes(unsafe { vpunpcklwd(a, b) }),
        [
            0, 100, 1, 101, 2, 102, 3, 103, 8, 108, 9, 109, 10, 110, 11, 111
        ]
    );
    assert_eq!(
        i16_lanes(unsafe { vpunpckhwd(a, b) }),
        [
            4, 104, 5, 105, 6, 106, 7, 107, 12, 112, 13, 113, 14, 114, 15, 115
        ]
    );

    let a = i32x8(core::array::from_fn(|lane| lane as i32));
    let b = i32x8(core::array::from_fn(|lane| lane as i32 + 100));
    assert_eq!(
        i32_lanes(unsafe { vpunpckldq(a, b) }),
        [0, 100, 1, 101, 4, 104, 5, 105]
    );
    assert_eq!(
        i32_lanes(unsafe { vpunpckhdq(a, b) }),
        [2, 102, 3, 103, 6, 106, 7, 107]
    );

    let a = i64x4([0, 1, 2, 3]);
    let b = i64x4([100, 101, 102, 103]);
    assert_eq!(i64_lanes(unsafe { vpunpcklqdq(a, b) }), [0, 100, 2, 102]);
    assert_eq!(i64_lanes(unsafe { vpunpckhqdq(a, b) }), [1, 101, 3, 103]);
}
