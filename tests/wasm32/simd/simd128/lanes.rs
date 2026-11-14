use rust_isa::wasm32::simd::simd128::V128;

pub fn i8x16(values: [i8; 16]) -> V128 {
    V128::from_bytes(values.map(|value| value as u8))
}

pub fn i8_lanes(value: V128) -> [i8; 16] {
    value.to_bytes().map(|byte| byte as i8)
}

pub fn i16x8(values: [i16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<2>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub fn i16_lanes(value: V128) -> [i16; 8] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<2>().0.iter();
    core::array::from_fn(|_| i16::from_le_bytes(*lanes.next().unwrap()))
}

pub fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub fn i32_lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<4>().0.iter();
    core::array::from_fn(|_| i32::from_le_bytes(*lanes.next().unwrap()))
}

pub fn i64x2(values: [i64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<8>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub fn i64_lanes(value: V128) -> [i64; 2] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<8>().0.iter();
    core::array::from_fn(|_| i64::from_le_bytes(*lanes.next().unwrap()))
}

pub fn f32x4(values: [f32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub fn f32_lanes(value: V128) -> [f32; 4] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<4>().0.iter();
    core::array::from_fn(|_| f32::from_le_bytes(*lanes.next().unwrap()))
}

pub fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<8>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub fn f64_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<8>().0.iter();
    core::array::from_fn(|_| f64::from_le_bytes(*lanes.next().unwrap()))
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
