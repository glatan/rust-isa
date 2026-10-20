use rust_isa::x86::simd::sse2::{paddb, paddd, pand, pcmpeqb, pmullw, v128_load, v128_store, V128};

#[test]
fn paddd_adds_each_i32_lane() {
    let a = V128::from_bytes([
        1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0,
    ]);
    let b = V128::from_bytes([
        10, 0, 0, 0, 20, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0,
    ]);
    let c = paddd(a, b);
    assert_eq!(
        c.to_bytes(),
        [
            11, 0, 0, 0, 22, 0, 0, 0, 33, 0, 0, 0, 44, 0, 0, 0,
        ]
    );
}

#[test]
fn load_store_roundtrip() {
    let source = [
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ];
    let value = unsafe { v128_load(source.as_ptr()) };
    assert_eq!(value.to_bytes(), source);

    let mut output = [0u8; 16];
    unsafe {
        v128_store(output.as_mut_ptr(), value);
    }
    assert_eq!(output, source);
}

#[test]
fn paddb_wraps_bytes() {
    let a = V128::from_bytes([250; 16]);
    let b = V128::from_bytes([10; 16]);
    assert_eq!(paddb(a, b).to_bytes(), [4; 16]);
}

#[test]
fn pand_and_pcmpeqb() {
    let a = V128::from_bytes([0b1010_1010; 16]);
    let b = V128::from_bytes([0b1100_1100; 16]);
    assert_eq!(pand(a, b).to_bytes(), [0b1000_1000; 16]);
    assert_eq!(pcmpeqb(a, a).to_bytes(), [0xff; 16]);
    assert_eq!(pcmpeqb(a, b).to_bytes(), [0; 16]);
}

#[test]
fn pmullw_keeps_low_16_bits() {
    let a = V128::from_bytes([2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0]);
    let b = V128::from_bytes([4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 11, 0]);
    assert_eq!(
        pmullw(a, b).to_bytes(),
        [8, 0, 15, 0, 24, 0, 35, 0, 48, 0, 63, 0, 80, 0, 99, 0]
    );
}
