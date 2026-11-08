use rust_isa::x86::crypto::aes::{aesdec, aesdeclast, aesenc, aesenclast, aesimc, aeskeygenassist};
use rust_isa::x86::crypto::gfni::{gf2p8affineqb, gf2p8mulb};
use rust_isa::x86::crypto::pclmulqdq::pclmulqdq;
use rust_isa::x86::crypto::sha::{
    sha1msg1, sha1msg2, sha1nexte, sha1rnds4, sha256msg1, sha256msg2, sha256rnds2,
};
use rust_isa::x86::crypto::vaes::{
    vaesdec, vaesdec_v128, vaesdec_v512, vaesdeclast, vaesdeclast_v128, vaesdeclast_v512, vaesenc,
    vaesenc_v128, vaesenc_v512, vaesenclast, vaesenclast_v128, vaesenclast_v512,
};
use rust_isa::x86::crypto::vpclmulqdq::{vpclmulqdq_v256, vpclmulqdq_v512};
use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::sse2::V128;

fn aes_enabled() -> bool {
    is_x86_feature_detected!("aes")
}

fn sha_enabled() -> bool {
    is_x86_feature_detected!("sha")
}

fn gfni_enabled() -> bool {
    is_x86_feature_detected!("gfni")
}

fn clmul_enabled() -> bool {
    is_x86_feature_detected!("pclmulqdq")
}

fn vaes_enabled() -> bool {
    is_x86_feature_detected!("vaes") && is_x86_feature_detected!("avx2")
}

fn vaes512_enabled() -> bool {
    is_x86_feature_detected!("vaes") && is_x86_feature_detected!("avx512f")
}

fn vpclmul_enabled() -> bool {
    is_x86_feature_detected!("vpclmulqdq")
        && is_x86_feature_detected!("avx512f")
        && is_x86_feature_detected!("avx512vl")
}

fn quad(value: u64) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[0..8].copy_from_slice(&value.to_le_bytes());
    V128::from_bytes(bytes)
}

fn sequential(start: u8) -> V128 {
    V128::from_bytes(core::array::from_fn(|index| start + index as u8))
}

fn sequential256(start: u8) -> V256 {
    V256::from_bytes(core::array::from_fn(|index| start + (index % 16) as u8))
}

fn sequential512(start: u8) -> V512 {
    V512::from_bytes(core::array::from_fn(|index| start + (index % 16) as u8))
}

#[test]
fn aes_rounds() {
    if !aes_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let zero = V128::from_bytes([0; 16]);
    assert_eq!(unsafe { aesenc(zero, zero) }.to_bytes(), [0x63; 16]);
    assert_eq!(unsafe { aesenclast(zero, zero) }.to_bytes(), [0x63; 16]);
    assert_eq!(unsafe { aesdec(zero, zero) }.to_bytes(), [0x52; 16]);
    assert_eq!(unsafe { aesdeclast(zero, zero) }.to_bytes(), [0x52; 16]);
    assert_eq!(unsafe { aesimc(zero) }.to_bytes(), [0; 16]);
    let imc = unsafe { aesimc(sequential(0)) }.to_bytes();
    assert_ne!(imc, [0; 16], "aesimc returned zero for nonzero input");
    assert_ne!(imc, sequential(0).to_bytes(), "aesimc returned its input");

    let a = sequential(0);
    let b = sequential(16);
    assert_eq!(
        unsafe { aesenc(a, b) }.to_bytes(),
        [
            0x7a, 0x7b, 0x4e, 0x56, 0x38, 0x78, 0x25, 0x46, 0xa8, 0xc0, 0x47, 0x7a, 0x3b, 0x81,
            0x3f, 0x43
        ]
    );
    assert_eq!(
        unsafe { aesdec(a, b) }.to_bytes(),
        [
            0x03, 0xb2, 0x84, 0x0f, 0x3a, 0x0c, 0x3f, 0x79, 0x0f, 0x56, 0xe8, 0xa9, 0xda, 0xa0,
            0x85, 0x4e
        ]
    );
    assert_eq!(
        unsafe { aesenclast(a, b) }.to_bytes(),
        [
            0x73, 0x7a, 0x75, 0x65, 0xe6, 0x14, 0xbd, 0x6c, 0x28, 0xce, 0x6d, 0xde, 0xe2, 0x61,
            0x71, 0x34
        ]
    );
    assert_eq!(
        unsafe { aesdeclast(a, b) }.to_bytes(),
        [
            0x42, 0xe2, 0xb1, 0x2b, 0x24, 0x1c, 0xc1, 0x89, 0xa7, 0x2f, 0x70, 0xe0, 0x9d, 0x5d,
            0xbb, 0xca
        ]
    );
}

