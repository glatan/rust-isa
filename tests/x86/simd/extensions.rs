use rust_isa::x86::simd::avx::{
    vaddpd, vaddps, vaddsd, vaddss, vandps, vbroadcastsd, vbroadcastss, vdivps, vmovmskps, vmulpd,
    vmulps, vorps, vsubps, vxorps, vzeroupper,
};
use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::f16c::{vcvtph2ps, vcvtph2ps_v256, vcvtps2ph, vcvtps2ph_v256};
use rust_isa::x86::simd::sse2::V128;
use rust_isa::x86::simd::sse3::{
    addsubpd, addsubps, haddpd, haddps, hsubpd, hsubps, lddqu, movddup, movshdup, movsldup,
};
use rust_isa::x86::simd::sse4_1::{
    blendpd, blendps, blendvpd, blendvps, dppd, dpps, insertps, mpsadbw, packusdw, pblendvb,
    pblendw, phminposuw, pmaxud, pmaxuw, pminud, pminuw, pmovsxbd, pmovsxbw, pmovzxbw, pmulld,
    ptest, roundpd, roundps, roundsd, roundss,
};
use rust_isa::x86::simd::sse4_2::{crc32_u8, pcmpgtq};
use rust_isa::x86::simd::ssse3::{
    pabsb, pabsd, pabsw, palignr, phaddd, phaddsw, phaddw, phsubd, phsubsw, phsubw, pmaddubsw,
    pmulhrsw, pshufb, psignb, psignd, psignw,
};

fn f32x4(values: [f32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn f32_lanes(value: V128) -> [f32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&values[0].to_le_bytes());
    bytes[8..].copy_from_slice(&values[1].to_le_bytes());
    V128::from_bytes(bytes)
}

fn f64_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    [
        f64::from_le_bytes(bytes[..8].try_into().unwrap()),
        f64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ]
}

