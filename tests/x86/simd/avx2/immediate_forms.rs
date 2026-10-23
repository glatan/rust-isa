use rust_isa::x86::simd::avx2::{
    vextracti128, vinserti128, vperm2i128, vpermps, vpermq, vpshufd, vpshufhw, vpshuflw,
};

use super::support::*;

#[test]
fn immediate_forms() {
    if !enabled() {
        return;
    }
    let a = i64x4([10, 20, 30, 40]);
    let b = i64x4([100, 200, 300, 400]);
    assert_eq!(i64_lanes(unsafe { vpermq::<0x1b>(a) }), [40, 30, 20, 10]);
    assert_eq!(
        i64_lanes(unsafe { vperm2i128::<0x21>(a, b) }),
        [30, 40, 100, 200]
    );
    assert_eq!(
        i32_lanes(unsafe {
            vpermps(
                i32x8([1, 2, 3, 4, 5, 6, 7, 8]),
                i32x8([7, 6, 5, 4, 3, 2, 1, 0]),
            )
        }),
        [8, 7, 6, 5, 4, 3, 2, 1]
    );
    assert_eq!(
        i64_lanes(unsafe { vinserti128::<1>(a, i64x2([100, 200])) }),
        [10, 20, 100, 200]
    );
    assert_eq!(i64x2_lanes(unsafe { vextracti128::<1>(a) }), [30, 40]);

    let a = i32x8(core::array::from_fn(|lane| lane as i32 + 1));
    assert_eq!(
        i32_lanes(unsafe { vpshufd::<0x1b>(a) }),
        [4, 3, 2, 1, 8, 7, 6, 5]
    );
    let a = i16x16(core::array::from_fn(|lane| lane as i16 + 1));
    assert_eq!(
        i16_lanes(unsafe { vpshufhw::<0x1b>(a) }),
        [1, 2, 3, 4, 8, 7, 6, 5, 9, 10, 11, 12, 16, 15, 14, 13]
    );
    assert_eq!(
        i16_lanes(unsafe { vpshuflw::<0x1b>(a) }),
        [4, 3, 2, 1, 5, 6, 7, 8, 12, 11, 10, 9, 13, 14, 15, 16]
    );
}