#[test]
fn sha_zero_schedule() {
    if !sha_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let zero = V128::from_bytes([0; 16]);
    assert_eq!(unsafe { sha256msg1(zero, zero) }.to_bytes(), [0; 16]);
    assert_eq!(unsafe { sha256msg2(zero, zero) }.to_bytes(), [0; 16]);
    assert_eq!(unsafe { sha1msg1(zero, zero) }.to_bytes(), [0; 16]);
    assert_eq!(unsafe { sha1msg2(zero, zero) }.to_bytes(), [0; 16]);
    assert_eq!(unsafe { sha1nexte(zero, zero) }.to_bytes(), [0; 16]);
}

#[test]
fn sha_message_schedule_is_not_zero_for_nonzero_input() {
    if !sha_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = sequential(0);
    let b = sequential(16);
    for (name, out) in [
        ("sha256msg1", unsafe { sha256msg1(a, b) }.to_bytes()),
        ("sha256msg2", unsafe { sha256msg2(a, b) }.to_bytes()),
        ("sha1msg1", unsafe { sha1msg1(a, b) }.to_bytes()),
        ("sha1msg2", unsafe { sha1msg2(a, b) }.to_bytes()),
        ("sha1nexte", unsafe { sha1nexte(a, b) }.to_bytes()),
    ] {
        assert_ne!(out, [0; 16], "{name} returned zero for nonzero input");
        assert_ne!(out, a.to_bytes(), "{name} returned its first input");
    }
}

#[test]
fn gfni_multiply() {
    if !gfni_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = V128::from_bytes([2; 16]);
    let b = V128::from_bytes([0x80; 16]);
    assert_eq!(unsafe { gf2p8mulb(a, b) }.to_bytes(), [0x1b; 16]);
}

#[test]
fn carry_less_multiply() {
    if !clmul_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = quad(3);
    let b = quad(5);
    let low = unsafe { pclmulqdq::<0x00>(a, b) }.to_bytes();
    assert_eq!(u128::from_le_bytes(low), 15);

    let mut a_high = [0u8; 16];
    a_high[8..16].copy_from_slice(&7u64.to_le_bytes());
    let mut b_high = [0u8; 16];
    b_high[8..16].copy_from_slice(&11u64.to_le_bytes());
    let high = unsafe { pclmulqdq::<0x11>(V128::from_bytes(a_high), V128::from_bytes(b_high)) };
    assert_eq!(u128::from_le_bytes(high.to_bytes()), 49);
}

#[test]
fn vaes_rounds() {
    if !vaes_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let zero128 = V128::from_bytes([0; 16]);
    assert_eq!(
        unsafe { vaesenc_v128(zero128, zero128) }.to_bytes(),
        [0x63; 16]
    );
    let zero256 = V256::from_bytes([0; 32]);
    assert_eq!(unsafe { vaesenc(zero256, zero256) }.to_bytes(), [0x63; 32]);

    let a = sequential(0);
    let b = sequential(16);
    let expected = [
        0x7a, 0x7b, 0x4e, 0x56, 0x38, 0x78, 0x25, 0x46, 0xa8, 0xc0, 0x47, 0x7a, 0x3b, 0x81, 0x3f,
        0x43,
    ];
    assert_eq!(unsafe { vaesenc_v128(a, b) }.to_bytes(), expected);
    let out = unsafe { vaesenc(sequential256(0), sequential256(16)) }.to_bytes();
    assert_eq!(&out[..16], &expected);
    assert_eq!(&out[16..], &expected);
}

#[test]
fn vaes_512_rounds() {
    if !vaes512_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let zero = V512::from_bytes([0; 64]);
    assert_eq!(unsafe { vaesenc_v512(zero, zero) }.to_bytes(), [0x63; 64]);
    assert_eq!(unsafe { vaesdec_v512(zero, zero) }.to_bytes(), [0x52; 64]);
    assert_eq!(
        unsafe { vaesenclast_v512(zero, zero) }.to_bytes(),
        [0x63; 64]
    );
    assert_eq!(
        unsafe { vaesdeclast_v512(zero, zero) }.to_bytes(),
        [0x52; 64]
    );

    let a = sequential512(0);
    let b = sequential512(16);
    let expected_enc = [
        0x7a, 0x7b, 0x4e, 0x56, 0x38, 0x78, 0x25, 0x46, 0xa8, 0xc0, 0x47, 0x7a, 0x3b, 0x81, 0x3f,
        0x43,
    ];
    let expected_dec = [
        0x03, 0xb2, 0x84, 0x0f, 0x3a, 0x0c, 0x3f, 0x79, 0x0f, 0x56, 0xe8, 0xa9, 0xda, 0xa0, 0x85,
        0x4e,
    ];
    let out = unsafe { vaesenc_v512(a, b) }.to_bytes();
    for lane in 0..4 {
        assert_eq!(&out[lane * 16..lane * 16 + 16], &expected_enc);
    }
    let out = unsafe { vaesdec_v512(a, b) }.to_bytes();
    for lane in 0..4 {
        assert_eq!(&out[lane * 16..lane * 16 + 16], &expected_dec);
    }
}

