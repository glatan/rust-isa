use rust_isa::wasm32::wide_arithmetic::{add128, mul_wide_s, mul_wide_u, sub128};

#[test]
fn add_and_subtract_128_bit_values() {
    assert_eq!(unsafe { add128((u64::MAX, 0), (1, 0)) }, (0, 1));
    assert_eq!(unsafe { add128((1, 1), (u64::MAX, 0)) }, (0, 2));
    assert_eq!(unsafe { sub128((0, 1), (1, 0)) }, (u64::MAX, 0));
    assert_eq!(unsafe { sub128((0, 2), (1, 1)) }, (u64::MAX, 0));
}

#[test]
fn wide_unsigned_multiply() {
    assert_eq!(unsafe { mul_wide_u(u64::MAX, 2) }, (u64::MAX - 1, 1));
    assert_eq!(unsafe { mul_wide_u(1 << 32, 1 << 32) }, (0, 1));
}

#[test]
fn wide_signed_multiply() {
    assert_eq!(unsafe { mul_wide_s(-1, 1) }, (-1, -1));
    assert_eq!(unsafe { mul_wide_s(-2, 3) }, (-6, -1));
    assert_eq!(unsafe { mul_wide_s(i64::MIN, -1) }, (i64::MIN, 0));
}
