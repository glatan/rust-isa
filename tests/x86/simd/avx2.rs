use rust_isa::x86::simd::avx2::{
    V256, v256_load, v256_store, vpaddb, vpaddd, vpaddq, vpaddw, vpand, vpbroadcastd, vpcmpeqb,
    vpcmpeqd, vpcmpeqw, vpermd, vpmulld, vpmullw, vpor, vpshufb, vpslld, vpsrld, vpsubb, vpsubd,
    vpsubq, vpsubw, vpxor,
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

fn i16x16(values: [i16; 16]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i16_lanes(value: V256) -> [i16; 16] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

fn i64x4(values: [i64; 4]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i64_lanes(value: V256) -> [i64; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

#[test]
fn byte_and_word_arithmetic() {
    if !enabled() {
        return;
    }
    assert_eq!(
        unsafe { vpsubb(V256::from_bytes([10; 32]), V256::from_bytes([3; 32])) }.to_bytes(),
        [7; 32]
    );
    let a = i16x16(core::array::from_fn(|index| index as i16 + 1));
    let b = i16x16(core::array::from_fn(|index| (index as i16 + 1) * 10));
    assert_eq!(
        i16_lanes(unsafe { vpaddw(a, b) }),
        core::array::from_fn(|index| (index as i16 + 1) * 11)
    );
    assert_eq!(
        i16_lanes(unsafe { vpsubw(b, a) }),
        core::array::from_fn(|index| (index as i16 + 1) * 9)
    );
    assert_eq!(
        i16_lanes(unsafe { vpmullw(i16x16([300; 16]), i16x16([3; 16])) }),
        [900; 16]
    );
}

#[test]
fn dword_and_qword_arithmetic() {
    if !enabled() {
        return;
    }
    let a = i32x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = i32x8([10, 20, 30, 40, 50, 60, 70, 80]);
    assert_eq!(
        i32_lanes(unsafe { vpsubd(b, a) }),
        [9, 18, 27, 36, 45, 54, 63, 72]
    );
    assert_eq!(
        i64_lanes(unsafe { vpaddq(i64x4([1, i64::MAX, -1, 0]), i64x4([2, 1, 1, 5])) }),
        [3, i64::MIN, 0, 5]
    );
    assert_eq!(
        i64_lanes(unsafe { vpsubq(i64x4([0, 5, -1, 7]), i64x4([1, 2, 1, 3])) }),
        [-1, 3, -2, 4]
    );
}

#[test]
fn compare_and_shift() {
    if !enabled() {
        return;
    }
    assert_eq!(
        unsafe { vpcmpeqw(i16x16([5; 16]), i16x16([5; 16])) }.to_bytes(),
        [0xff; 32]
    );
    assert_eq!(
        unsafe { vpcmpeqd(i32x8([5; 8]), i32x8([6; 8])) }.to_bytes(),
        [0; 32]
    );
    assert_eq!(
        i32_lanes(unsafe { vpslld(i32x8([1, 2, 3, 4, 5, 6, 7, 8]), 1) }),
        [2, 4, 6, 8, 10, 12, 14, 16]
    );
    assert_eq!(
        i32_lanes(unsafe { vpsrld(i32x8([-1; 8]), 1) }),
        [0x7fff_ffff; 8]
    );
}

#[test]
fn bitwise_or_and_xor() {
    if !enabled() {
        return;
    }
    let a = V256::from_bytes([0b1010_1010; 32]);
    let b = V256::from_bytes([0b1100_1100; 32]);
    assert_eq!(unsafe { vpor(a, b) }.to_bytes(), [0b1110_1110; 32]);
    assert_eq!(unsafe { vpxor(a, b) }.to_bytes(), [0b0110_0110; 32]);
}
