use rust_isa::x86::simd::{Mask1, Mask2, Mask4, Mask8, Mask16, Mask32, Mask64};

#[test]
fn mask1_is_one_lane() {
    assert_eq!(Mask1::LANES, 1);
    assert_eq!(Mask1::ALL.bits(), 0b1);
    assert_eq!(Mask1::from_bits(0xff).bits(), 0b1);
    assert_eq!(Mask1::from_bits_checked(0b10), None);
    assert_eq!(Mask1::default(), Mask1::NONE);
    assert_eq!(!Mask1::NONE, Mask1::ALL);
}

#[test]
fn mask2_and_mask4_truncate_high_bits() {
    assert_eq!(Mask2::from_bits(0xff).bits(), 0b11);
    assert_eq!(Mask4::from_bits(0xff).bits(), 0b1111);
    assert_eq!(Mask4::from_bits_checked(0x10), None);
    assert_eq!((!Mask4::from_bits(0b1010)).bits(), 0b0101);
}

#[test]
fn mask8_and_larger_store_full_width() {
    assert_eq!(Mask8::ALL.bits(), 0xff);
    assert_eq!(Mask16::ALL.bits(), 0xffff);
    assert_eq!(Mask32::ALL.bits(), 0xffff_ffff);
    assert_eq!(Mask64::ALL.bits(), 0xffff_ffff_ffff_ffff);
}

#[test]
fn mask_bit_operations_stay_in_range() {
    let a = Mask4::from_bits(0b1100);
    let b = Mask4::from_bits(0b1010);
    assert_eq!((a & b).bits(), 0b1000);
    assert_eq!((a | b).bits(), 0b1110);
    assert_eq!((a ^ b).bits(), 0b0110);
    assert_eq!((!a).bits(), 0b0011);
}
