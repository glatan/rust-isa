use rust_isa::x86::simd::avx2::{
    V256, vpackssdw, vpacksswb, vpackusdw, vpackuswb, vpaddsb, vpaddsw, vpaddusb, vpaddusw,
    vphaddd, vphaddsw, vphaddw, vphsubd, vphsubsw, vphsubw, vpmaddubsw, vpmaddwd, vpmaxsb, vpmaxsd,
    vpmaxsw, vpmaxub, vpmaxud, vpminsb, vpminsd, vpminsw, vpminub, vpminud, vpmovmskb, vpmulhrsw,
    vpmulhuw, vpmulhw, vpsadbw, vpsignb, vpsignd, vpsignw, vpsubsb, vpsubsw, vpsubusb, vpsubusw,
    vpunpckhbw, vpunpckhdq, vpunpckhqdq, vpunpckhwd, vpunpcklbw, vpunpckldq, vpunpcklqdq,
    vpunpcklwd,
};

fn enabled() -> bool {
    is_x86_feature_detected!("avx2")
}

fn u8x32(values: [u8; 32]) -> V256 {
    V256::from_bytes(values)
}

fn i8x32(values: [i8; 32]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane] = *value as u8;
    }
    V256::from_bytes(bytes)
}

fn i8_lanes(value: V256) -> [i8; 32] {
    value.to_bytes().map(|byte| byte as i8)
}

