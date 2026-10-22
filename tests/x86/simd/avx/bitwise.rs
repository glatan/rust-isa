use rust_isa::x86::simd::avx::{vandps, vmovmskps, vorps, vxorps, vzeroupper};
use rust_isa::x86::simd::avx2::V256;

#[test]
fn bitwise_and_mask() {
    if !is_x86_feature_detected!("avx") {
        return;
    }
    let a = V256::from_bytes([0b1010_1010; 32]);
    let b = V256::from_bytes([0b1100_1100; 32]);
    assert_eq!(unsafe { vandps(a, b) }.to_bytes(), [0b1000_1000; 32]);
    assert_eq!(unsafe { vorps(a, b) }.to_bytes(), [0b1110_1110; 32]);
    assert_eq!(unsafe { vxorps(a, b) }.to_bytes(), [0b0110_0110; 32]);

    let mut bytes = [0u8; 32];
    for lane in [0usize, 3, 7] {
        bytes[lane * 4 + 3] = 0x80;
    }
    assert_eq!(unsafe { vmovmskps(V256::from_bytes(bytes)) }, 0b1000_1001);
    unsafe { vzeroupper() };
}
