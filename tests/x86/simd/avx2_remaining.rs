use rust_isa::x86::simd::avx2::{
    V256, vextracti128, vinserti128, vpblendd, vpblendvb, vpblendw, vpbroadcastb, vpbroadcastq,
    vpbroadcastw, vperm2i128, vpermps, vpermq, vpgatherdd, vpgatherdq, vpgatherqd, vpgatherqq,
    vpmaskmovd_load, vpmaskmovd_store, vpmaskmovq_load, vpmaskmovq_store, vpshufd, vpshufhw,
    vpshuflw, vpsllvd, vpsllvq, vpsravd, vpsrlvd, vpsrlvq,
};
use rust_isa::x86::simd::sse2::V128;

fn enabled() -> bool {
    is_x86_feature_detected!("avx2")
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

fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn i32x4_lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn i64x2(values: [i64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn i64x2_lanes(value: V128) -> [i64; 2] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

#[test]
fn variable_shifts() {
    if !enabled() {
        return;
    }
    assert_eq!(
        i32_lanes(unsafe {
            vpsllvd(
                i32x8([1, 2, 3, 4, 5, 6, 7, 8]),
                i32x8([0, 1, 2, 3, 4, 5, 6, 7]),
            )
        }),
        [1, 4, 12, 32, 80, 192, 448, 1024]
    );
    assert_eq!(
        i32_lanes(unsafe { vpsrlvd(i32x8([16; 8]), i32x8([0, 1, 2, 3, 4, 5, 6, 7])) }),
        [16, 8, 4, 2, 1, 0, 0, 0]
    );
    assert_eq!(
        i32_lanes(unsafe { vpsravd(i32x8([-16; 8]), i32x8([0, 1, 2, 3, 4, 5, 6, 7])) }),
        [-16, -8, -4, -2, -1, -1, -1, -1]
    );
    assert_eq!(
        i64_lanes(unsafe { vpsllvq(i64x4([1, 2, 3, 4]), i64x4([1, 2, 3, 4])) }),
        [2, 8, 24, 64]
    );
    assert_eq!(
        i64_lanes(unsafe { vpsrlvq(i64x4([64; 4]), i64x4([0, 1, 2, 3])) }),
        [64, 32, 16, 8]
    );
}

#[test]
fn broadcast_and_blend() {
    if !enabled() {
        return;
    }
    assert_eq!(unsafe { vpbroadcastb(0xab) }.to_bytes(), [0xab; 32]);
    assert_eq!(u16_lanes(unsafe { vpbroadcastw(0x1234) }), [0x1234; 16]);
    assert_eq!(
        i64_lanes(unsafe { vpbroadcastq(0x0123_4567_89ab_cdef) }),
        [0x0123_4567_89ab_cdef; 4]
    );

    let mask = V256::from_bytes(core::array::from_fn(|index| {
        if index % 2 == 0 { 0x80 } else { 0x00 }
    }));
    assert_eq!(
        unsafe { vpblendvb(V256::from_bytes([1; 32]), V256::from_bytes([2; 32]), mask) }.to_bytes(),
        core::array::from_fn(|index| if index % 2 == 0 { 2 } else { 1 })
    );
}

#[test]
fn immediate_forms() {
    if !enabled() {
        return;
    }
    let a = i64x4([10, 20, 30, 40]);
    let b = i64x4([100, 200, 300, 400]);
    assert_eq!(i64_lanes(unsafe { vpermq::<0x1b>(a) }), [40, 30, 20, 10]);
    assert_eq!(
        i64_lanes(unsafe { vperm2i128::<0x21>(a, b) }),
        [30, 40, 100, 200]
    );
    assert_eq!(
        i32_lanes(unsafe {
            vpermps(
                i32x8([1, 2, 3, 4, 5, 6, 7, 8]),
                i32x8([7, 6, 5, 4, 3, 2, 1, 0]),
            )
        }),
        [8, 7, 6, 5, 4, 3, 2, 1]
    );
    assert_eq!(
        i64_lanes(unsafe { vinserti128::<1>(a, i64x2([100, 200])) }),
        [10, 20, 100, 200]
    );
    assert_eq!(i64x2_lanes(unsafe { vextracti128::<1>(a) }), [30, 40]);

    let a = i16x16(core::array::from_fn(|lane| lane as i16 + 1));
    let b = i16x16(core::array::from_fn(|lane| lane as i16 + 101));
    assert_eq!(
        i16_lanes(unsafe { vpblendw::<0x55>(a, b) }),
        [
            101, 2, 103, 4, 105, 6, 107, 8, 109, 10, 111, 12, 113, 14, 115, 16
        ]
    );

    let a = i32x8(core::array::from_fn(|lane| lane as i32 + 1));
    let b = i32x8(core::array::from_fn(|lane| lane as i32 + 101));
    assert_eq!(
        i32_lanes(unsafe { vpblendd::<0x55>(a, b) }),
        [101, 2, 103, 4, 105, 6, 107, 8]
    );
    assert_eq!(
        i32_lanes(unsafe { vpshufd::<0x1b>(a) }),
        [4, 3, 2, 1, 8, 7, 6, 5]
    );
    let a = i16x16(core::array::from_fn(|lane| lane as i16 + 1));
    assert_eq!(
        i16_lanes(unsafe { vpshufhw::<0x1b>(a) }),
        [1, 2, 3, 4, 8, 7, 6, 5, 9, 10, 11, 12, 16, 15, 14, 13]
    );
    assert_eq!(
        i16_lanes(unsafe { vpshuflw::<0x1b>(a) }),
        [4, 3, 2, 1, 5, 6, 7, 8, 12, 11, 10, 9, 13, 14, 15, 16]
    );
}

#[test]
fn gathers() {
    if !enabled() {
        return;
    }
    let data = [10i32, 20, 30, 40, 50, 60, 70, 80];
    assert_eq!(
        i32_lanes(unsafe {
            vpgatherdd(
                i32x8([0; 8]),
                data.as_ptr().cast(),
                i32x8([0, 4, 8, 12, 16, 20, 24, 28]),
                i32x8([-1; 8]),
            )
        }),
        data
    );
    assert_eq!(
        i32_lanes(unsafe {
            vpgatherdd(
                i32x8([99; 8]),
                data.as_ptr().cast(),
                i32x8([0, 4, 8, 12, 16, 20, 24, 28]),
                i32x8([0, -1, -1, -1, -1, -1, -1, -1]),
            )
        }),
        [99, 20, 30, 40, 50, 60, 70, 80]
    );

    let data = [10i32, 20, 30, 40];
    assert_eq!(
        i32x4_lanes(unsafe {
            vpgatherqd(
                i32x4([0; 4]),
                data.as_ptr().cast(),
                i64x4([0, 4, 8, 12]),
                i32x4([-1; 4]),
            )
        }),
        data
    );

    let data = [10i64, 20, 30, 40];
    assert_eq!(
        i64_lanes(unsafe {
            vpgatherdq(
                i64x4([0; 4]),
                data.as_ptr().cast(),
                i32x4([0, 8, 16, 24]),
                i64x4([-1; 4]),
            )
        }),
        data
    );
    assert_eq!(
        i64_lanes(unsafe {
            vpgatherqq(
                i64x4([0; 4]),
                data.as_ptr().cast(),
                i64x4([0, 8, 16, 24]),
                i64x4([-1; 4]),
            )
        }),
        data
    );
}

#[test]
fn mask_load_and_store() {
    if !enabled() {
        return;
    }
    let data = [10i32, 20, 30, 40, 50, 60, 70, 80];
    assert_eq!(
        i32_lanes(unsafe { vpmaskmovd_load(data.as_ptr().cast(), i32x8([-1; 8])) }),
        data
    );
    assert_eq!(
        i32_lanes(unsafe {
            vpmaskmovd_load(data.as_ptr().cast(), i32x8([0, -1, -1, -1, -1, -1, -1, -1]))
        }),
        [0, 20, 30, 40, 50, 60, 70, 80]
    );

    let mut output = [0i32; 8];
    unsafe {
        vpmaskmovd_store(
            output.as_mut_ptr().cast(),
            i32x8([-1, -1, 0, -1, -1, -1, -1, -1]),
            i32x8(data),
        );
    }
    assert_eq!(output, [10, 20, 0, 40, 50, 60, 70, 80]);

    let data = [10i64, 20, 30, 40];
    assert_eq!(
        i64_lanes(unsafe { vpmaskmovq_load(data.as_ptr().cast(), i64x4([-1; 4])) }),
        data
    );
    let mut output = [0i64; 4];
    unsafe {
        vpmaskmovq_store(
            output.as_mut_ptr().cast(),
            i64x4([-1, 0, -1, -1]),
            i64x4(data),
        );
    }
    assert_eq!(output, [10, 0, 30, 40]);
}
