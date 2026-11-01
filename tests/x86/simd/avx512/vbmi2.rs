use rust_isa::x86::simd::Mask64;
use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::avx512vbmi2::{vpcompressb, vpexpandb};

#[test]
fn avx512vbmi2_compress_and_expand() {
    if !is_x86_feature_detected!("avx512vbmi2") {
        return;
    }
    let source = V512::from_bytes(core::array::from_fn(|index| index as u8));
    let compressed = unsafe { vpcompressb(source, Mask64::from_bits(0b0101)) }.to_bytes();
    assert_eq!(&compressed[..4], &[0, 2, 0, 0]);
    assert!(compressed[4..].iter().all(|value| *value == 0));

    let source = V512::from_bytes(core::array::from_fn(|index| index as u8 + 10));
    let expanded = unsafe { vpexpandb(source, Mask64::from_bits(0b0101)) }.to_bytes();
    assert_eq!(&expanded[..4], &[10, 0, 11, 0]);
    assert!(expanded[4..].iter().all(|value| *value == 0));
}