fn i16x8(values: [i16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn i16_lanes(value: V128) -> [i16; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn i32_lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn u16x8(values: [u16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn u16_lanes(value: V128) -> [u16; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

#[test]
fn sse3_addsub_and_horizontal() {
    if !is_x86_feature_detected!("sse3") {
        return;
    }
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    assert_eq!(
        f32_lanes(unsafe { addsubps(a, b) }),
        [-9.0, 22.0, -27.0, 44.0]
    );
    assert_eq!(f32_lanes(unsafe { haddps(a, b) }), [3.0, 7.0, 30.0, 70.0]);
    assert_eq!(
        f32_lanes(unsafe { hsubps(a, b) }),
        [-1.0, -1.0, -10.0, -10.0]
    );
    assert_eq!(f32_lanes(unsafe { movsldup(a) }), [1.0, 1.0, 3.0, 3.0]);
    assert_eq!(f32_lanes(unsafe { movshdup(a) }), [2.0, 2.0, 4.0, 4.0]);

    let a = f64x2([1.0, 2.0]);
    let b = f64x2([10.0, 20.0]);
    assert_eq!(f64_lanes(unsafe { addsubpd(a, b) }), [-9.0, 22.0]);
    assert_eq!(f64_lanes(unsafe { haddpd(a, b) }), [3.0, 30.0]);
    assert_eq!(f64_lanes(unsafe { hsubpd(a, b) }), [-1.0, -10.0]);
    assert_eq!(f64_lanes(unsafe { movddup(a) }), [1.0, 1.0]);
}

#[test]
fn sse3_lddqu_loads_unaligned() {
    if !is_x86_feature_detected!("sse3") {
        return;
    }
    let memory = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
    let value = unsafe { lddqu(memory.as_ptr().add(1)) };
    assert_eq!(
        value.to_bytes(),
        [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
    );
}

#[test]
fn ssse3_abs_and_shuffle() {
    if !is_x86_feature_detected!("ssse3") {
        return;
    }
    let a = V128::from_bytes([
        0xff, 0xfe, 2, 0x80, 0, 1, 0xff, 0x7f, 3, 4, 5, 6, 7, 8, 9, 10,
    ]);
    assert_eq!(&unsafe { pabsb(a) }.to_bytes()[..4], &[1, 2, 2, 0x80]);
    assert_eq!(
        i16_lanes(unsafe { pabsw(i16x8([-1, -2, 3, -32768, 5, 6, 7, 8])) }),
        [1, 2, 3, -32768, 5, 6, 7, 8]
    );
    assert_eq!(
        i32_lanes(unsafe { pabsd(i32x4([-1, 2, -3, i32::MIN])) }),
        [1, 2, 3, i32::MIN]
    );

    let source = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let identity = V128::from_bytes(core::array::from_fn(|index| index as u8));
    assert_eq!(
        unsafe { pshufb(source, identity) }.to_bytes(),
        source.to_bytes()
    );
    let zero = V128::from_bytes([0x80; 16]);
    assert_eq!(unsafe { pshufb(source, zero) }.to_bytes(), [0; 16]);
    let reverse = V128::from_bytes(core::array::from_fn(|index| 15 - index as u8));
    assert_eq!(
        unsafe { pshufb(source, reverse) }.to_bytes(),
        core::array::from_fn(|index| 15 - index as u8)
    );
}

#[test]
fn ssse3_horizontal_and_saturating() {
    if !is_x86_feature_detected!("ssse3") {
        return;
    }
    let a = i16x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = i16x8([10, 20, 30, 40, 50, 60, 70, 80]);
    assert_eq!(
        i16_lanes(unsafe { phaddw(a, b) }),
        [3, 7, 11, 15, 30, 70, 110, 150]
    );
    assert_eq!(
        i16_lanes(unsafe { phsubw(a, b) }),
        [-1, -1, -1, -1, -10, -10, -10, -10]
    );
    assert_eq!(
        i16_lanes(unsafe { phaddsw(i16x8([32767, 1, 0, 0, 0, 0, 0, 0]), i16x8([0; 8])) }),
        [32767, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        i16_lanes(unsafe { phsubsw(i16x8([-32768, 1, 0, 0, 0, 0, 0, 0]), i16x8([0; 8])) }),
        [-32768, 0, 0, 0, 0, 0, 0, 0]
    );

    let a = i32x4([1, 2, 3, 4]);
    let b = i32x4([10, 20, 30, 40]);
    assert_eq!(i32_lanes(unsafe { phaddd(a, b) }), [3, 7, 30, 70]);
    assert_eq!(i32_lanes(unsafe { phsubd(a, b) }), [-1, -1, -10, -10]);
}

#[test]
fn ssse3_multiply_and_sign() {
    if !is_x86_feature_detected!("ssse3") {
        return;
    }
    let a = V128::from_bytes([
        1, 0xff, 2, 0xfe, 3, 0xff, 4, 0xfe, 5, 0xff, 6, 0xfe, 7, 0xff, 8, 0xfe,
    ]);
    let b = V128::from_bytes([1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8]);
    assert_eq!(
        i16_lanes(unsafe { pmaddubsw(a, b) }),
        [256, 512, 774, 1032, 1300, 1560, 1834, 2096]
    );
    assert_eq!(
        i16_lanes(unsafe { pmulhrsw(i16x8([0x4000; 8]), i16x8([0x4000; 8])) }),
        [8192; 8]
    );

    let a = V128::from_bytes([
        1, 0xff, 2, 0xfe, 3, 0xff, 4, 0xfe, 5, 0xff, 6, 0xfe, 7, 0xff, 8, 0xfe,
    ]);
    let b = V128::from_bytes([
        1, 0xff, 2, 0xfe, 3, 0xff, 4, 0xfe, 5, 0xff, 6, 0xfe, 7, 0xff, 8, 0xfe,
    ]);
    assert_eq!(
        unsafe { psignb(a, b) }.to_bytes(),
        [1, 1, 2, 2, 3, 1, 4, 2, 5, 1, 6, 2, 7, 1, 8, 2]
    );
    let zero = V128::from_bytes([0; 16]);
    assert_eq!(unsafe { psignb(a, zero) }.to_bytes(), [0; 16]);
    assert_eq!(
        i16_lanes(unsafe { psignw(i16x8([1, -2, 3, -4, 5, -6, 7, -8]), i16x8([-1; 8])) }),
        [-1, 2, -3, 4, -5, 6, -7, 8]
    );
    assert_eq!(
        i32_lanes(unsafe { psignd(i32x4([1, -2, 3, -4]), i32x4([-1; 4])) }),
        [-1, 2, -3, 4]
    );
}

#[test]
fn ssse3_palignr() {
    if !is_x86_feature_detected!("ssse3") {
        return;
    }
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8 + 16));
    let b = V128::from_bytes(core::array::from_fn(|index| index as u8));
    assert_eq!(unsafe { palignr::<0>(a, b) }.to_bytes(), b.to_bytes());
    assert_eq!(
        unsafe { palignr::<4>(a, b) }.to_bytes(),
        core::array::from_fn(|index| (index + 4) as u8)
    );
    assert_eq!(unsafe { palignr::<16>(a, b) }.to_bytes(), a.to_bytes());
}

#[test]
fn sse41_integer_and_mask_operations() {
    if !is_x86_feature_detected!("sse4.1") {
        return;
    }
    assert_eq!(
        i32_lanes(unsafe { pmulld(i32x4([2, 3, 4, 5]), i32x4([6, 7, 8, 9])) }),
        [12, 21, 32, 45]
    );
    let a = V128::from_bytes([0xff, 0xff, 0xff, 0xff, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
    let b = V128::from_bytes([0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 4, 0, 0, 0]);
    assert_eq!(
        unsafe { pminud(a, b) }.to_bytes(),
        [0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0]
    );
    assert_eq!(
        unsafe { pmaxud(a, b) }.to_bytes(),
        [0xff, 0xff, 0xff, 0xff, 2, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0]
    );
    assert_eq!(
        u16_lanes(unsafe {
            pminuw(
                u16x8([5, 1, 3, 9, 0, 7, 2, 8]),
                u16x8([1, 2, 3, 4, 5, 6, 7, 8]),
            )
        }),
        [1, 1, 3, 4, 0, 6, 2, 8]
    );
    assert_eq!(
        u16_lanes(unsafe {
            pmaxuw(
                u16x8([5, 1, 3, 9, 0, 7, 2, 8]),
                u16x8([1, 2, 3, 4, 5, 6, 7, 8]),
            )
        }),
        [5, 2, 3, 9, 5, 7, 7, 8]
    );
}

#[test]
fn sse41_test_blend_and_pack() {
    if !is_x86_feature_detected!("sse4.1") {
        return;
    }
    assert!(unsafe { ptest(V128::from_bytes([0; 16]), V128::from_bytes([0xff; 16])) });
    assert!(!unsafe { ptest(V128::from_bytes([1; 16]), V128::from_bytes([1; 16])) });

    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    let mask = V128::from_bytes([
        0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
    ]);
    assert_eq!(
        f32_lanes(unsafe { blendvps(a, b, mask) }),
        [10.0, 2.0, 30.0, 4.0]
    );
    let mask = V128::from_bytes([
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    assert_eq!(
        f64_lanes(unsafe { blendvpd(f64x2([1.0, 2.0]), f64x2([10.0, 20.0]), mask) }),
        [10.0, 2.0]
    );

    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([2; 16]);
    let mask = V128::from_bytes([
        0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0,
    ]);
    assert_eq!(
        unsafe { pblendvb(a, b, mask) }.to_bytes(),
        [2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1]
    );

    assert_eq!(
        u16_lanes(unsafe { packusdw(i32x4([1, -1, 70000, -70000]), i32x4([0; 4])) }),
        [1, 0, 0xffff, 0, 0, 0, 0, 0]
    );
}

#[test]
fn sse41_minpos_and_extend() {
    if !is_x86_feature_detected!("sse4.1") {
        return;
    }
    let input = u16x8([5, 1, 3, 9, 0, 7, 2, 8]);
    let out = u16_lanes(unsafe { phminposuw(input) });
    assert_eq!(out[0], 0);
    assert_eq!(out[1], 4);
    for lane in 2..8 {
        assert_eq!(out[lane], 0);
    }

    let bytes = V128::from_bytes([0xff, 0x7f, 0x80, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(i32_lanes(unsafe { pmovsxbd(bytes) }), [-1, 127, -128, 1]);
    assert_eq!(
        i16_lanes(unsafe { pmovsxbw(bytes) }),
        [-1, 127, -128, 1, 0, 0, 0, 0]
    );
    assert_eq!(
        u16_lanes(unsafe { pmovzxbw(bytes) }),
        [255, 127, 128, 1, 0, 0, 0, 0]
    );
}

#[test]
fn sse42_compare_and_crc32() {
    if !is_x86_feature_detected!("sse4.2") {
        return;
    }
    let a = V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80]);
    let b = V128::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let out = unsafe { pcmpgtq(a, b) }.to_bytes();
    assert_eq!(&out[..8], &[0xff; 8]);
    assert_eq!(&out[8..], &[0; 8]);

    let mut crc = 0xffff_ffffu32;
    for byte in b"123456789" {
        crc = unsafe { crc32_u8(crc, *byte) };
    }
    assert_eq!(crc ^ 0xffff_ffff, 0xe306_9283);
}

fn f32x8(values: [f32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn f32x8_lanes(value: V256) -> [f32; 8] {
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

fn f64x4_lanes(value: V256) -> [f64; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

#[test]
fn avx_float_arithmetic() {
    if !is_x86_feature_detected!("avx") {
        return;
    }
    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = f32x8([10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
    assert_eq!(
        f32x8_lanes(unsafe { vaddps(a, b) }),
        [11.0, 22.0, 33.0, 44.0, 55.0, 66.0, 77.0, 88.0]
    );
    assert_eq!(
        f32x8_lanes(unsafe { vmulps(a, b) }),
        [10.0, 40.0, 90.0, 160.0, 250.0, 360.0, 490.0, 640.0]
    );
    assert_eq!(
        f32x8_lanes(unsafe { vdivps(b, a) }),
        [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0]
    );
    assert_eq!(
        f32x8_lanes(unsafe { vsubps(b, a) }),
        [9.0, 18.0, 27.0, 36.0, 45.0, 54.0, 63.0, 72.0]
    );

    let a = f64x4([1.0, 2.0, 3.0, 4.0]);
    let b = f64x4([10.0, 20.0, 30.0, 40.0]);
    assert_eq!(
        f64x4_lanes(unsafe { vaddpd(a, b) }),
        [11.0, 22.0, 33.0, 44.0]
    );
    assert_eq!(
        f64x4_lanes(unsafe { vmulpd(a, b) }),
        [10.0, 40.0, 90.0, 160.0]
    );
}

#[test]
fn avx_bitwise_and_broadcast() {
    if !is_x86_feature_detected!("avx") {
        return;
    }
    let a = V256::from_bytes([0b1010_1010; 32]);
    let b = V256::from_bytes([0b1100_1100; 32]);
    assert_eq!(unsafe { vandps(a, b) }.to_bytes(), [0b1000_1000; 32]);
    assert_eq!(unsafe { vorps(a, b) }.to_bytes(), [0b1110_1110; 32]);
    assert_eq!(unsafe { vxorps(a, b) }.to_bytes(), [0b0110_0110; 32]);
    assert_eq!(f32x8_lanes(unsafe { vbroadcastss(1.5) }), [1.5; 8]);
    assert_eq!(f64x4_lanes(unsafe { vbroadcastsd(2.5) }), [2.5; 4]);

    let mut bytes = [0u8; 32];
    for lane in [0usize, 3, 7] {
        bytes[lane * 4 + 3] = 0x80;
    }
    assert_eq!(unsafe { vmovmskps(V256::from_bytes(bytes)) }, 0b1000_1001);
    unsafe { vzeroupper() };
}

#[test]
fn avx_scalar_and_f16c() {
    if !is_x86_feature_detected!("avx") || !is_x86_feature_detected!("f16c") {
        return;
    }
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    let out = unsafe { vaddss(a, b) };
    assert_eq!(f32_lanes(out), [11.0, 2.0, 3.0, 4.0]);

    let a = f64x2([1.0, 2.0]);
    let b = f64x2([10.0, 20.0]);
    assert_eq!(f64_lanes(unsafe { vaddsd(a, b) }), [11.0, 2.0]);

    let floats = f32x4([1.0, 0.5, -2.0, 3.0]);
    let half = unsafe { vcvtps2ph::<0>(floats) };
    assert_eq!(f32_lanes(unsafe { vcvtph2ps(half) }), [1.0, 0.5, -2.0, 3.0]);

    let floats = f32x8([1.0, 0.5, -2.0, 3.0, 4.0, -0.25, 8.0, 16.0]);
    let half = unsafe { vcvtps2ph_v256::<0>(floats) };
    assert_eq!(
        f32x8_lanes(unsafe { vcvtph2ps_v256(half) }),
        [1.0, 0.5, -2.0, 3.0, 4.0, -0.25, 8.0, 16.0]
    );
}

#[test]
fn optional_extension_api_surface() {
    let _: unsafe fn(V128, V128) -> V128 = rust_isa::x86::simd::sse4a::extrq;
    let _: unsafe fn(V128, V128) -> V128 = rust_isa::x86::simd::sse4a::insertq;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::fma4::macc_ps;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::xop::vpperm;
    let _: unsafe fn(V256, V256) -> V256 = rust_isa::x86::simd::avx10_1::vaddps_v256;
    let _: unsafe fn(
        rust_isa::x86::simd::avx512f::V512,
        rust_isa::x86::simd::avx512f::V512,
    ) -> rust_isa::x86::simd::avx512f::V512 = rust_isa::x86::simd::avx10_2::vmulps_v512;
    assert_eq!(
        core::mem::size_of::<rust_isa::x86::matrix::amx_tile::TileConfig>(),
        64
    );
}

#[test]
fn sse41_immediate_operations() {
    if !is_x86_feature_detected!("sse4.1") {
        return;
    }

    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    assert_eq!(
        f32_lanes(unsafe { blendps::<0b0101>(a, b) }),
        [10.0, 2.0, 30.0, 4.0]
    );
    let a = f64x2([1.0, 2.0]);
    let b = f64x2([10.0, 20.0]);
    assert_eq!(f64_lanes(unsafe { blendpd::<0b01>(a, b) }), [10.0, 2.0]);

    let a = i16x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = i16x8([10, 20, 30, 40, 50, 60, 70, 80]);
    assert_eq!(
        i16_lanes(unsafe { pblendw::<0x55>(a, b) }),
        [10, 2, 30, 4, 50, 6, 70, 8]
    );

    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    assert_eq!(
        f32_lanes(unsafe { insertps::<0x00>(a, b) }),
        [10.0, 2.0, 3.0, 4.0]
    );

    let a = f32x4([1.7, 2.2, 3.8, 4.1]);
    assert_eq!(f32_lanes(unsafe { roundps::<0>(a) }), [2.0, 2.0, 4.0, 4.0]);
    assert_eq!(f32_lanes(unsafe { roundps::<1>(a) }), [1.0, 2.0, 3.0, 4.0]);
    assert_eq!(f32_lanes(unsafe { roundps::<2>(a) }), [2.0, 3.0, 4.0, 5.0]);
    assert_eq!(f32_lanes(unsafe { roundps::<3>(a) }), [1.0, 2.0, 3.0, 4.0]);

    let a = f64x2([1.7, 2.2]);
    assert_eq!(f64_lanes(unsafe { roundpd::<0>(a) }), [2.0, 2.0]);

    let a = f32x4([9.0; 4]);
    let b = f32x4([1.7, 2.2, 3.8, 4.1]);
    assert_eq!(
        f32_lanes(unsafe { roundss::<0>(a, b) }),
        [2.0, 9.0, 9.0, 9.0]
    );
    let a = f64x2([9.0, 9.0]);
    let b = f64x2([1.7, 2.2]);
    assert_eq!(f64_lanes(unsafe { roundsd::<0>(a, b) }), [2.0, 9.0]);

    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([1.0; 4]);
    assert_eq!(f32_lanes(unsafe { dpps::<0xff>(a, b) }), [10.0; 4]);
    let a = f64x2([1.0, 2.0]);
    let b = f64x2([1.0; 2]);
    assert_eq!(f64_lanes(unsafe { dppd::<0xff>(a, b) }), [3.0; 2]);

    assert_eq!(
        unsafe { mpsadbw::<0>(V128::from_bytes([0; 16]), V128::from_bytes([0; 16])) }.to_bytes(),
        [0; 16]
    );
}
