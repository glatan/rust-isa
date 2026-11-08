use rust_isa::x86::simd::avx2::{V256, vpbroadcastd, vpermd, vpshufb};

use super::support::*;

#[test]
fn shuffle_and_broadcast() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let source = V256::from_bytes(core::array::from_fn::<u8, 32, _>(|index| index as u8));
    let identity = V256::from_bytes(core::array::from_fn::<u8, 32, _>(|index| index as u8));
    assert_eq!(
        unsafe { vpshufb(source, identity) }.to_bytes(),
        source.to_bytes()
    );

    let indices = i32x8([7, 6, 5, 4, 3, 2, 1, 0]);
    let values = i32x8([10, 20, 30, 40, 50, 60, 70, 80]);
    assert_eq!(
        i32_lanes(unsafe { vpermd(values, indices) }),
        [80, 70, 60, 50, 40, 30, 20, 10]
    );

    assert_eq!(i32_lanes(unsafe { vpbroadcastd(7) }), [7; 8]);
}
