use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512bw::*;
use rust_isa::x86::simd::avx512dq::{
    vandpd, vcmppd, vcmpps, vcvtpd2qq, vcvtqq2pd, vfixupimmpd, vfpclasspd, vfpclassps, vgetexppd,
    vgetmantpd, vpmovd2m, vpmovm2d, vrcp14ps, vreducepd, vscalefpd,
};
use rust_isa::x86::simd::avx512f::*;
use rust_isa::x86::simd::avx512fp16::{vcmpph, vcvtph2pd, vmaxph, vreduceph, vsqrtph};
use rust_isa::x86::simd::avx512ifma::vpmadd52huq as vpmadd52huq_512;
use rust_isa::x86::simd::avx512vl::*;
use rust_isa::x86::simd::avx512vnni::{vpdpbusds, vpdpwssd, vpdpwssds};
use rust_isa::x86::simd::sse2::V128;

fn f_enabled() -> bool {
    is_x86_feature_detected!("avx512f")
}

fn bw_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512bw")
}

fn dq_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512dq")
}

fn vl_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512vl")
}

fn vbmi_enabled() -> bool {
    vl_enabled() && is_x86_feature_detected!("avx512vbmi")
}

fn vbmi2_enabled() -> bool {
    vl_enabled() && is_x86_feature_detected!("avx512vbmi2")
}

fn vnni_enabled() -> bool {
    vl_enabled() && is_x86_feature_detected!("avx512vnni")
}

fn ifma_enabled() -> bool {
    vl_enabled() && is_x86_feature_detected!("avx512ifma")
}

fn bitalg_enabled() -> bool {
    vl_enabled() && is_x86_feature_detected!("avx512bitalg")
}

fn fp16_enabled() -> bool {
    is_x86_feature_detected!("avx512fp16")
}

fn i8x64(values: [i8; 64]) -> V512 {
    V512::from_bytes(core::array::from_fn(|index| values[index] as u8))
}

fn i8_lanes(value: V512) -> [i8; 64] {
    value.to_bytes().map(|byte| byte as i8)
}

