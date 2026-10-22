use rust_isa::x86::simd::avx512bitalg::{vpopcntb, vpshufbitqmb};
use rust_isa::x86::simd::avx512f::V512;

#[test]
fn avx512bitalg() {
    if !is_x86_feature_detected!("avx512bitalg") {
        return;
    }
    let all = V512::from_bytes([0xff; 64]);
    let out = unsafe { vpopcntb(all) }.to_bytes();
    assert_eq!(&out[..4], &[8, 8, 8, 8]);

    let all = V512::from_bytes([0xff; 64]);
    assert_eq!(unsafe { vpshufbitqmb(all, all) }, u64::MAX);
}
