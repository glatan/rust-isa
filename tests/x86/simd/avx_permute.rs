use rust_isa::x86::simd::avx::{
    V256, vblendpd, vblendps, vblendvpd, vblendvps, vbroadcastf128, vextractf128, vinsertf128,
    vlddqu, vmaskmovpd_load, vmaskmovpd_store, vmaskmovps_load, vmaskmovps_store, vmovd,
    vmovd_to_u32, vmovddup, vmovmskpd, vmovshdup, vmovsldup, vperm2f128, vpermilpd, vpermilps,
    vpsadbw, vroundpd, vroundps, vroundsd, vroundss, vshufpd, vshufps, vunpckhpd, vunpckhps,
    vunpcklpd, vunpcklps,
};
use rust_isa::x86::simd::sse2::V128;

fn enabled() -> bool {
    is_x86_feature_detected!("avx")
}

fn f32x8(values: [f32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn f32_lanes(value: V256) -> [f32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn f64x4(values: [f64; 4]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn f64_lanes(value: V256) -> [f64; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
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

fn u32x4_lanes(value: V128) -> [u32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn i64x2_lanes(value: V128) -> [i64; 2] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn f32x4(values: [f32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn f32x4_lanes(value: V128) -> [f32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn f64x2_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

#[test]
fn blends() {
    if !enabled() {
        return;
    }
    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = f32x8([101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0]);
    assert_eq!(
        f32_lanes(unsafe { vblendps::<0x55>(a, b) }),
        [101.0, 2.0, 103.0, 4.0, 105.0, 6.0, 107.0, 8.0]
    );
    let a = f64x4([1.0, 2.0, 3.0, 4.0]);
    let b = f64x4([101.0, 102.0, 103.0, 104.0]);
    assert_eq!(
        f64_lanes(unsafe { vblendpd::<0b0101>(a, b) }),
        [101.0, 2.0, 103.0, 4.0]
    );

    let a = f32x8([1.0; 8]);
    let b = f32x8([2.0; 8]);
    let mask = f32x8([-1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0]);
    assert_eq!(
        f32_lanes(unsafe { vblendvps(a, b, mask) }),
        [2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0]
    );
    let a = f64x4([1.0; 4]);
    let b = f64x4([2.0; 4]);
    let mask = f64x4([-1.0, 0.0, -1.0, 0.0]);
    assert_eq!(
        f64_lanes(unsafe { vblendvpd(a, b, mask) }),
        [2.0, 1.0, 2.0, 1.0]
    );
}

#[test]
fn lane_insert_extract_and_permute() {
    if !enabled() {
        return;
    }
    let a = i32x8([1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(i32x4_lanes(unsafe { vextractf128::<1>(a) }), [5, 6, 7, 8]);
    assert_eq!(
        i32_lanes(unsafe { vinsertf128::<1>(a, i32x4([101, 102, 103, 104])) }),
        [1, 2, 3, 4, 101, 102, 103, 104]
    );

    let b = i32x8([101, 102, 103, 104, 105, 106, 107, 108]);
    assert_eq!(
        i32_lanes(unsafe { vperm2f128::<0x21>(a, b) }),
        [5, 6, 7, 8, 101, 102, 103, 104]
    );

    assert_eq!(
        i32_lanes(unsafe { vpermilps::<0x1b>(a) }),
        [4, 3, 2, 1, 8, 7, 6, 5]
    );
    assert_eq!(
        f64_lanes(unsafe { vpermilpd::<0x5>(f64x4([1.0, 2.0, 3.0, 4.0])) }),
        [2.0, 1.0, 4.0, 3.0]
    );

    let source = i32x4([10, 20, 30, 40]);
    assert_eq!(
        i32_lanes(unsafe { vbroadcastf128(source.as_ptr()) }),
        [10, 20, 30, 40, 10, 20, 30, 40]
    );
}

#[test]
fn round() {
    if !enabled() {
        return;
    }
    let a = f32x8([1.7, 2.2, 3.8, 4.1, -1.7, -2.2, -3.8, -4.1]);
    assert_eq!(
        f32_lanes(unsafe { vroundps::<0>(a) }),
        [2.0, 2.0, 4.0, 4.0, -2.0, -2.0, -4.0, -4.0]
    );
    let a = f64x4([1.7, 2.2, 3.8, 4.1]);
    assert_eq!(f64_lanes(unsafe { vroundpd::<0>(a) }), [2.0, 2.0, 4.0, 4.0]);
    let a = f32x4([9.0; 4]);
    let b = f32x4([1.7, 2.2, 3.8, 4.1]);
    assert_eq!(
        f32x4_lanes(unsafe { vroundss::<0>(a, b) }),
        [2.0, 9.0, 9.0, 9.0]
    );
    let a = f64x2([9.0, 9.0]);
    let b = f64x2([1.7, 2.2]);
    assert_eq!(f64x2_lanes(unsafe { vroundsd::<0>(a, b) }), [2.0, 9.0]);
}

#[test]
fn shuffle_and_unpack() {
    if !enabled() {
        return;
    }
    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = f32x8([101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0]);
    assert_eq!(
        f32_lanes(unsafe { vshufps::<0x1b>(a, b) }),
        [4.0, 3.0, 102.0, 101.0, 8.0, 7.0, 106.0, 105.0]
    );
    let a = f64x4([1.0, 2.0, 3.0, 4.0]);
    let b = f64x4([101.0, 102.0, 103.0, 104.0]);
    assert_eq!(
        f64_lanes(unsafe { vshufpd::<0b01>(a, b) }),
        [2.0, 101.0, 3.0, 103.0]
    );

    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = f32x8([101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0]);
    assert_eq!(
        f32_lanes(unsafe { vunpcklps(a, b) }),
        [1.0, 101.0, 2.0, 102.0, 5.0, 105.0, 6.0, 106.0]
    );
    assert_eq!(
        f32_lanes(unsafe { vunpckhps(a, b) }),
        [3.0, 103.0, 4.0, 104.0, 7.0, 107.0, 8.0, 108.0]
    );
    let a = f64x4([1.0, 2.0, 3.0, 4.0]);
    let b = f64x4([101.0, 102.0, 103.0, 104.0]);
    assert_eq!(
        f64_lanes(unsafe { vunpcklpd(a, b) }),
        [1.0, 101.0, 3.0, 103.0]
    );
    assert_eq!(
        f64_lanes(unsafe { vunpckhpd(a, b) }),
        [2.0, 102.0, 4.0, 104.0]
    );

    assert_eq!(
        i64x2_lanes(unsafe { vpsadbw(V128::from_bytes([1; 16]), V128::from_bytes([0; 16])) }),
        [8; 2]
    );
}

#[test]
fn masked_memory_and_duplicates() {
    if !enabled() {
        return;
    }
    let data = f32x8([10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
    assert_eq!(
        f32_lanes(unsafe { vmaskmovps_load(data.as_ptr(), f32x8([-1.0; 8])) }),
        [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]
    );
    let mut output = [0.0f32; 8];
    unsafe {
        vmaskmovps_store(
            output.as_mut_ptr().cast(),
            f32x8([-1.0, -1.0, 0.0, -1.0, -1.0, -1.0, -1.0, -1.0]),
            data,
        );
    }
    assert_eq!(output, [10.0, 20.0, 0.0, 40.0, 50.0, 60.0, 70.0, 80.0]);

    let data = f64x4([10.0, 20.0, 30.0, 40.0]);
    assert_eq!(
        f64_lanes(unsafe { vmaskmovpd_load(data.as_ptr(), f64x4([-1.0; 4])) }),
        [10.0, 20.0, 30.0, 40.0]
    );
    let mut output = [0.0f64; 4];
    unsafe {
        vmaskmovpd_store(
            output.as_mut_ptr().cast(),
            f64x4([-1.0, 0.0, -1.0, -1.0]),
            data,
        );
    }
    assert_eq!(output, [10.0, 0.0, 30.0, 40.0]);

    assert_eq!(
        u32x4_lanes(unsafe { vmovd(0x1234_5678) }),
        [0x1234_5678, 0, 0, 0]
    );
    assert_eq!(unsafe { vmovd_to_u32(vmovd(0x1234_5678)) }, 0x1234_5678);

    assert_eq!(unsafe { vmovmskpd(f64x4([-1.0, 1.0, -2.0, 2.0])) }, 0b0101);
    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    assert_eq!(
        f32_lanes(unsafe { vmovshdup(a) }),
        [2.0, 2.0, 4.0, 4.0, 6.0, 6.0, 8.0, 8.0]
    );
    assert_eq!(
        f32_lanes(unsafe { vmovsldup(a) }),
        [1.0, 1.0, 3.0, 3.0, 5.0, 5.0, 7.0, 7.0]
    );
    assert_eq!(
        f64_lanes(unsafe { vmovddup(f64x4([1.0, 2.0, 3.0, 4.0])) }),
        [1.0, 1.0, 3.0, 3.0]
    );

    let source = [0u8; 33];
    assert_eq!(
        unsafe { vlddqu(source.as_ptr().add(1)) }.to_bytes(),
        [0; 32]
    );
}
