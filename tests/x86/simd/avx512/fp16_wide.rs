use rust_isa::x86::simd::avx512fp16::{vcmpph, vcvtph2pd, vmaxph, vreduceph, vsqrtph};
use rust_isa::x86::simd::sse2::V128;

use super::support::*;

#[test]
fn fp16_new_operations() {
    if !fp16_enabled() {
        return;
    }
    let four = f16x32(0x4400);
    assert_eq!(
        u16::from_le_bytes(unsafe { vsqrtph(four) }.to_bytes()[..2].try_into().unwrap()),
        0x4000
    );
    assert_eq!(
        u16::from_le_bytes(
            unsafe { vmaxph(f16x32(0x3c00), f16x32(0x4000)) }.to_bytes()[..2]
                .try_into()
                .unwrap()
        ),
        0x4000
    );
    assert_eq!(
        unsafe { vcmpph::<0>(f16x32(0x3c00), f16x32(0x3c00)) },
        u32::MAX
    );
    let doubles = unsafe { vcvtph2pd(V128::from_bytes(f16x8(0x3c00))) };
    assert_eq!(f64_lanes(doubles)[0], 1.0);
    // imm 0 removes the integer part with round-to-nearest-even.
    let reduced = f16x32_pattern([
        0x3e00, 0x4200, 0x3c00, 0xbe00, 0xc200, 0x3f00, 0xbf00, 0x4000,
    ]);
    let expected = f16x32_pattern([
        0xb800, 0x0000, 0x0000, 0x3800, 0x0000, 0xb400, 0x3400, 0x0000,
    ]);
    assert_eq!(
        unsafe { vreduceph::<0>(reduced) }.to_bytes(),
        expected.to_bytes()
    );
}
