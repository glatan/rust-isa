use rust_isa::x86::simd::sse2::{
    V128, addpd, addps, andnps, andps, cmpeqpd, cmpeqps, cmplepd, cmpleps, cmpltpd, cmpltps,
    cvtdq2ps, cvtpd2ps, cvtps2dq, cvtps2pd, cvtsd2si, cvtsi2sd, cvtsi2ss, cvtss2si, cvttps2dq,
    cvttsd2si, cvttss2si, divpd, divps, maskmovdqu, maxpd, maxps, minpd, minps, movd, movmskpd,
    movmskps, movq, mulpd, mulps, orps, packssdw, packsswb, packuswb, paddb, paddd, paddq, paddsb,
    paddsw, paddusb, paddusw, paddw, pand, pavgb, pavgw, pcmpeqb, pcmpeqd, pcmpeqw, pcmpgtb,
    pcmpgtd, pcmpgtw, pmaddwd, pmaxsw, pmaxub, pmaxw, pminsw, pminub, pminw, pmovmskb, pmulhuw,
    pmulhw, pmullw, pmuludq, por, psadbw, pshufd, pshufhw, pshuflw, pslld, pslldq, psllq, psllw,
    psrad, psraw, psrld, psrldq, psrlq, psrlw, psubb, psubd, psubq, psubsb, psubsw, psubusb,
    psubusw, psubw, punpckhbw, punpckhdq, punpckhqdq, punpckhwd, punpcklbw, punpckldq, punpcklqdq,
    punpcklwd, pxor, shufpd, shufps, sqrtpd, sqrtps, subpd, subps, unpckhpd, unpckhps, unpcklpd,
    unpcklps, v128_load, v128_store, xorps,
};
use rust_isa::x86::simd::sse2::{
    comisd, comiss, cvtdq2pd, cvtpd2dq, cvttpd2dq, movhlps, movhps, movlhps, movlps, movntdq,
    movnti, movntpd, movntps, movntq, movsd, movss, pandn, ucomisd, ucomiss,
};

