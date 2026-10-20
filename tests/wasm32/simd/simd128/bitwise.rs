use rust_isa::wasm32::simd::simd128::*;

#[test]
fn bitwise_logic_and_any_true() {
    let ones = V128::from_bytes([0xff; 16]);
    let low_nibble = V128::from_bytes([0x0f; 16]);
    assert_eq!(v128_and(ones, low_nibble).to_bytes(), [0x0f; 16]);
    assert_eq!(
        v128_or(low_nibble, V128::from_bytes([0xf0; 16])).to_bytes(),
        [0xff; 16]
    );
    assert_eq!(v128_xor(ones, low_nibble).to_bytes(), [0xf0; 16]);
    assert_eq!(v128_not(low_nibble).to_bytes(), [0xf0; 16]);
    assert_eq!(v128_andnot(ones, low_nibble).to_bytes(), [0xf0; 16]);
    assert_eq!(
        v128_bitselect(ones, V128::from_bytes([0; 16]), low_nibble).to_bytes(),
        [0x0f; 16]
    );
    assert_eq!(v128_any_true(V128::from_bytes([0; 16])), 0);
    assert_eq!(
        v128_any_true(V128::from_bytes([
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ])),
        1
    );
}
