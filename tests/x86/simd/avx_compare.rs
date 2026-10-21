use rust_isa::x86::simd::avx::{
    vaddsubpd, vaddsubps, vandnpd, vandnps, vandpd, vcmppd, vcmpps, vcmpsd, vcmpss, vcomisd,
    vcomiss, vcvtdq2ps, vcvtpd2ps, vcvtps2dq, vcvtps2pd, vcvtsd2ss, vcvtss2sd, vcvttps2dq, vdppd,
    vdpps, vmaxpd, vmaxps, vminpd, vminps, vmovntpd, vmovntps, vmovq, vmovq_to_u64, vptest, vrcpps,
    vsqrtpd, vsqrtps, vtestpd, vtestps, vucomisd, vucomiss, vxorpd,
};
use rust_isa::x86::simd::avx2::V256;
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
fn arithmetic_and_bitwise() {
    if !enabled() {
        return;
    }
    assert_eq!(
        f32_lanes(unsafe { vaddsubps(f32x8([1.0; 8]), f32x8([2.0; 8])) }),
        [-1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0]
    );
    assert_eq!(
        f64_lanes(unsafe { vaddsubpd(f64x4([1.0; 4]), f64x4([2.0; 4])) }),
        [-1.0, 3.0, -1.0, 3.0]
    );

    let a = V256::from_bytes([0b1010_1010; 32]);
    let b = V256::from_bytes([0b1100_1100; 32]);
    assert_eq!(unsafe { vandpd(a, b) }.to_bytes(), [0b1000_1000; 32]);
    assert_eq!(unsafe { vandnps(a, b) }.to_bytes(), [0b0100_0100; 32]);
    assert_eq!(unsafe { vandnpd(a, b) }.to_bytes(), [0b0100_0100; 32]);
    assert_eq!(unsafe { vxorpd(a, b) }.to_bytes(), [0b0110_0110; 32]);

    let one = f32x8([1.0; 8]);
    let two = f32x8([2.0; 8]);
    assert_eq!(f32_lanes(unsafe { vmaxps(one, two) }), [2.0; 8]);
    assert_eq!(f32_lanes(unsafe { vminps(one, two) }), [1.0; 8]);
    let one = f64x4([1.0; 4]);
    let two = f64x4([2.0; 4]);
    assert_eq!(f64_lanes(unsafe { vmaxpd(one, two) }), [2.0; 4]);
    assert_eq!(f64_lanes(unsafe { vminpd(one, two) }), [1.0; 4]);

    assert_eq!(
        f32_lanes(unsafe { vsqrtps(f32x8([4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0, 81.0])) }),
        [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        f64_lanes(unsafe { vsqrtpd(f64x4([4.0, 9.0, 16.0, 25.0])) }),
        [2.0, 3.0, 4.0, 5.0]
    );
    let reciprocal = unsafe { vrcpps(f32x8([4.0; 8])) };
    for lane in f32_lanes(reciprocal) {
        assert!((lane - 0.25).abs() < 0.001);
    }
}

#[test]
fn conversions_and_moves() {
    if !enabled() {
        return;
    }
    assert_eq!(
        f64_lanes(unsafe { vcvtps2pd(f32x4([1.5, 2.5, 3.5, 4.5])) }),
        [1.5, 2.5, 3.5, 4.5]
    );
    assert_eq!(
        f32x4_lanes(unsafe { vcvtpd2ps(f64x4([1.5, 2.5, 3.5, 4.5])) }),
        [1.5, 2.5, 3.5, 4.5]
    );
    assert_eq!(
        f64x2_lanes(unsafe { vcvtss2sd(f64x2([9.0, 8.0]), f32x4([1.5, 0.0, 0.0, 0.0])) }),
        [1.5, 8.0]
    );
    assert_eq!(
        f32x4_lanes(unsafe { vcvtsd2ss(f32x4([9.0, 8.0, 7.0, 6.0]), f64x2([1.5, 0.0])) }),
        [1.5, 8.0, 7.0, 6.0]
    );
    assert_eq!(
        f32_lanes(unsafe { vcvtdq2ps(i32x8([1, -2, 3, 4, 5, 6, 7, 8])) }),
        [1.0, -2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
    );
    assert_eq!(
        i32_lanes(unsafe { vcvtps2dq(f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])) }),
        [1, 2, 3, 4, 5, 6, 7, 8]
    );
    assert_eq!(
        i32_lanes(unsafe { vcvttps2dq(f32x8([1.9, -1.9, 2.1, -2.1, 3.9, -3.9, 4.1, -4.1])) }),
        [1, -1, 2, -2, 3, -3, 4, -4]
    );
    assert_eq!(
        unsafe { vmovq_to_u64(vmovq(0x0123_4567_89ab_cdef)) },
        0x0123_4567_89ab_cdef
    );
}