#[test]
fn vpclmulqdq_lanes() {
    if !vpclmul_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut bytes = [0u8; 32];
    for lane in 0..2 {
        bytes[lane * 16..lane * 16 + 8].copy_from_slice(&3u64.to_le_bytes());
    }
    let a = V256::from_bytes(bytes);
    let mut bytes = [0u8; 32];
    for lane in 0..2 {
        bytes[lane * 16..lane * 16 + 8].copy_from_slice(&5u64.to_le_bytes());
    }
    let b = V256::from_bytes(bytes);
    let out = unsafe { vpclmulqdq_v256(a, b, 0) }.to_bytes();
    for lane in 0..2 {
        assert_eq!(
            u128::from_le_bytes(out[lane * 16..lane * 16 + 16].try_into().unwrap()),
            15
        );
    }

    let mut bytes = [0u8; 64];
    for lane in 0..4 {
        bytes[lane * 16..lane * 16 + 8].copy_from_slice(&3u64.to_le_bytes());
    }
    let a = V512::from_bytes(bytes);
    let mut bytes = [0u8; 64];
    for lane in 0..4 {
        bytes[lane * 16..lane * 16 + 8].copy_from_slice(&5u64.to_le_bytes());
    }
    let b = V512::from_bytes(bytes);
    let out = unsafe { vpclmulqdq_v512(a, b, 0) }.to_bytes();
    for lane in 0..4 {
        assert_eq!(
            u128::from_le_bytes(out[lane * 16..lane * 16 + 16].try_into().unwrap()),
            15
        );
    }
}

#[test]
fn sha_rounds() {
    if !sha_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = sequential(0);
    let b = sequential(16);
    let k = sequential(32);
    assert_eq!(
        unsafe { sha256rnds2(a, b, k) }.to_bytes(),
        [
            0xd5, 0x6c, 0x14, 0xbe, 0x29, 0x46, 0xca, 0x1a, 0xb4, 0x90, 0x7b, 0x69, 0x8d, 0xd0,
            0x95, 0x98
        ]
    );
    assert_eq!(
        unsafe { sha1rnds4::<0>(a, b) }.to_bytes(),
        [
            0x4d, 0x0e, 0x99, 0x97, 0x9f, 0xee, 0xc9, 0x11, 0x7b, 0x66, 0x58, 0x60, 0xa6, 0xe7,
            0x24, 0x83
        ]
    );
    assert_eq!(
        unsafe { sha1rnds4::<1>(a, b) }.to_bytes(),
        [
            0xd2, 0xed, 0xb1, 0x9f, 0x54, 0xde, 0x2d, 0x9c, 0x52, 0xe6, 0xab, 0x23, 0xfa, 0x60,
            0xf0, 0xe6
        ]
    );
}

#[test]
fn gfni_affine_identity() {
    if !gfni_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let matrix = V128::from_bytes(core::array::from_fn(|index| 1u8 << (7 - (index % 8))));
    assert_eq!(
        unsafe { gf2p8affineqb::<0>(V128::from_bytes([1; 16]), matrix) }.to_bytes(),
        [1; 16]
    );
    assert_eq!(
        unsafe { gf2p8affineqb::<0>(V128::from_bytes([2; 16]), matrix) }.to_bytes(),
        [2; 16]
    );

    let matrix = V128::from_bytes([0x01; 16]);
    assert_eq!(
        unsafe { gf2p8affineqb::<0>(V128::from_bytes([1; 16]), matrix) }.to_bytes(),
        [0xff; 16]
    );
    assert_eq!(
        unsafe { gf2p8affineqb::<0>(V128::from_bytes([2; 16]), matrix) }.to_bytes(),
        [0; 16]
    );
}

