use rust_isa::x86::simd::avx512bw::*;
use rust_isa::x86::simd::avx512f::*;

use super::support::*;

#[test]
fn bw_byte_arithmetic() {
    if !bw_enabled() {
        return;
    }
    let bytes = V512::from_bytes([250; 64]);
    assert_eq!(
        unsafe { vpaddb(bytes, V512::from_bytes([10; 64])) }.to_bytes(),
        [4; 64]
    );
}

#[test]
fn bw_word_arithmetic() {
    if !bw_enabled() {
        return;
    }
    assert_eq!(
        unsafe { vpsubb(V512::from_bytes([10; 64]), V512::from_bytes([3; 64])) }.to_bytes(),
        [7; 64]
    );
    assert_eq!(
        i16_lanes(unsafe { vpmullw(i16x32([300; 32]), i16x32([3; 32])) }),
        [900; 32]
    );
}

#[test]
fn bw_extended_operations() {
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
        i8_lanes(unsafe { vpaddusb(i8x64([-6; 64]), i8x64([10; 64])) })[0],
        -1
    );
    assert_eq!(
        i8_lanes(unsafe { vpaddsb(i8x64([127; 64]), i8x64([1; 64])) })[0],
        127
    );
    assert_eq!(
        i8_lanes(unsafe { vpaddsb(i8x64([-128; 64]), i8x64([-1; 64])) })[0],
        -128
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
        i16_lanes(unsafe { vpmulhw(i16x32([0x7fff; 32]), i16x32([0x7fff; 32])) })[0],
        0x3fff
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
