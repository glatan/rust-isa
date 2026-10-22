use rust_isa::x86::simd::avx::{
    vbroadcastf128, vextractf128, vinsertf128, vperm2f128, vpermilpd, vpermilps,
};

use super::support::*;

#[test]
fn lane_insert_extract_and_permute() {
    if !enabled() {
        return;
    }
    let a = i32x8([1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(i32x4_lanes(unsafe { vextractf128::<1>(a) }), [5, 6, 7, 8]);
    assert_eq!(
        i32_lanes(unsafe { vinsertf128::<1>(a, i32x4([101, 102, 103, 104])) }),
        [1, 2, 3, 4, 101, 102, 103, 104]
    );

    let b = i32x8([101, 102, 103, 104, 105, 106, 107, 108]);
    assert_eq!(
        i32_lanes(unsafe { vperm2f128::<0x21>(a, b) }),
        [5, 6, 7, 8, 101, 102, 103, 104]
    );

    assert_eq!(
        i32_lanes(unsafe { vpermilps::<0x1b>(a) }),
        [4, 3, 2, 1, 8, 7, 6, 5]
    );
    assert_eq!(
        f64_lanes(unsafe { vpermilpd::<0x5>(f64x4([1.0, 2.0, 3.0, 4.0])) }),
        [2.0, 1.0, 4.0, 3.0]
    );

    let source = i32x4([10, 20, 30, 40]);
    assert_eq!(
        i32_lanes(unsafe { vbroadcastf128(source.as_ptr()) }),
        [10, 20, 30, 40, 10, 20, 30, 40]
    );
}