#[test]
fn aes_key_schedule() {
    if !aes_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    assert_eq!(
        unsafe { aeskeygenassist::<1>(sequential(0)) }.to_bytes(),
        [
            0xf2, 0x6b, 0x6f, 0xc5, 0x6a, 0x6f, 0xc5, 0xf2, 0xfe, 0xd7, 0xab, 0x76, 0xd6, 0xab,
            0x76, 0xfe
        ]
    );
    assert_ne!(
        unsafe { aeskeygenassist::<1>(sequential(0)) }.to_bytes(),
        unsafe { aeskeygenassist::<2>(sequential(0)) }.to_bytes(),
        "the immediate operand was ignored"
    );
}

#[test]
fn vaes_sub_bytes_and_last_rounds() {
    if !vaes_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let zero128 = V128::from_bytes([0; 16]);
    assert_eq!(
        unsafe { vaesdec_v128(zero128, zero128) }.to_bytes(),
        [0x52; 16]
    );
    assert_eq!(
        unsafe { vaesenclast_v128(zero128, zero128) }.to_bytes(),
        [0x63; 16]
    );
    assert_eq!(
        unsafe { vaesdeclast_v128(zero128, zero128) }.to_bytes(),
        [0x52; 16]
    );

    let zero256 = V256::from_bytes([0; 32]);
    assert_eq!(unsafe { vaesdec(zero256, zero256) }.to_bytes(), [0x52; 32]);
    assert_eq!(
        unsafe { vaesenclast(zero256, zero256) }.to_bytes(),
        [0x63; 32]
    );
    assert_eq!(
        unsafe { vaesdeclast(zero256, zero256) }.to_bytes(),
        [0x52; 32]
    );

    let a = sequential(0);
    let b = sequential(16);
    let expected_dec = [
        0x03, 0xb2, 0x84, 0x0f, 0x3a, 0x0c, 0x3f, 0x79, 0x0f, 0x56, 0xe8, 0xa9, 0xda, 0xa0, 0x85,
        0x4e,
    ];
    let expected_enc_last = [
        0x73, 0x7a, 0x75, 0x65, 0xe6, 0x14, 0xbd, 0x6c, 0x28, 0xce, 0x6d, 0xde, 0xe2, 0x61, 0x71,
        0x34,
    ];
    let expected_dec_last = [
        0x42, 0xe2, 0xb1, 0x2b, 0x24, 0x1c, 0xc1, 0x89, 0xa7, 0x2f, 0x70, 0xe0, 0x9d, 0x5d, 0xbb,
        0xca,
    ];
    assert_eq!(unsafe { vaesdec_v128(a, b) }.to_bytes(), expected_dec);
    assert_eq!(
        unsafe { vaesenclast_v128(a, b) }.to_bytes(),
        expected_enc_last
    );
    assert_eq!(
        unsafe { vaesdeclast_v128(a, b) }.to_bytes(),
        expected_dec_last
    );

    let a = sequential256(0);
    let b = sequential256(16);
    let out = unsafe { vaesdec(a, b) }.to_bytes();
    assert_eq!(&out[..16], &expected_dec);
    assert_eq!(&out[16..], &expected_dec);
    let out = unsafe { vaesenclast(a, b) }.to_bytes();
    assert_eq!(&out[..16], &expected_enc_last);
    assert_eq!(&out[16..], &expected_enc_last);
    let out = unsafe { vaesdeclast(a, b) }.to_bytes();
    assert_eq!(&out[..16], &expected_dec_last);
    assert_eq!(&out[16..], &expected_dec_last);
}

