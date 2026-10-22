use rust_isa::x86::simd::avx::{
    vcmppd, vcmpps, vcmpsd, vcmpss, vcomisd, vcomiss, vdppd, vdpps, vptest, vtestpd, vtestps,
    vucomisd, vucomiss,
};
use rust_isa::x86::simd::avx2::V256;

use super::support::*;

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
