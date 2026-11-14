use rust_isa::x86::simd::sse2::V128;
use rust_isa::x86::simd::xop::{vpcomb, vpcomub, vphaddbw};

fn xop_available() -> bool {
    if !is_x86_feature_detected!("avx") {
        return false;
    }
    let leaf = core::arch::x86_64::__cpuid(0x8000_0001);
    leaf.ecx & (1 << 11) != 0
}

fn i16_lanes(value: V128) -> [i16; 8] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<2>().0.iter();
    core::array::from_fn(|_| i16::from_le_bytes(*lanes.next().unwrap()))
}

#[test]
fn xop_values() {
    if !xop_available() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }

    let a = V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    let b = V128::from_bytes([1, 0, 3, 0, 5, 0, 7, 0, 9, 0, 11, 0, 13, 0, 15, 0]);
    assert_eq!(
        unsafe { vpcomb(a, b, 4) }.to_bytes(),
        [
            0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0,
        ]
    );
    assert_eq!(
        unsafe { vpcomb(V128::from_bytes([1; 16]), V128::from_bytes([2; 16]), 0) }.to_bytes(),
        [0xff; 16]
    );
    assert_eq!(
        unsafe { vpcomub(V128::from_bytes([1; 16]), V128::from_bytes([2; 16]), 0) }.to_bytes(),
        [0xff; 16]
    );
    assert_eq!(
        unsafe { vpcomub(V128::from_bytes([0xff; 16]), V128::from_bytes([1; 16]), 0) }.to_bytes(),
        [0; 16]
    );

    let input = V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    assert_eq!(
        i16_lanes(unsafe { vphaddbw(input) }),
        [3, 7, 11, 15, 19, 23, 27, 31]
    );
}
