use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::avx512fp16::{
    vaddph, vcmpph, vcvtph2pd, vcvtph2psx, vcvtps2phx, vmaxph, vreduceph, vsqrtph,
};
use rust_isa::x86::simd::sse2::V128;

use super::support::*;

#[test]
fn avx512fp16_arithmetic() {
    if !is_x86_feature_detected!("avx512fp16") {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut one = [0u8; 64];
    let mut two = [0u8; 64];
    for lane in 0..32 {
        one[lane * 2..lane * 2 + 2].copy_from_slice(&0x3c00u16.to_le_bytes());
        two[lane * 2..lane * 2 + 2].copy_from_slice(&0x4000u16.to_le_bytes());
    }
    let out = unsafe { vaddph(V512::from_bytes(one), V512::from_bytes(two)) }.to_bytes();
    assert_eq!(u16::from_le_bytes(out[0..2].try_into().unwrap()), 0x4200);

    let mut half = [0u8; 32];
    for lane in 0..16 {
        half[lane * 2..lane * 2 + 2].copy_from_slice(&0x3c00u16.to_le_bytes());
    }
    let out = unsafe { vcvtph2psx(V256::from_bytes(half)) }.to_bytes();
    assert_eq!(f32::from_le_bytes(out[0..4].try_into().unwrap()), 1.0);

    let mut floats = [0u8; 64];
    for lane in 0..16 {
        floats[lane * 4..lane * 4 + 4].copy_from_slice(&1.0f32.to_le_bytes());
    }
    let compressed = unsafe { vcvtps2phx(V512::from_bytes(floats)) }.to_bytes();
    assert_eq!(
        u16::from_le_bytes(compressed[0..2].try_into().unwrap()),
        0x3c00
    );
}

#[test]
fn fp16_extended_operations() {
    if !fp16_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
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
        unsafe { vcmpph::<0>(f16x32(0x3c00), f16x32(0x3c00)).bits() },
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
