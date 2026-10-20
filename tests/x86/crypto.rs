use rust_isa::x86::crypto::aes::{aesdec, aesdeclast, aesenc, aesenclast, aesimc};
use rust_isa::x86::crypto::gfni::gf2p8mulb;
use rust_isa::x86::crypto::pclmulqdq::pclmulqdq;
use rust_isa::x86::crypto::sha::{sha1msg1, sha1msg2, sha1nexte, sha256msg1, sha256msg2};
use rust_isa::x86::crypto::vaes::{vaesenc, vaesenc_v128};
use rust_isa::x86::crypto::vpclmulqdq::{vpclmulqdq_v256, vpclmulqdq_v512};
use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::fma::vfmadd132ps;
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

fn vpclmul_enabled() -> bool {
    is_x86_feature_detected!("vpclmulqdq")
        && is_x86_feature_detected!("avx512f")
        && is_x86_feature_detected!("avx512vl")
}

fn fma_enabled() -> bool {
    is_x86_feature_detected!("fma") && is_x86_feature_detected!("avx2")
}

fn quad(value: u64) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[0..8].copy_from_slice(&value.to_le_bytes());
    V128::from_bytes(bytes)
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
fn fma() {
    if !fma_enabled() {
        return;
    }
    let mut a = [0u8; 32];
    let mut b = [0u8; 32];
    let mut c = [0u8; 32];
    for lane in 0..8 {
        a[lane * 4..lane * 4 + 4].copy_from_slice(&2.0f32.to_le_bytes());
        b[lane * 4..lane * 4 + 4].copy_from_slice(&3.0f32.to_le_bytes());
        c[lane * 4..lane * 4 + 4].copy_from_slice(&4.0f32.to_le_bytes());
    }
    let out = unsafe {
        vfmadd132ps(
            V256::from_bytes(a),
            V256::from_bytes(b),
            V256::from_bytes(c),
        )
    };
    for lane in 0..8 {
        let bytes = out.to_bytes();
        assert_eq!(
            f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap()),
            10.0
        );
    }
}