#[test]
fn compare_flags() {
    if !enabled() {
        return;
    }
    let one = f32x4([1.0; 4]);
    let two = f32x4([2.0; 4]);
    assert_eq!(unsafe { vcomiss(one, two) }, 1);
    assert_eq!(unsafe { vcomiss(two, one) }, 0);
    assert_eq!(unsafe { vcomiss(one, one) }, 4);
    assert_eq!(unsafe { vcomiss(f32x4([f32::NAN; 4]), one) }, 7);
    let one = f64x2([1.0; 2]);
    let two = f64x2([2.0; 2]);
    assert_eq!(unsafe { vcomisd(one, two) }, 1);
    assert_eq!(unsafe { vcomisd(one, one) }, 4);
    assert_eq!(unsafe { vucomiss(f32x4([1.0; 4]), f32x4([2.0; 4])) }, 1);
    assert_eq!(unsafe { vucomisd(f64x2([1.0; 2]), f64x2([1.0; 2])) }, 4);

    assert_eq!(
        unsafe { vptest(V256::from_bytes([0; 32]), V256::from_bytes([1; 32])) },
        4
    );
    assert_eq!(
        unsafe { vptest(V256::from_bytes([0xff; 32]), V256::from_bytes([0xff; 32])) },
        1
    );
    assert_eq!(unsafe { vtestps(f32x8([0.0; 8]), f32x8([1.0; 8])) }, 4);
    assert_eq!(unsafe { vtestps(f32x8([-1.0; 8]), f32x8([-1.0; 8])) }, 1);
    assert_eq!(unsafe { vtestpd(f64x4([0.0; 4]), f64x4([1.0; 4])) }, 4);
    assert_eq!(unsafe { vtestpd(f64x4([-1.0; 4]), f64x4([-1.0; 4])) }, 1);
}

#[test]
fn compare_masks_and_dot_products() {
    if !enabled() {
        return;
    }
    let mask = unsafe {
        vcmpps::<0>(
            f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
            f32x8([1.0, 0.0, 3.0, 0.0, 5.0, 0.0, 7.0, 0.0]),
        )
    };
    let bytes = mask.to_bytes();
    for lane in 0..8 {
        let expected = if lane % 2 == 0 { 0xff } else { 0x00 };
        assert_eq!(&bytes[lane * 4..lane * 4 + 4], &[expected; 4]);
    }
    let mask = unsafe { vcmppd::<0>(f64x4([1.0, 2.0, 3.0, 4.0]), f64x4([1.0, 0.0, 3.0, 0.0])) };
    let bytes = mask.to_bytes();
    for lane in 0..4 {
        let expected = if lane % 2 == 0 { 0xff } else { 0x00 };
        assert_eq!(&bytes[lane * 8..lane * 8 + 8], &[expected; 8]);
    }

    let mask = unsafe { vcmpss::<0>(f32x4([9.0; 4]), f32x4([9.0, 0.0, 0.0, 0.0])) }.to_bytes();
    assert_eq!(&mask[0..4], &[0xff; 4]);
    assert_eq!(&mask[4..16], &f32x4([9.0; 4]).to_bytes()[4..16]);
    let mask = unsafe { vcmpsd::<0>(f64x2([9.0, 8.0]), f64x2([9.0, 0.0])) }.to_bytes();
    assert_eq!(&mask[0..8], &[0xff; 8]);
    assert_eq!(&mask[8..16], &f64x2([9.0, 8.0]).to_bytes()[8..16]);

    assert_eq!(
        f32_lanes(unsafe {
            vdpps::<0xff>(
                f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
                f32x8([1.0; 8]),
            )
        }),
        [10.0, 10.0, 10.0, 10.0, 26.0, 26.0, 26.0, 26.0]
    );
    assert_eq!(
        f64x2_lanes(unsafe { vdppd::<0xff>(f64x2([1.0, 2.0]), f64x2([1.0; 2])) }),
        [3.0, 3.0]
    );
}

#[repr(align(32))]
struct Aligned([u8; 32]);

#[test]
fn non_temporal_stores() {
    if !enabled() {
        return;
    }
    let value = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let mut output = Aligned([0; 32]);
    unsafe {
        vmovntps(output.0.as_mut_ptr(), value);
    }
    assert_eq!(output.0, value.to_bytes());

    let value = f64x4([1.0, 2.0, 3.0, 4.0]);
    let mut output = Aligned([0; 32]);
    unsafe {
        vmovntpd(output.0.as_mut_ptr(), value);
    }
    assert_eq!(output.0, value.to_bytes());
}
