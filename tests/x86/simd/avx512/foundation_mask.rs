use rust_isa::x86::simd::avx512dq::{vpmovd2m, vpmovm2d};
use rust_isa::x86::simd::avx512f::*;

use super::support::*;

#[test]
fn foundation_mask_bitwise_and_permute() {
    if !f_enabled() {
        return;
    }
    let a = V512::from_bytes([0xf0; 64]);
    let b = V512::from_bytes([0xcc; 64]);
    assert_eq!(unsafe { vpandq(a, b) }.to_bytes(), [0xc0; 64]);
    assert_eq!(unsafe { vpandnd(a, b) }.to_bytes(), [0x0c; 64]);
    assert_eq!(unsafe { vporq(a, b) }.to_bytes(), [0xfc; 64]);
    assert_eq!(unsafe { vpxorq(a, b) }.to_bytes(), [0x3c; 64]);

    let c = V512::from_bytes([0xaa; 64]);
    assert_eq!(
        unsafe { vpternlogd::<0x96>(a, b, c) }.to_bytes(),
        [0x96; 64]
    );

    let lhs = i32x16(core::array::from_fn(|lane| lane as i32));
    let rhs = i32x16(core::array::from_fn(|lane| {
        if lane < 4 { lane as i32 } else { 100 }
    }));
    assert_eq!(unsafe { vpcmpd::<0>(lhs, rhs) }, 0x000f);
    assert_eq!(unsafe { vpcmpud::<1>(lhs, rhs) }, 0xfff0);
    assert_eq!(unsafe { vpcmpq::<4>(i64x8([1; 8]), i64x8([2; 8])) }, 0xff);

    let bits = i32x16(core::array::from_fn(
        |lane| {
            if lane < 3 { 1 << lane } else { 0 }
        },
    ));
    assert_eq!(unsafe { vptestmd(bits, bits) }, 0x0007);
    assert_eq!(unsafe { vptestnmd(bits, i32x16([0; 16])) }, 0xffff);
    assert_eq!(i32_lanes(unsafe { vpmovm2d(0b0101) })[..4], [-1, 0, -1, 0]);
    assert_eq!(
        unsafe { vpmovd2m(i32x16([-1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])) },
        0b0101
    );
    assert_eq!(i64_lanes(unsafe { vpbroadcastq(9) }), [9; 8]);

    let indices = i64x8([7, 6, 5, 4, 3, 2, 1, 0]);
    assert_eq!(
        i64_lanes(unsafe { vpermq(i64x8([0, 1, 2, 3, 4, 5, 6, 7]), indices) }),
        [7, 6, 5, 4, 3, 2, 1, 0]
    );

    let floats = f32x16(core::array::from_fn(|lane| lane as f32));
    assert_eq!(
        f32_lanes(unsafe { vpermilps::<0x1b>(floats) })[..4],
        [3.0, 2.0, 1.0, 0.0]
    );
    let other = f32x16(core::array::from_fn(|lane| lane as f32 + 100.0));
    assert_eq!(
        f32_lanes(unsafe { vshufps::<0x00>(floats, other) })[..4],
        [0.0, 0.0, 100.0, 100.0]
    );

    let block = f32x16(core::array::from_fn(|lane| lane as f32));
    let insert = i32x4([100, 101, 102, 103]);
    let inserted = unsafe { vinsertf32x4::<1>(block, insert) };
    assert_eq!(i32_lanes(inserted)[4..8], [100, 101, 102, 103]);
    let extracted = unsafe { vextractf32x4::<2>(block) };
    assert_eq!(f32x4_lanes(extracted), [8.0, 9.0, 10.0, 11.0]);
}