#[test]
fn sha512_message_schedule_and_rounds() {
    if !is_x86_feature_detected!("sha512") {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    use rust_isa::x86::crypto::sha512::{vsha512msg1, vsha512msg2, vsha512rnds2};

    let a = [
        0x0123_4567_89ab_cdef,
        0xfedc_ba98_7654_3210,
        0x0f1e_2d3c_4b5a_6978,
        0x8877_6655_4433_2211,
    ];
    let b = [0xdead_beef_cafe_babe, 0x1122_3344_5566_7788];
    assert_eq!(
        v256_to_u64x4(unsafe { vsha512msg1(u64x4_to_v256(a), u64x2_to_v128(b)) }),
        sha512_msg1_ref(a, b)
    );

    let b4 = [
        0x0123_4567_89ab_cdef,
        0xfedc_ba98_7654_3210,
        0x0f1e_2d3c_4b5a_6978,
        0x8877_6655_4433_2211,
    ];
    assert_eq!(
        v256_to_u64x4(unsafe { vsha512msg2(u64x4_to_v256(a), u64x4_to_v256(b4)) }),
        sha512_msg2_ref(a, b4)
    );

    let cdgh = [
        0x1111_2222_3333_4444,
        0x5555_6666_7777_8888,
        0x9999_aaaa_bbbb_cccc,
        0xdddd_eeee_ffff_0000,
    ];
    let abef = [
        0x0123_4567_89ab_cdef,
        0xfedc_ba98_7654_3210,
        0x0f1e_2d3c_4b5a_6978,
        0x8877_6655_4433_2211,
    ];
    let wk = [0x1234_5678_9abc_def0, 0x0fed_cba9_8765_4321];
    assert_eq!(
        v256_to_u64x4(unsafe {
            vsha512rnds2(u64x4_to_v256(cdgh), u64x4_to_v256(abef), u64x2_to_v128(wk))
        }),
        sha512_rnds2_ref(cdgh, abef, wk)
    );
}

#[test]
fn sm3_message_expansion_and_rounds() {
    if !is_x86_feature_detected!("sm3") {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    use rust_isa::x86::crypto::sm3::{vsm3msg1, vsm3msg2, vsm3rnds2};

    let a = [0x0123_4567, 0x89ab_cdef, 0xfedc_ba98, 0x7654_3210];
    let b = [0x0f1e_2d3c, 0x4b5a_6978, 0x8877_6655, 0x4433_2211];
    let c = [0xdead_beef, 0xcafe_babe, 0x1122_3344, 0x5566_7788];
    assert_eq!(
        v128_to_u32x4(unsafe { vsm3msg1(u32x4_to_v128(a), u32x4_to_v128(b), u32x4_to_v128(c)) }),
        sm3_msg1_ref(a, b, c)
    );
    assert_eq!(
        v128_to_u32x4(unsafe { vsm3msg2(u32x4_to_v128(a), u32x4_to_v128(b), u32x4_to_v128(c)) }),
        sm3_msg2_ref(a, b, c)
    );
    assert_eq!(
        v128_to_u32x4(unsafe {
            vsm3rnds2::<0>(u32x4_to_v128(a), u32x4_to_v128(b), u32x4_to_v128(c))
        }),
        sm3_rnds2_ref(a, b, c, 0)
    );
    assert_eq!(
        v128_to_u32x4(unsafe {
            vsm3rnds2::<2>(u32x4_to_v128(a), u32x4_to_v128(b), u32x4_to_v128(c))
        }),
        sm3_rnds2_ref(a, b, c, 2)
    );
    assert_eq!(
        v128_to_u32x4(unsafe {
            vsm3rnds2::<1>(u32x4_to_v128(a), u32x4_to_v128(b), u32x4_to_v128(c))
        }),
        sm3_rnds2_ref(a, b, c, 1)
    );
    assert_eq!(
        v128_to_u32x4(unsafe {
            vsm3rnds2::<3>(u32x4_to_v128(a), u32x4_to_v128(b), u32x4_to_v128(c))
        }),
        sm3_rnds2_ref(a, b, c, 3)
    );
}

#[test]
fn sm4_key_schedule_and_rounds() {
    if !is_x86_feature_detected!("sm4") {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    use rust_isa::x86::crypto::sm4::{vsm4key4, vsm4rnds4};

    let mut rk = [0u32; 32];
    let mk = [0x0123_4567, 0x89ab_cdef, 0xfedc_ba98, 0x7654_3210];
    let fk = [0xa3b1_bac6, 0x56aa_3350, 0x677d_9197, 0xb270_22dc];
    let mut k = [mk[0] ^ fk[0], mk[1] ^ fk[1], mk[2] ^ fk[2], mk[3] ^ fk[3]];
    for round in 0..8 {
        let ck = [
            sm4_ck(round * 4),
            sm4_ck(round * 4 + 1),
            sm4_ck(round * 4 + 2),
            sm4_ck(round * 4 + 3),
        ];
        let out = v128_to_u32x4(unsafe { vsm4key4(u32x4_to_v128(ck), u32x4_to_v128(k)) });
        rk[round * 4..round * 4 + 4].copy_from_slice(&out);
        k = out;
    }

    let mut state = [0x0123_4567, 0x89ab_cdef, 0xfedc_ba98, 0x7654_3210];
    for round in 0..8 {
        let key = [
            rk[round * 4],
            rk[round * 4 + 1],
            rk[round * 4 + 2],
            rk[round * 4 + 3],
        ];
        state = v128_to_u32x4(unsafe { vsm4rnds4(u32x4_to_v128(key), u32x4_to_v128(state)) });
    }
    state.reverse();
    assert_eq!(state, [0x681e_df34, 0xd206_965e, 0x86b3_e94f, 0x536e_4246]);
}

fn u32x4_to_v128(values: [u32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn v128_to_u32x4(value: V128) -> [u32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn u64x2_to_v128(values: [u64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn u64x4_to_v256(values: [u64; 4]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn v256_to_u64x4(value: V256) -> [u64; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn sha512_msg1_ref(a: [u64; 4], b: [u64; 2]) -> [u64; 4] {
    let s0 = |x: u64| x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7);
    let w = [a[0], a[1], a[2], a[3], b[0]];
    [
        w[0].wrapping_add(s0(w[1])),
        w[1].wrapping_add(s0(w[2])),
        w[2].wrapping_add(s0(w[3])),
        w[3].wrapping_add(s0(w[4])),
    ]
}

fn sha512_msg2_ref(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let s1 = |x: u64| x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6);
    let w16 = a[0].wrapping_add(s1(b[2]));
    let w17 = a[1].wrapping_add(s1(b[3]));
    let w18 = a[2].wrapping_add(s1(w16));
    let w19 = a[3].wrapping_add(s1(w17));
    [w16, w17, w18, w19]
}

fn sha512_rnds2_ref(cdgh: [u64; 4], abef: [u64; 4], wk: [u64; 2]) -> [u64; 4] {
    let mut a = abef[3];
    let mut b = abef[2];
    let mut c = cdgh[3];
    let mut d = cdgh[2];
    let mut e = abef[1];
    let mut f = abef[0];
    let mut g = cdgh[1];
    let mut h = cdgh[0];

    for word in wk {
        let ch = (e & f) ^ ((!e) & g);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let sigma1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
        let sigma0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
        let t1 = h
            .wrapping_add(ch)
            .wrapping_add(sigma1)
            .wrapping_add(word)
            .wrapping_add(maj)
            .wrapping_add(sigma0);
        let t2 = ch
            .wrapping_add(sigma1)
            .wrapping_add(word)
            .wrapping_add(h)
            .wrapping_add(d);
        d = c;
        c = b;
        b = a;
        a = t1;
        h = g;
        g = f;
        f = e;
        e = t2;
    }

    [f, e, b, a]
}

fn rol32(value: u32, amount: u32) -> u32 {
    value.rotate_left(amount)
}

fn sm3_p0(value: u32) -> u32 {
    value ^ rol32(value, 9) ^ rol32(value, 17)
}

fn sm3_p1(value: u32) -> u32 {
    value ^ rol32(value, 15) ^ rol32(value, 23)
}

fn sm3_ff(x: u32, y: u32, z: u32, round: u32) -> u32 {
    if round < 16 {
        x ^ y ^ z
    } else {
        (x & y) | (x & z) | (y & z)
    }
}

fn sm3_gg(x: u32, y: u32, z: u32, round: u32) -> u32 {
    if round < 16 {
        x ^ y ^ z
    } else {
        (x & y) | ((!x) & z)
    }
}

fn sm3_msg1_ref(a: [u32; 4], b: [u32; 4], c: [u32; 4]) -> [u32; 4] {
    let mut w = [0u32; 16];
    w[0..4].copy_from_slice(&c);
    w[7..11].copy_from_slice(&a);
    w[13..16].copy_from_slice(&b[..3]);
    let tmp0 = w[7] ^ w[0] ^ rol32(w[13], 15);
    let tmp1 = w[8] ^ w[1] ^ rol32(w[14], 15);
    let tmp2 = w[9] ^ w[2] ^ rol32(w[15], 15);
    let tmp3 = w[10] ^ w[3];
    [sm3_p1(tmp0), sm3_p1(tmp1), sm3_p1(tmp2), sm3_p1(tmp3)]
}

fn sm3_msg2_ref(a: [u32; 4], b: [u32; 4], c: [u32; 4]) -> [u32; 4] {
    let mut w = [0u32; 16];
    w[3..7].copy_from_slice(&b);
    w[10..14].copy_from_slice(&c);
    let mut out = [0u32; 4];
    for lane in 0..4 {
        out[lane] = rol32(w[3 + lane], 7) ^ w[10 + lane] ^ a[lane];
    }
    out[3] ^= rol32(out[0], 6) ^ rol32(out[0], 15) ^ rol32(out[0], 30);
    out
}

fn sm3_rnds2_ref(a: [u32; 4], b: [u32; 4], c: [u32; 4], round: u32) -> [u32; 4] {
    let mut a0 = b[3];
    let mut b0 = b[2];
    let mut c0 = a[3];
    let mut d0 = a[2];
    let mut e0 = b[1];
    let mut f0 = b[0];
    let mut g0 = a[1];
    let mut h0 = a[0];
    let w = [c[0], c[1], c[2], c[3]];

    c0 = rol32(c0, 9);
    d0 = rol32(d0, 9);
    g0 = rol32(g0, 19);
    h0 = rol32(h0, 19);
    let masked_round = round & 0x3e;
    let mut constant = if masked_round < 16 {
        0x79cc_4519
    } else {
        0x7a87_9d8a
    };
    constant = rol32(constant, masked_round);

    for lane in 0..2 {
        let s1 = rol32(rol32(a0, 12).wrapping_add(e0).wrapping_add(constant), 7);
        let s2 = s1 ^ rol32(a0, 12);
        let wi = w[lane];
        let wi4 = w[lane + 2];
        let t1 = sm3_ff(a0, b0, c0, masked_round)
            .wrapping_add(d0)
            .wrapping_add(s2)
            .wrapping_add(wi ^ wi4);
        let t2 = sm3_gg(e0, f0, g0, masked_round)
            .wrapping_add(h0)
            .wrapping_add(s1)
            .wrapping_add(wi);
        d0 = c0;
        c0 = rol32(b0, 9);
        b0 = a0;
        a0 = t1;
        h0 = g0;
        g0 = rol32(f0, 19);
        f0 = e0;
        e0 = sm3_p0(t2);
        constant = rol32(constant, 1);
    }

    [f0, e0, b0, a0]
}

const SM4_SBOX: [u8; 256] = [
    0xd6, 0x90, 0xe9, 0xfe, 0xcc, 0xe1, 0x3d, 0xb7, 0x16, 0xb6, 0x14, 0xc2, 0x28, 0xfb, 0x2c, 0x05,
    0x2b, 0x67, 0x9a, 0x76, 0x2a, 0xbe, 0x04, 0xc3, 0xaa, 0x44, 0x13, 0x26, 0x49, 0x86, 0x06, 0x99,
    0x9c, 0x42, 0x50, 0xf4, 0x91, 0xef, 0x98, 0x7a, 0x33, 0x54, 0x0b, 0x43, 0xed, 0xcf, 0xac, 0x62,
    0xe4, 0xb3, 0x1c, 0xa9, 0xc9, 0x08, 0xe8, 0x95, 0x80, 0xdf, 0x94, 0xfa, 0x75, 0x8f, 0x3f, 0xa6,
    0x47, 0x07, 0xa7, 0xfc, 0xf3, 0x73, 0x17, 0xba, 0x83, 0x59, 0x3c, 0x19, 0xe6, 0x85, 0x4f, 0xa8,
    0x68, 0x6b, 0x81, 0xb2, 0x71, 0x64, 0xda, 0x8b, 0xf8, 0xeb, 0x0f, 0x4b, 0x70, 0x56, 0x9d, 0x35,
    0x1e, 0x24, 0x0e, 0x5e, 0x63, 0x58, 0xd1, 0xa2, 0x25, 0x22, 0x7c, 0x3b, 0x01, 0x21, 0x78, 0x87,
    0xd4, 0x00, 0x46, 0x57, 0x9f, 0xd3, 0x27, 0x52, 0x4c, 0x36, 0x02, 0xe7, 0xa0, 0xc4, 0xc8, 0x9e,
    0xea, 0xbf, 0x8a, 0xd2, 0x40, 0xc7, 0x38, 0xb5, 0xa3, 0xf7, 0xf2, 0xce, 0xf9, 0x61, 0x15, 0xa1,
    0xe0, 0xae, 0x5d, 0xa4, 0x9b, 0x34, 0x1a, 0x55, 0xad, 0x93, 0x32, 0x30, 0xf5, 0x8c, 0xb1, 0xe3,
    0x1d, 0xf6, 0xe2, 0x2e, 0x82, 0x66, 0xca, 0x60, 0xc0, 0x29, 0x23, 0xab, 0x0d, 0x53, 0x4e, 0x6f,
    0xd5, 0xdb, 0x37, 0x45, 0xde, 0xfd, 0x8e, 0x2f, 0x03, 0xff, 0x6a, 0x72, 0x6d, 0x6c, 0x5b, 0x51,
    0x8d, 0x1b, 0xaf, 0x92, 0xbb, 0xdd, 0xbc, 0x7f, 0x11, 0xd9, 0x5c, 0x41, 0x1f, 0x10, 0x5a, 0xd8,
    0x0a, 0xc1, 0x31, 0x88, 0xa5, 0xcd, 0x7b, 0xbd, 0x2d, 0x74, 0xd0, 0x12, 0xb8, 0xe5, 0xb4, 0xb0,
    0x89, 0x69, 0x97, 0x4a, 0x0c, 0x96, 0x77, 0x7e, 0x65, 0xb9, 0xf1, 0x09, 0xc5, 0x6e, 0xc6, 0x84,
    0x18, 0xf0, 0x7d, 0xec, 0x3a, 0xdc, 0x4d, 0x20, 0x79, 0xee, 0x5f, 0x3e, 0xd7, 0xcb, 0x39, 0x48,
];

fn sm4_ck(round: usize) -> u32 {
    let mut value = 0u32;
    for lane in 0..4 {
        value = (value << 8) | (((round * 4 + lane) * 7 % 256) as u32);
    }
    value
}

fn sm4_lower_t(value: u32) -> u32 {
    let bytes = value.to_le_bytes();
    u32::from_le_bytes([
        SM4_SBOX[bytes[0] as usize],
        SM4_SBOX[bytes[1] as usize],
        SM4_SBOX[bytes[2] as usize],
        SM4_SBOX[bytes[3] as usize],
    ])
}

fn sm4_t_key(value: u32) -> u32 {
    let value = sm4_lower_t(value);
    value ^ rol32(value, 13) ^ rol32(value, 23)
}

fn sm4_t_rnd(value: u32) -> u32 {
    let value = sm4_lower_t(value);
    value ^ rol32(value, 2) ^ rol32(value, 10) ^ rol32(value, 18) ^ rol32(value, 24)
}

fn sm4_key4_ref(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    let f = |x0, x1, x2, x3, key| x0 ^ sm4_t_key(x1 ^ x2 ^ x3 ^ key);
    let c0 = f(b[0], b[1], b[2], b[3], a[0]);
    let c1 = f(b[1], b[2], b[3], c0, a[1]);
    let c2 = f(b[2], b[3], c0, c1, a[2]);
    let c3 = f(b[3], c0, c1, c2, a[3]);
    [c0, c1, c2, c3]
}

fn sm4_rnds4_ref(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    let f = |x0, x1, x2, x3, key| x0 ^ sm4_t_rnd(x1 ^ x2 ^ x3 ^ key);
    let c0 = f(b[0], b[1], b[2], b[3], a[0]);
    let c1 = f(b[1], b[2], b[3], c0, a[1]);
    let c2 = f(b[2], b[3], c0, c1, a[2]);
    let c3 = f(b[3], c0, c1, c2, a[3]);
    [c0, c1, c2, c3]
}

#[test]
fn sm4_instruction_reference() {
    if !is_x86_feature_detected!("sm4") {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    use rust_isa::x86::crypto::sm4::{vsm4key4, vsm4rnds4};

    let a = [0x0007_0e15, 0x1c23_2a31, 0x383f_464d, 0x545b_6269];
    let b = [0x0123_4567, 0x89ab_cdef, 0xfedc_ba98, 0x7654_3210];
    assert_eq!(
        v128_to_u32x4(unsafe { vsm4key4(u32x4_to_v128(a), u32x4_to_v128(b)) }),
        sm4_key4_ref(a, b)
    );
    assert_eq!(
        v128_to_u32x4(unsafe { vsm4rnds4(u32x4_to_v128(a), u32x4_to_v128(b)) }),
        sm4_rnds4_ref(a, b)
    );
}

#[test]
fn sm4_reference_key_schedule_values() {
    let k = [
        0x0123_4567 ^ 0xa3b1_bac6,
        0x89ab_cdef ^ 0x56aa_3350,
        0xfedc_ba98 ^ 0x677d_9197,
        0x7654_3210 ^ 0xb270_22dc,
    ];
    let ck = [sm4_ck(0), sm4_ck(1), sm4_ck(2), sm4_ck(3)];
    let rk = [0xf121_86f9, 0x4166_2b61, 0x5a6a_b19a, 0x7ba9_2077];
    assert_eq!(sm4_key4_ref(ck, k), rk);
    assert_eq!(
        sm4_rnds4_ref(rk, [0x0123_4567, 0x89ab_cdef, 0xfedc_ba98, 0x7654_3210],),
        [0x27fa_d345, 0xa18b_4cb2, 0x11c1_e22a, 0xcc13_e2ee]
    );
}
