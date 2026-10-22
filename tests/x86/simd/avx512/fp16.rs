use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::avx512fp16::{vaddph, vcvtph2psx, vcvtps2phx};

#[test]
fn avx512fp16_arithmetic() {
    if !is_x86_feature_detected!("avx512fp16") {
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
