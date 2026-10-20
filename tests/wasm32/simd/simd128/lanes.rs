use rust_isa::wasm32::simd::simd128::V128;

pub fn i8x16(values: [i8; 16]) -> V128 {
    V128::from_bytes(values.map(|value| value as u8))
}

pub fn i8_lanes(value: V128) -> [i8; 16] {
    value.to_bytes().map(|byte| byte as i8)
}

pub fn i16x8(values: [i16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

pub fn i16_lanes(value: V128) -> [i16; 8] {
    let bytes = value.to_bytes();
    [0, 1, 2, 3, 4, 5, 6, 7]
        .map(|lane| i16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap()))
}

pub fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

pub fn i32_lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    [0, 1, 2, 3].map(|lane| i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap()))
}

pub fn i64x2(values: [i64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&values[0].to_le_bytes());
    bytes[8..].copy_from_slice(&values[1].to_le_bytes());
    V128::from_bytes(bytes)
}

pub fn i64_lanes(value: V128) -> [i64; 2] {
    let bytes = value.to_bytes();
    [
        i64::from_le_bytes(bytes[..8].try_into().unwrap()),
        i64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ]
}

pub fn f32x4(values: [f32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

pub fn f32_lanes(value: V128) -> [f32; 4] {
    let bytes = value.to_bytes();
    [0, 1, 2, 3].map(|lane| f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap()))
}

pub fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&values[0].to_le_bytes());
    bytes[8..].copy_from_slice(&values[1].to_le_bytes());
    V128::from_bytes(bytes)
}

pub fn f64_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    [
        f64::from_le_bytes(bytes[..8].try_into().unwrap()),
        f64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ]
}

pub fn true_lane(value: V128, lane: usize, width: usize) -> bool {
    let bytes = value.to_bytes();
    let chunk = &bytes[lane * width..(lane + 1) * width];
    assert!(
        chunk.iter().all(|byte| *byte == chunk[0]) && (chunk[0] == 0 || chunk[0] == 0xff),
        "lane {lane} is not a compare mask: {chunk:?}"
    );
    chunk[0] == 0xff
}
