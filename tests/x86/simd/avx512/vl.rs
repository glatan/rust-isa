use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512vl::*;
use rust_isa::x86::simd::sse2::V128;
use rust_isa::x86::simd::{Mask4, Mask8, Mask16};

use super::support::*;

#[test]
fn vl_mullo() {
    if !vl_dq_enabled() {
        return;
    }
    let a = V128::from_bytes([3, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0]);
    let b = V128::from_bytes([7, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0]);
    let c = unsafe { vpmullq_v128(a, b) };
    assert_eq!(&c.to_bytes()[0..8], &21i64.to_le_bytes());
    assert_eq!(&c.to_bytes()[8..16], &55i64.to_le_bytes());

    let mut bytes = [0u8; 32];
    for index in 0..4 {
        bytes[index * 8..index * 8 + 8].copy_from_slice(&(index as i64 + 2).to_le_bytes());
    }
    let a = V256::from_bytes(bytes);
    for index in 0..4 {
        bytes[index * 8..index * 8 + 8].copy_from_slice(&(index as i64 + 3).to_le_bytes());
    }
    let b = V256::from_bytes(bytes);
    let c = unsafe { vpmullq_v256(a, b) };
    let out = c.to_bytes();
    for index in 0..4 {
        assert_eq!(
            i64::from_le_bytes(out[index * 8..index * 8 + 8].try_into().unwrap()),
            (index as i64 + 2) * (index as i64 + 3)
        );
    }
}

#[test]
fn vl_narrow_forms() {
    if !vl_enabled() {
        return;
    }
    if bw_enabled() {
        let a = i8x16(core::array::from_fn(|lane| lane as i8));
        let b = i8x16([1; 16]);
        let sum = unsafe { vpaddb_v128(a, b) };
        assert_eq!(sum.to_bytes()[..4], [1, 2, 3, 4]);
        assert_eq!(
            unsafe { vptestmb_v128(i8x16([-1; 16]), i8x16([-1; 16])).bits() },
            0xffff
        );
    }
    assert_eq!(
        unsafe { vpcmpd_v256::<0>(i32x8([1; 8]), i32x8([1; 8])).bits() },
        0xff
    );

    let source = V256::from_bytes(core::array::from_fn(|index| index as u8));
    let compressed = unsafe { vcompressps_v256(source, Mask8::from_bits(0b0101)) };
    assert_eq!(&compressed.to_bytes()[..8], &[0, 1, 2, 3, 8, 9, 10, 11]);

    if vbmi_enabled() {
        let indices = V256::from_bytes(core::array::from_fn(|index| 31 - index as u8));
        assert_eq!(
            unsafe { vpermb_v256(source, indices) }.to_bytes(),
            core::array::from_fn::<u8, 32, _>(|index| 31 - index as u8)
        );
    }
    if vbmi2_enabled() {
        assert_eq!(
            unsafe {
                vpcompressb_v128(
                    V128::from_bytes(core::array::from_fn(|index| index as u8)),
                    Mask16::from_bits(0b0101),
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
            unsafe { vpshufbitqmb_v128(i8x16([-1; 16]), i8x16([0; 16])).bits() },
            0xffff
        );
    }
    if is_x86_feature_detected!("avx512vp2intersect") {
        let values = i32x4([1, 2, 3, 4]);
        assert_eq!(
            unsafe { vp2intersectd_v128(values, values) },
            (Mask4::from_bits(0b1111), Mask4::from_bits(0b1111))
        );
    }
    let gathered = unsafe {
        vgatherdps_v256(
            f32x8([0.0; 8]),
            [1.0f32, 2.0, 3.0, 4.0].as_ptr().cast(),
            i32x8([0, 1, 2, 3, 0, 0, 0, 0]),
            Mask8::from_bits(0x0f),
        )
    };
    assert_eq!(f32x8_lanes(gathered)[..4], [1.0, 2.0, 3.0, 4.0]);
}
