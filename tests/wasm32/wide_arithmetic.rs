use rust_isa::wasm32::wide_arithmetic::{i64_add128, i64_mul_wide_s, i64_mul_wide_u, i64_sub128};

#[test]
fn add_and_subtract_128_bit_values() {
    assert_eq!(unsafe { i64_add128((u64::MAX, 0), (1, 0)) }, (0, 1));
    assert_eq!(unsafe { i64_add128((1, 1), (u64::MAX, 0)) }, (0, 2));
    assert_eq!(unsafe { i64_add128((u64::MAX, u64::MAX), (1, 0)) }, (0, 0));
    assert_eq!(unsafe { i64_sub128((0, 1), (1, 0)) }, (u64::MAX, 0));
    assert_eq!(unsafe { i64_sub128((0, 2), (1, 1)) }, (u64::MAX, 0));
    assert_eq!(unsafe { i64_sub128((0, 0), (0, 1)) }, (0, u64::MAX));
}

#[test]
fn wide_unsigned_multiply() {
    assert_eq!(unsafe { i64_mul_wide_u(u64::MAX, 2) }, (u64::MAX - 1, 1));
    assert_eq!(unsafe { i64_mul_wide_u(1 << 32, 1 << 32) }, (0, 1));
    assert_eq!(
        unsafe { i64_mul_wide_u(u64::MAX, u64::MAX) },
        (1, u64::MAX - 1)
    );
}

#[test]
fn wide_signed_multiply() {
    assert_eq!(unsafe { i64_mul_wide_s(-1, 1) }, (-1, -1));
    assert_eq!(unsafe { i64_mul_wide_s(-2, 3) }, (-6, -1));
    assert_eq!(unsafe { i64_mul_wide_s(i64::MIN, -1) }, (i64::MIN, 0));
    assert_eq!(unsafe { i64_mul_wide_s(i64::MIN, i64::MIN) }, (0, 1 << 62));
    assert_eq!(unsafe { i64_mul_wide_s(-1, -1) }, (1, 0));
}