fn i16x32(values: [i16; 32]) -> V512 {
    let mut bytes = [0u8; 64];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

fn i16_lanes(value: V512) -> [i16; 32] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

fn i32x16(values: [i32; 16]) -> V512 {
    let mut bytes = [0u8; 64];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

fn i32_lanes(value: V512) -> [i32; 16] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn i64x8(values: [i64; 8]) -> V512 {
    let mut bytes = [0u8; 64];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

fn i64_lanes(value: V512) -> [i64; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn f32x16(values: [f32; 16]) -> V512 {
    let mut bytes = [0u8; 64];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

fn f32_lanes(value: V512) -> [f32; 16] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn f64x8(values: [f64; 8]) -> V512 {
    let mut bytes = [0u8; 64];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

fn f64_lanes(value: V512) -> [f64; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn i8x16(values: [i8; 16]) -> V128 {
    V128::from_bytes(core::array::from_fn(|index| values[index] as u8))
}

fn i32x4(values: [i32; 4]) -> V128 {
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

fn i32x4_lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn i32x8(values: [i32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i32x8_lanes(value: V256) -> [i32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
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

fn v256_from_f32x16_prefix(value: V512) -> V256 {
    V256::from_bytes(value.to_bytes()[..32].try_into().unwrap())
}

#[test]
fn foundation_mask_bitwise_and_permute() {
    if !f_enabled() {
        return;
    }
    let a = V512::from_bytes([0xf0; 64]);
    let b = V512::from_bytes([0xcc; 64]);
    assert_eq!(unsafe { vpandq(a, b) }.to_bytes(), [0xc0; 64]);
    assert_eq!(unsafe { vpandnd(a, b) }.to_bytes(), [0x0c; 64]);
    assert_eq!(unsafe { vporq(a, b) }.to_bytes(), [0xfc; 64]);
    assert_eq!(unsafe { vpxorq(a, b) }.to_bytes(), [0x3c; 64]);

    let c = V512::from_bytes([0xaa; 64]);
    assert_eq!(
        unsafe { vpternlogd::<0x96>(a, b, c) }.to_bytes(),
        [0x96; 64]
    );

    let lhs = i32x16(core::array::from_fn(|lane| lane as i32));
    let rhs = i32x16(core::array::from_fn(|lane| {
        if lane < 4 { lane as i32 } else { 100 }
    }));
    assert_eq!(unsafe { vpcmpd::<0>(lhs, rhs) }, 0x000f);
    assert_eq!(unsafe { vpcmpud::<1>(lhs, rhs) }, 0xfff0);
    assert_eq!(unsafe { vpcmpq::<4>(i64x8([1; 8]), i64x8([2; 8])) }, 0xff);

    let bits = i32x16(core::array::from_fn(
        |lane| {
            if lane < 3 { 1 << lane } else { 0 }
        },
    ));
    assert_eq!(unsafe { vptestmd(bits, bits) }, 0x0007);
    assert_eq!(unsafe { vptestnmd(bits, i32x16([0; 16])) }, 0xffff);
    assert_eq!(i32_lanes(unsafe { vpmovm2d(0b0101) })[..4], [-1, 0, -1, 0]);
    assert_eq!(
        unsafe { vpmovd2m(i32x16([-1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])) },
        0b0101
    );
    assert_eq!(i64_lanes(unsafe { vpbroadcastq(9) }), [9; 8]);

    let indices = i64x8([7, 6, 5, 4, 3, 2, 1, 0]);
    assert_eq!(
        i64_lanes(unsafe { vpermq(i64x8([0, 1, 2, 3, 4, 5, 6, 7]), indices) }),
        [7, 6, 5, 4, 3, 2, 1, 0]
    );

    let floats = f32x16(core::array::from_fn(|lane| lane as f32));
    assert_eq!(
        f32_lanes(unsafe { vpermilps::<0x1b>(floats) })[..4],
        [3.0, 2.0, 1.0, 0.0]
    );
    let other = f32x16(core::array::from_fn(|lane| lane as f32 + 100.0));
    assert_eq!(
        f32_lanes(unsafe { vshufps::<0x00>(floats, other) })[..4],
        [0.0, 0.0, 100.0, 100.0]
    );

    let block = f32x16(core::array::from_fn(|lane| lane as f32));
    let insert = i32x4([100, 101, 102, 103]);
    let inserted = unsafe { vinsertf32x4::<1>(block, insert) };
    assert_eq!(i32_lanes(inserted)[4..8], [100, 101, 102, 103]);
    let extracted = unsafe { vextractf32x4::<2>(block) };
    assert_eq!(f32x4_lanes(extracted), [8.0, 9.0, 10.0, 11.0]);
}

#[test]
fn foundation_compress_expand_blend_and_move() {
    if !f_enabled() {
        return;
    }
    let source = f32x16(core::array::from_fn(|lane| lane as f32));
    let compressed = f32_lanes(unsafe { vcompressps(source, 0b0101) });
    assert_eq!(compressed[..4], [0.0, 2.0, 0.0, 0.0]);
    let expanded = f32_lanes(unsafe { vexpandps(source, 0b0101) });
    assert_eq!(expanded[..4], [0.0, 0.0, 1.0, 0.0]);

    let other = f32x16([100.0; 16]);
    let blended = f32_lanes(unsafe { vblendmps(source, other, 0x00f0) });
    assert_eq!(blended[3], 3.0);
    assert_eq!(blended[4], 100.0);

    let moved = f32_lanes(unsafe { vmovmps(other, source, 0x0003) });
    assert_eq!(moved[0], 0.0);
    assert_eq!(moved[1], 1.0);
    assert_eq!(moved[2], 100.0);
}

#[test]
fn foundation_float_and_convert() {
    if !f_enabled() {
        return;
    }
    let roots = f32_lanes(unsafe {
        vsqrtps(f32x16([
            4.0, 9.0, 16.0, 25.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ]))
    });
    assert_eq!(roots[..4], [2.0, 3.0, 4.0, 5.0]);

    assert_eq!(f32_lanes(unsafe { vgetexpps(f32x16([3.0; 16])) })[0], 1.0);
    assert_eq!(
        f32_lanes(unsafe { vgetmantps::<0>(f32x16([3.0; 16])) })[0],
        1.5
    );
    assert_eq!(
        f32_lanes(unsafe { vscalefps(f32x16([1.0; 16]), f32x16([2.0; 16])) })[0],
        4.0
    );
    assert_eq!(
        f32_lanes(unsafe {
            vrndscaleps::<0>(f32x16([
                1.7, 2.2, -1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ]))
        })[..3],
        [2.0, 2.0, -2.0]
    );

    let doubles = unsafe { vcvtps2pd(v256_from_f32x16_prefix(f32x16([1.0; 16]))) };
    assert_eq!(f64_lanes(doubles)[0], 1.0);
    let singles = unsafe { vcvtpd2ps(f64x8([2.0; 8])) };
    assert_eq!(f32x8_lanes(singles)[0], 2.0);
    assert_eq!(i32_lanes(unsafe { vcvttps2dq(f32x16([1.9; 16])) })[0], 1);
    assert_eq!(f32_lanes(unsafe { vcvtdq2ps(i32x16([7; 16])) })[0], 7.0);
}

#[test]
fn foundation_gather_and_scatter() {
    if !f_enabled() {
        return;
    }
    let data = [10.0f32, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
    let indices = i32x16([0, 2, 4, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let gathered = unsafe { vgatherdps(f32x16([0.0; 16]), data.as_ptr().cast(), indices, 0x000f) };
    assert_eq!(f32_lanes(gathered)[..4], [10.0, 30.0, 50.0, 70.0]);

    let mut output = [0.0f32; 8];
    unsafe {
        vscatterdps(
            output.as_mut_ptr().cast(),
            indices,
            f32x16([
                1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ]),
            0x000f,
        );
    }
    assert_eq!(output, [1.0, 0.0, 2.0, 0.0, 3.0, 0.0, 4.0, 0.0]);
}

#[test]
fn byte_word_operations() {
    if !bw_enabled() {
        return;
    }
    assert_eq!(
        i16_lanes(unsafe { vpaddw(i16x32([1000; 32]), i16x32([234; 32])) })[0],
        1234
    );
    assert_eq!(
        i8_lanes(unsafe { vpaddsb(i8x64([100; 64]), i8x64([100; 64])) })[0],
        127
    );
    assert_eq!(
        i8_lanes(unsafe { vpaddusb(i8x64([100; 64]), i8x64([100; 64])) })[0],
        -56
    );
    assert_eq!(
        i8_lanes(unsafe { vpsubusb(i8x64([10; 64]), i8x64([20; 64])) })[0],
        0
    );
    assert_eq!(
        i8_lanes(unsafe { vpmaxsb(i8x64([-5; 64]), i8x64([-3; 64])) })[0],
        -3
    );
    assert_eq!(
        i8_lanes(unsafe { vpminub(i8x64([10; 64]), i8x64([20; 64])) })[0],
        10
    );
    assert_eq!(
        i16_lanes(unsafe { vpmulhw(i16x32([300; 32]), i16x32([3; 32])) })[0],
        0
    );
    assert_eq!(
        i16_lanes(unsafe { vpmullw(i16x32([300; 32]), i16x32([3; 32])) })[0],
        900
    );
    let madd = i16x32(core::array::from_fn(
        |lane| {
            if lane % 2 == 0 { 1 } else { 2 }
        },
    ));
    assert_eq!(i32_lanes(unsafe { vpmaddwd(madd, i16x32([3; 32])) })[0], 9);
    assert_eq!(
        i8_lanes(unsafe { vpavgb(i8x64([10; 64]), i8x64([21; 64])) })[0],
        16
    );

    let bytes = V512::from_bytes(core::array::from_fn(|index| index as u8));
    let packed = unsafe { vpacksswb(i8x64([-128; 64]), i8x64([127; 64])) };
    assert_eq!(packed.to_bytes()[0], 128);
    assert_eq!(packed.to_bytes()[8], 127);
    let unpacked = unsafe { vpunpcklbw(bytes, bytes) }.to_bytes();
    assert_eq!(&unpacked[..6], &[0, 0, 1, 1, 2, 2]);

    assert_eq!(i16_lanes(unsafe { vpsllw(i16x32([1; 32]), 3) })[0], 8);
    assert_eq!(i16_lanes(unsafe { vpsraw(i16x32([-8; 32]), 2) })[0], -2);
    let shifted =
        unsafe { vpslldq::<2>(V512::from_bytes(core::array::from_fn(|index| index as u8))) }
            .to_bytes();
    assert_eq!(&shifted[..4], &[0, 0, 0, 1]);

    let narrow =
        unsafe { vpmovwb(i16x32(core::array::from_fn(|lane| lane as i16 + 1))) }.to_bytes();
    assert_eq!(&narrow[..4], &[1, 2, 3, 4]);
    let saturated = unsafe { vpmovswb(i16x32([300; 32])) }.to_bytes();
    assert_eq!(saturated[0], 127);

    let cmp_a = i8x64(core::array::from_fn(|lane| lane as i8));
    let cmp_b = i8x64(core::array::from_fn(
        |lane| if lane < 3 { lane as i8 } else { 99 },
    ));
    assert_eq!(unsafe { vpcmpb::<0>(cmp_a, cmp_b) }, 0b111);
    assert_eq!(unsafe { vptestmb(cmp_a, cmp_a) }, u64::MAX ^ 1);
    assert_eq!(unsafe { vpmovb2m(i8x64([-1; 64])) }, u64::MAX);
    assert_eq!(i8_lanes(unsafe { vpmovm2b(0xff) })[0], -1);
    assert_eq!(i8_lanes(unsafe { vpbroadcastb(7) })[..4], [7; 4]);
    assert_eq!(i16_lanes(unsafe { vpbroadcastw(300) })[..4], [300; 4]);
}

#[test]
fn dq_float_and_conversions() {
    if !dq_enabled() {
        return;
    }
    let a = f64x8([1.0; 8]);
    let b = f64x8([2.0; 8]);
    assert_eq!(f64_lanes(unsafe { vandpd(a, b) })[0], 0.0);
    assert_eq!(
        unsafe { vcmpps::<0>(f32x16([1.0; 16]), f32x16([1.0; 16])) },
        0xffff
    );
    assert_eq!(unsafe { vcmppd::<0>(a, b) }, 0);

    let classes = f64x8([f64::INFINITY, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(unsafe { vfpclasspd::<0x08>(classes) }, 0b1);
    assert_eq!(unsafe { vfpclassps::<0x08>(f32x16([0.0; 16])) }, 0);

    assert_eq!(f64_lanes(unsafe { vsqrtpd(f64x8([4.0; 8])) })[0], 2.0);
    assert_eq!(f64_lanes(unsafe { vgetexppd(f64x8([3.0; 8])) })[0], 1.0);
    assert_eq!(
        f64_lanes(unsafe { vgetmantpd::<0>(f64x8([3.0; 8])) })[0],
        1.5
    );
    assert_eq!(
        f64_lanes(unsafe { vscalefpd(f64x8([1.0; 8]), f64x8([3.0; 8])) })[0],
        8.0
    );
    assert_eq!(
        f64_lanes(unsafe { vreducepd::<0>(f64x8([0.0; 8])) })[0],
        0.0
    );
    assert!((f32_lanes(unsafe { vrcp14ps(f32x16([4.0; 16])) })[0] - 0.25).abs() < 0.001);

    assert_eq!(i64_lanes(unsafe { vcvtpd2qq(f64x8([3.0; 8])) }), [3; 8]);
    assert_eq!(f64_lanes(unsafe { vcvtqq2pd(i64x8([5; 8])) }), [5.0; 8]);
    // Token 7 (positive finite) selects table bits 31:28. Use different
    // response codes per lane so lane-wise corruption is visible.
    let table = i64x8([
        0x8 << 28,
        0x9 << 28,
        0xa << 28,
        0xb << 28,
        0x8 << 28,
        0x9 << 28,
        0xa << 28,
        0xb << 28,
    ]);
    assert_eq!(
        f64_lanes(unsafe {
            vfixupimmpd::<0>(
                f64x8([2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]),
                f64x8([1.5; 8]),
                table,
            )
        }),
        [0.0, -1.0, 1.0, 0.5, 0.0, -1.0, 1.0, 0.5]
    );
}

#[test]
fn vnni_and_ifma_extensions() {
    if !vnni_enabled() {
        return;
    }
    let acc = V512::from_bytes([0; 64]);
    let bytes = V512::from_bytes([1; 64]);
    let words = i16x32([1; 32]);
    assert_eq!(i32_lanes(unsafe { vpdpbusds(acc, bytes, bytes) }), [4; 16]);
    assert_eq!(i32_lanes(unsafe { vpdpwssd(acc, words, words) }), [2; 16]);
    assert_eq!(i32_lanes(unsafe { vpdpwssds(acc, words, words) }), [2; 16]);

    if ifma_enabled() {
        let acc = i64x8([7; 8]);
        let a = i64x8([1 << 26; 8]);
        let b = i64x8([1 << 27; 8]);
        assert_eq!(i64_lanes(unsafe { vpmadd52huq_512(acc, a, b) }), [9; 8]);
        assert_eq!(
            i32x4_lanes(unsafe {
                vpmadd52luq_v128(
                    V128::from_bytes([7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                    V128::from_bytes([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                    V128::from_bytes([5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                )
            })[0],
            22
        );
    }
}

#[test]
fn vl_128_and_256() {
    if !vl_enabled() {
        return;
    }
    if bw_enabled() {
        let a = i8x16(core::array::from_fn(|lane| lane as i8));
        let b = i8x16([1; 16]);
        let sum = unsafe { vpaddb_v128(a, b) };
        assert_eq!(sum.to_bytes()[..4], [1, 2, 3, 4]);
        assert_eq!(
            unsafe { vptestmb_v128(i8x16([-1; 16]), i8x16([-1; 16])) },
            0xffff
        );
    }
    assert_eq!(
        unsafe { vpcmpd_v256::<0>(i32x8([1; 8]), i32x8([1; 8])) },
        0xff
    );

    let source = V256::from_bytes(core::array::from_fn(|index| index as u8));
    let compressed = unsafe { vcompressps_v256(source, 0b0101) };
    assert_eq!(&compressed.to_bytes()[..8], &[0, 1, 2, 3, 8, 9, 10, 11]);

    if vbmi_enabled() {
        let indices = V256::from_bytes(core::array::from_fn(|index| 31 - index as u8));
        assert_eq!(unsafe { vpermb_v256(source, indices) }.to_bytes()[0], 31);
    }
    if vbmi2_enabled() {
        assert_eq!(
            unsafe {
                vpcompressb_v128(
                    V128::from_bytes(core::array::from_fn(|index| index as u8)),
                    0b0101,
                )
            }
            .to_bytes()[..4],
            [0, 2, 0, 0]
        );
        let shifted = unsafe { vpshldd_v128::<4>(i32x4([0x12, 0, 0, 0]), i32x4([0, 0, 0, 0])) };
        assert_eq!(i32x4_lanes(shifted)[0], 0x120);
        let shifted = unsafe {
            vpshrdq_v128::<4>(
                V128::from_bytes([0x12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                V128::from_bytes([0; 16]),
            )
        };
        assert_eq!(shifted.to_bytes()[0], 1);
    }
    if vnni_enabled() {
        let acc = V128::from_bytes([0; 16]);
        let ones = V128::from_bytes([1; 16]);
        assert_eq!(
            i32x4_lanes(unsafe { vpdpbusd_v128(acc, ones, ones) }),
            [4; 4]
        );
    }
    if bitalg_enabled() {
        assert_eq!(
            unsafe { vpopcntb_v128(i8x16([-1; 16])) }.to_bytes(),
            [8; 16]
        );
        assert_eq!(
            unsafe { vpshufbitqmb_v128(i8x16([-1; 16]), i8x16([0; 16])) },
            0xffff
        );
    }
    if is_x86_feature_detected!("avx512vp2intersect") {
        let values = i32x4([1, 2, 3, 4]);
        assert_eq!(
            unsafe { vp2intersectd_v128(values, values) },
            (0b1111, 0b1111)
        );
    }
    let gathered = unsafe {
        vgatherdps_v256(
            f32x8([0.0; 8]),
            [1.0f32, 2.0, 3.0, 4.0].as_ptr().cast(),
            i32x8([0, 1, 2, 3, 0, 0, 0, 0]),
            0x0f,
        )
    };
    assert_eq!(f32x8_lanes(gathered)[..4], [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn fp16_new_operations() {
    if !fp16_enabled() {
        return;
    }
    let four = f16x32(0x4400);
    assert_eq!(
        u16::from_le_bytes(unsafe { vsqrtph(four) }.to_bytes()[..2].try_into().unwrap()),
        0x4000
    );
    assert_eq!(
        u16::from_le_bytes(
            unsafe { vmaxph(f16x32(0x3c00), f16x32(0x4000)) }.to_bytes()[..2]
                .try_into()
                .unwrap()
        ),
        0x4000
    );
    assert_eq!(
        unsafe { vcmpph::<0>(f16x32(0x3c00), f16x32(0x3c00)) },
        u32::MAX
    );
    let doubles = unsafe { vcvtph2pd(V128::from_bytes(f16x8(0x3c00))) };
    assert_eq!(f64_lanes(doubles)[0], 1.0);
    // imm 0 removes the integer part with round-to-nearest-even.
    let reduced = f16x32_pattern([
        0x3e00, 0x4200, 0x3c00, 0xbe00, 0xc200, 0x3f00, 0xbf00, 0x4000,
    ]);
    let expected = f16x32_pattern([
        0xb800, 0x0000, 0x0000, 0x3800, 0x0000, 0xb400, 0x3400, 0x0000,
    ]);
    assert_eq!(
        unsafe { vreduceph::<0>(reduced) }.to_bytes(),
        expected.to_bytes()
    );
}

fn f16x32(value: u16) -> V512 {
    let mut bytes = [0u8; 64];
    for lane in 0..32 {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

fn f16x32_pattern(values: [u16; 8]) -> V512 {
    let mut bytes = [0u8; 64];
    for lane in 0..32 {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&values[lane % 8].to_le_bytes());
    }
    V512::from_bytes(bytes)
}

fn f16x8(value: u16) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    for lane in 0..8 {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    bytes
}

#[test]
fn avx10_1_integer_and_float() {
    if !f_enabled() {
        return;
    }
    use rust_isa::x86::simd::avx10_1::{
        vcvtps2pd_v256, vpaddb_v256, vpaddq_v512, vpcmpd_v512, vpopcntd_v512, vsqrtps_v512,
    };
    if bw_enabled() && is_x86_feature_detected!("avx512vl") {
        let sum = unsafe { vpaddb_v256(V256::from_bytes([1; 32]), V256::from_bytes([2; 32])) };
        assert_eq!(sum.to_bytes(), [3; 32]);
    }
    let sum = unsafe { vpaddq_v512(i64x8([1; 8]), i64x8([2; 8])) };
    assert_eq!(i64_lanes(sum), [3; 8]);
    assert_eq!(
        unsafe { vpcmpd_v512::<0>(i32x16([1; 16]), i32x16([1; 16])) },
        0xffff
    );
    if is_x86_feature_detected!("avx512vpopcntdq") {
        assert_eq!(
            i32_lanes(unsafe { vpopcntd_v512(V512::from_bytes([0xff; 64])) })[0],
            32
        );
    }
    assert_eq!(
        f32_lanes(unsafe { vsqrtps_v512(f32x16([9.0; 16])) })[0],
        3.0
    );
    if vl_enabled() {
        let doubles = unsafe { vcvtps2pd_v256(v256_from_f32x16_prefix(f32x16([2.0; 16]))) };
        assert_eq!(f64_lanes(doubles)[0], 2.0);
    }

    use rust_isa::x86::simd::avx10_1::{
        vcvtpd2qq_v512, vgatherdps_v256, vpdpbusds_v512, vpshldd_v256, vptestnmd_v512,
    };
    if is_x86_feature_detected!("avx512dq") {
        assert_eq!(
            i64_lanes(unsafe { vcvtpd2qq_v512(f64x8([3.0; 8])) }),
            [3; 8]
        );
    }
    assert_eq!(
        unsafe { vptestnmd_v512(i32x16([0; 16]), i32x16([0; 16])) },
        0xffff
    );
    if is_x86_feature_detected!("avx512vnni") {
        assert_eq!(
            i32_lanes(unsafe {
                vpdpbusds_v512(
                    V512::from_bytes([0; 64]),
                    V512::from_bytes([1; 64]),
                    V512::from_bytes([1; 64]),
                )
            }),
            [4; 16]
        );
    }
    if is_x86_feature_detected!("avx512vbmi2") && is_x86_feature_detected!("avx512vl") {
        let shifted = unsafe { vpshldd_v256::<4>(i32x8([0x12; 8]), i32x8([0; 8])) };
        assert_eq!(i32x8_lanes(shifted)[0], 0x120);
    }
    if vl_enabled() {
        let data = [1.0f32, 2.0, 3.0, 4.0];
        let gathered = unsafe {
            vgatherdps_v256(
                f32x8([0.0; 8]),
                data.as_ptr().cast(),
                i32x8([0, 1, 2, 3, 0, 0, 0, 0]),
                0x0f,
            )
        };
        assert_eq!(f32x8_lanes(gathered)[..4], [1.0, 2.0, 3.0, 4.0]);
    }
}

#[test]
fn amx_api_is_gated() {
    if !amx_tile_available() {
        return;
    }
    use rust_isa::x86::matrix::amx_tile::{AmxGuard, TileConfig, ldtilecfg, sttilecfg};
    assert_eq!(core::mem::align_of::<TileConfig>(), 64);
    let mut bytes = [0u8; 64];
    bytes[0] = 1;
    bytes[16] = 64;
    bytes[17] = 0;
    bytes[48] = 16;
    let config = TileConfig::from_bytes(bytes);
    let mut readback = TileConfig::from_bytes([0; 64]);
    unsafe {
        ldtilecfg(&config);
        let _guard = AmxGuard::enter(&config);
        sttilecfg(&mut readback);
    }
    assert_eq!(readback.to_bytes(), config.to_bytes());
}

fn amx_tile_available() -> bool {
    use core::arch::x86_64::{__cpuid, __cpuid_count, _xgetbv};

    let leaf1 = __cpuid(1);
    if leaf1.ecx & (1 << 27) == 0 {
        return false;
    }
    let leaf = __cpuid_count(7, 0);
    if leaf.edx & (1 << 24) == 0 {
        return false;
    }

    // XTILEDATA is a dynamically enabled xstate feature on Linux. Requesting
    // permission is required even when the XCR0 bit is already set.
    #[cfg(target_os = "linux")]
    {
        if !request_amx_tile_data_permission() {
            return false;
        }
        // TILECFG is required by ldtilecfg. TILEDATA may remain disabled until
        // the first tile-data instruction, so only check TILECFG here.
        let xcr0 = unsafe { _xgetbv(0) };
        xcr0 & (1 << 17) != 0
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

#[cfg(target_os = "linux")]
fn request_amx_tile_data_permission() -> bool {
    const SYS_ARCH_PRCTL: i64 = 158;
    const ARCH_REQ_XCOMP_PERM: i64 = 0x1023;
    const XFEATURE_XTILEDATA: i64 = 18;

    let result: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") SYS_ARCH_PRCTL => result,
            in("rdi") ARCH_REQ_XCOMP_PERM,
            in("rsi") XFEATURE_XTILEDATA,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack),
        );
    }
    result == 0
}
