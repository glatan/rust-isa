use rust_isa::x86::simd::avx512bf16::{vcvtneps2bf16, vdpbf16ps};
use rust_isa::x86::simd::avx512f::V512;

#[test]
fn avx512bf16_convert_and_dot() {
    if !is_x86_feature_detected!("avx512bf16") {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut floats = [0u8; 64];
    for chunk in floats.as_chunks_mut::<4>().0.iter_mut() {
        *chunk = 1.0f32.to_le_bytes();
    }
    let out = unsafe { vcvtneps2bf16(V512::from_bytes(floats)) }.to_bytes();
    assert_eq!(u16::from_le_bytes(out[0..2].try_into().unwrap()), 0x3f80);

    let mut a = [0u8; 64];
    let mut b = [0u8; 64];
    for (a_chunk, b_chunk) in a
        .as_chunks_mut::<2>()
        .0
        .iter_mut()
        .zip(b.as_chunks_mut::<2>().0.iter_mut())
    {
        a_chunk.copy_from_slice(&0x3f80u16.to_le_bytes());
        b_chunk.copy_from_slice(&0x4000u16.to_le_bytes());
    }
    let out = unsafe {
        vdpbf16ps(
            V512::from_bytes([0; 64]),
            V512::from_bytes(a),
            V512::from_bytes(b),
        )
    };
    let bytes = out.to_bytes();
    for lane in 0..16 {
        assert_eq!(
            f32::from_le_bytes(*bytes[lane * 4..].first_chunk().unwrap()),
            4.0
        );
    }
}
