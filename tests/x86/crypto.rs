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
        return;
    }
    let zero = V128::from_bytes([0; 16]);
    assert_eq!(unsafe { aesenc(zero, zero) }.to_bytes(), [0x63; 16]);
    assert_eq!(unsafe { aesenclast(zero, zero) }.to_bytes(), [0x63; 16]);
    assert_eq!(unsafe { aesdec(zero, zero) }.to_bytes(), [0x52; 16]);
    assert_eq!(unsafe { aesdeclast(zero, zero) }.to_bytes(), [0x52; 16]);
    assert_eq!(unsafe { aesimc(zero) }.to_bytes(), [0; 16]);

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
fn gfni_multiply() {
    if !gfni_enabled() {
        return;
    }
    let a = V128::from_bytes([2; 16]);
    let b = V128::from_bytes([0x80; 16]);
    assert_eq!(unsafe { gf2p8mulb(a, b) }.to_bytes(), [0x1b; 16]);
}

#[test]
fn carry_less_multiply() {
    if !clmul_enabled() {
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
}

#[test]
fn aes_key_schedule() {
    if !aes_enabled() {
        return;
    }
    assert_eq!(
        unsafe { aeskeygenassist::<1>(sequential(0)) }.to_bytes(),
        [
            0xf2, 0x6b, 0x6f, 0xc5, 0x6a, 0x6f, 0xc5, 0xf2, 0xfe, 0xd7, 0xab, 0x76, 0xd6, 0xab,
            0x76, 0xfe
        ]
    );
}

#[test]
fn vaes_sub_bytes_and_last_rounds() {
    if !vaes_enabled() {
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
        return;
    }
    use rust_isa::x86::crypto::sha512::{vsha512msg1, vsha512msg2, vsha512rnds2};

    let one = V256::from_bytes([1; 32]);
    let zero = V256::from_bytes([0; 32]);
    let k = V128::from_bytes([0; 16]);
    assert_ne!(unsafe { vsha512msg1(one, k) }.to_bytes(), zero.to_bytes());
    assert_ne!(unsafe { vsha512msg2(one, one) }.to_bytes(), zero.to_bytes());
    assert_ne!(
        unsafe { vsha512rnds2(one, one, k) }.to_bytes(),
        zero.to_bytes()
    );
}

#[test]
fn sm3_message_expansion_and_rounds() {
    if !is_x86_feature_detected!("sm3") {
        return;
    }
    use rust_isa::x86::crypto::sm3::{vsm3msg1, vsm3msg2, vsm3rnds2};

    let one = V128::from_bytes([1; 16]);
    let zero = V128::from_bytes([0; 16]);
    assert_ne!(
        unsafe { vsm3msg1(one, one, one) }.to_bytes(),
        zero.to_bytes()
    );
    assert_ne!(
        unsafe { vsm3msg2(one, one, one) }.to_bytes(),
        zero.to_bytes()
    );
    assert_ne!(
        unsafe { vsm3rnds2::<0>(one, one, one) }.to_bytes(),
        zero.to_bytes()
    );
    assert_ne!(
        unsafe { vsm3rnds2::<3>(one, one, one) }.to_bytes(),
        zero.to_bytes()
    );
}

#[test]
fn sm4_key_schedule_and_rounds() {
    if !is_x86_feature_detected!("sm4") {
        return;
    }
    use rust_isa::x86::crypto::sm4::{vsm4key4, vsm4rnds4};

    let one = V128::from_bytes([1; 16]);
    let zero = V128::from_bytes([0; 16]);
    assert_ne!(unsafe { vsm4key4(one, one) }.to_bytes(), zero.to_bytes());
    assert_ne!(unsafe { vsm4rnds4(one, one) }.to_bytes(), zero.to_bytes());
}
