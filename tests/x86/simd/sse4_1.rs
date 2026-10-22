use rust_isa::x86::simd::sse2::V128;
use rust_isa::x86::simd::sse4_1::{
    blendpd, blendps, blendvpd, blendvps, dppd, dpps, insertps, mpsadbw, packusdw, pblendvb,
    pblendw, phminposuw, pmaxud, pmaxuw, pminud, pminuw, pmovsxbd, pmovsxbw, pmovzxbw, pmulld,
    ptest, roundpd, roundps, roundsd, roundss,
};
use rust_isa::x86::simd::sse4_1::{
    movntdqa, pextrb, pextrd, pextrw, pinsrb, pinsrd, pinsrw, pmaxsb, pmaxsd, pminsb, pminsd,
    pmovsxbq, pmovsxdq, pmovsxwd, pmovsxwq, pmovzxbd, pmovzxbq, pmovzxdq, pmovzxwd, pmovzxwq,
    pmuldq,
};

use super::support::*;

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

#[test]
fn sse41_signed_minmax_and_extend() {
    if !is_x86_feature_detected!("sse4.1") {
        return;
    }

    let a = V128::from_bytes([0x80, 0x7f, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    let b = V128::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let min = unsafe { pminsb(a, b) }.to_bytes();
    let max = unsafe { pmaxsb(a, b) }.to_bytes();
    assert_eq!(&min[..3], &[0x80, 0, 0]);
    assert_eq!(&max[..3], &[0, 0x7f, 1]);

    let a = i32x4([i32::MIN, -1, 1, i32::MAX]);
    let b = i32x4([0, 0, 0, 0]);
    assert_eq!(i32_lanes(unsafe { pminsd(a, b) }), [i32::MIN, -1, 0, 0]);
    assert_eq!(i32_lanes(unsafe { pmaxsd(a, b) }), [0, 0, 1, i32::MAX]);

    assert_eq!(
        i64_lanes(unsafe {
            pmovsxbq(V128::from_bytes([
                0x80, 0x7f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]))
        }),
        [-128, 127]
    );
    assert_eq!(
        i32_lanes(unsafe { pmovsxwd(i16x8([-32768, 32767, 1, -1, 0, 0, 0, 0])) }),
        [-32768, 32767, 1, -1]
    );
    assert_eq!(
        i64_lanes(unsafe { pmovsxwq(i16x8([-32768, 32767, 0, 0, 0, 0, 0, 0])) }),
        [-32768, 32767]
    );
    assert_eq!(
        i64_lanes(unsafe { pmovsxdq(i32x4([i32::MIN, i32::MAX, 0, 0])) }),
        [i32::MIN as i64, i32::MAX as i64]
    );

    assert_eq!(
        i64_lanes(unsafe {
            pmovzxbq(V128::from_bytes([
                0xff, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]))
        }),
        [255, 128]
    );
    assert_eq!(
        i32_lanes(unsafe { pmovzxwd(u16x8([0xffff, 0x8000, 1, 2, 0, 0, 0, 0])) }),
        [65535, 32768, 1, 2]
    );
    assert_eq!(
        i64_lanes(unsafe { pmovzxwq(u16x8([0xffff, 0x8000, 0, 0, 0, 0, 0, 0])) }),
        [65535, 32768]
    );
    assert_eq!(
        i64_lanes(unsafe { pmovzxdq(i32x4([-1, i32::MIN, 0, 0])) }),
        [u32::MAX as i64, 0x8000_0000]
    );

    let bytes = V128::from_bytes([0xff, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(i32_lanes(unsafe { pmovzxbd(bytes) }), [255, 2, 3, 4]);
    assert_eq!(
        i64_lanes(unsafe { pmuldq(i32x4([-1, 2, -3, 4]), i32x4([2, 2, 4, 4])) }),
        [-2, -12]
    );

    // MOVNTDQA requires 16-byte alignment; V128 is repr(align(16)).
    let source = V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    assert_eq!(
        unsafe { movntdqa(source.as_ptr()) }.to_bytes(),
        source.to_bytes()
    );
}

#[test]
fn sse41_insert_and_extract() {
    if !is_x86_feature_detected!("sse4.1") {
        return;
    }
    let zero = V128::from_bytes([0; 16]);
    let value = unsafe { pinsrb::<3>(zero, 0xab) }.to_bytes();
    assert_eq!(value[3], 0xab);
    assert_eq!(unsafe { pextrb::<3>(V128::from_bytes(value)) }, 0xab);

    let value = unsafe { pinsrw::<2>(zero, 0x1234) }.to_bytes();
    assert_eq!(&value[4..6], &[0x34, 0x12]);
    assert_eq!(unsafe { pextrw::<2>(V128::from_bytes(value)) }, 0x1234);

    let value = unsafe { pinsrd::<2>(zero, 0x1234_5678) }.to_bytes();
    assert_eq!(&value[8..12], &[0x78, 0x56, 0x34, 0x12]);
    assert_eq!(unsafe { pextrd::<2>(V128::from_bytes(value)) }, 0x1234_5678);

    #[cfg(target_arch = "x86_64")]
    {
        use rust_isa::x86::simd::sse4_1::{pextrq, pinsrq};
        let value = unsafe { pinsrq::<1>(zero, 0x0123_4567_89ab_cdef) }.to_bytes();
        assert_eq!(
            &value[8..16],
            &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]
        );
        assert_eq!(
            unsafe { pextrq::<1>(V128::from_bytes(value)) },
            0x0123_4567_89ab_cdef
        );
    }
}