fn u16x16(values: [u16; 16]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
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

fn u16_lanes(value: V256) -> [u16; 16] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

fn u32x8(values: [u32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i32x8(values: [i32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i32_lanes(value: V256) -> [i32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
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
fn saturating_arithmetic() {
    if !enabled() {
        return;
    }
    assert_eq!(
        unsafe { vpaddsb(i8x32([100; 32]), i8x32([100; 32])) }.to_bytes(),
        [127; 32]
    );
    assert_eq!(
        unsafe { vpsubsb(i8x32([-100; 32]), i8x32([100; 32])) }.to_bytes(),
        [0x80; 32]
    );
    assert_eq!(
        unsafe { vpaddusb(u8x32([200; 32]), u8x32([100; 32])) }.to_bytes(),
        [255; 32]
    );
    assert_eq!(
        unsafe { vpsubusb(u8x32([100; 32]), u8x32([200; 32])) }.to_bytes(),
        [0; 32]
    );

    assert_eq!(
        i16_lanes(unsafe { vpaddsw(i16x16([30_000; 16]), i16x16([30_000; 16])) }),
        [32_767; 16]
    );
    assert_eq!(
        i16_lanes(unsafe { vpsubsw(i16x16([-30_000; 16]), i16x16([30_000; 16])) }),
        [-32_768; 16]
    );
    assert_eq!(
        u16_lanes(unsafe { vpaddusw(u16x16([60_000; 16]), u16x16([10_000; 16])) }),
        [65_535; 16]
    );
    assert_eq!(
        u16_lanes(unsafe { vpsubusw(u16x16([10_000; 16]), u16x16([60_000; 16])) }),
        [0; 16]
    );
}

#[test]
fn min_and_max() {
    if !enabled() {
        return;
    }
    assert_eq!(
        unsafe { vpminub(u8x32([10; 32]), u8x32([20; 32])) }.to_bytes(),
        [10; 32]
    );
    assert_eq!(
        unsafe { vpmaxub(u8x32([10; 32]), u8x32([20; 32])) }.to_bytes(),
        [20; 32]
    );
    assert_eq!(
        unsafe { vpminsb(i8x32([-10; 32]), i8x32([10; 32])) }.to_bytes(),
        [0xf6; 32]
    );
    assert_eq!(
        unsafe { vpmaxsb(i8x32([-10; 32]), i8x32([10; 32])) }.to_bytes(),
        [10; 32]
    );
    assert_eq!(
        i16_lanes(unsafe { vpminsw(i16x16([-1_000; 16]), i16x16([1_000; 16])) }),
        [-1_000; 16]
    );
    assert_eq!(
        i16_lanes(unsafe { vpmaxsw(i16x16([-1_000; 16]), i16x16([1_000; 16])) }),
        [1_000; 16]
    );
    assert_eq!(
        i32_lanes(unsafe { vpminud(u32x8([u32::MAX; 8]), u32x8([1; 8])) }),
        [1; 8]
    );
    assert_eq!(
        unsafe { vpmaxud(u32x8([u32::MAX; 8]), u32x8([1; 8])) }.to_bytes(),
        [0xff; 32]
    );
    assert_eq!(
        i32_lanes(unsafe { vpminsd(i32x8([-1; 8]), i32x8([1; 8])) }),
        [-1; 8]
    );
    assert_eq!(
        i32_lanes(unsafe { vpmaxsd(i32x8([-1; 8]), i32x8([1; 8])) }),
        [1; 8]
    );
}

#[test]
fn multiply_and_sum_of_differences() {
    if !enabled() {
        return;
    }
    assert_eq!(
        i16_lanes(unsafe { vpmulhw(i16x16([1_000; 16]), i16x16([1_000; 16])) }),
        [15; 16]
    );
    assert_eq!(
        u16_lanes(unsafe { vpmulhuw(u16x16([1_000; 16]), u16x16([1_000; 16])) }),
        [15; 16]
    );
    assert_eq!(
        i16_lanes(unsafe { vpmulhrsw(i16x16([16_384; 16]), i16x16([16_384; 16])) }),
        [8_192; 16]
    );

    let words = i16x16(core::array::from_fn(|lane| lane as i16 + 1));
    assert_eq!(
        i32_lanes(unsafe { vpmaddwd(words, i16x16([1; 16])) }),
        [3, 7, 11, 15, 19, 23, 27, 31]
    );
    assert_eq!(
        i16_lanes(unsafe { vpmaddubsw(u8x32([1; 32]), i8x32([1; 32])) }),
        [2; 16]
    );
    assert_eq!(
        i64_lanes(unsafe { vpsadbw(u8x32([1; 32]), u8x32([0; 32])) }),
        [8; 4]
    );
}

#[test]
fn pack_and_unpack() {
    if !enabled() {
        return;
    }
    let a = i16x16(core::array::from_fn(|lane| lane as i16 + 1));
    let b = i16x16(core::array::from_fn(|lane| lane as i16 + 17));
    assert_eq!(
        unsafe { vpacksswb(a, b) }.to_bytes(),
        [
            1, 2, 3, 4, 5, 6, 7, 8, 17, 18, 19, 20, 21, 22, 23, 24, 9, 10, 11, 12, 13, 14, 15, 16,
            25, 26, 27, 28, 29, 30, 31, 32
        ]
    );
    assert_eq!(
        unsafe { vpackuswb(a, b) }.to_bytes(),
        [
            1, 2, 3, 4, 5, 6, 7, 8, 17, 18, 19, 20, 21, 22, 23, 24, 9, 10, 11, 12, 13, 14, 15, 16,
            25, 26, 27, 28, 29, 30, 31, 32
        ]
    );

    let a = i32x8(core::array::from_fn(|lane| lane as i32 + 1));
    let b = i32x8(core::array::from_fn(|lane| lane as i32 + 9));
    assert_eq!(
        i16_lanes(unsafe { vpackssdw(a, b) }),
        [1, 2, 3, 4, 9, 10, 11, 12, 5, 6, 7, 8, 13, 14, 15, 16]
    );
    assert_eq!(
        i16_lanes(unsafe { vpackusdw(a, b) }),
        [1, 2, 3, 4, 9, 10, 11, 12, 5, 6, 7, 8, 13, 14, 15, 16]
    );

    let a = i16x16([-200, 200, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    let saturated = unsafe { vpacksswb(a, a) }.to_bytes();
    assert_eq!(saturated[0], 0x80);
    assert_eq!(saturated[1], 0x7f);
    let unsigned = unsafe {
        vpackuswb(
            i16x16([-1, 256, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]),
            a,
        )
    }
    .to_bytes();
    assert_eq!(unsigned[0], 0);
    assert_eq!(unsigned[1], 0xff);
    let saturated =
        unsafe { vpackssdw(i32x8([i32::MIN, i32::MAX, 0, 1, 2, 3, 4, 5]), i32x8([0; 8])) }
            .to_bytes();
    assert_eq!(&saturated[0..4], &[0x00, 0x80, 0xff, 0x7f]);
    let unsigned =
        unsafe { vpackusdw(i32x8([-1, 65_536, 0, 1, 2, 3, 4, 5]), i32x8([0; 8])) }.to_bytes();
    assert_eq!(&unsigned[0..4], &[0x00, 0x00, 0xff, 0xff]);

    let a = V256::from_bytes(core::array::from_fn(|index| index as u8));
    let b = V256::from_bytes(core::array::from_fn(|index| index as u8 + 100));
    assert_eq!(
        unsafe { vpunpcklbw(a, b) }.to_bytes(),
        [
            0, 100, 1, 101, 2, 102, 3, 103, 4, 104, 5, 105, 6, 106, 7, 107, 16, 116, 17, 117, 18,
            118, 19, 119, 20, 120, 21, 121, 22, 122, 23, 123
        ]
    );
    assert_eq!(
        unsafe { vpunpckhbw(a, b) }.to_bytes(),
        [
            8, 108, 9, 109, 10, 110, 11, 111, 12, 112, 13, 113, 14, 114, 15, 115, 24, 124, 25, 125,
            26, 126, 27, 127, 28, 128, 29, 129, 30, 130, 31, 131
        ]
    );

    let a = i16x16(core::array::from_fn(|lane| lane as i16));
    let b = i16x16(core::array::from_fn(|lane| lane as i16 + 100));
    assert_eq!(
        i16_lanes(unsafe { vpunpcklwd(a, b) }),
        [
            0, 100, 1, 101, 2, 102, 3, 103, 8, 108, 9, 109, 10, 110, 11, 111
        ]
    );
    assert_eq!(
        i16_lanes(unsafe { vpunpckhwd(a, b) }),
        [
            4, 104, 5, 105, 6, 106, 7, 107, 12, 112, 13, 113, 14, 114, 15, 115
        ]
    );

    let a = i32x8(core::array::from_fn(|lane| lane as i32));
    let b = i32x8(core::array::from_fn(|lane| lane as i32 + 100));
    assert_eq!(
        i32_lanes(unsafe { vpunpckldq(a, b) }),
        [0, 100, 1, 101, 4, 104, 5, 105]
    );
    assert_eq!(
        i32_lanes(unsafe { vpunpckhdq(a, b) }),
        [2, 102, 3, 103, 6, 106, 7, 107]
    );

    let a = i64x4([0, 1, 2, 3]);
    let b = i64x4([100, 101, 102, 103]);
    assert_eq!(i64_lanes(unsafe { vpunpcklqdq(a, b) }), [0, 100, 2, 102]);
    assert_eq!(i64_lanes(unsafe { vpunpckhqdq(a, b) }), [1, 101, 3, 103]);
}

#[test]
fn movemask_and_horizontal() {
    if !enabled() {
        return;
    }
    let mut bytes = [0u8; 32];
    bytes[0] = 0x80;
    bytes[3] = 0x81;
    bytes[31] = 0xff;
    assert_eq!(unsafe { vpmovmskb(V256::from_bytes(bytes)) }, 0x8000_0009);

    let a = i16x16(core::array::from_fn(|lane| lane as i16 + 1));
    let b = i16x16(core::array::from_fn(|lane| lane as i16 + 17));
    assert_eq!(
        i16_lanes(unsafe { vphaddw(a, b) }),
        [3, 7, 11, 15, 35, 39, 43, 47, 19, 23, 27, 31, 51, 55, 59, 63]
    );
    assert_eq!(
        i16_lanes(unsafe { vphaddsw(i16x16([30_000; 16]), i16x16([30_000; 16])) }),
        [32_767; 16]
    );
    assert_eq!(i16_lanes(unsafe { vphsubw(a, b) }), [-1; 16]);

    let a = i32x8(core::array::from_fn(|lane| lane as i32 + 1));
    let b = i32x8(core::array::from_fn(|lane| lane as i32 + 9));
    assert_eq!(
        i32_lanes(unsafe { vphaddd(a, b) }),
        [3, 7, 19, 23, 11, 15, 27, 31]
    );
    assert_eq!(i32_lanes(unsafe { vphsubd(a, b) }), [-1; 8]);
    let a = i16x16(core::array::from_fn(|lane| {
        if lane % 2 == 0 { 30_000 } else { -30_000 }
    }));
    assert_eq!(i16_lanes(unsafe { vphsubsw(a, a) }), [32_767; 16]);
}

#[test]
fn sign() {
    if !enabled() {
        return;
    }
    let signs = i8x32(core::array::from_fn(|lane| match lane % 3 {
        0 => 1,
        1 => -1,
        _ => 0,
    }));
    let expected = i8x32(core::array::from_fn(|lane| match lane % 3 {
        0 => 5,
        1 => -5,
        _ => 0,
    }));
    assert_eq!(
        i8_lanes(unsafe { vpsignb(i8x32([5; 32]), signs) }),
        i8_lanes(expected)
    );

    let signs = i16x16(core::array::from_fn(|lane| match lane % 3 {
        0 => 1,
        1 => -1,
        _ => 0,
    }));
    assert_eq!(
        i16_lanes(unsafe { vpsignw(i16x16([100; 16]), signs) }),
        [
            100, -100, 0, 100, -100, 0, 100, -100, 0, 100, -100, 0, 100, -100, 0, 100
        ]
    );

    let signs = i32x8(core::array::from_fn(|lane| match lane % 3 {
        0 => 1,
        1 => -1,
        _ => 0,
    }));
    assert_eq!(
        i32_lanes(unsafe { vpsignd(i32x8([100; 8]), signs) }),
        [100, -100, 0, 100, -100, 0, 100, -100]
    );
}
