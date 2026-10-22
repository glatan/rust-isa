use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::sse2::V128;

pub(super) fn enabled() -> bool {
    is_x86_feature_detected!("avx2")
}

pub(super) fn i32x8(values: [i32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (index, value) in values.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

pub(super) fn i32_lanes(value: V256) -> [i32; 8] {
    let bytes = value.to_bytes();
    let mut lanes = [0i32; 8];
    for (index, lane) in lanes.iter_mut().enumerate() {
        *lane = i32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap());
    }
    lanes
}

pub(super) fn i16x16(values: [i16; 16]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

pub(super) fn i16_lanes(value: V256) -> [i16; 16] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

pub(super) fn i64x4(values: [i64; 4]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

pub(super) fn i64_lanes(value: V256) -> [i64; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

pub(super) fn u8x32(values: [u8; 32]) -> V256 {
    V256::from_bytes(values)
}

pub(super) fn i8x32(values: [i8; 32]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane] = *value as u8;
    }
    V256::from_bytes(bytes)
}

pub(super) fn i8_lanes(value: V256) -> [i8; 32] {
    value.to_bytes().map(|byte| byte as i8)
}

pub(super) fn u16x16(values: [u16; 16]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

pub(super) fn u16_lanes(value: V256) -> [u16; 16] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

pub(super) fn u32x8(values: [u32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

pub(super) fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

pub(super) fn i32x4_lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

pub(super) fn i64x2(values: [i64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

pub(super) fn i64x2_lanes(value: V128) -> [i64; 2] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}