#[test]
fn paddd_adds_each_i32_lane() {
    let a = V128::from_bytes([1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
    let b = V128::from_bytes([10, 0, 0, 0, 20, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0]);
    let c = paddd(a, b);
    assert_eq!(
        c.to_bytes(),
        [11, 0, 0, 0, 22, 0, 0, 0, 33, 0, 0, 0, 44, 0, 0, 0,]
    );
}

#[test]
fn load_store_roundtrip() {
    let source = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let value = unsafe { v128_load(source.as_ptr()) };
    assert_eq!(value.to_bytes(), source);

    let mut output = [0u8; 16];
    unsafe {
        v128_store(output.as_mut_ptr(), value);
    }
    assert_eq!(output, source);
}

#[test]
fn paddb_wraps_bytes() {
    let a = V128::from_bytes([250; 16]);
    let b = V128::from_bytes([10; 16]);
    assert_eq!(paddb(a, b).to_bytes(), [4; 16]);
}

#[test]
fn pand_and_pcmpeqb() {
    let a = V128::from_bytes([0b1010_1010; 16]);
    let b = V128::from_bytes([0b1100_1100; 16]);
    assert_eq!(pand(a, b).to_bytes(), [0b1000_1000; 16]);
    assert_eq!(pcmpeqb(a, a).to_bytes(), [0xff; 16]);
    assert_eq!(pcmpeqb(a, b).to_bytes(), [0; 16]);
}

#[test]
fn pmullw_keeps_low_16_bits() {
    let a = V128::from_bytes([2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0]);
    let b = V128::from_bytes([4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 11, 0]);
    assert_eq!(
        pmullw(a, b).to_bytes(),
        [8, 0, 15, 0, 24, 0, 35, 0, 48, 0, 63, 0, 80, 0, 99, 0]
    );
}

#[test]
fn shifts() {
    let a = V128::from_bytes([1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
    assert_eq!(
        pslld(a, 1).to_bytes(),
        [2, 0, 0, 0, 4, 0, 0, 0, 6, 0, 0, 0, 8, 0, 0, 0]
    );

    let negative = V128::from_bytes([
        0xf8, 0xff, 0xff, 0xff, 0xf0, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    assert_eq!(
        psrad(negative, 1).to_bytes(),
        [
            0xfc, 0xff, 0xff, 0xff, 0xf8, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
}

#[test]
fn unpack_low_bytes() {
    let a = V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    let b = V128::from_bytes([
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    ]);
    assert_eq!(
        punpcklbw(a, b).to_bytes(),
        [1, 11, 2, 12, 3, 13, 4, 14, 5, 15, 6, 16, 7, 17, 8, 18]
    );
}

#[test]
fn float_arithmetic() {
    let a = V128::from_bytes([
        1.0f32.to_le_bytes()[0],
        1.0f32.to_le_bytes()[1],
        1.0f32.to_le_bytes()[2],
        1.0f32.to_le_bytes()[3],
        2.0f32.to_le_bytes()[0],
        2.0f32.to_le_bytes()[1],
        2.0f32.to_le_bytes()[2],
        2.0f32.to_le_bytes()[3],
        3.0f32.to_le_bytes()[0],
        3.0f32.to_le_bytes()[1],
        3.0f32.to_le_bytes()[2],
        3.0f32.to_le_bytes()[3],
        4.0f32.to_le_bytes()[0],
        4.0f32.to_le_bytes()[1],
        4.0f32.to_le_bytes()[2],
        4.0f32.to_le_bytes()[3],
    ]);
    let b = V128::from_bytes([
        10.0f32.to_le_bytes()[0],
        10.0f32.to_le_bytes()[1],
        10.0f32.to_le_bytes()[2],
        10.0f32.to_le_bytes()[3],
        20.0f32.to_le_bytes()[0],
        20.0f32.to_le_bytes()[1],
        20.0f32.to_le_bytes()[2],
        20.0f32.to_le_bytes()[3],
        30.0f32.to_le_bytes()[0],
        30.0f32.to_le_bytes()[1],
        30.0f32.to_le_bytes()[2],
        30.0f32.to_le_bytes()[3],
        40.0f32.to_le_bytes()[0],
        40.0f32.to_le_bytes()[1],
        40.0f32.to_le_bytes()[2],
        40.0f32.to_le_bytes()[3],
    ]);
    let sum = addps(a, b).to_bytes();
    let lanes: Vec<f32> = sum
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();
    assert_eq!(lanes, [11.0, 22.0, 33.0, 44.0]);

    let root = sqrtps(V128::from_bytes([
        4.0f32.to_le_bytes()[0],
        4.0f32.to_le_bytes()[1],
        4.0f32.to_le_bytes()[2],
        4.0f32.to_le_bytes()[3],
        9.0f32.to_le_bytes()[0],
        9.0f32.to_le_bytes()[1],
        9.0f32.to_le_bytes()[2],
        9.0f32.to_le_bytes()[3],
        16.0f32.to_le_bytes()[0],
        16.0f32.to_le_bytes()[1],
        16.0f32.to_le_bytes()[2],
        16.0f32.to_le_bytes()[3],
        25.0f32.to_le_bytes()[0],
        25.0f32.to_le_bytes()[1],
        25.0f32.to_le_bytes()[2],
        25.0f32.to_le_bytes()[3],
    ]))
    .to_bytes();
    let lanes: Vec<f32> = root
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();
    assert_eq!(lanes, [2.0, 3.0, 4.0, 5.0]);

    let one = 1.0f64.to_le_bytes();
    let two = 2.0f64.to_le_bytes();
    let mut a = [0u8; 16];
    a[0..8].copy_from_slice(&one);
    a[8..16].copy_from_slice(&two);
    let mut b = [0u8; 16];
    b[0..8].copy_from_slice(&two);
    b[8..16].copy_from_slice(&one);
    let out = addpd(V128::from_bytes(a), V128::from_bytes(b)).to_bytes();
    assert_eq!(f64::from_le_bytes(out[0..8].try_into().unwrap()), 3.0);
    assert_eq!(f64::from_le_bytes(out[8..16].try_into().unwrap()), 3.0);

    let four = 4.0f64.to_le_bytes();
    let mut c = [0u8; 16];
    c[0..8].copy_from_slice(&four);
    c[8..16].copy_from_slice(&four);
    let out = sqrtpd(V128::from_bytes(c)).to_bytes();
    assert_eq!(f64::from_le_bytes(out[0..8].try_into().unwrap()), 2.0);
}

#[test]
fn signed_compare_and_multiply() {
    let a = V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    let b = V128::from_bytes([1, 0, 3, 0, 5, 0, 7, 0, 9, 0, 11, 0, 13, 0, 15, 0]);
    let eq = pcmpgtb(a, b);
    assert_eq!(eq.to_bytes()[0], 0);
    assert_eq!(eq.to_bytes()[1], 0xff);

    let a = V128::from_bytes([3, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0]);
    let b = V128::from_bytes([5, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0]);
    let product = pmuludq(a, b).to_bytes();
    assert_eq!(u64::from_le_bytes(product[0..8].try_into().unwrap()), 15);
    assert_eq!(u64::from_le_bytes(product[8..16].try_into().unwrap()), 77);

    let a = V128::from_bytes([1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0]);
    let b = V128::from_bytes([10, 0, 20, 0, 30, 0, 40, 0, 50, 0, 60, 0, 70, 0, 80, 0]);
    let out = pmaddwd(a, b).to_bytes();
    assert_eq!(i32::from_le_bytes(out[0..4].try_into().unwrap()), 50);
    assert_eq!(i32::from_le_bytes(out[4..8].try_into().unwrap()), 250);
}

#[test]
fn sum_of_absolute_differences() {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([2; 16]);
    let out = psadbw(a, b).to_bytes();
    assert_eq!(u64::from_le_bytes(out[0..8].try_into().unwrap()), 8);
    assert_eq!(u64::from_le_bytes(out[8..16].try_into().unwrap()), 8);
}

#[test]
fn lane_shuffles() {
    let a = V128::from_bytes([1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
    assert_eq!(
        pshufd::<0x1b>(a).to_bytes(),
        [4, 0, 0, 0, 3, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0]
    );
    assert_eq!(pshufd::<0xe4>(a).to_bytes(), a.to_bytes());

    let words = V128::from_bytes([1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0]);
    assert_eq!(
        pshuflw::<0x1b>(words).to_bytes(),
        [4, 0, 3, 0, 2, 0, 1, 0, 5, 0, 6, 0, 7, 0, 8, 0]
    );
    assert_eq!(
        pshufhw::<0x1b>(words).to_bytes(),
        [1, 0, 2, 0, 3, 0, 4, 0, 8, 0, 7, 0, 6, 0, 5, 0]
    );

    let b = V128::from_bytes([10, 0, 0, 0, 20, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0]);
    assert_eq!(
        shufps::<0xe4>(a, b).to_bytes(),
        [1, 0, 0, 0, 2, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0]
    );

    let mut a64 = [0u8; 16];
    a64[0..8].copy_from_slice(&1i64.to_le_bytes());
    a64[8..16].copy_from_slice(&2i64.to_le_bytes());
    let mut b64 = [0u8; 16];
    b64[0..8].copy_from_slice(&3i64.to_le_bytes());
    b64[8..16].copy_from_slice(&4i64.to_le_bytes());
    let out = shufpd::<0b00>(V128::from_bytes(a64), V128::from_bytes(b64)).to_bytes();
    assert_eq!(i64::from_le_bytes(out[0..8].try_into().unwrap()), 1);
    assert_eq!(i64::from_le_bytes(out[8..16].try_into().unwrap()), 3);
    let out = shufpd::<0b11>(V128::from_bytes(a64), V128::from_bytes(b64)).to_bytes();
    assert_eq!(i64::from_le_bytes(out[0..8].try_into().unwrap()), 2);
    assert_eq!(i64::from_le_bytes(out[8..16].try_into().unwrap()), 4);
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

fn i64x2(values: [i64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&values[0].to_le_bytes());
    bytes[8..].copy_from_slice(&values[1].to_le_bytes());
    V128::from_bytes(bytes)
}

fn i64_lanes(value: V128) -> [i64; 2] {
    let bytes = value.to_bytes();
    [
        i64::from_le_bytes(bytes[..8].try_into().unwrap()),
        i64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ]
}

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

#[test]
fn bitwise_or_and_xor() {
    let a = V128::from_bytes([0b1010_1010; 16]);
    let b = V128::from_bytes([0b1100_1100; 16]);
    assert_eq!(por(a, b).to_bytes(), [0b1110_1110; 16]);
    assert_eq!(pxor(a, b).to_bytes(), [0b0110_0110; 16]);
}

#[test]
fn word_arithmetic_compare_and_minmax() {
    let a = i16x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = i16x8([10, 20, 30, 40, 50, 60, 70, 80]);
    assert_eq!(i16_lanes(paddw(a, b)), [11, 22, 33, 44, 55, 66, 77, 88]);
    assert_eq!(i16_lanes(psubw(b, a)), [9, 18, 27, 36, 45, 54, 63, 72]);
    assert_eq!(
        pcmpeqw(
            i16x8([5, 1, 5, 1, 5, 1, 5, 1]),
            i16x8([5, 2, 5, 2, 5, 2, 5, 2])
        )
        .to_bytes(),
        [
            0xff, 0xff, 0, 0, 0xff, 0xff, 0, 0, 0xff, 0xff, 0, 0, 0xff, 0xff, 0, 0
        ]
    );
    assert_eq!(
        i16_lanes(pcmpgtw(i16x8([1, -1, 2, -2, 3, -3, 4, -4]), i16x8([0; 8]))),
        [-1, 0, -1, 0, -1, 0, -1, 0]
    );
    assert_eq!(
        i16_lanes(pminw(
            i16x8([-1, 7, 3, 9, -5, 2, 8, 0]),
            i16x8([1, 6, 4, 8, -4, 3, 7, 1])
        )),
        [-1, 6, 3, 8, -5, 2, 7, 0]
    );
    assert_eq!(
        i16_lanes(pmaxw(
            i16x8([-1, 7, 3, 9, -5, 2, 8, 0]),
            i16x8([1, 6, 4, 8, -4, 3, 7, 1])
        )),
        [1, 7, 4, 9, -4, 3, 8, 1]
    );
    assert_eq!(i16_lanes(pavgw(i16x8([1; 8]), i16x8([2; 8]))), [2; 8]);
}

#[test]
fn dword_arithmetic_and_compare() {
    let a = i32x4([1, 2, 3, 4]);
    let b = i32x4([10, 20, 30, 40]);
    assert_eq!(i32_lanes(psubd(b, a)), [9, 18, 27, 36]);
    assert_eq!(
        pcmpeqd(i32x4([5, 1, 5, 1]), i32x4([5, 2, 5, 2])).to_bytes(),
        [
            0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0
        ]
    );
    assert_eq!(
        i32_lanes(pcmpgtd(i32x4([1, -1, 2, -2]), i32x4([0; 4]))),
        [-1, 0, -1, 0]
    );
}

#[test]
fn qword_arithmetic_wraps() {
    assert_eq!(
        i64_lanes(paddq(i64x2([i64::MAX, 5]), i64x2([1, 2]))),
        [i64::MIN, 7]
    );
    assert_eq!(i64_lanes(psubq(i64x2([0, 5]), i64x2([1, 2]))), [-1, 3]);
}

#[test]
fn byte_subtract_and_unsigned_minmax() {
    let a = V128::from_bytes([250; 16]);
    let b = V128::from_bytes([10; 16]);
    assert_eq!(psubb(a, b).to_bytes(), [240; 16]);
    assert_eq!(pminub(a, b).to_bytes(), [10; 16]);
    assert_eq!(pmaxub(a, b).to_bytes(), [250; 16]);
    assert_eq!(
        pavgb(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes(),
        [2; 16]
    );
}

#[test]
fn word_and_qword_shifts() {
    assert_eq!(
        i16_lanes(psllw(i16x8([1, 2, 3, 4, 5, 6, 7, 8]), 1)),
        [2, 4, 6, 8, 10, 12, 14, 16]
    );
    assert_eq!(i64_lanes(psllq(i64x2([1, 2]), 2)), [4, 8]);
    assert_eq!(i16_lanes(psrlw(i16x8([i16::MIN; 8]), 1)), [0x4000; 8]);
    assert_eq!(i32_lanes(psrld(i32x4([-1; 4]), 1)), [0x7fff_ffff; 4]);
    assert_eq!(
        i64_lanes(psrlq(i64x2([-1; 2]), 1)),
        [0x7fff_ffff_ffff_ffff; 2]
    );
    assert_eq!(i16_lanes(psraw(i16x8([-2; 8]), 1)), [-1; 8]);
}

#[test]
fn unpack_high_and_wide_lanes() {
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8 + 1));
    let b = V128::from_bytes(core::array::from_fn(|index| index as u8 + 17));
    assert_eq!(
        punpckhbw(a, b).to_bytes(),
        [
            9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31, 16, 32
        ]
    );

    let a = i16x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = i16x8([11, 12, 13, 14, 15, 16, 17, 18]);
    assert_eq!(i16_lanes(punpcklwd(a, b)), [1, 11, 2, 12, 3, 13, 4, 14]);
    assert_eq!(i16_lanes(punpckhwd(a, b)), [5, 15, 6, 16, 7, 17, 8, 18]);

    let a = i32x4([1, 2, 3, 4]);
    let b = i32x4([11, 12, 13, 14]);
    assert_eq!(i32_lanes(punpckldq(a, b)), [1, 11, 2, 12]);
    assert_eq!(i32_lanes(punpckhdq(a, b)), [3, 13, 4, 14]);

    let a = i64x2([1, 2]);
    let b = i64x2([3, 4]);
    assert_eq!(i64_lanes(punpcklqdq(a, b)), [1, 3]);
    assert_eq!(i64_lanes(punpckhqdq(a, b)), [2, 4]);
}

#[test]
fn f32_arithmetic_and_comparison() {
    let a = f32x4([1.0, 6.0, 3.0, -2.0]);
    let b = f32x4([2.0, 2.0, 4.0, -8.0]);
    assert_eq!(f32_lanes(subps(a, b)), [-1.0, 4.0, -1.0, 6.0]);
    assert_eq!(f32_lanes(mulps(a, b)), [2.0, 12.0, 12.0, 16.0]);
    assert_eq!(f32_lanes(divps(a, b)), [0.5, 3.0, 0.75, 0.25]);
    assert_eq!(f32_lanes(minps(a, b)), [1.0, 2.0, 3.0, -8.0]);
    assert_eq!(f32_lanes(maxps(a, b)), [2.0, 6.0, 4.0, -2.0]);
    assert_eq!(cmpeqps(a, a).to_bytes(), [0xff; 16]);
    assert_eq!(
        cmpltps(a, b).to_bytes(),
        [
            0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0
        ]
    );
    assert_eq!(
        cmpleps(a, b).to_bytes(),
        [
            0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0
        ]
    );
}

#[test]
fn f64_arithmetic_and_comparison() {
    let a = f64x2([1.0, 2.0]);
    let b = f64x2([2.0, 2.0]);
    assert_eq!(f64_lanes(subpd(a, b)), [-1.0, 0.0]);
    assert_eq!(f64_lanes(mulpd(a, b)), [2.0, 4.0]);
    assert_eq!(f64_lanes(divpd(a, b)), [0.5, 1.0]);
    assert_eq!(f64_lanes(minpd(a, b)), [1.0, 2.0]);
    assert_eq!(f64_lanes(maxpd(a, b)), [2.0, 2.0]);
    assert_eq!(cmpeqpd(a, a).to_bytes(), [0xff; 16]);
    assert_eq!(
        cmpltpd(a, b).to_bytes(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
    assert_eq!(cmplepd(a, b).to_bytes(), [0xff; 16]);
}

#[test]
fn saturating_arithmetic() {
    assert_eq!(
        paddsb(V128::from_bytes([120; 16]), V128::from_bytes([20; 16])).to_bytes(),
        [127; 16]
    );
    assert_eq!(
        paddsb(V128::from_bytes([0x88; 16]), V128::from_bytes([0xec; 16])).to_bytes(),
        [0x80; 16]
    );
    assert_eq!(
        paddusb(V128::from_bytes([250; 16]), V128::from_bytes([10; 16])).to_bytes(),
        [255; 16]
    );
    assert_eq!(
        psubsb(V128::from_bytes([0x88; 16]), V128::from_bytes([20; 16])).to_bytes(),
        [0x80; 16]
    );
    assert_eq!(
        psubusb(V128::from_bytes([5; 16]), V128::from_bytes([10; 16])).to_bytes(),
        [0; 16]
    );

    assert_eq!(
        i16_lanes(paddsw(i16x8([30_000; 8]), i16x8([10_000; 8]))),
        [32_767; 8]
    );
    assert_eq!(
        u16_lanes(paddusw(u16x8([60_000; 8]), u16x8([10_000; 8]))),
        [65_535; 8]
    );
    assert_eq!(
        i16_lanes(psubsw(i16x8([-30_000; 8]), i16x8([10_000; 8]))),
        [-32_768; 8]
    );
    assert_eq!(u16_lanes(psubusw(u16x8([5; 8]), u16x8([10; 8]))), [0; 8]);
}

#[test]
fn pack_and_high_multiply() {
    let packed = packsswb(i16x8([300, -300, 1, 2, 3, 4, 5, 6]), i16x8([7; 8])).to_bytes();
    assert_eq!(&packed[..3], &[127, 0x80, 1]);

    let packed = packssdw(i32x4([70_000, -70_000, 1, 2]), i32x4([3; 4])).to_bytes();
    assert_eq!(
        &i16_lanes(V128::from_bytes(packed))[..3],
        &[32_767, -32_768, 1]
    );

    let packed = packuswb(i16x8([-1, 300, 1, 2, 3, 4, 5, 6]), i16x8([7; 8])).to_bytes();
    assert_eq!(&packed[..2], &[0, 255]);

    let a = i16x8([0x4000; 8]);
    assert_eq!(i16_lanes(pmulhw(a, a)), [0x1000; 8]);
    assert_eq!(
        u16_lanes(pmulhuw(u16x8([0x4000; 8]), u16x8([0x4000; 8]))),
        [0x1000; 8]
    );
}

#[test]
fn mask_extraction() {
    let mut bytes = [0u8; 16];
    bytes[0] = 0x80;
    bytes[2] = 0x80;
    assert_eq!(pmovmskb(V128::from_bytes(bytes)), 0b101);

    assert_eq!(movmskps(f32x4([-1.0, 1.0, -0.0, 1.0])), 0b0101);
    assert_eq!(movmskpd(f64x2([-1.0, 1.0])), 0b01);
}

#[test]
fn signed_word_minmax_alias() {
    let a = i16x8([1, -2, 3, -4, 5, -6, 7, -8]);
    let b = i16x8([-1, 2, -3, 4, -5, 6, -7, 8]);
    assert_eq!(i16_lanes(pminsw(a, b)), [-1, -2, -3, -4, -5, -6, -7, -8]);
    assert_eq!(i16_lanes(pmaxsw(a, b)), [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn float_logical_and_unpack() {
    let a = V128::from_bytes([0b1010_1010; 16]);
    let b = V128::from_bytes([0b1100_1100; 16]);
    assert_eq!(andps(a, b).to_bytes(), [0b1000_1000; 16]);
    assert_eq!(andnps(a, b).to_bytes(), [0b0100_0100; 16]);
    assert_eq!(orps(a, b).to_bytes(), [0b1110_1110; 16]);
    assert_eq!(xorps(a, b).to_bytes(), [0b0110_0110; 16]);

    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    assert_eq!(f32_lanes(unpcklps(a, b)), [1.0, 10.0, 2.0, 20.0]);
    assert_eq!(f32_lanes(unpckhps(a, b)), [3.0, 30.0, 4.0, 40.0]);
    let a = f64x2([1.0, 2.0]);
    let b = f64x2([10.0, 20.0]);
    assert_eq!(f64_lanes(unpcklpd(a, b)), [1.0, 10.0]);
    assert_eq!(f64_lanes(unpckhpd(a, b)), [2.0, 20.0]);
}

#[test]
fn scalar_and_vector_conversions() {
    assert_eq!(f32_lanes(cvtsi2ss(-7))[0], -7.0);
    assert_eq!(f64_lanes(cvtsi2sd(-7))[0], -7.0);
    assert_eq!(cvtss2si(f32x4([3.7, 0.0, 0.0, 0.0])), 4);
    assert_eq!(cvttss2si(f32x4([3.7, 0.0, 0.0, 0.0])), 3);
    assert_eq!(cvtsd2si(f64x2([3.7, 0.0])), 4);
    assert_eq!(cvttsd2si(f64x2([3.7, 0.0])), 3);

    assert_eq!(f64_lanes(cvtps2pd(f32x4([1.5, 2.5, 3.5, 4.5]))), [1.5, 2.5]);
    assert_eq!(f32_lanes(cvtpd2ps(f64x2([1.5, 2.5]))), [1.5, 2.5, 0.0, 0.0]);
    assert_eq!(
        f32_lanes(cvtdq2ps(i32x4([1, 2, 3, 4]))),
        [1.0, 2.0, 3.0, 4.0]
    );
    assert_eq!(
        i32_lanes(cvtps2dq(f32x4([1.4, 2.6, -1.4, -2.6]))),
        [1, 3, -1, -3]
    );
    assert_eq!(
        i32_lanes(cvttps2dq(f32x4([1.4, 2.6, -1.4, -2.6]))),
        [1, 2, -1, -2]
    );
}

#[test]
fn byte_shifts_and_mask_store() {
    let source = V128::from_bytes(core::array::from_fn(|index| index as u8));
    assert_eq!(
        pslldq::<1>(source).to_bytes(),
        [0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
    );
    assert_eq!(
        psrldq::<1>(source).to_bytes(),
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0]
    );

    let source = V128::from_bytes(core::array::from_fn(|index| index as u8 + 1));
    let mut mask = [0u8; 16];
    for index in [0usize, 4, 8, 12] {
        mask[index] = 0x80;
    }
    let mut out = [0u8; 16];
    unsafe {
        maskmovdqu(source, V128::from_bytes(mask), out.as_mut_ptr());
    }
    assert_eq!(out, [1, 0, 0, 0, 5, 0, 0, 0, 9, 0, 0, 0, 13, 0, 0, 0]);
}

#[test]
fn movd_and_movq_extract_low_bits() {
    let value = V128::from_bytes([
        0x78, 0x56, 0x34, 0x12, 0xef, 0xcd, 0xab, 0x89, 1, 2, 3, 4, 5, 6, 7, 8,
    ]);
    assert_eq!(movd(value), 0x1234_5678);
    assert_eq!(movq(value), 0x89ab_cdef_1234_5678);
}

#[test]
fn scalar_moves_comparisons_and_non_temporal_stores() {
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    assert_eq!(f32_lanes(movss(a, b)), [10.0, 2.0, 3.0, 4.0]);
    assert_eq!(
        f64_lanes(movsd(f64x2([1.0, 2.0]), f64x2([10.0, 20.0]))),
        [10.0, 2.0]
    );
    assert_eq!(f32_lanes(movlps(a, b)), [10.0, 20.0, 3.0, 4.0]);
    assert_eq!(f32_lanes(movhps(a, b)), [1.0, 2.0, 10.0, 20.0]);
    assert_eq!(f32_lanes(movlhps(a, b)), [1.0, 2.0, 10.0, 20.0]);
    assert_eq!(f32_lanes(movhlps(a, b)), [30.0, 40.0, 3.0, 4.0]);
    assert_eq!(
        pandn(V128::from_bytes([0xaa; 16]), V128::from_bytes([0xcc; 16])).to_bytes(),
        [0x44; 16]
    );

    assert_eq!(comiss(a, b), -1);
    assert_eq!(comiss(b, a), 1);
    assert_eq!(comiss(a, a), 0);
    assert_eq!(comiss(f32x4([f32::NAN, 0.0, 0.0, 0.0]), a), 2);
    assert_eq!(comisd(f64x2([1.0, 0.0]), f64x2([2.0, 0.0])), -1);
    assert_eq!(ucomiss(a, b), -1);
    assert_eq!(ucomiss(f32x4([f32::NAN, 0.0, 0.0, 0.0]), a), 2);
    assert_eq!(ucomisd(f64x2([f64::NAN, 0.0]), f64x2([1.0, 0.0])), 2);

    assert_eq!(f64_lanes(cvtdq2pd(i32x4([1, -2, 3, 4]))), [1.0, -2.0]);
    assert_eq!(&i32_lanes(cvtpd2dq(f64x2([1.5, 2.5])))[..2], &[2, 2]);
    assert_eq!(&i32_lanes(cvttpd2dq(f64x2([1.9, -1.9])))[..2], &[1, -1]);

    #[repr(align(16))]
    struct Aligned([u8; 16]);
    let mut output = Aligned([0; 16]);
    unsafe { movntps(output.0.as_mut_ptr().cast::<f32>(), a) };
    assert_eq!(output.0, a.to_bytes());
    unsafe { movntpd(output.0.as_mut_ptr().cast::<f64>(), f64x2([1.5, 2.5])) };
    assert_eq!(
        f64_lanes(unsafe { v128_load(output.0.as_ptr()) }),
        [1.5, 2.5]
    );
    unsafe { movntdq(output.0.as_mut_ptr(), V128::from_bytes([7; 16])) };
    assert_eq!(output.0, [7; 16]);
    let mut integer = 0i32;
    unsafe { movnti(&mut integer, 1234) };
    assert_eq!(integer, 1234);
    #[cfg(target_arch = "x86_64")]
    {
        unsafe { movntq(output.0.as_mut_ptr(), V128::from_bytes([9; 16])) };
        assert_eq!(&output.0[..8], &[9; 8]);
    }
}
