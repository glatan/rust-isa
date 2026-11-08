use rust_isa::x86::simd::Mask16;
use rust_isa::x86::simd::avx512f::*;

use super::support::*;

#[test]
fn avx512f_data_movement() {
    if !f_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let source = f32x16(core::array::from_fn(|lane| lane as f32));
    let compressed = f32_lanes(unsafe { vcompressps(source, Mask16::from_bits(0b0101)) });
    assert_eq!(compressed[..4], [0.0, 2.0, 0.0, 0.0]);
    let expanded = f32_lanes(unsafe { vexpandps(source, Mask16::from_bits(0b0101)) });
    assert_eq!(expanded[..4], [0.0, 0.0, 1.0, 0.0]);

    let other = f32x16([100.0; 16]);
    let blended = f32_lanes(unsafe { vblendmps(source, other, Mask16::from_bits(0x00f0)) });
    assert_eq!(blended[3], 3.0);
    assert_eq!(blended[4], 100.0);

    let moved = f32_lanes(unsafe { vmovaps(other, source, Mask16::from_bits(0x0003)) });
    assert_eq!(moved[0], 0.0);
    assert_eq!(moved[1], 1.0);
    assert_eq!(moved[2], 100.0);
}
