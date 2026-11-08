use rust_isa::x86::simd::avx2::{
    V256, vpunpckhbw, vpunpckhdq, vpunpckhqdq, vpunpckhwd, vpunpcklbw, vpunpckldq, vpunpcklqdq,
    vpunpcklwd,
};

use super::support::*;

#[test]
fn unpack_operations() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
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
