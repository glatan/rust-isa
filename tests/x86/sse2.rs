use rust_isa::x86::simd::sse2::{
    V128, addpd, addps, paddb, paddd, pand, pcmpeqb, pcmpgtb, pmaddwd, pmullw, pmuludq, psadbw,
    pshufd, pshufhw, pshuflw, pslld, psrad, punpcklbw, shufpd, shufps, sqrtpd, sqrtps, v128_load,
    v128_store,
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
