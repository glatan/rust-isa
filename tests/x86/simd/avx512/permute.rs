use rust_isa::x86::simd::avx512f::*;

use super::support::*;

#[test]
fn avx512f_permute_and_shuffle() {
    if !f_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
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
}
