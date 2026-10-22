use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::sse2::V128;

pub(crate) fn f_enabled() -> bool {
    is_x86_feature_detected!("avx512f")
}

pub(crate) fn bw_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512bw")
}

pub(crate) fn dq_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512dq")
}

pub(crate) fn ifma_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512ifma")
}

pub(crate) fn vbmi_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512vbmi")
}

pub(crate) fn vnni_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512vnni")
}

pub(crate) fn vl_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512dq") && is_x86_feature_detected!("avx512vl")
}

pub(crate) fn i32x16(values: [i32; 16]) -> V512 {
    let mut bytes = [0u8; 64];
    for (index, value) in values.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

pub(crate) fn i32_lanes(value: V512) -> [i32; 16] {
    let bytes = value.to_bytes();
    let mut lanes = [0i32; 16];
    for (index, lane) in lanes.iter_mut().enumerate() {
        *lane = i32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap());
    }
    lanes
}

pub(crate) fn i64x8(values: [i64; 8]) -> V512 {
    let mut bytes = [0u8; 64];
    for (index, value) in values.iter().enumerate() {
        bytes[index * 8..index * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

pub(crate) fn i64_lanes(value: V512) -> [i64; 8] {
    let bytes = value.to_bytes();
    let mut lanes = [0i64; 8];
    for (index, lane) in lanes.iter_mut().enumerate() {
        *lane = i64::from_le_bytes(bytes[index * 8..index * 8 + 8].try_into().unwrap());
    }
    lanes
}

pub(crate) fn i16x32(values: [i16; 32]) -> V512 {
    let mut bytes = [0u8; 64];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

pub(crate) fn i16_lanes(value: V512) -> [i16; 32] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

pub(crate) fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

pub(crate) fn i32x4_lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

pub(crate) fn i16x8(values: [i16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

pub(crate) fn u16x8(values: [u16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

pub(crate) fn i64x2(values: [i64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&values[0].to_le_bytes());
    bytes[8..].copy_from_slice(&values[1].to_le_bytes());
    V128::from_bytes(bytes)
}

pub(crate) fn i64x2_lanes(value: V128) -> [i64; 2] {
    let bytes = value.to_bytes();
    [
        i64::from_le_bytes(bytes[..8].try_into().unwrap()),
        i64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ]
}

pub(crate) fn u16x16(values: [u16; 16]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

pub(crate) fn i32x8(values: [i32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

pub(crate) fn i32x8_lanes(value: V256) -> [i32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

pub(crate) fn f32x8_lanes(value: V256) -> [f32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

pub(crate) fn i32x8_as_f32(values: [f32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

pub(crate) fn vbmi2_enabled() -> bool {
    vl_enabled() && is_x86_feature_detected!("avx512vbmi2")
}

pub(crate) fn bitalg_enabled() -> bool {
    vl_enabled() && is_x86_feature_detected!("avx512bitalg")
}

pub(crate) fn fp16_enabled() -> bool {
    is_x86_feature_detected!("avx512fp16")
}

pub(crate) fn i8x64(values: [i8; 64]) -> V512 {
    V512::from_bytes(core::array::from_fn(|index| values[index] as u8))
}

pub(crate) fn i8_lanes(value: V512) -> [i8; 64] {
    value.to_bytes().map(|byte| byte as i8)
}

pub(crate) fn f32x16(values: [f32; 16]) -> V512 {
    let mut bytes = [0u8; 64];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

pub(crate) fn f32_lanes(value: V512) -> [f32; 16] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

pub(crate) fn f64x8(values: [f64; 8]) -> V512 {
    let mut bytes = [0u8; 64];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

pub(crate) fn f64_lanes(value: V512) -> [f64; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

pub(crate) fn i8x16(values: [i8; 16]) -> V128 {
    V128::from_bytes(core::array::from_fn(|index| values[index] as u8))
}

pub(crate) fn f32x4_lanes(value: V128) -> [f32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

pub(crate) fn f32x8(values: [f32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

pub(crate) fn v256_from_f32x16_prefix(value: V512) -> V256 {
    V256::from_bytes(value.to_bytes()[..32].try_into().unwrap())
}

pub(crate) fn f16x32(value: u16) -> V512 {
    let mut bytes = [0u8; 64];
    for lane in 0..32 {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

pub(crate) fn f16x32_pattern(values: [u16; 8]) -> V512 {
    let mut bytes = [0u8; 64];
    for lane in 0..32 {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&values[lane % 8].to_le_bytes());
    }
    V512::from_bytes(bytes)
}

pub(crate) fn f16x8(value: u16) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    for lane in 0..8 {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    bytes
}
