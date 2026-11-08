use rust_isa::x86::simd::avx512f::*;

use super::support::*;

#[test]
fn avx512f_compare_and_test() {
    if !f_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let lhs = i32x16(core::array::from_fn(|lane| lane as i32));
    let rhs = i32x16(core::array::from_fn(|lane| {
        if lane < 4 { lane as i32 } else { 100 }
    }));
    assert_eq!(unsafe { vpcmpd::<0>(lhs, rhs).bits() }, 0x000f);
    assert_eq!(unsafe { vpcmpud::<1>(lhs, rhs).bits() }, 0xfff0);
    assert_eq!(
        unsafe { vpcmpq::<4>(i64x8([1; 8]), i64x8([2; 8])).bits() },
        0xff
    );

    let bits = i32x16(core::array::from_fn(
        |lane| {
            if lane < 3 { 1 << lane } else { 0 }
        },
    ));
    assert_eq!(unsafe { vptestmd(bits, bits).bits() }, 0x0007);
    assert_eq!(unsafe { vptestnmd(bits, i32x16([0; 16])).bits() }, 0xffff);
}
