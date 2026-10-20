use rust_isa::x86::simd::avx2::{
    V256, v256_load, v256_store, vpaddb, vpaddd, vpand, vpbroadcastd, vpcmpeqb, vpermd, vpmulld,
    vpshufb,
};

fn enabled() -> bool {
    is_x86_feature_detected!("avx2")
}

fn i32x8(values: [i32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (index, value) in values.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i32_lanes(value: V256) -> [i32; 8] {
    let bytes = value.to_bytes();
    let mut lanes = [0i32; 8];
    for (index, lane) in lanes.iter_mut().enumerate() {
        *lane = i32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap());
    }
    lanes
}

#[test]
fn load_store_roundtrip() {
    if !enabled() {
        return;
    }
    let source = core::array::from_fn::<u8, 32, _>(|index| index as u8);
    let value = unsafe { v256_load(source.as_ptr()) };
    assert_eq!(value.to_bytes(), source);

    let mut output = [0u8; 32];
    unsafe {
        v256_store(output.as_mut_ptr(), value);
    }
    assert_eq!(output, source);
}

#[test]
fn integer_arithmetic() {
    if !enabled() {
        return;
    }
    let a = i32x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = i32x8([10, 20, 30, 40, 50, 60, 70, 80]);
    assert_eq!(
        i32_lanes(unsafe { vpaddd(a, b) }),
        [11, 22, 33, 44, 55, 66, 77, 88]
    );
    assert_eq!(
        i32_lanes(unsafe { vpmulld(a, b) }),
        [10, 40, 90, 160, 250, 360, 490, 640]
    );

    let bytes = V256::from_bytes([1; 32]);
    assert_eq!(unsafe { vpaddb(bytes, bytes) }.to_bytes(), [2; 32]);
}

#[test]
fn bitwise_and_compare() {
    if !enabled() {
        return;
    }
    let a = V256::from_bytes([0b1010_1010; 32]);
    let b = V256::from_bytes([0b1100_1100; 32]);
    assert_eq!(unsafe { vpand(a, b) }.to_bytes(), [0b1000_1000; 32]);
    assert_eq!(unsafe { vpcmpeqb(a, a) }.to_bytes(), [0xff; 32]);
    assert_eq!(unsafe { vpcmpeqb(a, b) }.to_bytes(), [0; 32]);
}

#[test]
fn shuffle_and_broadcast() {
    if !enabled() {
        return;
    }
    let source = V256::from_bytes(core::array::from_fn::<u8, 32, _>(|index| index as u8));
    let identity = V256::from_bytes(core::array::from_fn::<u8, 32, _>(|index| index as u8));
    assert_eq!(
        unsafe { vpshufb(source, identity) }.to_bytes(),
        source.to_bytes()
    );

    let indices = i32x8([7, 6, 5, 4, 3, 2, 1, 0]);
    let values = i32x8([10, 20, 30, 40, 50, 60, 70, 80]);
    assert_eq!(
        i32_lanes(unsafe { vpermd(values, indices) }),
        [80, 70, 60, 50, 40, 30, 20, 10]
    );

    assert_eq!(i32_lanes(unsafe { vpbroadcastd(7) }), [7; 8]);
}
